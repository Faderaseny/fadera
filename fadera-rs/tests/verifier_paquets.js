// Vérifie le gestionnaire de paquets (« fadera paquet ») et l'accès réseau de la ligne de commande, sans internet :
// les sources sont des dossiers locaux et un petit serveur HTTP lancé ici, sur 127.0.0.1.
//   node fadera-rs/tests/verifier_paquets.js chemin/vers/fadera[.exe]
const fs = require("fs"), os = require("os"), path = require("path"), http = require("http"), cp = require("child_process");
const bin = process.argv[2] && path.resolve(process.argv[2]);
if (!bin) { console.error("usage : node verifier_paquets.js chemin/vers/fadera"); process.exit(2); }
const racine = fs.mkdtempSync(path.join(os.tmpdir(), "fadera_paquets_"));
const env = { ...process.env };
for (const k of ["HTTP_PROXY", "http_proxy", "HTTPS_PROXY", "https_proxy", "ALL_PROXY", "all_proxy"]) delete env[k];
let n = 0, ko = 0;
const verifier = (nom, ok, detail) => { n++; if (!ok) { ko++; console.log("  ✗ " + nom + (detail ? " : " + detail : "")); } };

function lancer(cwd, args) {
  return new Promise(res => {
    const p = cp.spawn(bin, args, { cwd, env });
    let out = "", err = "";
    p.stdout.on("data", d => out += d); p.stderr.on("data", d => err += d);
    p.on("close", code => res({ code, out, err }));
  });
}
function ecrire(dir, fichiers) {
  for (const [f, c] of Object.entries(fichiers)) {
    fs.mkdirSync(path.dirname(path.join(dir, f)), { recursive: true });
    fs.writeFileSync(path.join(dir, f), c);
  }
}
const manifeste = (o) => JSON.stringify({ nom: "outils", version: "1.0.0", principal: "principal.fa", fichiers: ["principal.fa", "aide.fa"], ...o });
const BON = {
  "fadera-paquet.json": manifeste({}),
  "principal.fa": 'importer "aide" comme a\nfonction doubler(x)\n  renvoyer a.fois2(x)\nfin\n',
  "aide.fa": "fonction fois2(x)\n  renvoyer x * 2\nfin\n",
};

(async () => {
  // 1. création, test, installation locale, utilisation (avec un import interne au paquet)
  const a = path.join(racine, "a"); fs.mkdirSync(a);
  let r = await lancer(a, ["paquet", "creer", "salutations"]);
  verifier("creer : réussit", r.code === 0, r.err);
  r = await lancer(a, ["tester", "salutations/tests.fa"]);
  verifier("creer : le squelette passe ses propres tests", r.code === 0 && /1 réussi/.test(r.out), r.out + r.err);
  r = await lancer(a, ["paquet", "creer", "salutations"]);
  verifier("creer : refuse d'écraser", r.code === 1);
  r = await lancer(a, ["paquet", "creer", "Mauvais Nom"]);
  verifier("creer : refuse un nom invalide", r.code === 1);

  const projet = path.join(racine, "projet"); fs.mkdirSync(projet);
  const src = path.join(racine, "src_outils"); ecrire(src, BON);
  r = await lancer(projet, ["paquet", "installer", src]);
  verifier("installer local : réussit", r.code === 0 && /outils 1\.0\.0 installé/.test(r.out), r.out + r.err);
  ecrire(projet, { "app.fa": 'importer "paquets/outils/principal" comme o\nafficher o.doubler(21)\n' });
  r = await lancer(projet, ["run", "app.fa"]);
  verifier("utilisation par importer (avec import interne)", r.out.trim() === "42", r.out + r.err);
  const dep = JSON.parse(fs.readFileSync(path.join(projet, "fadera.json"), "utf8"));
  verifier("fadera.json enregistre la dépendance", dep.dependances && dep.dependances.outils === src);
  r = await lancer(projet, ["paquet", "lister"]);
  verifier("lister", r.out.trim() === "outils 1.0.0", r.out);

  // 2. réinstaller depuis fadera.json, et remplacer proprement une ancienne version
  fs.rmSync(path.join(projet, "paquets"), { recursive: true });
  r = await lancer(projet, ["paquet", "installer"]);
  verifier("installer sans argument : refait tout depuis fadera.json", r.code === 0 && fs.existsSync(path.join(projet, "paquets/outils/principal.fa")), r.err);
  ecrire(src, { "fadera-paquet.json": manifeste({ version: "2.0.0", fichiers: ["principal.fa"] }), "principal.fa": "fonction doubler(x)\n  renvoyer x * 3\nfin\n" });
  r = await lancer(projet, ["paquet", "installer", src]);
  verifier("nouvelle version : remplace l'ancienne", r.code === 0 && !fs.existsSync(path.join(projet, "paquets/outils/aide.fa")), r.err);
  r = await lancer(projet, ["paquet", "retirer", "outils"]);
  verifier("retirer", r.code === 0 && !fs.existsSync(path.join(projet, "paquets/outils")) && Object.keys(JSON.parse(fs.readFileSync(path.join(projet, "fadera.json"), "utf8")).dependances).length === 0, r.err);
  r = await lancer(projet, ["paquet", "retirer", "outils"]);
  verifier("retirer un paquet absent : erreur claire", r.code === 1);

  // 3. refus : rien ne doit être écrit quand le paquet est piégé ou mal formé
  const pieges = {
    "chemin avec ..": { "fadera-paquet.json": manifeste({ fichiers: ["principal.fa", "../evil.fa"] }), "principal.fa": "x", "../evil.fa": "x" },
    "chemin absolu": { "fadera-paquet.json": manifeste({ fichiers: ["principal.fa", "/etc/passwd.fa"] }), "principal.fa": "x" },
    "lecteur Windows": { "fadera-paquet.json": manifeste({ fichiers: ["principal.fa", "C:evil.fa"] }), "principal.fa": "x" },
    "antislash": { "fadera-paquet.json": manifeste({ fichiers: ["principal.fa", "a\\b.fa"] }), "principal.fa": "x" },
    "extension interdite": { "fadera-paquet.json": manifeste({ fichiers: ["principal.fa", "virus.exe"] }), "principal.fa": "x", "virus.exe": "x" },
    "nom invalide": { "fadera-paquet.json": manifeste({ nom: "../dehors" }), "principal.fa": "x", "aide.fa": "x" },
    "nom avec majuscule": { "fadera-paquet.json": manifeste({ nom: "Outils" }), "principal.fa": "x", "aide.fa": "x" },
    "principal absent de la liste": { "fadera-paquet.json": manifeste({ principal: "autre.fa" }), "principal.fa": "x", "aide.fa": "x" },
    "fichier listé mais absent": { "fadera-paquet.json": manifeste({}), "principal.fa": "x" },
    "JSON invalide": { "fadera-paquet.json": "{ pas du json", "principal.fa": "x" },
    "liste vide": { "fadera-paquet.json": manifeste({ fichiers: [] }) },
  };
  for (const [nom, contenu] of Object.entries(pieges)) {
    const d = path.join(racine, "piege_" + nom.replace(/\W+/g, "_"));
    const p = path.join(racine, "p_" + nom.replace(/\W+/g, "_")); fs.mkdirSync(p);
    ecrire(d, contenu);
    const r = await lancer(p, ["paquet", "installer", d]);
    verifier("refus : " + nom, r.code === 1 && !fs.existsSync(path.join(p, "paquets")) && !fs.existsSync(path.join(p, "fadera.json")), "code " + r.code + " " + r.err);
  }

  // 4. source web (serveur local) : installation, 404, et http_obtenir / http_json depuis un programme
  const serveur = http.createServer((req, rep) => {
    if (req.url.startsWith("/echo")) {
      let corps = ""; req.on("data", d => corps += d);
      req.on("end", () => { rep.writeHead(200, { "Content-Type": "application/json" }); rep.end(JSON.stringify({ methode: req.method, auth: req.headers["authorization"] || "", cle: req.headers["x-cle"] || "", type: req.headers["content-type"] || "", corps })); });
      return;
    }
    const f = path.join(racine, "web", decodeURIComponent(req.url.split("?")[0]));
    if (f.startsWith(path.join(racine, "web")) && fs.existsSync(f) && fs.statSync(f).isFile()) { rep.writeHead(200); rep.end(fs.readFileSync(f)); }
    else { rep.writeHead(404); rep.end("introuvable"); }
  });
  await new Promise(ok => serveur.listen(0, "127.0.0.1", ok));
  const base = "http://127.0.0.1:" + serveur.address().port;
  ecrire(path.join(racine, "web", "outils"), BON);
  ecrire(path.join(racine, "web"), { "donnee.json": '{"x": 7}', "texte.txt": "bonjour réseau" });
  const w = path.join(racine, "w"); fs.mkdirSync(w);
  r = await lancer(w, ["paquet", "installer", base + "/outils"]);
  verifier("installer depuis une adresse web", r.code === 0 && fs.existsSync(path.join(w, "paquets/outils/aide.fa")) && /confiance/.test(r.out), r.out + r.err);
  r = await lancer(w, ["paquet", "installer", base + "/inexistant"]);
  verifier("source web absente (404) : erreur claire", r.code === 1 && /404/.test(r.err), r.err);
  ecrire(w, { "net.fa": `afficher http_obtenir("${base}/texte.txt")\nafficher lireCle(http_json("${base}/donnee.json"), "x")\nsoit rep = http_obtenir("${base}/rien")\nafficher rep\n` });
  r = await lancer(w, ["run", "net.fa"]);
  verifier("http_obtenir / http_json dans la ligne de commande", r.out.trim().split("\n").slice(0, 2).join("|") === "bonjour réseau|7", r.out + r.err);
  // en-têtes facultatifs (clé d'accès) : mêmes résultats avec le moteur Rust et le moteur JavaScript
  const prog = `soit h = { Authorization: "Bearer jeton", "X-Cle": "abc" }
soit a = http_json("${base}/echo", h)
afficher a.auth + "|" + a.cle + "|" + a.methode
soit b = lireCle(http_json("${base}/echo"), "auth")
afficher "[" + b + "]"
soit c = http_envoyer("${base}/echo", { x: 1 }, h)
afficher c.statut + " " + c.ok
afficher lireCle(json_lire(c.corps), "type") + "|" + lireCle(json_lire(c.corps), "corps")
essayer
  http_json("${base}/echo", 5)
attraper e
  afficher e
fin
`;
  ecrire(w, { "ent.fa": prog });
  r = await lancer(w, ["run", "ent.fa"]);
  const attendu = 'Bearer jeton|abc|GET\n[]\n200 vrai\napplication/json|{"x":1}\nhttp_json : les en-têtes doivent être une structure.';
  verifier("en-têtes http_json / http_obtenir / http_envoyer (Rust)", r.out.trim() === attendu, r.out + r.err);
  try {
    const html = fs.readFileSync(path.join(__dirname, "..", "..", "index.html"), "utf8");
    const a0 = html.indexOf("      const TYPE = {"), b0 = html.indexOf("      // ---------- MODALE / TOAST");
    const M = new Function("module", "require", "window", "globalThis", html.slice(a0, b0) + "\nreturn {executerProgramme,decrireErreur};")({ exports: {} }, require, undefined, globalThis);
    const sortie = [];
    await M.executerProgramme(prog, { afficher: t => sortie.push(t), saisir: () => "", lireFichier: () => { throw new Error("absent"); }, ecrireFichier: () => {} });
    verifier("en-têtes : le moteur JavaScript donne exactement la même sortie", sortie.join("\n") === attendu, sortie.join("\n"));
  } catch (e) { verifier("en-têtes : moteur JavaScript", false, String(e && e.message || e)); }
  serveur.close();
  r = await lancer(w, ["run", "net.fa"]);
  verifier("réseau coupé : message en français, pas de plantage", r.code !== 0 && /requête impossible/.test(r.err), r.err);

  fs.rmSync(racine, { recursive: true, force: true });
  console.log(ko ? `✗ ${ko} vérification(s) en échec sur ${n}.` : `✓ ${n} vérifications du gestionnaire de paquets et du réseau, toutes réussies.`);
  process.exit(ko ? 1 : 0);
})();

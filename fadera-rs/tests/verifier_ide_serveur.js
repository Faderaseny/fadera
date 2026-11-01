#!/usr/bin/env node
// Vérifie « fadera ide » : le petit serveur local qui sert l'éditeur et transmet le fichier double-cliqué.
//   usage : node verifier_ide_serveur.js <index.html> <fadera[.exe]>
const http = require("http"), fs = require("fs"), os = require("os"), path = require("path"), cp = require("child_process");
const [html, exe] = process.argv.slice(2);
if (!html || !exe) { console.error("usage : verifier_ide_serveur.js index.html fadera"); process.exit(2); }
const PORT = 4747;
let ok = 0, ko = 0;
const verifier = (nom, cond, detail) => { if (cond) ok++; else { ko++; console.log("  ✗ " + nom + (detail !== undefined ? "  → " + JSON.stringify(detail) : "")); } };
const requete = (methode, chemin, corps, hote = "127.0.0.1:" + PORT, plus = {}) =>
  new Promise((res) => {
    const r = http.request({ host: "127.0.0.1", port: PORT, method: methode, path: chemin, headers: { Host: hote, ...plus, ...(corps ? { "Content-Type": "application/json", "Content-Length": Buffer.byteLength(corps) } : {}) } }, (rep) => {
      const morceaux = [];
      rep.on("data", (c) => morceaux.push(c));
      rep.on("end", () => res({ code: rep.statusCode, texte: Buffer.concat(morceaux).toString("utf8") }));
    });
    r.on("error", () => res({ code: 0, texte: "" }));
    if (corps) r.write(corps);
    r.end();
  });
const attendre = (ms) => new Promise((r) => setTimeout(r, ms));

(async () => {
  const dossier = fs.mkdtempSync(path.join(os.tmpdir(), "fadera-ide-"));
  fs.mkdirSync(path.join(dossier, "ide"));
  fs.copyFileSync(html, path.join(dossier, "ide", "index.html"));
  const binaire = path.join(dossier, path.basename(exe));
  fs.copyFileSync(exe, binaire);
  fs.chmodSync(binaire, 0o755);
  const espaceDir = path.join(dossier, "espace");
  fs.writeFileSync(path.join(dossier, "x.fa"), 'afficher "élève — « salut »"\n');
  fs.writeFileSync(path.join(dossier, "y.fa"), 'afficher "deuxième"\n');
  if ((await requete("GET", "/__fadera")).code !== 0) { console.log("Le port " + PORT + " est déjà utilisé : arrêtez l'éditeur Fadéra puis relancez."); process.exit(1); }
  const serveur = cp.spawn(binaire, ["ide", "--sans-navigateur", "--discret", "x.fa"], { cwd: dossier, stdio: "ignore", env: { ...process.env, FADERA_ESPACE: espaceDir } });
  try {
    let pret = false;
    for (let i = 0; i < 50 && !pret; i++) { pret = (await requete("GET", "/__fadera")).texte === "fadera-ide"; if (!pret) await attendre(100); }
    verifier("le serveur répond « fadera-ide »", pret);
    if (!pret) throw new Error("serveur non démarré");

    const accueil = await requete("GET", "/");
    verifier("GET / renvoie l'éditeur", accueil.code === 200 && accueil.texte.includes("Fadéra"), accueil.code);

    const ouvert = await requete("GET", "/__ouvrir");
    let j = null; try { j = JSON.parse(ouvert.texte); } catch (e) {}
    verifier("le fichier double-cliqué est transmis (accents compris)", ouvert.code === 200 && j && j.nom === "x.fa" && j.texte === 'afficher "élève — « salut »"\n', ouvert.texte);
    verifier("il n'est transmis qu'une fois", (await requete("GET", "/__ouvrir")).code === 204);

    // un second « fadera ide fichier » réutilise le serveur déjà lancé
    const second = cp.spawnSync(binaire, ["ide", "--sans-navigateur", "y.fa"], { cwd: dossier, encoding: "utf8", timeout: 15000 });
    verifier("la seconde commande se termine sans erreur", second.status === 0, second.status);
    const recu = await requete("GET", "/__ouvrir");
    let r2 = null; try { r2 = JSON.parse(recu.texte); } catch (e) {}
    verifier("… et lui transmet son fichier", recu.code === 200 && r2 && r2.nom === "y.fa" && r2.texte.includes("deuxième"), recu.texte);

    // terminal PowerShell / cmd et dossier réel : réservés à la page d'éditeur (jeton secret)
    verifier("terminal sans jeton refusé", (await requete("POST", "/__terminal", '{"commande":"echo non","shell":"powershell"}')).code === 403);
    verifier("dossier réel sans jeton refusé", (await requete("GET", "/__espace")).code === 403);
    const jetonM = /name="fadera-jeton" content="([0-9a-f]{64})"/.exec(accueil.texte);
    verifier("la page d'éditeur reçoit un jeton de 64 caractères", !!jetonM, accueil.texte.slice(-200));
    const J = { "X-Fadera-Jeton": jetonM ? jetonM[1] : "" };
    const hote = "127.0.0.1:" + PORT;
    verifier("mauvais jeton refusé", (await requete("GET", "/__espace", null, hote, { "X-Fadera-Jeton": "0".repeat(64) })).code === 403);
    verifier("origine étrangère refusée même avec le bon jeton", (await requete("GET", "/__espace", null, hote, { ...J, Origin: "http://evil.example" })).code === 403);
    verifier("hôte étranger refusé même avec le bon jeton", (await requete("GET", "/__espace", null, "evil.example.com", J)).code === 403);
    const esp = await requete("GET", "/__espace", null, hote, J);
    verifier("dossier réel accessible avec le jeton", esp.code === 200 && JSON.parse(esp.texte).dossier === espaceDir, esp.texte);
    const ecr = (nom, texte) => requete("POST", "/__espace/ecrire", JSON.stringify({ nom, texte }), hote, J);
    verifier("écrire dans le dossier réel", (await ecr("essai.fa", 'afficher "salut"\n')).code === 200 && fs.readFileSync(path.join(espaceDir, "essai.fa"), "utf8") === 'afficher "salut"\n');
    verifier("lire depuis le dossier réel", JSON.parse((await requete("GET", "/__espace/fichier?nom=essai.fa", null, hote, J)).texte).texte === 'afficher "salut"\n');
    for (const mauvais of ["../evil.fa", "a/../../evil.fa", "C:/evil.fa", "..\\evil.fa", "/etc/x.fa", "virus.exe", ".cache/x.fa", "sans_extension"]) {
      verifier("nom refusé : " + mauvais, (await ecr(mauvais, "x")).code === 400);
    }
    verifier("rien n'a été écrit hors du dossier", !fs.existsSync(path.join(dossier, "evil.fa")) && !fs.existsSync(path.join(os.tmpdir(), "evil.fa")));
    for (const shell of ["powershell", "cmd"]) {
      const t = await requete("POST", "/__terminal", JSON.stringify({ commande: "echo bonjour_fadera", shell }), hote, J);
      let tj = null; try { tj = JSON.parse(t.texte); } catch (e) {}
      verifier("terminal " + shell + " : exécute une commande", t.code === 200 && tj && tj.code === 0 && tj.sortie.includes("bonjour_fadera"), t.texte);
    }
    const tf = await requete("POST", "/__terminal", JSON.stringify({ commande: "fadera run essai.fa", shell: "powershell" }), hote, J);
    let tfj = null; try { tfj = JSON.parse(tf.texte); } catch (e) {}
    verifier("« fadera run » fonctionne dans le terminal, dans le dossier réel", tfj && tfj.code === 0 && tfj.sortie.includes("salut"), tf.texte);

    const ed = await requete("POST", "/__terminal", JSON.stringify({ commande: "fadera editer essai.fa", shell: "cmd" }), hote, J);
    verifier("« fadera editer » s'exécute dans le terminal", ed.code === 200 && JSON.parse(ed.texte).code === 0, ed.texte);
    const ouv = await requete("GET", "/__ouvrir");
    let oj = null; try { oj = JSON.parse(ouv.texte); } catch (e) {}
    verifier("… et ouvre le fichier dans l'éditeur déjà ouvert", ouv.code === 200 && oj && oj.nom === "essai.fa" && oj.texte.includes("salut"), ouv.texte);

    // sécurité
    verifier("sortie du dossier refusée (..)", (await requete("GET", "/../x.fa")).code === 404);
    verifier("chemin Windows refusé", (await requete("GET", "/..\\x.fa")).code === 404);
    verifier("lecteur refusé (C:)", (await requete("GET", "/C:/Windows/win.ini")).code === 404);
    verifier("hôte étranger refusé (DNS rebinding)", (await requete("GET", "/", null, "evil.example.com")).code === 403);
    verifier("JSON invalide refusé", (await requete("POST", "/__ouvrir", "pas du json")).code === 400);
    verifier("méthode inconnue refusée", (await requete("DELETE", "/")).code === 405);
    verifier("fichier absent → 404", (await requete("GET", "/nexiste-pas.js")).code === 404);
    verifier("signal de vie accepté", (await requete("GET", "/__vivant")).code === 200);
  } catch (e) {
    ko++; console.log("  ✗ " + e.message);
  } finally {
    serveur.kill();
    // Windows garde le dossier verrouillé un instant après l'arrêt du serveur (ou le temps d'une analyse antivirus) :
    // on réessaie quelques fois, et un échec de nettoyage n'est jamais une erreur du test.
    for (let i = 0; i < 20; i++) {
      try { fs.rmSync(dossier, { recursive: true, force: true }); break; }
      catch (e) { await new Promise(r => setTimeout(r, 250)); }
    }
  }
  console.log(`${ok} vérification(s) réussie(s), ${ko} échouée(s).`);
  process.exit(ko ? 1 : 0);
})();

#!/usr/bin/env node
// Vérifie « fadera ide » : le petit serveur local qui sert l'éditeur et transmet le fichier double-cliqué.
//   usage : node verifier_ide_serveur.js <index.html> <fadera[.exe]>
const http = require("http"), fs = require("fs"), os = require("os"), path = require("path"), cp = require("child_process");
const [html, exe] = process.argv.slice(2);
if (!html || !exe) { console.error("usage : verifier_ide_serveur.js index.html fadera"); process.exit(2); }
const PORT = 4747;
let ok = 0, ko = 0;
const verifier = (nom, cond, detail) => { if (cond) ok++; else { ko++; console.log("  ✗ " + nom + (detail !== undefined ? "  → " + JSON.stringify(detail) : "")); } };
const requete = (methode, chemin, corps, hote = "127.0.0.1:" + PORT) =>
  new Promise((res) => {
    const r = http.request({ host: "127.0.0.1", port: PORT, method: methode, path: chemin, headers: { Host: hote, ...(corps ? { "Content-Type": "application/json", "Content-Length": Buffer.byteLength(corps) } : {}) } }, (rep) => {
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
  fs.writeFileSync(path.join(dossier, "x.fa"), 'afficher "élève — « salut »"\n');
  fs.writeFileSync(path.join(dossier, "y.fa"), 'afficher "deuxième"\n');
  if ((await requete("GET", "/__fadera")).code !== 0) { console.log("Le port " + PORT + " est déjà utilisé : arrêtez l'éditeur Fadéra puis relancez."); process.exit(1); }
  const serveur = cp.spawn(binaire, ["ide", "--sans-navigateur", "--discret", "x.fa"], { cwd: dossier, stdio: "ignore" });
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
    fs.rmSync(dossier, { recursive: true, force: true });
  }
  console.log(`${ok} vérification(s) réussie(s), ${ko} échouée(s).`);
  process.exit(ko ? 1 : 0);
})();

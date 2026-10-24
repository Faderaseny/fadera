// Vérifie que chaque exemple de docs/exemples/*.fa donne le même résultat sur le moteur JS
// et sur le moteur Rust, et que ce résultat correspond au fichier .sortie voisin.
//   node docs/verifier_exemples.js            vérifie
//   node docs/verifier_exemples.js --ecrire   (re)génère les fichiers .sortie
// Variables : FADERA_BIN = chemin de l'exécutable Rust (défaut : fadera-rs/target/release/fadera)
const fs = require("fs"), cp = require("child_process"), os = require("os"), path = require("path");
const RACINE = path.join(__dirname, "..");
const BIN = process.env.FADERA_BIN || path.join(RACINE, "fadera-rs/target/release/fadera");
const DOSSIER = path.join(__dirname, "exemples");
const ecrire = process.argv.includes("--ecrire");
// Exemples qui dépendent du réseau : pas de sortie de référence figée, résultat variable selon la connexion.
const RESEAU = new Set(["24_meteo_http_json.fa", "25_verificateur_sites.fa"]);

const html = fs.readFileSync(path.join(RACINE, "index.html"), "utf8");
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis",
  html.slice(a, b) + "\nreturn {executerProgramme,decrireErreur};")({ exports: {} }, require, undefined, globalThis);

async function sortieJs(code) {
  const out = [], fx = {};
  const hote = { afficher: t => out.push(t), saisir: () => "",
    lireFichier: p => { if (p in fx) return fx[p]; throw new Error("absent"); },
    ecrireFichier: (p, c) => { fx[p] = c; } };
  try { await M.executerProgramme(code, hote); } catch (e) { out.push("ERR:" + M.decrireErreur(e, code).texte); }
  return out.join("\n");
}
function sortieRs(fichier) {
  const r = cp.spawnSync(BIN, ["--parite", fichier], { encoding: "utf8", env: { ...process.env, TZ: "UTC" } });
  return r.stdout + (r.status ? "\n<<rust statut " + r.status + " " + (r.stderr || "").slice(0, 300) + ">>" : "");
}
process.env.TZ = "UTC";
(async () => {
  let ko = 0, n = 0;
  for (const f of fs.readdirSync(DOSSIER).filter(x => x.endsWith(".fa")).sort()) {
    n++;
    const chemin = path.join(DOSSIER, f), code = fs.readFileSync(chemin, "utf8");
    const j = await sortieJs(code), r = sortieRs(chemin);
    const ref = chemin.replace(/\.fa$/, ".sortie");
    let probleme = "";
    // Exemples réseau : la sortie dépend de la connexion (la CI a internet, pas toujours votre poste) ;
    // on exige seulement qu'ils se terminent proprement sur les deux moteurs, sans erreur non rattrapée.
    if (RESEAU.has(f)) { if (/ERR:|<<rust statut/.test(j + r)) probleme = "erreur non rattrapée"; }
    else if (j !== r) probleme = "les deux moteurs diffèrent";
    else if (j.includes("ERR:") && !RESEAU.has(f)) probleme = "erreur d'exécution";
    else if (!RESEAU.has(f)) {
      if (ecrire) fs.writeFileSync(ref, j + "\n");
      else if (!fs.existsSync(ref)) probleme = "fichier .sortie manquant (lancez avec --ecrire)";
      else if (fs.readFileSync(ref, "utf8").replace(/\r\n/g, "\n").replace(/\n$/, "") !== j) probleme = "la sortie ne correspond plus au fichier .sortie";
    }
    console.log((probleme ? "✗ " : "✓ ") + f + (probleme ? "  — " + probleme : ""));
    if (probleme) ko++;
  }
  // les blocs « tester » de l'exemple 29 ne tournent que via « fadera tester »
  const t = cp.spawnSync(BIN, ["tester", path.join(DOSSIER, "29_tests_integres.fa")], { encoding: "utf8" });
  const ok = t.status === 0 && /4 réussi\(s\), 0 échoué/.test(t.stdout);
  console.log((ok ? "✓ " : "✗ ") + "29_tests_integres.fa (fadera tester)" + (ok ? "" : "  — " + t.stdout + t.stderr));
  if (!ok) ko++;
  console.log(ko ? `\n${ko} exemple(s) en échec sur ${n}.` : `\n✓ ${n} exemples identiques sur les deux moteurs.`);
  process.exit(ko ? 1 : 0);
})();

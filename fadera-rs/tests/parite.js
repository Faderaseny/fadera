#!/usr/bin/env node
// Test de parité : exécute les mêmes programmes Fadéra avec le moteur JS (index.html) et le moteur Rust,
// puis compare les sorties caractère par caractère.
//   usage : node tests/parite.js [chemin/index.html] [chemin/binaire fadera]
const fs = require("fs"), path = require("path"), cp = require("child_process");
const racine = path.resolve(__dirname, "..");
// dossier contenant index.html, tests_fadera.js, … : FADERA_DOSSIER (défini par verifier_tout.js), sinon le dossier parent du projet
const DOSSIER_LIVRES = process.env.FADERA_DOSSIER ? path.resolve(process.env.FADERA_DOSSIER) : path.join(racine, "..");
const html = fs.readFileSync(path.resolve(process.argv[2] || path.join(DOSSIER_LIVRES, "index.html")), "utf8");
const bin = path.resolve(process.argv[3] || path.join(racine, "target", "release", "fadera"));
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis",
  html.slice(a, b) + "\nreturn {executerProgramme,decrireErreur,lancerTests};")({ exports: {} }, require, undefined, globalThis);
// Fichiers virtuels : la MÊME liste que tests_fadera.js (ligne « const FICH = {...}; »), écrite dans un dossier lu par le binaire Rust.
const srcTests = fs.readFileSync(path.join(DOSSIER_LIVRES, "tests_fadera.js"), "utf8");
const mF = /^const FICH = (\{.*\});\s*$/m.exec(srcTests);
if (!mF) { console.error("const FICH introuvable dans tests_fadera.js"); process.exit(2); }
const FICH = new Function("return " + mF[1])();
const DOSSIER_FICH = fs.mkdtempSync(path.join(require("os").tmpdir(), "fad_parite_fich_"));
for (const [nom, contenu] of Object.entries(FICH)) {
  const dest = path.join(DOSSIER_FICH, nom);
  fs.mkdirSync(path.dirname(dest), { recursive: true });
  fs.writeFileSync(dest, contenu);
}

async function runJS(code, tester) {
  const out = [];
  const hote = { afficher: (t) => out.push(t), saisir: () => "", lireFichier: (p) => { if (p in FICH) return FICH[p]; throw new Error("absent"); }, ecrireFichier: () => {} };
  try { if (tester) await M.lancerTests(code, hote); else await M.executerProgramme(code, hote); } catch (e) { out.push("ERR:" + M.decrireErreur(e, code).texte); }
  return out.join("\n");
}
function runRust(code, i, tester) {
  const f = path.join(require("os").tmpdir(), "fad_parite_" + process.pid + "_" + i + ".fa");
  fs.writeFileSync(f, code);
  const r = cp.spawnSync(bin, tester ? ["tester", f] : ["--parite", f, DOSSIER_FICH], { encoding: "utf8", timeout: 60000 });
  fs.unlinkSync(f);
  return (tester ? r.stdout.replace(/\n$/, "") : r.stdout) + (r.status && !(tester && r.status === 1) ? "\n<<rust status " + r.status + " " + (r.stderr || "").slice(0, 300) + ">>" : "");
}

// --- corpus : tests du moteur JS + exercices Markdown + cas supplémentaires
const corpus = [];
const src = fs.readFileSync(path.join(DOSSIER_LIVRES, "tests_fadera.js"), "utf8");
const deb = src.indexOf("async function moteur() {"), fin = src.indexOf("async function shell()");
const corps = src.slice(deb, fin).replace("async function moteur() {", "").replace(/}\s*$/, "");
new Function("t", "tt", "groupe", "return (async()=>{" + corps + "})()")((nom, code) => { corpus.push({ nom, code }); return Promise.resolve(); }, (nom, code) => { corpus.push({ nom: "tester:" + nom, code, tester: true }); return Promise.resolve(); }, "");
for (const f of ["EXERCICES_FADERA_MATHS.md", "EXERCICES_FADERA_MATHS_AVANCES.md"]) {
  const p = path.join(DOSSIER_LIVRES, f);
  if (!fs.existsSync(p)) continue;
  const blocs = [...fs.readFileSync(p, "utf8").matchAll(/```(?:fa|fadera)?\n([\s\S]*?)```/g)].map((m) => m[1]);
  blocs.forEach((c, i) => corpus.push({ nom: f + "#" + i, code: c }));
}
const extra = path.join(__dirname, "cas_supplementaires.json");
if (fs.existsSync(extra)) JSON.parse(fs.readFileSync(extra, "utf8")).forEach((c, i) => corpus.push({ nom: "extra#" + i, code: c }));

const ALEA = /alea_|pause|chrono|http_|hasard|maintenant|date_iso|formater_date|aleatoire|crypto_chiffrer|crypto_generer|chiffrer\(|creerReseau|creer_reseau|ia_creer|ia_entrainer|ia_reseau|iaPredire|iaEntrainer|saisir|creer_fichier|ecrire_log|ecrireFichier|plageReseau\s*\(\s*saisir/;
(async () => {
  let ok = 0, ko = 0, saut = 0, i = 0;
  for (const c of corpus) {
    i++;
    if (ALEA.test(c.code)) { saut++; continue; }
    const j = await runJS(c.code, c.tester), r = runRust(c.code, i, c.tester);
    const norm = (x) => x.replace(/(JSON invalide|expression régulière invalide) \(.*\)\.$/m, "$1 (…).");
    if (norm(j) === norm(r)) ok++;
    else { ko++; console.log("✗ " + c.nom + "\n--- code ---\n" + c.code.slice(0, 400) + "\n--- JS ---\n" + j.slice(0, 500) + "\n--- Rust ---\n" + r.slice(0, 500) + "\n"); }
  }
  console.log(`\nParité JS/Rust : ${ok} identiques, ${ko} différents, ${saut} ignorés (aléatoire/temps/IO) sur ${corpus.length} programmes.`);
  process.exit(ko ? 1 : 0);
})();

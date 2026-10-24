#!/usr/bin/env node
// Formateur : parité JS/Rust (`fadera fmt` ≡ formaterSource), idempotence, et conservation du comportement des programmes.
//   usage : node tests/parite_fmt.js [chemin/index.html] [chemin/binaire fadera]
const fs = require("fs"), path = require("path"), cp = require("child_process"), os = require("os");
const racine = path.resolve(__dirname, "..");
const DOSSIER_LIVRES = process.env.FADERA_DOSSIER ? path.resolve(process.env.FADERA_DOSSIER) : path.join(racine, "..");
const html = fs.readFileSync(path.resolve(process.argv[2] || path.join(DOSSIER_LIVRES, "index.html")), "utf8");
const bin = path.resolve(process.argv[3] || path.join(racine, "target", "release", "fadera"));
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis",
  html.slice(a, b) + "\nreturn {executerProgramme,decrireErreur,formaterSource};")({ exports: {} }, require, undefined, globalThis);

const corpus = [];
const src = fs.readFileSync(path.join(DOSSIER_LIVRES, "tests_fadera.js"), "utf8");
const deb = src.indexOf("async function moteur() {"), fin = src.indexOf("async function shell()");
const corps = src.slice(deb, fin).replace("async function moteur() {", "").replace(/}\s*$/, "");
new Function("t", "tt", "groupe", "return (async()=>{" + corps + "})()")((nom, code) => { corpus.push({ nom, code }); return Promise.resolve(); }, (nom, code) => { corpus.push({ nom: "tester:" + nom, code }); return Promise.resolve(); }, "");
for (const f of fs.readdirSync(path.join(racine, "src", "std"))) corpus.push({ nom: "std/" + f, code: fs.readFileSync(path.join(racine, "src", "std", f), "utf8"), figé: true });
const spec = fs.readFileSync(path.join(DOSSIER_LIVRES, "SPEC_FADERA.md"), "utf8").replace(/\r\n/g, "\n");
[...spec.matchAll(/```fa-test\n([\s\S]*?)```/g)].forEach((m, i) => corpus.push({ nom: "spec#" + i, code: m[1] }));
for (const f of ["EXERCICES_FADERA_MATHS.md", "EXERCICES_FADERA_MATHS_AVANCES.md", "GUIDE_FADERA.md"]) {
  const p = path.join(DOSSIER_LIVRES, f);
  if (!fs.existsSync(p)) continue;
  [...fs.readFileSync(p, "utf8").matchAll(/```(?:fa|fadera)?\n([\s\S]*?)```/g)].forEach((m, i) => corpus.push({ nom: f + "#" + i, code: m[1] }));
}
const extra = path.join(__dirname, "cas_fmt.json");
if (fs.existsSync(extra)) JSON.parse(fs.readFileSync(extra, "utf8")).forEach((c, i) => corpus.push({ nom: "fmt#" + i, code: c }));
const extra2 = path.join(__dirname, "cas_supplementaires.json");
if (fs.existsSync(extra2)) JSON.parse(fs.readFileSync(extra2, "utf8")).forEach((c, i) => corpus.push({ nom: "extra#" + i, code: c }));

const ALEA = /alea_|pause|chrono|http_|hasard|maintenant|date_iso|formater_date|aleatoire|crypto_chiffrer|crypto_generer|chiffrer\(|creerReseau|creer_reseau|ia_creer|ia_entrainer|ia_reseau|saisir|creer_fichier|ecrire_log|ecrireFichier|lireFichier|csv_|importer/;
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "fad_fmt_"));
function rust(code, i, extraArgs) {
  const f = path.join(tmp, "c" + i + ".fa");
  fs.writeFileSync(f, code);
  const r = cp.spawnSync(bin, ["fmt", ...(extraArgs || []), f], { encoding: "utf8", timeout: 60000 });
  if (r.status === 0) return { ok: true, texte: r.stdout };
  return { ok: false, texte: (r.stderr || "").replace(/^.*?c\d+\.fa : /, "").replace(/\n$/, "") };
}
function js(code, ind) {
  try { return { ok: true, texte: M.formaterSource(code, ind) }; } catch (e) { return { ok: false, texte: M.decrireErreur(e, code).texte }; }
}
async function sortie(code) {
  const out = [];
  try { await M.executerProgramme(code, { afficher: (t) => out.push(t), saisir: () => "", lireFichier: () => { throw new Error("absent"); }, ecrireFichier: () => {} }); } catch (e) { out.push("ERR:" + M.decrireErreur(e, code).texte.replace(/^Ligne \d+ : /, "")); }
  return out.join("\n");
}
(async () => {
  let ok = 0, ko = 0, invalides = 0, i = 0;
  const echec = (nom, msg) => { ko++; if (ko <= 15) console.log("✗ " + nom + "\n   " + msg.split("\n").slice(0, 12).join("\n   ")); };
  for (const c of corpus) {
    i++;
    for (const ind of [4, 2]) {
      const j = js(c.code, ind), r = rust(c.code, i, ind === 4 ? [] : ["--indentation", "2"]);
      if (j.ok !== r.ok || j.texte !== r.texte) { { let k = 0; while (k < j.texte.length && j.texte[k] === r.texte[k]) k++; echec(c.nom + " (indentation " + ind + ")", "ok " + j.ok + "/" + r.ok + " écart à " + k + "\nJS  : " + JSON.stringify(j.texte.slice(Math.max(0, k - 60), k + 80)) + "\nRust: " + JSON.stringify(r.texte.slice(Math.max(0, k - 60), k + 80))); } continue; }
      if (!j.ok) { if (ind === 4) invalides++; ok++; continue; }
      // idempotence
      const j2 = js(j.texte, ind);
      if (!j2.ok || j2.texte !== j.texte) { echec(c.nom + " (idempotence, indentation " + ind + ")", "1re : " + JSON.stringify(j.texte).slice(0, 300) + "\n2e  : " + JSON.stringify(j2.texte).slice(0, 300)); continue; }
      // comportement conservé
      if (ind === 4 && !ALEA.test(c.code) && !c.figé) {
        const s1 = await sortie(c.code), s2 = await sortie(j.texte);
        if (s1 !== s2) { echec(c.nom + " (comportement)", "avant : " + JSON.stringify(s1).slice(0, 300) + "\naprès : " + JSON.stringify(s2).slice(0, 300)); continue; }
      }
      ok++;
    }
  }
  console.log("Formateur JS ≡ Rust : " + ok + " vérifications réussies, " + ko + " en échec, sur " + corpus.length + " programmes (" + invalides + " non formatables : code invalide, même message des deux côtés).");
  process.exit(ko ? 1 : 0);
})();

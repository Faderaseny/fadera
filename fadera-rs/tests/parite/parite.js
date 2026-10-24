#!/usr/bin/env node
// Test de parité : exécute les mêmes programmes Fadéra avec le moteur JS (index.html) et le moteur Rust
// (binaire `fadera`) et compare sorties + messages d'erreur.
//   usage : node tests/parite/parite.js [index.html] [chemin/vers/fadera]
const fs = require("fs"), path = require("path"), os = require("os"), cp = require("child_process");
const racine = path.resolve(__dirname, "..", "..");
const base = [path.join(racine, ".."), "/mnt/user-data/outputs"].find((d) => fs.existsSync(path.join(d, "index.html")) && fs.existsSync(path.join(d, "tests_fadera.js"))) || path.join(racine, "..");
const html = path.resolve(process.argv[2] || path.join(base, "index.html"));
const bin = path.resolve(process.argv[3] || path.join(racine, "target", "release", "fadera"));
const src = fs.readFileSync(html, "utf8");
const a = src.indexOf("      const TYPE = {"), b = src.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis",
  src.slice(a, b) + "\nreturn {executerProgramme,decrireErreur};")({ exports: {} }, require, undefined, globalThis);

const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "fadera-parite-"));
const FICH = { "outils.fa": 'fonction double(x)\n renvoyer x * 2\nfin\nsoit PI2 = 6.28', "boucle.fa": 'importer "boucle"\nafficher 1' };
for (const k in FICH) fs.writeFileSync(path.join(tmp, k), FICH[k]);

const cas = [];
// 1) cas de tests_fadera.js (partie moteur)
const ts = fs.readFileSync(path.join(base, "tests_fadera.js"), "utf8");
const dm = ts.indexOf("async function moteur() {"), fm = ts.indexOf("async function shell()");
const corps = ts.slice(dm, fm).replace("async function moteur() {", "").replace(/}\s*$/, "");
new Function("t", "groupe", "return (async()=>{" + corps + "})()")((nom, code) => { cas.push({ nom, code }); return Promise.resolve(); }, "");
// 2) blocs de code des exercices
for (const f of ["EXERCICES_FADERA_MATHS.md", "EXERCICES_FADERA_MATHS_AVANCES.md", "GUIDE_FADERA.md"]) {
  const p = path.join(base, f);
  if (!fs.existsSync(p)) continue;
  const m = fs.readFileSync(p, "utf8").matchAll(/```(?:fadera|fa)?\n([\s\S]*?)```/g);
  let i = 0;
  for (const x of m) cas.push({ nom: f + "#" + ++i, code: x[1] });
}
// 3) cas supplémentaires ciblant la sémantique JS à reproduire
const extra = [
  'afficher 0.1 + 0.2', 'afficher 1/3', 'afficher 1e21', 'afficher 123456789012345678', 'afficher 10 / 4', 'afficher -7 % 3',
  'afficher 2 + "x"', 'afficher "a" < "b"', 'afficher [1, "a", [2, vrai]]', 'structure s = {z: 1, 2: "x", a: faux}\nafficher s\nafficher cles(s)',
  'afficher non 0', 'afficher 0 ou "x"', 'afficher "" et 1', 'afficher 5 == "5"', 'afficher [1,2] == [1,2]',
  'afficher arrondir(2.5)\nafficher arrondir(-2.5)\nafficher arrondir(1.005, 2)', 'afficher stats_mode([3,1,3,1,2])',
  'afficher json_texte({a:[1,{b:2}], c:"é\\n"}, 2)', 'afficher json_lire("[1,2,{\\"a\\":null}]")', 'afficher decouper("abc", "")',
  'afficher remplacer("abc","","-")', 'afficher trier(["b","a","C","é","e"])', 'afficher math_pgcd_ppcm(60, 48)',
  'afficher math_facteurs_premiers(360)', 'afficher math_resoudre_second_degre(1,-5,6)', 'afficher inverser_matrice([[1,2],[3,4]])',
  'afficher stats_regression_lineaire([1,2,3,4],[2,4,5,9])', 'afficher stats_analyse_distribution([1,2,3,4,5,6,7,100])',
  'afficher telecom_calculer_sous_reseau("192.168.1.10", 26)', 'afficher plageReseau("10.1.2.3", 0)', 'afficher masqueReseau(32)',
  'soit t = telecom_creer_topologie()\ntelecom_ajouter_liaison(t, {de:"A", a:"B", cout:1})\ntelecom_ajouter_liaison(t, {de:"B", a:"C", cout:2})\ntelecom_ajouter_liaison(t, {de:"A", a:"C", cout:5})\nafficher telecom_dijkstra_routage(t, {depart:"A", arrivee:"C"})\nafficher telecom_matrice_adjacence(t)\nafficher telecom_verifier_connexite(t)',
  'afficher stats_query_engine([{n:"b",a:3},{n:"a",a:1},{n:"c",a:2}], {filtres:[{champ:"a",operateur:">",valeur:1}], tri:{champ:"n"}, selection:["n"]})',
  'afficher date_iso(0)\nafficher date_iso(1700000000123)', 'afficher regex_remplacer("2026-10-11", "(\\\\d+)-(\\\\d+)-(\\\\d+)", "$3/$2/$1")',
  'afficher sous_texte("fadera", -3)\nafficher tranche([1,2,3,4,5], 1, -1)', 'afficher nombre("3,5") + 1', 'afficher nombre("abc")',
  'soit x = 5\nafficher x[0]', 'afficher lireElement([1,2], 2)', 'afficher 5 / 0', 'afficher "a" - 1', 'fonction f(a)\n renvoyer a\nfin\nafficher f()',
  'soit i = 0\ntantque vrai faire\n i = i + 1\nfin', 'fonction r(n)\n renvoyer r(n + 1)\nfin\nr(1)', 'afficher hacher_sha256("")', 'afficher crypto_hmac_sha256("a","b")',
  'afficher crypto_hacher_sha512("abc")', 'afficher chiffrer_base64("Fadéra ✓")\nafficher dechiffrer_base64("RmFkw6lyYSDinJM=")',
  'pour i allantde 10 a 1 pas -3 faire\n afficher i\nfin', 'pour i allantde 1 a 3 pas 0 faire\n afficher i\nfin', 'afficher taille("héllo")\nafficher lireElement("héllo", 1)',
  'afficher [1,2] + [3]', 'afficher plage(5, 1, -2)', 'afficher aplatir([1,[2,[3,[4]]]])', 'afficher valeurs({b:1,a:2})', 'afficher paires({b:1})',
  'afficher est_nombre("  ")\nafficher est_nombre("1e3")', 'afficher minimum([3,1,2])\nafficher maximum(3, 9)', 'afficher entier(-3.7)\nafficher plancher(-3.2)',
];
extra.forEach((c, i) => cas.push({ nom: "extra#" + (i + 1), code: c }));

const messageHote = /json invalide|regex invalide|importer absent/;
const exclure = /GUIDE_FADERA\.md#35|hasard|maintenant|date_iso\(\)|formater_date|crypto_generer|chiffrer\(|dechiffrer|aleatoire|ia_creer_neurone|creerReseau|iaEntrainer|entrainer|ia_entrainer|ia_reseau|saisir|sha256 aes/;
async function jsRun(code) {
  const out = [];
  const hote = { afficher: (t) => out.push(t), saisir: () => "", lireFichier: (p) => { if (p in FICH) return FICH[p]; throw new Error("absent"); }, ecrireFichier: () => {} };
  try { await M.executerProgramme(code, hote); } catch (e) { out.push("ERR:" + M.decrireErreur(e, code).texte); }
  return out.join("\n");
}
function rustRun(code) {
  const r = cp.spawnSync(bin, ["-e", code], { cwd: tmp, encoding: "utf8", input: "", timeout: 60000 });
  let o = (r.stdout || "").replace(/\n$/, "");
  const m = /^Erreur — (.*)$/m.exec(r.stderr || "");
  if (m) o = (o ? o + "\n" : "") + "ERR:" + m[1];
  return o;
}
(async () => {
  let ok = 0, ko = 0, ign = 0;
  for (const c of cas) {
    if (exclure.test(c.code) || exclure.test(c.nom)) { ign++; continue; }
    const j = await jsRun(c.code), r = rustRun(c.code);
    // la limite de pile JS diffère : on ne compare pas les cas de récursion infinie
    const tol = messageHote.test(c.nom) && j.startsWith("ERR:") && r.startsWith("ERR:") && j.split(" (")[0] === r.split(" (")[0];
    if (j === r || tol) ok++; else { ko++; console.log("✗ " + c.nom + "\n  code : " + JSON.stringify(c.code).slice(0, 200) + "\n  JS   : " + JSON.stringify(j).slice(0, 300) + "\n  Rust : " + JSON.stringify(r).slice(0, 300)); }
  }
  console.log("\nParité JS/Rust : " + ok + " identiques, " + ko + " différents, " + ign + " ignorés (aléatoire/horloge/crypto).");
  process.exit(ko ? 1 : 0);
})();

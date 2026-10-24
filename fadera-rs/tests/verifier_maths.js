#!/usr/bin/env node
// Compare bit à bit les fonctions mathématiques du moteur Rust (examples/dump_maths) et du portage JavaScript (MathFD dans index.html),
// sur des entrées aléatoires de toutes tailles, plus les vecteurs mpmath de tests/maths_vecteurs.txt.
//   usage : node tests/verifier_maths.js [index.html] [binaire dump_maths] [nb_par_fonction]
const fs = require("fs"), path = require("path"), cp = require("child_process");
const racine = path.resolve(__dirname, "..");
// dossier contenant index.html, tests_fadera.js, … : FADERA_DOSSIER (défini par verifier_tout.js), sinon le dossier parent du projet
const DOSSIER_LIVRES = process.env.FADERA_DOSSIER ? path.resolve(process.env.FADERA_DOSSIER) : path.join(racine, "..");
const html = fs.readFileSync(path.resolve(process.argv[2] || path.join(DOSSIER_LIVRES, "index.html")), "utf8");
const bin = path.resolve(process.argv[3] || path.join(racine, "target", "release", "examples", "dump_maths"));
const N = parseInt(process.argv[4] || "40000", 10);
const a = html.indexOf("const MathFD = (() => {"), b = html.indexOf("})();", a) + 5;
if (a < 0) { console.error("MathFD introuvable"); process.exit(2); }
const MathFD = new Function(html.slice(a, b) + "\nreturn MathFD;")();
const dv = new DataView(new ArrayBuffer(8));
const bits = (x) => (dv.setFloat64(0, x), dv.getBigUint64(0).toString(16).padStart(16, "0"));
const dep = (h) => (dv.setBigUint64(0, BigInt("0x" + h)), dv.getFloat64(0));
let graine = 12345;
const alea = () => { graine = (graine * 1664525 + 1013904223) >>> 0; return graine / 4294967296; };
const gen = {
  sin: () => (alea() - 0.5) * Math.pow(10, alea() * 9 - 3) * 2,
  exp: () => (alea() - 0.5) * (alea() < 0.2 ? 1500 : 60),
  log: () => Math.pow(10, (alea() - 0.5) * 600),
  unit: () => (alea() - 0.5) * 2,
  atan: () => (alea() - 0.5) * Math.pow(10, alea() * 12 - 6) * 2,
  cbrt: () => (alea() - 0.5) * Math.pow(10, alea() * 40 - 20) * 2,
  pw: () => [Math.pow(10, (alea() - 0.5) * 8), (alea() - 0.5) * 40],
};
const jobs = [];
for (const f of ["sin", "cos", "tan"]) for (let i = 0; i < N; i++) jobs.push([f, gen.sin()]);
for (let i = 0; i < N; i++) jobs.push(["exp", gen.exp()]);
for (const f of ["log", "log2", "log10"]) for (let i = 0; i < N; i++) jobs.push([f, gen.log()]);
for (const f of ["asin", "acos"]) for (let i = 0; i < N; i++) jobs.push([f, gen.unit()]);
for (let i = 0; i < N; i++) jobs.push(["atan", gen.atan()]);
for (let i = 0; i < N; i++) jobs.push(["cbrt", gen.cbrt()]);
for (let i = 0; i < N; i++) jobs.push(["atan2", gen.atan(), gen.atan()]);
for (let i = 0; i < N; i++) { const [x, y] = gen.pw(); jobs.push(["pow", x, y]); }
for (let i = 0; i < N / 4; i++) jobs.push(["pow", Math.floor(alea() * 50) - 25 + 0.0, Math.floor(alea() * 20) - 10]);
// cas particuliers
for (const f of ["sin", "cos", "tan", "exp", "log", "atan", "cbrt", "asin", "acos"]) for (const x of [0, -0, 1, -1, 0.5, 1e-9, 1e-300, 5e-324, 709.7, 710, -700, -750, 1e9, Infinity, -Infinity, NaN, Math.PI, Math.PI / 2]) jobs.push([f, x]);
for (const [x, y] of [[0, 0], [1, Infinity], [-1, Infinity], [1, NaN], [2, 1024], [2, -1075], [-8, 1 / 3], [-8, 3], [0, -1], [10, 308], [10, 309], [Infinity, 2], [2, 0.5]]) jobs.push(["pow", x, y]);

const entree = jobs.map((j) => j[0] + " " + bits(j[1]) + (j.length > 2 ? " " + bits(j[2]) : "")).join("\n") + "\n";
const r = cp.spawnSync(bin, [], { input: entree, encoding: "utf8", maxBuffer: 1 << 28 });
if (r.status) { console.error(r.stderr); process.exit(2); }
const rust = r.stdout.trim().split("\n");
const ecarts = {}, totaux = {};
let ko = 0;
jobs.forEach((j, i) => {
  const [f, x, y] = j;
  const js = MathFD[f](x, y);
  const attendu = rust[i], obtenu = bits(js);
  totaux[f] = (totaux[f] || 0) + 1;
  const memeNaN = Number.isNaN(js) && Number.isNaN(dep(attendu));
  if (obtenu !== attendu && !memeNaN) {
    ko++; ecarts[f] = (ecarts[f] || 0) + 1;
    if (ecarts[f] <= 3) console.log(`✗ ${f}(${x}${y !== undefined ? ", " + y : ""}) : JS ${js} ≠ Rust ${dep(attendu)}`);
  }
});
// vecteurs mpmath (référence mathématique) — le portage JS doit aussi les satisfaire
let nv = 0, kov = 0;
for (const l of fs.readFileSync(path.join(racine, "tests", "maths_vecteurs.txt"), "utf8").split("\n").filter(Boolean)) {
  const p = l.split(" ");
  let js;
  if (p[0] === "atan2") { const [y, x] = p[1].split("_"); js = MathFD.atan2(dep(y), dep(x)); }
  else {
    const x = dep(p[1]);
    const m = /^pow_(.+)$/.exec(p[0]);
    if (m) {
      const t = m[1];
      js = t === "3_7" ? MathFD.pow(x, 3.7) : t === "x_1_5" ? MathFD.pow(x, 1.5) : t === "2_x" ? MathFD.pow(2, x) : t === "10_x" ? MathFD.pow(10, x) : t === "1_1_x" ? MathFD.pow(1.1, x) : t === "x_3" ? MathFD.pow(x, 3) : MathFD.pow(x, -5);
    } else js = MathFD[p[0]](x);
  }
  nv++;
  if (BigInt("0x" + bits(js)) !== BigInt("0x" + p[2])) { kov++; if (kov <= 5) console.log("✗ vecteur " + l + " : JS " + bits(js)); }
}
console.log(`Maths JS ≡ Rust : ${jobs.length - ko} identiques, ${ko} différents sur ${jobs.length} calculs.`);
console.log(`Vecteurs mpmath (JS) : ${nv - kov}/${nv}.`);
if (ko) console.log("Écarts par fonction : " + JSON.stringify(ecarts));
process.exit(ko || kov ? 1 : 0);

// Vérifie la table des arguments des fonctions natives (docs/reference/arites.txt) :
//   1. index.html contient exactement cette table ;
//   2. chaque fonction native (moteur JS et moteur Rust) y figure ;
//   3. pour le moteur JS, le nombre maximal de la table est le nombre de paramètres déclarés de la fonction.
//   usage : node tests/verifier_arites.js [dossier du projet]
const fs = require("fs"), path = require("path"), cp = require("child_process");
const racine = path.resolve(process.argv[2] || path.join(__dirname, "..", ".."));
let ko = 0;
const echec = (m) => { ko++; console.log("✗ " + m); };

const r = cp.spawnSync("node", [path.join(racine, "docs", "generer_arites.js"), "--verifier"], { encoding: "utf8" });
if (r.status !== 0) echec("index.html n'est pas à jour : " + (r.stderr || r.stdout).trim());

const table = {};
for (const l of fs.readFileSync(path.join(racine, "docs", "reference", "arites.txt"), "utf8").split("\n")) {
  const m = /^([A-Za-z_0-9]+) (\d+) (\d+)$/.exec(l.trim());
  if (m) table[m[1]] = [+m[2], +m[3]];
}
for (const [n, [a, b]] of Object.entries(table)) if (a > b) echec(n + " : minimum " + a + " > maximum " + b);

// moteur JS
const html = fs.readFileSync(path.join(racine, "index.html"), "utf8");
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis", html.slice(a, b) + "\nreturn {FONCTIONS_NATIVES}")({}, require, {}, globalThis);
let nJs = 0;
for (const [n, f] of Object.entries(M.FONCTIONS_NATIVES)) {
  if (n === "__instance") continue; // interne (instanciation des classes), non contrôlée
  nJs++;
  if (!table[n]) { echec("fonction native JS sans entrée dans arites.txt : " + n); continue; }
  const m = /^[^(]*\(([^)]*)\)/.exec(f.toString());
  const decl = m[1].split(",").map((s) => s.trim()).filter(Boolean);
  if (decl.some((p) => p.startsWith("..."))) continue; // fonction à nombre variable d'arguments : contrôlée à la main
  if (decl.length !== table[n][1]) echec(n + " : " + decl.length + " paramètre(s) déclaré(s) en JS, maximum " + table[n][1] + " dans arites.txt");
}

// moteur Rust : noms des fonctions reconnues par les quatre `trouver`
const noms = new Set();
for (const f of ["natives.rs", "serveur.rs", "stdlib.rs", "modules.rs"]) {
  const s = fs.readFileSync(path.join(racine, "fadera-rs", "src", f), "utf8");
  const i = s.indexOf("pub fn trouver"), j = s.indexOf("\n}\n", i);
  for (const m of s.slice(i, j).matchAll(/((?:"[A-Za-z_0-9]+"\s*\|?\s*)+)=>/g))
    for (const n of m[1].matchAll(/"([A-Za-z_0-9]+)"/g)) noms.add(n[1]);
}
noms.delete("__instance"); // interne (instanciation des classes)
for (const n of noms) if (!table[n]) echec("fonction native Rust sans entrée dans arites.txt : " + n);
for (const n of Object.keys(table)) if (!noms.has(n)) echec("arites.txt contient « " + n + " » qui n'est pas une fonction native Rust");
console.log(ko ? "\n✗ " + ko + " problème(s)." : "✓ Arguments des fonctions natives : " + Object.keys(table).length + " fonctions, table identique dans index.html, " + nJs + " fonctions JS et " + noms.size + " fonctions Rust couvertes.");
process.exit(ko ? 1 : 0);

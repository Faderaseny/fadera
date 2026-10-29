#!/usr/bin/env node
// Lance TOUTES les vérifications d'un coup ; code de sortie 1 si l'une échoue.
//   Prérequis : Node.js et Rust (cargo). Placez ce fichier à côté de index.html, tests_fadera.js, SPEC_FADERA.md, verifier_spec.js,
//   et du dossier du projet Rust (dézippé de fadera-rs.zip : il peut être dans fadera-rs/ OU dans fadera-rs/fadera-rs/).
//   usage : node verifier_tout.js
const cp = require("child_process"), path = require("path"), fs = require("fs");
const ici = __dirname;
// trouve le dossier du projet Rust (celui qui contient Cargo.toml), quel que soit le niveau d'imbrication du dézippage
function trouverProjet() {
  const pile = [path.join(ici, "fadera-rs")];
  while (pile.length) {
    const d = pile.shift();
    if (!fs.existsSync(d)) continue;
    if (fs.existsSync(path.join(d, "Cargo.toml"))) return d;
    for (const e of fs.readdirSync(d, { withFileTypes: true }))
      if (e.isDirectory() && !["target", "pkg", ".git", ".cargo", "src", "tests", "examples"].includes(e.name)) pile.push(path.join(d, e.name));
  }
  return null;
}
const rs = trouverProjet();
if (!rs) { console.error("Projet Rust introuvable : dézippez fadera-rs.zip à côté de verifier_tout.js (il faut un dossier contenant Cargo.toml)."); process.exit(2); }
for (const f of ["index.html", "tests_fadera.js", "SPEC_FADERA.md", "verifier_spec.js", "docs/verifier_exemples.js"])
  if (!fs.existsSync(path.join(ici, f))) { console.error("Fichier manquant à côté de verifier_tout.js : " + f); process.exit(2); }
console.log("Projet Rust : " + rs + "\n");
const exe = process.platform === "win32" ? ".exe" : "";
const env = { FADERA_DOSSIER: ici };
const etapes = [
  ["Compilation Rust (binaire + outil de comparaison des maths)", "cargo", ["build", "--release", "--bins", "--examples"], rs],
  ["Tests du moteur Rust", "cargo", ["test", "--release"], rs],
  ["Tests du moteur JavaScript, du shell et du débogueur", "node", ["tests_fadera.js", "index.html"], ici],
  ["Parité JS/Rust (tous les programmes de test)", "node", [path.join(rs, "tests", "parite.js"), path.join(ici, "index.html"), path.join(rs, "target", "release", "fadera" + exe)], ici, { TZ: "UTC" }],
  ["Formateur JS ≡ Rust (parité, idempotence, comportement conservé)", "node", [path.join(rs, "tests", "parite_fmt.js"), path.join(ici, "index.html"), path.join(rs, "target", "release", "fadera" + exe)], ici],
  ["Parité du débogueur JS/Rust (pauses, variables, pile)", "node", [path.join(rs, "tests", "parite_debogueur.js"), path.join(ici, "index.html"), path.join(rs, "target", "release", "fadera" + exe)], ici],
  ["Spécification exécutable (SPEC_FADERA.md)", "node", ["verifier_spec.js", "SPEC_FADERA.md", "index.html", path.join(rs, "target", "release", "fadera" + exe)], ici],
  ["Documentation : 40 exemples identiques sur les deux moteurs", "node", [path.join("docs", "verifier_exemples.js")], ici, { FADERA_BIN: path.join(rs, "target", "release", "fadera" + exe) }],
  ["Documentation : exercices corrigés identiques sur les deux moteurs", "node", [path.join("docs", "verifier_exemples.js"), "exercices"], ici, { FADERA_BIN: path.join(rs, "target", "release", "fadera" + exe) }],
  ["Documentation : page des exercices à jour", "node", [path.join("docs", "generer_exercices.js"), "--verifier"], ici],
  ["Documentation : référence des fonctions à jour et complète", "node", [path.join("docs", "generer_reference.js"), "--verifier"], ici, { FADERA_BIN: path.join(rs, "target", "release", "fadera" + exe) }],
  ["Documentation : tutoriel à jour (tous les exemples exécutés)", "node", [path.join("docs", "generer_tutoriel.js"), "--verifier"], ici, { FADERA_BIN: path.join(rs, "target", "release", "fadera" + exe) }],
  ["IDE dans un vrai navigateur (onglets, coloration, complétion, recherche, IndexedDB, zip) — facultatif", "node", [path.join(rs, "tests", "verifier_ide.js"), path.join(ici, "index.html")], ici, null, true],
  ["Éditeur local « fadera ide » (serveur, fichier double-cliqué, sécurité)", "node", [path.join(rs, "tests", "verifier_ide_serveur.js"), path.join(ici, "index.html"), path.join(rs, "target", "release", "fadera" + exe)], ici],
  ["Gestionnaire de paquets et réseau de la ligne de commande", "node", [path.join(rs, "tests", "verifier_paquets.js"), path.join(rs, "target", "release", "fadera" + exe)], ici],
  ["Vitesse du moteur Rust (régression catastrophique)", "node", [path.join(rs, "tests", "vitesse.js"), path.join(ici, "index.html"), path.join(rs, "target", "release", "fadera" + exe)], ici],
  ["Maths : JS ≡ Rust bit à bit + valeurs mpmath", "node", [path.join(rs, "tests", "verifier_maths.js"), path.join(ici, "index.html"), path.join(rs, "target", "release", "examples", "dump_maths" + exe)], ici],
];
let ko = 0;
for (const [nom, cmd, args, cwd, extra, facultative] of etapes) {
  process.stdout.write("▶ " + nom + " … ");
  const r = cp.spawnSync(cmd, args, { cwd, encoding: "utf8", env: { ...process.env, ...env, ...(extra || {}) }, shell: process.platform === "win32" && cmd === "cargo", maxBuffer: 1 << 28 });
  const sortie = (r.stdout || "") + (r.stderr || "");
  const fin = sortie.trim().split("\n").filter(Boolean).slice(-2).join(" | ");
  if (facultative && r.status === 3) console.log("IGNORÉ  (" + fin.slice(0, 120) + ")");
  else if (r.status === 0) console.log("OK" + (fin ? "  (" + fin.slice(0, 160) + ")" : ""));
  else { ko++; console.log("ÉCHEC\n" + (r.error ? String(r.error) + "\n" : "") + sortie.split("\n").slice(-25).join("\n")); }
}
console.log(ko ? `\n✗ ${ko} étape(s) en échec.` : "\n✓ Tout est vert.");
process.exit(ko ? 1 : 0);

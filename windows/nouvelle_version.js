// Prépare une nouvelle version : node windows/nouvelle_version.js 0.8.2
// Change le numéro dans Cargo.toml (x2), Cargo.lock (x2), index.html (x2) et crée la section du journal.
const fs = require("fs");
const v = process.argv[2];
if (!/^\d+\.\d+\.\d+$/.test(v || "")) { console.error("Usage : node windows/nouvelle_version.js 0.8.2"); process.exit(1); }
const lire = f => fs.readFileSync(f, "utf8");
const ecrire = (f, t) => fs.writeFileSync(f, t);
function remplacer(f, re, par) {
  const t = lire(f), n = t.replace(re, par);
  if (n === t) { console.error("Rien à changer dans " + f + " (déjà à jour ?)"); return; }
  ecrire(f, n); console.log("ok  " + f);
}
remplacer("fadera-rs/Cargo.toml", /^version = "[^"]*"/m, `version = "${v}"`);
remplacer("fadera-ide/Cargo.toml", /^version = "[^"]*"/m, `version = "${v}"`);
remplacer("fadera-rs/Cargo.lock", /(name = "fadera"\r?\nversion = )"[^"]*"/, `$1"${v}"`);
remplacer("fadera-ide/Cargo.lock", /(name = "fadera-ide"\r?\nversion = )"[^"]*"/, `$1"${v}"`);
remplacer("index.html", /const VERSION_FADERA = "[^"]*"/, `const VERSION_FADERA = "${v}"`);
remplacer("index.html", /Démarrage de Fadéra v[0-9.]+…/, `Démarrage de Fadéra v${v}…`);
const date = new Date().toISOString().slice(0, 10);
const j = lire("CHANGELOG.md");
if (j.includes(`## [${v}]`)) console.log("Journal : section " + v + " déjà présente");
else { ecrire("CHANGELOG.md", j.replace("## [Non publié]", `## [Non publié]\n\n## [${v}] — ${date}`)); console.log("ok  CHANGELOG.md"); }

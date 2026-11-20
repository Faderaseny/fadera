#!/usr/bin/env node
// Recopie docs/reference/arites.txt (source unique) dans index.html, entre les marqueurs ARITES:DEBUT / ARITES:FIN.
//   node docs/generer_arites.js            écrit index.html
//   node docs/generer_arites.js --verifier échoue si index.html n'est pas à jour (utilisé par la vérification complète)
const fs = require("fs"), path = require("path");
const racine = path.join(__dirname, "..");
const src = fs.readFileSync(path.join(racine, "docs", "reference", "arites.txt"), "utf8");
const lignes = [];
for (const l of src.split("\n")) {
  if (!l.trim() || l.startsWith("#")) continue;
  const m = /^([A-Za-z_0-9]+) (\d+) (\d+)$/.exec(l.trim());
  if (!m) { console.error("✗ ligne invalide dans arites.txt : " + l); process.exit(1); }
  lignes.push(`${m[1]}: [${m[2]}, ${m[3]}]`);
}
const bloc = "/* ARITES:DEBUT (généré par docs/generer_arites.js à partir de docs/reference/arites.txt) */\n      const ARITES = {\n        " +
  lignes.join(",\n        ") + ",\n      };\n      /* ARITES:FIN */";
const f = path.join(racine, "index.html");
const html = fs.readFileSync(f, "utf8");
const re = /\/\* ARITES:DEBUT[\s\S]*?\/\* ARITES:FIN \*\//;
if (!re.test(html)) { console.error("✗ marqueurs ARITES:DEBUT / ARITES:FIN introuvables dans index.html"); process.exit(1); }
const neuf = html.replace(re, () => bloc);
if (process.argv.includes("--verifier")) {
  if (neuf !== html) { console.error("✗ index.html n'est pas à jour : lancez  node docs/generer_arites.js"); process.exit(1); }
  console.log("✓ table des arguments de index.html à jour (" + lignes.length + " fonctions)");
} else {
  fs.writeFileSync(f, neuf);
  console.log("✓ index.html mis à jour (" + lignes.length + " fonctions)");
}

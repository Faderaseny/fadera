// Nettoie la section « Non publié » / la dernière version du journal :
//  - retire une entrée déjà présente dans une section plus ancienne (doublon) ;
//  - fusionne les titres « ### … » répétés dans une même section.
//   usage : node windows/nettoyer_journal.js 0.8.5
const fs = require("fs");
const v = process.argv[2];
if (!v) { console.error("usage : node windows/nettoyer_journal.js 0.8.5"); process.exit(2); }
const brut = fs.readFileSync("CHANGELOG.md", "utf8");
const eol = brut.includes("\r\n") ? "\r\n" : "\n";
const t = brut.replace(/\r\n/g, "\n");
const i = t.indexOf("## [" + v + "]");
if (i < 0) { console.error("Section " + v + " introuvable."); process.exit(1); }
const j = t.indexOf("\n## [", i + 5);
const bloc = t.slice(i, j < 0 ? t.length : j), reste = j < 0 ? "" : t.slice(j);
const avant = t.slice(0, i), anterieur = reste;
// entrées = lignes commençant par « - » avec leurs lignes de continuation
const lignes = bloc.split("\n");
const entete = lignes.shift();
const sections = []; let courante = null;
for (const l of lignes) {
  if (/^### /.test(l)) { courante = sections.find((s) => s.titre === l.trim()) || (sections.push({ titre: l.trim(), entrees: [] }), sections[sections.length - 1]); continue; }
  if (!courante) continue;
  if (/^- /.test(l)) courante.entrees.push([l]); else if (l.trim() && courante.entrees.length) courante.entrees[courante.entrees.length - 1].push(l);
}
const cle = (e) => e.join(" ").replace(/\s+/g, " ").slice(0, 60);
let retirees = 0;
for (const s of sections) s.entrees = s.entrees.filter((e) => { const doublon = anterieur.replace(/\s+/g, " ").includes(cle(e)); if (doublon) retirees++; return !doublon; });
const sortie = [entete, ""].concat(...sections.filter((s) => s.entrees.length).map((s) => [s.titre, ...[].concat(...s.entrees), ""]));
fs.writeFileSync("CHANGELOG.md", (avant + sortie.join("\n") + (reste ? "\n" + reste.replace(/^\n/, "") : "")).replace(/\n/g, eol));
console.log("Journal nettoyé : " + retirees + " doublon(s) retiré(s), titres fusionnés.");

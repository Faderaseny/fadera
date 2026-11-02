// Réinsère dans CHANGELOG.md les sections 0.8.2 à 0.8.4 si elles ont été écrasées par un ancien fichier.
//   usage : node windows/restaurer_journal.js <commit-où-elles-existaient>   (ex. : node windows/restaurer_journal.js 9748d7b~1)
const fs = require("fs"), cp = require("child_process");
const ref = process.argv[2];
if (!ref) { console.error("usage : node windows/restaurer_journal.js <commit>"); process.exit(2); }
const ancien = cp.execFileSync("git", ["show", ref + ":CHANGELOG.md"], { encoding: "utf8", maxBuffer: 1 << 26 });
let actuel = fs.readFileSync("CHANGELOG.md", "utf8");
const eol = actuel.includes("\r\n") ? "\r\n" : "\n";
const norm = (t) => t.replace(/\r\n/g, "\n");
const a = norm(ancien); actuel = norm(actuel);
const debut = a.indexOf("## [0.8.4]"), fin = a.indexOf("## [0.8.1]");
if (debut < 0 || fin < 0 || fin < debut) { console.error("Les sections 0.8.4 à 0.8.2 ne sont pas dans " + ref + " : essayez un commit plus ancien (git log --oneline -- CHANGELOG.md)."); process.exit(1); }
if (actuel.includes("## [0.8.4]")) { console.log("Le journal contient déjà la section 0.8.4 : rien à faire."); process.exit(0); }
const bloc = a.slice(debut, fin);
const i = actuel.indexOf("## [0.8.1]");
if (i < 0) { console.error("## [0.8.1] introuvable dans le journal actuel."); process.exit(1); }
fs.writeFileSync("CHANGELOG.md", (actuel.slice(0, i) + bloc + actuel.slice(i)).replace(/\n/g, eol));
console.log("Sections 0.8.4, 0.8.3 et 0.8.2 réinsérées (" + bloc.split("\n").length + " lignes).");

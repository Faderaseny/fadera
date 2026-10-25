// Banc d'essai de vitesse du moteur Rust (ligne de commande).
//   node fadera-rs/tests/vitesse.js <index.html> <fadera[.exe]>
// Il exécute les programmes de « moteur bench tout » (définis dans index.html) et trois programmes plus lourds,
// affiche les durées, et échoue si l'un d'eux dépasse sa limite. Les limites sont volontairement très larges
// (20 à 100 fois les durées normales) : le but est de détecter une régression catastrophique (une boucle devenue
// quadratique, un cache perdu…), pas de mesurer finement une machine de CI qui varie d'un jour à l'autre.
const fs = require("fs"), cp = require("child_process"), os = require("os"), path = require("path");
const [, , htmlPath, bin] = process.argv;
if (!htmlPath || !bin) { console.error("usage : node vitesse.js index.html chemin/vers/fadera"); process.exit(2); }

const html = fs.readFileSync(htmlPath, "utf8");
const debut = html.indexOf("const SUITE_BENCH");
if (debut < 0) { console.error("SUITE_BENCH introuvable dans " + htmlPath); process.exit(2); }
const suite = new Function(html.slice(debut, html.indexOf("];", debut) + 2).replace("const SUITE_BENCH", "var SUITE_BENCH") + "\nreturn SUITE_BENCH;")();

const LIMITE_SUITE_MS = 3000;
const lourds = [
  ["boucle de 5 millions", 5000, "soit s = 0\npour i allantde 1 a 5000000 faire\n  s = s + i % 7\nfin\nafficher s", "15000000"],
  ["fib(27)", 5000, "fonction fib(n)\n  si n < 2 alors\n    renvoyer n\n  fin\n  renvoyer fib(n - 1) + fib(n - 2)\nfin\nafficher fib(27)", "196418"],
  ["tri de 200 000", 8000, "soit l = []\npour i allantde 1 a 200000 faire\n  ajouter(l, (i * 7919) % 10007)\nfin\nsoit t = trier(l, (a, b) => a - b)\nafficher taille(t) + \" \" + lireElement(t, 0) + \" \" + lireElement(t, 199999)", "200000 0 10006"],
];
const programmes = [
  ...suite.map(([nom, code]) => [nom, LIMITE_SUITE_MS, code, null]),
  ...lourds,
];

let ko = 0;
console.log("programme".padEnd(24) + "durée".padStart(10) + "   limite");
for (const [nom, limite, code, attendu] of programmes) {
  const f = path.join(os.tmpdir(), "vitesse_" + process.pid + ".fa");
  fs.writeFileSync(f, code);
  const t0 = process.hrtime.bigint();
  const r = cp.spawnSync(bin, ["run", f], { encoding: "utf8", timeout: limite * 4 });
  const ms = Number(process.hrtime.bigint() - t0) / 1e6;
  fs.unlinkSync(f);
  let probleme = "";
  if (r.status !== 0) probleme = "erreur : " + (r.stderr || r.stdout || r.error || "").toString().slice(0, 120);
  else if (attendu !== null && r.stdout.trim() !== attendu) probleme = "résultat inattendu : " + r.stdout.trim().slice(0, 60);
  else if (ms > limite) probleme = "trop lent";
  if (probleme) ko++;
  console.log(nom.padEnd(24) + (ms.toFixed(0) + " ms").padStart(10) + "   ≤ " + limite + " ms" + (probleme ? "   ✗ " + probleme : ""));
}
console.log(ko ? `\n✗ ${ko} programme(s) hors limite.` : `\n✓ ${programmes.length} programmes dans les limites de vitesse.`);
process.exit(ko ? 1 : 0);

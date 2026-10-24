#!/usr/bin/env node
// Parité du DÉBOGUEUR : mêmes programmes, mêmes commandes (plan), mêmes points d'arrêt — le moteur JS (index.html) et le moteur
// Rust (`fadera --deboguer-trace`) doivent s'arrêter aux mêmes lignes, avec les mêmes variables et la même pile.
//   usage : node tests/parite_debogueur.js [chemin/index.html] [chemin/binaire fadera]
const fs = require("fs"), path = require("path"), cp = require("child_process");
const racine = path.resolve(__dirname, "..");
const DOSSIER_LIVRES = process.env.FADERA_DOSSIER ? path.resolve(process.env.FADERA_DOSSIER) : path.join(racine, "..");
const html = fs.readFileSync(path.resolve(process.argv[2] || path.join(DOSSIER_LIVRES, "index.html")), "utf8");
const bin = path.resolve(process.argv[3] || path.join(racine, "target", "release", "fadera"));
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis",
  html.slice(a, b) + "\nreturn {Interpreteur,Debogueur,analyserCode,decrireErreur,SignalStop};")({ exports: {} }, require, undefined, globalThis);
const srcTests = fs.readFileSync(path.join(DOSSIER_LIVRES, "tests_fadera.js"), "utf8");
const FICH = new Function("return " + /^const FICH = (\{.*\});\s*$/m.exec(srcTests)[1])();
const DOSSIER = fs.mkdtempSync(path.join(require("os").tmpdir(), "fad_dbg_"));
for (const [n, c] of Object.entries(FICH)) { const d = path.join(DOSSIER, n); fs.mkdirSync(path.dirname(d), { recursive: true }); fs.writeFileSync(d, c); }

async function traceJS(code, plan, points) {
  const out = [];
  const hote = { afficher: (t) => out.push("out:" + t), saisir: () => "", lireFichier: (p) => { if (p in FICH) return FICH[p]; throw new Error("absent"); }, ecrireFichier: () => {} };
  const it = new M.Interpreteur(hote);
  plan = plan.slice();
  const vars = (l, trier) => { let x = l.map((v) => v.nom + "=" + M.Debogueur.resumer(v.valeur)); if (trier) x.sort(); return x.join(";"); };
  const d = new M.Debogueur({
    pause: async (it2, l, r) => {
      const s = M.Debogueur.instantane(it2, l);
      out.push("L" + l + " " + r + " | " + vars(s.locales) + " | " + vars(s.globales, true) + " | " + s.pile.map((p) => p.nom + ":" + p.ligne).join(","));
      for (;;) {
        const cmd = plan.length ? plan.shift() : "continuer";
        if (cmd.startsWith("eval:")) {
          const r2 = await M.Debogueur.evaluer(it2, cmd.slice(5));
          out.push("  eval " + (r2.ok ? "ok" : "erreur") + " : " + r2.texte);
          continue;
        }
        if (cmd === "stop") { d.arreter(); return; }
        d.reprendre(cmd);
        return;
      }
    },
  });
  points.forEach((p) => d.points.add(p));
  d.mode = points.length ? "continuer" : "entrer";
  it.dbg = d;
  try { await it.executer(M.analyserCode(code)); } catch (e) {
    if (e instanceof M.SignalStop) out.push("ERR:■ Exécution arrêtée.");
    else if (e && e.constructor && e.constructor.name === "SignalRetour") {}
    else out.push("ERR:" + M.decrireErreur(e, code).texte);
  }
  return out.join("\n");
}
function traceRust(code, plan, points, i) {
  const f = path.join(DOSSIER, "dbg_" + i + ".fa");
  fs.writeFileSync(f, code);
  const r = cp.spawnSync(bin, ["--deboguer-trace", f, plan.join(";;"), points.join(",")], { encoding: "utf8", timeout: 60000 });
  fs.unlinkSync(f);
  return r.stdout.replace(/\n$/, "") + (r.status > 1 ? "\n<<rust status " + r.status + " " + (r.stderr || "").slice(0, 300) + ">>" : "");
}

const FN = "fonction f(x)\n  soit y = x * 2\n  renvoyer y\nfin\nsoit a = f(3)\nafficher a";
const CLS = 'classe Point\n  fonction creer(x, y)\n    soi.x = x\n    soi.y = y\n  fin\n  fonction norme()\n    renvoyer racine(soi.x ** 2 + soi.y ** 2)\n  fin\nfin\nclasse P3 herite Point\n  fonction creer(x, y, z)\n    Point.creer(soi, x, y)\n    soi.z = z\n  fin\nfin\nsoit p = P3(3, 4, 5)\nafficher p.norme()\nafficher p.z';
const cas = [
  ["une pause par instruction", "soit a = 1\nsoit b = 2\nafficher a + b", ["entrer", "entrer"], []],
  ["passer saute la fonction", FN, ["passer", "passer"], []],
  ["entrer descend", FN, ["entrer", "entrer", "entrer"], []],
  ["sortir termine la fonction", FN, ["entrer", "sortir"], []],
  ["passer dans la fonction", FN, ["entrer", "passer", "passer"], []],
  ["continuer", FN, ["continuer"], []],
  ["point dans une boucle", "soit s = 0\npour i allantde 1 a 3 faire\n  s = s + i\nfin\nafficher s", [], [3]],
  ["point dans une fonction", FN, [], [3]],
  ["récursion", "fonction g(n)\n  si n > 0 alors\n    g(n - 1)\n  fin\nfin\ng(2)", [], [2]],
  ["tantque + pour chaque", "soit n = 0\ntantque n < 2 faire\n  n = n + 1\nfin\npour chaque c dans [7, 8] faire\n  afficher c\nfin", ["entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer"], []],
  ["si / sinon si / sinon", "soit a = 5\nsi a < 3 alors\n  afficher 1\nsinon si a < 6 alors\n  afficher 2\nsinon\n  afficher 3\nfin", ["entrer", "entrer", "entrer", "entrer", "entrer"], []],
  ["selon", 'soit a = 2\nselon a\n  cas 1\n    afficher "un"\n  cas 2\n    afficher "deux"\n  defaut\n    afficher "autre"\nfin', ["entrer", "entrer", "entrer", "entrer"], []],
  ["essayer / attraper", 'essayer\n  soit a = 1 / 0\nattraper e\n  afficher e\nfin\nafficher "fin"', ["entrer", "entrer", "entrer", "entrer", "entrer"], []],
  ["lambda et transformer", "soit d = x => x * 2\nsoit r = transformer([1, 2], d)\nafficher r", ["entrer", "entrer", "entrer", "entrer", "entrer"], []],
  ["fonction passée à une native", "fonction d(x)\n  renvoyer x * 2\nfin\nsoit r = transformer([1, 2], d)\nafficher r", ["entrer", "entrer", "entrer", "entrer", "entrer", "entrer"], []],
  ["classes : méthodes, héritage, constructeur généré", CLS, ["entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer", "entrer"], []],
  ["classes : passer sur le constructeur", CLS, ["entrer", "passer", "passer"], []],
  ["module importé : aucune pause dedans", 'importer "outils"\nafficher double(2)', ["entrer", "entrer", "entrer"], []],
  ["module avec alias", 'importer "outils" comme o\nafficher o.double(2)', ["entrer", "entrer", "entrer"], []],
  ["module standard", 'importer "std/listes"\nafficher premier([4, 5])', ["entrer", "entrer", "entrer"], []],
  ["déclaration de fonction sans pause", "fonction f()\n  renvoyer 1\nfin\nafficher f()", ["entrer", "entrer", "entrer"], []],
  ["évaluation d'une expression", "soit a = 5\nsoit b = 6\nafficher a * b", ["entrer", "eval:a + 1", "eval:a * 2 + b", "eval:nope", "eval:", "entrer"], []],
  ["évaluation d'instructions", "soit a = 5\nafficher a", ["entrer", "eval:soit z = 9", "eval:a = 7", "eval:afficher z", "entrer"], []],
  ["évaluation dans une fonction", FN, ["entrer", "entrer", "eval:x + y", "eval:x * 10", "continuer"], []],
  ["évaluation : appel de fonction", FN, ["entrer", "eval:f(4)", "continuer"], []],
  ["arrêt", "soit a = 1\nsoit b = 2\nafficher \"fin\"", ["entrer", "stop"], []],
  ["arrêt non rattrapé par essayer", "essayer\n  soit a = 1\n  soit b = 2\nattraper e\n  afficher \"pris\"\nfin\nafficher \"fin\"", ["entrer", "entrer", "entrer", "stop"], []],
  ["arrêt dans une fonction native", "fonction d(x)\n  renvoyer x * 2\nfin\nsoit r = transformer([1, 2, 3], d)\nafficher r", ["entrer", "entrer", "stop"], []],
  ["erreur d'exécution pendant le pas à pas", "soit a = 1\nsoit b = a / 0\nafficher b", ["entrer", "entrer", "entrer"], []],
  ["structures et listes (résumé)", 'soit s = {a: 1, b: [1, 2]}\nsoit l = liste(1)\npour i allantde 1 a 40 faire\n  ajouter(l, i)\nfin\nafficher taille(l)', ["entrer", "entrer", "entrer", "entrer", "continuer"], [5]],
  ["texte long (résumé tronqué)", 'soit t = "x"\npour i allantde 1 a 7 faire\n  t = t + "0123456789012345678901234567890123456789"\nfin\nafficher taille(t)', [], [6]],
  ["points d'arrêt multiples", "soit s = 0\npour i allantde 1 a 3 faire\n  s = s + i\n  afficher s\nfin", ["continuer", "passer", "continuer"], [3, 4]],
  ["lignes vides et commentaires", "# début\n\nsoit a = 1\n\n# milieu\nsoit b = 2\nafficher a + b", ["entrer", "entrer"], []],
  ["sortir depuis la fonction imbriquée", "fonction g()\n  soit k = 1\n  renvoyer k\nfin\nfonction f()\n  soit r = g()\n  renvoyer r\nfin\nafficher f()", ["entrer", "entrer", "entrer", "sortir", "sortir"], []],
];
(async () => {
  let ok = 0, ko = 0, i = 0;
  for (const [nom, code, plan, points] of cas) {
    i++;
    const j = await traceJS(code, plan, points), r = traceRust(code, plan, points, i);
    if (j === r) ok++;
    else { ko++; console.log("✗ " + nom + "\n--- JS ---\n" + j + "\n--- Rust ---\n" + r + "\n"); }
  }
  console.log(`Parité du débogueur JS/Rust : ${ok} identiques, ${ko} différents sur ${cas.length} scénarios.`);
  process.exit(ko ? 1 : 0);
})();

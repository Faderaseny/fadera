#!/usr/bin/env node
// Vérifie la plateforme d'apprentissage avec le VRAI moteur : chaque solution réussit tous ses tests,
// chaque code de départ échoue, et le correcteur détecte les erreurs.
//   usage : node site/apprendre/verifier_cours.js <fadera[.exe]>
const fs = require("fs"), path = require("path"), cp = require("child_process"), os = require("os");
const exe = process.argv[2] || process.env.FADERA_BIN;
if (!exe) { console.error("usage : verifier_cours.js fadera"); process.exit(2); }
const COURS = require("./cours.js");
const { corriger } = require("./correcteur.js");
let ok = 0, ko = 0;
const verifier = (nom, cond, detail) => { if (cond) ok++; else { ko++; console.log("  ✗ " + nom + (detail !== undefined ? "  → " + JSON.stringify(detail).slice(0, 400) : "")); } };
const dossier = fs.mkdtempSync(path.join(path.dirname(path.resolve(exe)), "fadera-cours-"));
// Imite le moteur WebAssembly du navigateur : l'invite de saisir() n'est pas affichée.
function executer(code, saisies) {
  const f = path.join(dossier, "p.fa");
  fs.writeFileSync(f, code.replace(/saisir\s*\(\s*(?:"[^"]*")?\s*\)/g, 'saisir("")'));
  const r = cp.spawnSync(exe, ["run", f], { input: saisies.join("\n") + "\n", encoding: "utf8", timeout: 20000 });
  if (r.status !== 0) { const m = /Ligne (\d+)/.exec(r.stderr + r.stdout); return JSON.stringify({ sortie: [], erreur: (r.stderr || r.stdout).trim().split("\n")[0], source: m ? m[1] : "" }); }
  const lignes = r.stdout.replace(/\r/g, "").split("\n"); if (lignes[lignes.length - 1] === "") lignes.pop();
  return JSON.stringify({ sortie: lignes, erreur: null });
}
try {
  const ids = new Set();
  for (const ex of COURS) {
    verifier(ex.id + " : identifiant unique", !ids.has(ex.id)); ids.add(ex.id);
    for (const champ of ["chapitre", "titre", "lecon", "exemple", "consigne", "depart", "indice", "solution"])
      verifier(ex.id + " : champ « " + champ + " » présent", typeof ex[champ] === "string" && ex[champ].trim().length > 0);
    verifier(ex.id + " : au moins 1 test", Array.isArray(ex.tests) && ex.tests.length >= 1);
    const ex_exemple = JSON.parse(executer(ex.exemple, ["3"]));
    verifier(ex.id + " : l'exemple de la leçon s'exécute sans erreur", !ex_exemple.erreur, ex_exemple.erreur);
    const bon = corriger(ex, ex.solution, executer);
    verifier(ex.id + " : la solution réussit tous les tests", bon.reussi, bon);
    const depart = corriger(ex, ex.depart, executer);
    verifier(ex.id + " : le code de départ n'est pas déjà accepté", !depart.reussi);
    // une réponse « codée en dur » (sans lire la saisie) doit échouer dès qu'il y a plusieurs tests différents
    const diffs = new Set(ex.tests.map((t) => t.sortie.join("\n")));
    if (diffs.size > 1) {
      const dur = corriger(ex, ex.tests[0].sortie.map((l) => 'afficher "' + l + '"').join("\n"), executer);
      verifier(ex.id + " : une réponse codée en dur est refusée", !dur.reussi);
    }
  }
  // le correcteur
  const ex0 = COURS[0];
  const faux = corriger(ex0, 'afficher "salut, fadéra !"', executer);
  verifier("la casse compte", !faux.reussi);
  const err = corriger(ex0, 'afficher prenon', executer);
  verifier("une erreur du programme est signalée", !err.reussi && /prenon/.test(err.erreur || ""), err);
  const espaces = corriger(ex0, 'afficher "Salut, Fadéra !"\n\n', executer);
  verifier("lignes vides finales ignorées", espaces.reussi, espaces);
  const html = fs.readFileSync(path.join(__dirname, "index.html"), "utf8");
  verifier("la page charge cours.js et correcteur.js", /src="cours\.js"/.test(html) && /src="correcteur\.js"/.test(html));
  verifier("la page charge le moteur depuis ../jouer/pkg", /\.\.\/jouer\/pkg\/fadera\.js/.test(html));
} finally {
  for (let i = 0; i < 20; i++) { try { fs.rmSync(dossier, { recursive: true, force: true }); break; } catch (e) { Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 250); } }
}
console.log(`${ok} vérification(s) réussie(s), ${ko} échouée(s).`);
process.exit(ko ? 1 : 0);

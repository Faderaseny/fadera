// Génère docs/REFERENCE.md : la référence de toutes les fonctions Fadéra.
//   node docs/generer_reference.js              écrit REFERENCE.md
//   node docs/generer_reference.js --verifier   échoue si REFERENCE.md n'est pas à jour (pour l'intégration continue)
// Chaque exemple est exécuté sur le moteur JS et sur le moteur Rust ; le résultat affiché est le résultat réel.
// La génération échoue si : un exemple est en erreur, les deux moteurs diffèrent, une fonction native n'est pas
// documentée, une fonction documentée n'existe pas, ou une fonction d'un module std/* n'est pas décrite.
const fs = require("fs"), cp = require("child_process"), path = require("path");
const RACINE = path.join(__dirname, "..");
const BIN = process.env.FADERA_BIN || path.join(RACINE, "fadera-rs/target/release/fadera");
const SORTIE = path.join(__dirname, "REFERENCE.md");
const verifier = process.argv.includes("--verifier");
process.env.TZ = "UTC";

const html = fs.readFileSync(path.join(RACINE, "index.html"), "utf8");
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis",
  html.slice(a, b) + "\nreturn {executerProgramme,decrireErreur,FONCTIONS_NATIVES};")({ exports: {} }, require, undefined, globalThis);

async function sortieJs(code) {
  const out = [];
  const hote = { afficher: t => out.push(t), saisir: () => "", lireFichier: () => { throw new Error("absent"); }, ecrireFichier: () => {} };
  try { await M.executerProgramme(code, hote); } catch (e) { out.push("ERR:" + M.decrireErreur(e, code).texte); }
  return out.join("\n");
}
function sortieRs(code) {
  const f = path.join(require("os").tmpdir(), "ref_" + process.pid + ".fa");
  fs.writeFileSync(f, code);
  const r = cp.spawnSync(BIN, ["--parite", f], { encoding: "utf8", env: { ...process.env, TZ: "UTC" } });
  fs.unlinkSync(f);
  return r.stdout + (r.status ? "\n<<rust statut " + r.status + " " + (r.stderr || "").slice(0, 200) + ">>" : "");
}
async function executer(code, etiquette, erreurs) {
  const j = await sortieJs(code), r = sortieRs(code);
  if (j !== r) erreurs.push(`${etiquette} : les deux moteurs diffèrent\n   JS   : ${j}\n   Rust : ${r}`);
  else if (j.includes("ERR:")) erreurs.push(`${etiquette} : erreur → ${j}`);
  return j;
}

const LANGAGE = new Set(["afficher", "saisir", "liste", "ajouter", "taille", "lireElement", "modifierElement", "lireCle", "modifierCle", "hasard", "lireFichier", "ecrireFichier"]);
const INTERNES = new Set(["__instance"]);
// Fonctions d'aide internes des modules std, volontairement non documentées
const AIDES_STD = new Set(["nettoyer_droite", "aplatir_vers", "inverser_liste", "comparer_naturel", "comparer_inverse", "ordre_naturel", "fusionner_triees", "comparer_multi"]);
const nomDe = sig => sig.match(/^[A-Za-z_0-9]+/)[0];

(async () => {
  const erreurs = [];
  // ---- 1. lire la source
  const sections = []; let cur = null;
  for (const brut of fs.readFileSync(path.join(__dirname, "reference/fonctions.txt"), "utf8").split("\n")) {
    const l = brut.trimEnd();
    if (!l || l.startsWith("#!")) continue;
    if (l.startsWith("## ")) { if (cur) cur.notes.push(l.slice(3)); continue; }
    if (l.startsWith("# ")) { cur = { titre: l.slice(2), notes: [], fonctions: [] }; sections.push(cur); continue; }
    const p = l.split("|").map(x => x.trim());
    if (p.length < 2) { erreurs.push("ligne illisible : " + l); continue; }
    cur.fonctions.push({ sig: p[0], desc: p[1], ex: p.slice(2).join("|").trim() });
  }
  // ---- 2. complétude
  const documentees = new Set(sections.flatMap(s => s.fonctions.map(f => nomDe(f.sig))));
  const natives = Object.keys(M.FONCTIONS_NATIVES).filter(n => !INTERNES.has(n));
  for (const n of natives) if (!documentees.has(n)) erreurs.push(`fonction native non documentée : ${n}`);
  for (const n of LANGAGE) if (!documentees.has(n)) erreurs.push(`fonction du langage non documentée : ${n}`);
  for (const n of documentees) if (!M.FONCTIONS_NATIVES[n] && !LANGAGE.has(n)) erreurs.push(`documentée mais inexistante : ${n}`);
  // ---- 3. exécuter les exemples
  let nbEx = 0;
  for (const s of sections) for (const f of s.fonctions) {
    if (!f.ex) continue;
    nbEx++;
    f.res = await executer("afficher " + f.ex, nomDe(f.sig), erreurs);
  }
  // ---- 4. modules std
  const dirStd = path.join(RACINE, "fadera-rs/src/std"), modules = [];
  for (const fich of fs.readdirSync(dirStd).filter(x => x.endsWith(".fa") && x !== "tests_std.fa").sort()) {
    const nom = fich.replace(/\.fa$/, ""), src = fs.readFileSync(path.join(dirStd, fich), "utf8");
    const entete = [];
    for (const l of src.split("\n")) { if (!l.startsWith("#")) break; entete.push(l.replace(/^#\s?/, "")); }
    // chaque fonction ou classe publique doit apparaître dans l'en-tête
    const texteEntete = entete.join("\n");
    for (const m of src.matchAll(/^(?:fonction|classe)\s+([A-Za-z_0-9]+)/gm)) {
      const id = m[1];
      if (!new RegExp("(^|[^A-Za-z_0-9])" + id + "\\b").test(texteEntete) && !AIDES_STD.has(id) && !/^_/.test(id) && !/^[a-z_]+_$/.test(id))
        erreurs.push(`std/${nom} : « ${id} » n'est pas décrit dans l'en-tête du module`);
    }
    const chemin = path.join(__dirname, "reference/modules_exemples", nom + ".fa");
    let ex = null, res = null;
    if (fs.existsSync(chemin)) { ex = fs.readFileSync(chemin, "utf8").trim(); res = await executer(ex, "std/" + nom, erreurs); }
    else erreurs.push(`std/${nom} : exemple manquant (docs/reference/modules_exemples/${nom}.fa)`);
    modules.push({ nom, entete: entete.slice(1).join("\n").trim(), ex, res });
  }
  if (erreurs.length) { console.error("✗ Référence non générée :\n - " + erreurs.join("\n - ")); process.exit(1); }

  // ---- 5. écrire le Markdown
  const L = [];
  const ancre = t => t.toLowerCase().normalize("NFD").replace(/[̀-ͯ]/g, "").replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
  L.push("# Référence des fonctions Fadéra", "",
    "> Ce fichier est **généré** par `node docs/generer_reference.js` : chaque exemple a été exécuté sur les deux moteurs (navigateur et `fadera`) et le résultat affiché est le résultat réel. Ne le modifiez pas à la main : éditez `docs/reference/fonctions.txt`.", "",
    `${natives.length} fonctions natives, ${LANGAGE.size} fonctions du langage, ${modules.length} modules standard. Pour une première approche, lisez plutôt le [tutoriel](TUTORIEL.md).`, "",
    "## Sommaire", "");
  for (const s of sections) L.push(`- [${s.titre}](#${ancre(s.titre)})`);
  L.push("- [Modules standard (`importer \"std/…\"`)](#modules-standard)", "");
  for (const s of sections) {
    L.push(`## ${s.titre}`, "");
    for (const n of s.notes) L.push(n, "");
    for (const f of s.fonctions) {
      L.push(`**\`${f.sig}\`** — ${f.desc}`, "");
      if (f.ex) {
        if (f.res.includes("\n")) L.push("```fadera", `afficher ${f.ex}`, "```", "", "Affiche :", "", "```", f.res, "```", "");
        else L.push("```fadera", `afficher ${f.ex}`, `# → ${f.res}`, "```", "");
      }
    }
  }
  L.push("## Modules standard", "",
    "Ces modules sont écrits en Fadéra et inclus dans l'exécutable et dans l'IDE : `importer \"std/nom\"`. Ils s'utilisent avec `importer \"std/nom\" comme alias` (puis `alias.fonction(…)`) ou directement.", "");
  for (const m of modules) {
    L.push(`### std/${m.nom}`, "", "```", m.entete, "```", "", "Exemple :", "", "```fadera", m.ex, "```", "", "Résultat :", "", "```", m.res, "```", "");
  }
  const texte = L.join("\n").replace(/\n{3,}/g, "\n\n");
  const final = texte.trimEnd() + "\n";
  if (verifier) {
    if (!fs.existsSync(SORTIE) || fs.readFileSync(SORTIE, "utf8").replace(/\r\n/g, "\n") !== final) { console.error("✗ docs/REFERENCE.md n'est pas à jour : lancez `node docs/generer_reference.js`."); process.exit(1); }
    console.log(`✓ REFERENCE.md à jour (${nbEx} exemples + ${modules.length} modules, tous identiques sur les deux moteurs).`);
  } else {
    fs.writeFileSync(SORTIE, final);
    console.log(`✓ docs/REFERENCE.md écrit : ${natives.length} natives, ${nbEx} exemples exécutés, ${modules.length} modules.`);
  }
})();

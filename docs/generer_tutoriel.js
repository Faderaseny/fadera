// Génère docs/TUTORIEL.md à partir de docs/tutoriel/TUTORIEL.src.md.
//   node docs/generer_tutoriel.js              écrit TUTORIEL.md
//   node docs/generer_tutoriel.js --verifier   échoue si TUTORIEL.md n'est pas à jour (intégration continue)
// Blocs de code du source :
//   ```fadera           exécuté sur les deux moteurs ; le résultat réel est ajouté sous le bloc
//   ```fadera erreur    doit se terminer par une erreur, qui est affichée
//   ```libre            non exécuté (réseau, saisie, plusieurs fichiers…) mais vérifié syntaxiquement
// La génération échoue si un résultat diffère entre les deux moteurs, si un bloc se comporte autrement que prévu,
// ou si un lien relatif pointe vers un fichier qui n'existe pas.
const fs = require("fs"), cp = require("child_process"), os = require("os"), path = require("path");
const RACINE = path.join(__dirname, "..");
const BIN = process.env.FADERA_BIN || path.join(RACINE, "fadera-rs/target/release/fadera");
const SRC = path.join(__dirname, "tutoriel/TUTORIEL.src.md"), SORTIE = path.join(__dirname, "TUTORIEL.md");
const verifier = process.argv.includes("--verifier");
process.env.TZ = "UTC";
const html = fs.readFileSync(path.join(RACINE, "index.html"), "utf8");
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis",
  html.slice(a, b) + "\nreturn {executerProgramme,decrireErreur};")({ exports: {} }, require, undefined, globalThis);

async function sortieJs(code) {
  const out = [], fx = {};
  const hote = { afficher: t => out.push(t), saisir: () => "",
    lireFichier: p => { if (p in fx) return fx[p]; throw new Error("absent"); }, ecrireFichier: (p, c) => { fx[p] = c; } };
  try { await M.executerProgramme(code, hote); } catch (e) { out.push("ERR:" + M.decrireErreur(e, code).texte); }
  return out.join("\n");
}
const tmp = ext => path.join(os.tmpdir(), "tuto_" + process.pid + ext);
function sortieRs(code) {
  const f = tmp(".fa"); fs.writeFileSync(f, code);
  const r = cp.spawnSync(BIN, ["--parite", f], { encoding: "utf8", env: { ...process.env, TZ: "UTC" } });
  fs.unlinkSync(f);
  return r.stdout + (r.status ? "\n<<rust statut " + r.status + " " + (r.stderr || "").slice(0, 200) + ">>" : "");
}
function syntaxeOk(code) {
  const f = tmp(".fa"); fs.writeFileSync(f, code);
  const r = cp.spawnSync(BIN, ["fmt", "--verifier", f], { encoding: "utf8" });
  fs.unlinkSync(f);
  // statut 1 = « pas au format standard » (sans importance ici) ; 2 = erreur de syntaxe
  return r.status === 2 ? (r.stdout + r.stderr).trim() : "";
}

(async () => {
  const erreurs = [], source = fs.readFileSync(SRC, "utf8");
  const sortie = [];
  const re = /```(fadera(?: erreur)?|libre)\n([\s\S]*?)```\n/g;
  let dernier = 0, m, nb = 0, nbLibre = 0;
  while ((m = re.exec(source))) {
    sortie.push(source.slice(dernier, m.index));
    dernier = re.lastIndex;
    const type = m[1], code = m[2], ligne = source.slice(0, m.index).split("\n").length;
    if (type === "libre") {
      nbLibre++;
      const e = syntaxeOk(code);
      if (e) erreurs.push(`ligne ${ligne} (bloc libre) : syntaxe invalide → ${e}`);
      sortie.push("```fadera\n" + code + "```\n");
      continue;
    }
    nb++;
    const j = await sortieJs(code), r = sortieRs(code);
    if (j !== r) erreurs.push(`ligne ${ligne} : les deux moteurs diffèrent\n   JS   : ${j}\n   Rust : ${r}`);
    const estErr = j.includes("ERR:");
    if (type === "fadera" && estErr) erreurs.push(`ligne ${ligne} : erreur inattendue → ${j}`);
    if (type === "fadera erreur" && !estErr) erreurs.push(`ligne ${ligne} : une erreur était attendue`);
    sortie.push("```fadera\n" + code + "```\n");
    if (j.trim()) sortie.push("\nRésultat :\n\n```\n" + j.replace(/^ERR:/gm, "Erreur : ") + "\n```\n");
  }
  sortie.push(source.slice(dernier));
  const texte = sortie.join("");
  // liens relatifs
  for (const l of texte.matchAll(/\]\(([^)#:]+?)(?:#[^)]*)?\)/g)) {
    if (!fs.existsSync(path.join(__dirname, l[1]))) erreurs.push(`lien cassé : ${l[1]}`);
  }
  if (erreurs.length) { console.error("✗ Tutoriel non généré :\n - " + erreurs.join("\n - ")); process.exit(1); }
  if (verifier) {
    if (!fs.existsSync(SORTIE) || fs.readFileSync(SORTIE, "utf8").replace(/\r\n/g, "\n") !== texte) { console.error("✗ docs/TUTORIEL.md n'est pas à jour : lancez `node docs/generer_tutoriel.js`."); process.exit(1); }
    console.log(`✓ TUTORIEL.md à jour (${nb} exemples exécutés, ${nbLibre} vérifiés syntaxiquement).`);
  } else {
    fs.writeFileSync(SORTIE, texte);
    console.log(`✓ docs/TUTORIEL.md écrit : ${nb} exemples exécutés, ${nbLibre} vérifiés syntaxiquement.`);
  }
})();

#!/usr/bin/env node
// Vérifie SPEC_FADERA.md : chaque bloc ```fa-test est exécuté par le moteur JS (index.html) ET par le moteur Rust ;
// les deux sorties doivent être identiques à celle du bloc ```sortie qui le suit.
//   usage : node verifier_spec.js [SPEC_FADERA.md] [index.html] [binaire fadera]
const fs = require("fs"), path = require("path"), cp = require("child_process"), os = require("os");
const spec = fs.readFileSync(path.resolve(process.argv[2] || "SPEC_FADERA.md"), "utf8").replace(/\r\n/g, "\n");
const html = fs.readFileSync(path.resolve(process.argv[3] || "index.html"), "utf8");
const bin = path.resolve(process.argv[4] || path.join("fadera-rs", "target", "release", "fadera"));
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis",
  html.slice(a, b) + "\nreturn {executerProgramme,decrireErreur};")({ exports: {} }, require, undefined, globalThis);
const dossier = fs.mkdtempSync(path.join(os.tmpdir(), "fad_spec_"));

async function runJS(code) {
  const out = [];
  const hote = { afficher: (t) => out.push(t), saisir: () => "", lireFichier: () => { throw new Error("absent"); }, ecrireFichier: () => {} };
  try { await M.executerProgramme(code, hote); } catch (e) { out.push("ERR:" + M.decrireErreur(e, code).texte); }
  return out.join("\n");
}
function runRust(code, i) {
  const f = path.join(dossier, "s" + i + ".fa");
  fs.writeFileSync(f, code);
  const r = cp.spawnSync(bin, ["--parite", f, dossier], { encoding: "utf8", timeout: 60000 });
  return r.stdout + (r.status ? "\n<<rust status " + r.status + " " + (r.stderr || "").slice(0, 300) + ">>" : "");
}

const re = /```fa-test\n([\s\S]*?)```\s*\n```sortie\n([\s\S]*?)```/g;
const cas = [...spec.matchAll(re)].map((m) => ({ code: m[1], att: m[2].replace(/\n$/, "") }));
const nbBlocs = (spec.match(/```fa-test/g) || []).length;
(async () => {
  if (cas.length !== nbBlocs) { console.error(`✗ ${nbBlocs} blocs fa-test mais ${cas.length} paires fa-test/sortie : un bloc n'est pas suivi de son bloc sortie.`); process.exit(2); }
  let ok = 0, ko = 0;
  for (let i = 0; i < cas.length; i++) {
    const c = cas[i], j = await runJS(c.code), r = runRust(c.code, i);
    const ligne = spec.slice(0, spec.indexOf(c.code)).split("\n").length;
    if (j === c.att && r === c.att) ok++;
    else {
      ko++;
      console.log(`✗ exemple ${i + 1} (SPEC, ligne ${ligne})\n--- code ---\n${c.code}--- attendu ---\n${c.att}\n--- JS ---\n${j}\n--- Rust ---\n${r}\n`);
    }
  }
  console.log(`Spécification : ${ok} exemples conformes sur ${cas.length} (moteurs JS et Rust), ${ko} en écart.`);
  process.exit(ko ? 1 : 0);
})();

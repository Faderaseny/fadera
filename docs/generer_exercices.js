// Génère docs/EXERCICES.md à partir de docs/exercices/*.fa (+ .sortie).
//   node docs/generer_exercices.js            écrit docs/EXERCICES.md
//   node docs/generer_exercices.js --verifier échoue si EXERCICES.md n'est pas à jour
// Chaque fichier commence par des lignes de commentaire :
//   # Exercice N — Titre
//   # Domaine : … · Niveau : ★…
//   # Énoncé :
//   # …
//   # Corrigé :
// puis le code du corrigé. L'exécution des corrigés est vérifiée par docs/verifier_exemples.js exercices.
const fs = require("fs"), path = require("path");
const DOSSIER = path.join(__dirname, "exercices"), SORTIE = path.join(__dirname, "EXERCICES.md");
const SANS_REFERENCE = {
  "132_http_json_tolerant": "Cet exercice interroge internet : le résultat dépend de votre connexion (sans réseau, le message poli s'affiche).",
  "152_xor_reseau_neurones": "L'initialisation du réseau est aléatoire : le résultat attendu est 0, 1, 1, 0, mais l'apprentissage peut parfois échouer ; relancez-le.",
};
const INTRO_DOMAINE = {
  "Industrie et métiers": `Fadéra n'est pas un langage pour piloter une machine en temps réel : c'est un outil pour **automatiser les calculs, les contrôles et les rapports** d'une petite ou moyenne entreprise, et pour **prototyper des règles métier** dans un français que le responsable de production, le comptable ou le technicien peut relire. Ces exercices reprennent des situations concrètes : qualité, maintenance, stocks, facturation, logistique, énergie, agriculture, finance, télécoms et sécurité. Les chiffres sont inventés, mais les calculs sont ceux du métier.`,
};
const exos = [];
for (const f of fs.readdirSync(DOSSIER).filter(x => x.endsWith(".fa")).sort()) {
  const id = f.replace(/\.fa$/, "");
  const lignes = fs.readFileSync(path.join(DOSSIER, f), "utf8").replace(/\r\n/g, "\n").split("\n");
  const iEnonce = lignes.findIndex(l => l.startsWith("# Énoncé :")), iCorrige = lignes.findIndex(l => l.startsWith("# Corrigé :"));
  const m = /^# Exercice (\d+) — (.+)$/.exec(lignes[0]), d = /^# Domaine : (.+) · Niveau : (★+)$/.exec(lignes[1]);
  if (!m || !d || iEnonce < 0 || iCorrige < 0) { console.error("En-tête invalide dans " + f); process.exit(1); }
  if (Number(m[1]) !== Number(id.slice(0, 3))) { console.error("Numéro incohérent dans " + f); process.exit(1); }
  const enonce = lignes.slice(iEnonce + 1, iCorrige).map(l => l.replace(/^# ?/, "")).join("\n").trim();
  const code = lignes.slice(iCorrige + 1).join("\n").trim();
  const ref = path.join(DOSSIER, id + ".sortie");
  const sortie = fs.existsSync(ref) ? fs.readFileSync(ref, "utf8").replace(/\n+$/, "") : null;
  exos.push({ id, n: Number(m[1]), titre: m[2], domaine: d[1], niveau: d[2], enonce, code, sortie, tester: /^tester "/m.test(code) });
}
const domaines = [...new Set(exos.map(e => e.domaine))];
const ancre = t => t.toLowerCase().normalize("NFD").replace(/[̀-ͯ]/g, "").replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
let md = `# Exercices Fadéra

${exos.length} exercices corrigés, classés par domaine, avec leur niveau (★ facile, ★★ moyen, ★★★ plus long), couvrant tout ce que Fadéra sait faire :
calculs, décisions et boucles, textes, listes, structures, fonctions, erreurs, fichiers (CSV, JSON), dates, mathématiques et statistiques,
classes, modules standard, tests intégrés, réseau, chiffrement, intelligence artificielle, robotique et voix, jeux, mini-projets
et applications dans l'industrie et les métiers (qualité, maintenance, stocks, facturation, logistique, énergie, agriculture, finance, télécoms, sécurité).

## Comment s'en servir

1. Lisez l'énoncé et **essayez sans regarder le corrigé** : copiez-le dans l'éditeur, lancez, corrigez.
2. Cliquez sur « Corrigé » pour comparer. Il y a souvent plusieurs bonnes solutions : seul le résultat compte.
3. Chaque corrigé est aussi un fichier prêt à l'emploi dans [\`docs/exercices/\`](exercices/) ; lancez-le avec \`fadera run docs/exercices/013_fizzbuzz.fa\`.

Les corrigés sont **exécutés automatiquement sur les deux moteurs** de Fadéra à chaque modification du projet
(\`node docs/verifier_exemples.js exercices\`) : le résultat affiché ici est le vrai.

## Sommaire

`;
for (const dom of domaines) {
  const l = exos.filter(e => e.domaine === dom);
  md += `- [${dom}](#${ancre(dom)}) — ${l.length} exercice${l.length > 1 ? "s" : ""} (${l.map(e => e.n).join(", ")})\n`;
}
for (const dom of domaines) {
  md += `\n## ${dom}\n`;
  if (INTRO_DOMAINE[dom]) md += `\n${INTRO_DOMAINE[dom]}\n`;
  for (const e of exos.filter(x => x.domaine === dom)) {
    md += `\n### Exercice ${e.n} — ${e.titre}\n\n*Niveau : ${e.niveau}*\n\n${e.enonce.split("\n").map(l => "> " + l).join("\n")}\n\n<details>\n<summary>Corrigé</summary>\n\n\`\`\`fadera\n${e.code}\n\`\`\`\n`;
    if (e.tester) md += `\nLancez les tests avec : \`fadera tester docs/exercices/${e.id}.fa\`\n`;
    else if (SANS_REFERENCE[e.id]) md += `\n${SANS_REFERENCE[e.id]}\n`;
    else if (e.sortie) md += `\nRésultat :\n\n\`\`\`\n${e.sortie}\n\`\`\`\n`;
    md += `\n</details>\n`;
  }
}
md += `
## Pour aller plus loin : exercices en ligne de commande

Ces trois exercices utilisent le terminal ou internet ; ils ne sont donc pas exécutés automatiquement.

**A. Installer un paquet.** Installez le paquet \`geometrie\` (\`fadera paquet installer Faderaseny/geometrie\`), puis écrivez un programme qui l'importe (\`importer "paquets/geometrie/principal" comme g\`) et calcule l'aire d'un triangle de côtés 3, 4 et 5 avec \`g.aire_triangle_cotes\`.
Voir [PAQUETS.md](PAQUETS.md).

**B. Créer son paquet.** Avec \`fadera paquet creer mon-outil\`, écrivez deux fonctions utiles dans \`principal.fa\`, testez-les avec des blocs \`tester\`, puis publiez le dossier dans un dépôt GitHub.

**C. Parler à un service.** Écrivez un programme qui interroge une API publique avec \`http_json\`, affiche trois champs et gère l'absence de réseau (voir l'exercice 132).
`;
if (process.argv.includes("--verifier")) {
  const actuel = fs.existsSync(SORTIE) ? fs.readFileSync(SORTIE, "utf8") : "";
  if (actuel !== md) { console.error("✗ docs/EXERCICES.md n'est pas à jour : lancez  node docs/generer_exercices.js"); process.exit(1); }
  console.log("✓ EXERCICES.md à jour (" + exos.length + " exercices)");
} else {
  fs.writeFileSync(SORTIE, md);
  console.log("✓ docs/EXERCICES.md écrit : " + exos.length + " exercices, " + domaines.length + " domaines");
}

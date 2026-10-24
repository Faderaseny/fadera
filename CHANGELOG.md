# Journal des versions

Le format suit [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/) et la numérotation [SemVer](https://semver.org/lang/fr/) :
`MAJEUR.MINEUR.CORRECTIF`. La version publiée est celle de `fadera-rs/Cargo.toml` ; l'étiquette Git est `v` + ce numéro.

## [Non publié]

## [0.6.0] — 2026-10-24

Documentation complète.

### Ajouté
- Documentation (`docs/`) : tutoriel en français en 17 chapitres (`docs/TUTORIEL.md`), 30 exemples complets avec leur sortie attendue
  (calculs, jeux, textes, CSV et JSON, dates, réseau, classes, tests), référence de toutes les fonctions natives et des 12 modules
  standard (`docs/REFERENCE.md`). Les exemples et les résultats affichés sont exécutés sur les deux moteurs ; la CI échoue si une
  fonction native n'est pas documentée ou si un exemple donne des résultats différents en JS et en Rust.

### Corrigé
- Test de l'IDE : ne plante plus quand la clé de stockage du navigateur est absente.

## [0.5.0] — 2026-10-23

Première version publiée du dépôt.

### Ajouté
- Langage : modules (`importer "fichier"`, `importer "m" comme o`), classes (`classe C herite P … fin`, `soi`, `creer`), appels chaînés,
  blocs `tester "nom" … fin` avec `verifier`, `verifier_egal`, `verifier_erreur`.
- Bibliothèque standard en Fadéra, `importer "std/…"` : dates, JSON, expressions régulières, maths, tri et recherche,
  pile, file, dictionnaire ordonné, ensemble, tas. Natif `lever(message)`.
- IDE (`index.html`) : onglets, recherche et remplacement, sauvegarde durable (IndexedDB), export et import .zip,
  coloration et complétion fondées sur l'analyseur du langage, formateur (`Maj+Alt+F`), partage par lien (`#p=…`).
- Ligne de commande : `fadera run fichier.fa`, `fadera fmt`, `fadera tester`, `fadera deboguer`, `fadera -e "code"`, mode interactif.
- Distribution : exécutables Linux, Windows (`fadera.exe`) et macOS construits par la CI, playground sur GitHub Pages,
  intégration continue (tests JS et Rust, parité JS/Rust, formateur, débogueur, spécification exécutable).

### Corrigé
- Formateur : `dans [1, 2]` et `pas -1` écrits de la même façon par les deux moteurs.

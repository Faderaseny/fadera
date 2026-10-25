# Journal des versions

Le format suit [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/) et la numérotation [SemVer](https://semver.org/lang/fr/) :
`MAJEUR.MINEUR.CORRECTIF`. La version publiée est celle de `fadera-rs/Cargo.toml` ; l'étiquette Git est `v` + ce numéro.

## [Non publié]

### Ajouté
- `docs/PAQUETS_CONNUS.md` : la liste des paquets publiés (`geometrie`, `notes`) et comment ajouter le sien.
- Banc d'essai de vitesse du moteur Rust (`fadera-rs/tests/vitesse.js`), lancé par `node verifier_tout.js` et donc par la CI :
  12 programmes avec des limites très larges, pour détecter une régression catastrophique de performance.

## [0.7.0] — 2026-10-25

### Ajouté
- **Gestionnaire de paquets** : `fadera paquet installer | lister | retirer | creer`. Un paquet est un dossier de fichiers Fadéra avec un
  `fadera-paquet.json` ; il s'installe depuis GitHub (`utilisateur/depot@version`), une adresse web ou un dossier, dans `paquets/<nom>/`,
  est noté dans `fadera.json` et s'utilise avec `importer`. Aucun nouveau mot-clé. Installation sûre : noms et chemins contrôlés, extensions
  limitées, tout est vérifié avant la première écriture (voir `docs/PAQUETS.md`).
- **Accès réseau dans la ligne de commande** : `http_obtenir`, `http_json` et `http_envoyer` fonctionnent dans `fadera.exe` (jusqu'ici, seul
  le terrain de jeu en navigateur y avait accès). Délai de 20 s, réponse limitée à 20 Mo, prise en charge du proxy d'entreprise.
- **En-têtes facultatifs** pour `http_obtenir(adresse, entetes)` et `http_json(adresse, entetes)`, comme pour `http_envoyer` : on peut
  désormais envoyer une clé d'accès (`Authorization`) avec une simple lecture. Identique dans les deux moteurs.
- Guide `docs/SERVICES.md` : interroger une API, une base de données hébergée (Supabase, Airtable…) ou tout service en ligne.
- Test `fadera-rs/tests/verifier_paquets.js` (29 vérifications, sans internet) branché dans `node verifier_tout.js` et donc dans la CI.

### Modifié
- Nouvelle dépendance du binaire en ligne de commande : `ureq` (client HTTP, absent de la version WebAssembly).

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

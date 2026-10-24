# Contribuer

Règle d'or : **les deux moteurs (JavaScript dans `index.html`, Rust dans `fadera-rs/`) doivent donner exactement les mêmes résultats.**
Toute modification de comportement se fait des deux côtés, avec un test.

```
node verifier_tout.js      # tout vérifier d'un coup (Node.js et Rust requis)
```

Cette commande compile le moteur Rust, lance ses tests, les tests JavaScript, la parité JS/Rust (programmes, formateur, débogueur),
la spécification exécutable (`SPEC_FADERA.md`) et, si Playwright est installé, l'IDE dans un vrai Chromium.
La CI GitHub lance la même chose à chaque `push` et chaque `pull request`.

- Pas de nouveau mot-clé sans discussion ; les fonctions natives sont déclarées dans `fadera-rs/src/{natives,stdlib,modules}.rs` et dans `FONCTIONS_NATIVES` (`index.html`).
- Les modules `std/…` s'écrivent en Fadéra dans `fadera-rs/src/std/*.fa` ; le texte intégré à `index.html` doit être identique.
- Ajoutez une ligne au `CHANGELOG.md` sous « Non publié ».

## Publier une version
1. Changer `version` dans `fadera-rs/Cargo.toml`, puis `cargo update -p fadera` (met à jour `Cargo.lock`).
2. Déplacer les notes de « Non publié » sous un titre `## [X.Y.Z] — date` dans `CHANGELOG.md`.
3. `git tag vX.Y.Z && git push origin vX.Y.Z` : le workflow `release.yml` vérifie que l'étiquette correspond à `Cargo.toml`,
   construit les exécutables Linux, Windows et macOS, et crée la publication GitHub avec les sommes SHA-256.

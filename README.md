# Fadéra

Un langage de programmation **en français**, avec un IDE dans le navigateur et un moteur en ligne de commande.

```
classe Compteur
  fonction creer()
    soi.n = 0
  fin
  fonction incrementer()
    soi.n = soi.n + 1
  fin
fin

soit c = Compteur()
c.incrementer()
afficher "Bonjour Fadéra, n = " + c.n
```

## Essayer

- **Playground en ligne** : après activation de GitHub Pages (Settings → Pages → Source : *GitHub Actions*), l'IDE est publié à
  `https://<compte>.github.io/<dépôt>/`. Le bouton de partage fabrique un lien `#p=…` : le code est dans le lien, rien n'est envoyé à un serveur,
  et l'ouverture d'un lien demande confirmation sans jamais exécuter le programme.
- **Hors ligne** : ouvrir `index.html` dans un navigateur (moteur JavaScript), ou `python serveur.py` pour activer aussi le moteur Rust/WebAssembly.
- **Ligne de commande** : télécharger l'exécutable de la [dernière version](../../releases/latest)
  (`fadera.exe` pour Windows, `fadera` pour Linux et macOS) :

```
fadera run programme.fa        # exécuter un fichier (ou : fadera programme.fa)
fadera -e 'afficher 2 + 3'     # exécuter du code
fadera fmt --ecrire f.fa       # mettre en forme
fadera tester f.fa             # lancer les blocs  tester "nom" … fin
fadera deboguer f.fa 3,7       # débogueur pas à pas
fadera                         # mode interactif
fadera --version
```

Sous Windows : décompresser l'archive, puis dans l'invite de commandes `fadera.exe run programme.fa`
(ajouter le dossier au `PATH` pour taper simplement `fadera`). Windows peut afficher un avertissement SmartScreen : l'exécutable n'est pas signé.

## Apprendre

- [**Tutoriel pas à pas**](docs/TUTORIEL.md) : de « Bonjour le monde » aux classes, aux fichiers et au réseau.
- [**30 exemples**](docs/exemples/) : calculs, jeux (devine le nombre, pendu, morpion, jeu de la vie, labyrinthe…), textes, CSV/JSON, dates, réseau, classes.
- [**Paquets**](docs/PAQUETS.md) et [**services en ligne**](docs/SERVICES.md) : installer du code d'autres personnes (`fadera paquet installer`), interroger des API et des bases hébergées.
- [**Référence des fonctions**](docs/REFERENCE.md) : les 195 fonctions natives et les 12 modules standard, avec un exemple vérifié pour chacune.

## Compiler soi-même

```
cd fadera-rs && cargo build --release        # → target/release/fadera  (fadera.exe sous Windows)
```

## Contenu du dépôt

| Chemin | Rôle |
|---|---|
| `index.html` | l'IDE et le moteur JavaScript (un seul fichier) |
| `fadera-rs/` | le moteur Rust, la ligne de commande, la cible WebAssembly |
| `GUIDE_FADERA.md`, `SPEC_FADERA.md` | guide d'utilisation ; spécification exécutable |
| `docs/` | tutoriel, 30 exemples, référence des fonctions (générés et vérifiés par la CI) |
| `tests_fadera.js`, `verifier_*.js` | tests JavaScript ; vérification complète (`node verifier_tout.js`) |
| `.github/workflows/` | CI, publication des versions, playground |

## Qualité : la parité JS/Rust

Les deux moteurs doivent donner exactement les mêmes résultats. À chaque `push` et `pull request`, la CI lance `node verifier_tout.js` :
tests Rust, tests JavaScript, comparaison des deux moteurs sur plusieurs centaines de programmes, du formateur et du débogueur,
spécification exécutable et IDE dans Chromium ; puis compile et teste sous Windows et macOS, et compile le moteur WebAssembly.

## Versions

Numérotation [SemVer](https://semver.org/lang/fr/), notes dans [CHANGELOG.md](CHANGELOG.md). Une étiquette `vX.Y.Z` déclenche la construction
des exécutables et la publication (voir [CONTRIBUTING.md](CONTRIBUTING.md)).

## Licence

[MIT](LICENSE).

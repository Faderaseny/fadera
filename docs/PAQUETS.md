# Les paquets : utiliser et partager du code Fadéra

Un **paquet** est un dossier de fichiers Fadéra que d'autres personnes peuvent installer en une commande.
Il n'y a ni nouveau mot-clé ni nouvelle fonction : un paquet s'utilise avec `importer`, comme n'importe quel module.

> Les paquets s'installent avec la ligne de commande (`fadera`). Le terrain de jeu en ligne n'a pas de dossier de projet : il utilise les modules standard (`importer "std/maths"`) et les fichiers que vous y importez.

## Utiliser un paquet

Dans le dossier de votre projet :

```
fadera paquet installer utilisateur/depot            # depuis GitHub (branche principale)
fadera paquet installer utilisateur/depot@v1.2.0     # une version précise (étiquette ou branche)
fadera paquet installer https://exemple.org/outils   # depuis une adresse web
fadera paquet installer ../mes-outils                # depuis un dossier de votre ordinateur
```

Le paquet est copié dans `paquets/<nom>/` et noté dans `fadera.json` :

```
{
  "dependances": {
    "outils": "utilisateur/depot@v1.2.0"
  }
}
```

Puis, dans votre programme :

```
importer "paquets/outils/principal" comme o
afficher o.doubler(21)
```

| Commande | Effet |
|---|---|
| `fadera paquet installer` | réinstalle tout ce que liste `fadera.json` (après avoir récupéré un projet, par exemple) |
| `fadera paquet lister` | affiche les paquets installés et leur version |
| `fadera paquet retirer nom` | supprime le paquet et l'enlève de `fadera.json` |
| `fadera paquet creer nom` | crée le squelette d'un nouveau paquet (code, tests, mode d'emploi) |

Conseil : mettez `paquets/` dans votre `.gitignore` et gardez `fadera.json` ; chacun réinstalle avec `fadera paquet installer`.

## Créer et partager un paquet

```
fadera paquet creer geometrie
fadera tester geometrie/tests.fa
```

Le dossier contient :

| Fichier | Rôle |
|---|---|
| `fadera-paquet.json` | la carte d'identité du paquet (voir ci-dessous) |
| `principal.fa` | le code : les fonctions et classes que vos utilisateurs appelleront |
| `tests.fa` | des blocs `tester "…" … fin` : la preuve que votre paquet marche |
| `LISEZMOI.md` | le mode d'emploi |

```
{
  "nom": "geometrie",
  "version": "0.1.0",
  "description": "Aires et périmètres",
  "principal": "principal.fa",
  "fichiers": ["principal.fa", "tests.fa", "LISEZMOI.md"]
}
```

Pour partager : publiez ce dossier sur GitHub (dépôt public) ; vos utilisateurs feront `fadera paquet installer votre-nom/geometrie`. Pour une nouvelle version, changez `"version"`, publiez, et ajoutez une étiquette Git (`v0.2.0`) : ils pourront choisir `votre-nom/geometrie@v0.2.0`.

Un paquet qui en utilise un autre le dit dans son LISEZMOI ; les fichiers d'un même paquet s'importent entre eux par leur nom (`importer "aide" comme a`).

## Règles de sécurité

Un paquet est du code qui s'exécute avec **vos** droits : il peut lire et écrire des fichiers et accéder au réseau. N'installez que des paquets dont vous connaissez l'auteur, et lisez-les avant de les utiliser. L'installateur limite les dégâts à l'installation même :

- le nom du paquet : minuscules sans accent, chiffres, `_` et `-` seulement ;
- les fichiers : uniquement `.fa`, `.md`, `.txt`, `.json`, `.sortie` ; 200 fichiers et 1 Mo par fichier au plus ;
- les chemins : jamais de `..`, de chemin absolu, de lecteur Windows ni d'antislash : un paquet ne peut écrire que dans son propre dossier `paquets/<nom>/` ;
- tout est téléchargé et vérifié **avant** la première écriture : un paquet refusé ne laisse aucune trace.

# Guide complet du langage Fadéra (.fa)

Fadéra est un langage de programmation en français : mots-clés, messages d'erreur et logique s'écrivent et se lisent en français. Ce guide couvre tout ce qu'il faut savoir pour écrire, comprendre et déboguer un programme Fadéra, du plus simple `afficher "bonjour"` jusqu'aux fonctions natives avancées (statistiques, mathématiques, réseau, intelligence artificielle, chiffrement).

Deux façons d'exécuter du Fadéra :
- **Référence précise** : `SPEC_FADERA.md` définit la grammaire, les priorités, l'égalité, la copie/référence, la portée et les erreurs. Ses exemples sont vérifiés automatiquement sur les deux moteurs (`node verifier_spec.js`).
- **En ligne de commande** : `node fadera.js monscript.fa` (ou l'exécutable packagé). Seule cette version donne accès à `lireFichier`/`ecrireFichier`.
- **Dans le navigateur (Fadéra OS)** : l'éditeur web publié en artefact. `saisir(...)` y ouvre une fenêtre au lieu de lire le clavier ; `lireFichier`/`ecrireFichier` n'y sont pas disponibles.

---

## Sommaire

1. [Premiers pas](#1-premiers-pas)
2. [Lexique : ce que le langage reconnaît](#2-lexique--ce-que-le-langage-reconnaît)
3. [Les types de valeurs](#3-les-types-de-valeurs)
4. [Variables](#4-variables)
5. [Opérateurs et priorités](#5-opérateurs-et-priorités)
6. [Afficher des résultats](#6-afficher-des-résultats)
7. [Structures de contrôle](#7-structures-de-contrôle)
8. [Fonctions](#8-fonctions)
9. [Listes](#9-listes)
10. [Structures (enregistrements)](#10-structures-enregistrements)
11. [Entrées / sorties](#11-entrées--sorties)
12. [Gestion des erreurs](#12-gestion-des-erreurs)
13. [Catalogue des fonctions natives](#13-catalogue-des-fonctions-natives)
14. [Bonnes pratiques et pièges à éviter](#14-bonnes-pratiques-et-pièges-à-éviter)
15. [Programme complet commenté](#15-programme-complet-commenté)
16. [Nouveautés : contrôle de flux, fonctions valeurs, texte, modules](#16-nouveautés--contrôle-de-flux-fonctions-valeurs-texte-modules)
17. [Le terminal à la Linux](#17-le-terminal-à-la-linux-dans-lide)
18. [Tester ses programmes](#18-tester-ses-programmes)
19. [Modules standard et paquets partageables](#19-modules-standard-et-paquets-partageables)
20. [Brancher le moteur Rust/WebAssembly dans l'IDE](#20-brancher-le-moteur-rustwebassembly-dans-lide)
21. [Déboguer pas à pas](#21-déboguer-pas-à-pas)
22. [Classes et méthodes](#22-classes-et-méthodes)
23. [L'éditeur : onglets, recherche, coloration, complétion](#23-léditeur--onglets-recherche-coloration-complétion)
24. [Sauvegarde durable, export et import](#24-sauvegarde-durable-export-et-import)
25. [Formateur de code : `fadera fmt`](#25-formateur-de-code--fadera-fmt)
26. [Distribution et partage](#26-distribution-et-partage)

---

## 1. Premiers pas

Un fichier Fadéra porte l'extension `.fa`. Voici le plus petit programme possible :

```
# Mon premier script Fadéra
afficher "Bonjour, monde !"
```

On l'exécute avec :

```
node fadera.js monscript.fa
```

Un script est une suite d'**instructions**, exécutées de haut en bas (sauf pour les fonctions, qui peuvent être appelées avant leur déclaration — voir section 8). Chaque instruction commence en général par un mot-clé (`soit`, `si`, `afficher`, `pour`...) ou par le nom d'une variable/fonction.

---

## 2. Lexique : ce que le langage reconnaît

### Commentaires
Deux syntaxes acceptées, tout ce qui suit jusqu'à la fin de la ligne est ignoré :
```
# ceci est un commentaire
// ceci aussi
```

### Nombres
Entiers ou décimaux (avec un point, pas de virgule) : `42`, `3.14`, `0.5`. Pas de séparateur de milliers. La **notation scientifique** est acceptée : `1e7` (10 000 000), `2.5e-3` (0,0025), `4E+2` (400). Le `e` doit être collé au nombre et suivi d'un chiffre (`3e` ou `2 e5` ne sont pas des nombres).

### Texte (chaînes de caractères)
Entre guillemets doubles **ou** simples :
```
afficher "Bonjour"
afficher 'Aussi valide'
```
Caractères spéciaux échappables avec `\` : `\"`, `\'`, `\\`, `\n` (retour à la ligne), `\t` (tabulation).
```
afficher "Il a dit : \"Salut\"\nLigne suivante"
```

### Identifiants (noms de variables et de fonctions)
- Lettres (accents compris : `é`, `à`, `ç`, `ô`...), chiffres, et underscore `_`.
- Doivent commencer par une lettre ou `_` (jamais par un chiffre).
- **Sensible à la casse** : `age` et `Age` sont deux variables différentes.
- **Aucune contrainte de nom particulière** au-delà des mots réservés ci-dessous : `resultat`, `x1`, `_temp`, `nombreDeVoitures`, `prénom`, `a`, `b` sont tous valides.

Exemple de noms valides : `age`, `_compteur`, `prénom`, `résultat_final`, `x1`, `montant2024`.

### Mots réservés (à ne jamais utiliser comme nom de variable)
```
soit  si  alors  sinon  fin  afficher  tantque  faire  fonction  renvoyer
saisir  liste  ajouter  taille  lireElement  modifierElement  modifierCle
hasard  structure  lireCle  pour  allantde  lireFichier  ecrireFichier
vrai  faux  non  et  ou
```
Si vous essayez d'utiliser l'un de ces mots comme nom de variable, Fadéra refuse avec un message clair :
```
soit si = 5   # ❌ "si" est un mot réservé du langage : choisissez un autre nom.
```

> **À noter** : `a` (le séparateur de la boucle `pour ... allantde ... a ...`) et `pas` (le pas optionnel) ne sont **pas** des mots réservés globaux — ils ne sont interprétés spécialement qu'à cet endroit précis de la grammaire. Vous pouvez donc avoir une variable nommée `a` ailleurs dans votre script sans problème.

### Opérateurs reconnus
```
=   +   -   *   **   /   %   ==   !=   <   >   <=   >=
```
Le `!` seul est accepté comme synonyme de `non` (négation logique).

### Symboles de structure
```
( )   [ ]   { }   ,   :
```
Le point-virgule `;` est toléré en fin d'instruction mais totalement facultatif — il n'a aucun effet.

---

## 3. Les types de valeurs

| Type | Exemples | `taille(...)` fonctionne ? |
|---|---|---|
| **nombre** | `42`, `-3.5`, `0` | non |
| **texte** | `"bonjour"` | oui (nombre de caractères) |
| **booléen** | `vrai`, `faux` | non |
| **liste** | `[1, 2, 3]` | oui (nombre d'éléments) |
| **structure** | `{ nom: "Awa", age: 20 }` | oui (nombre de champs) |
| **rien** | valeur d'une fonction qui ne `renvoyer` rien | — |

Fadéra convertit automatiquement en texte quand on utilise `+` avec au moins un texte :
```
soit age = 20
afficher "J'ai " + age + " ans"   # "J'ai 20 ans"
```

---

## 4. Variables

### Déclaration : `soit`
```
soit age = 20
soit nom = "Awa"
soit actif = vrai
```
Une variable doit être déclarée avec `soit` **avant** d'être utilisée.

### Réaffectation (sans `soit`)
Une fois déclarée, on change sa valeur simplement avec `=` :
```
soit compteur = 0
compteur = compteur + 1   # pas de "soit" ici : la variable existe déjà
afficher compteur          # 1
```
Écrire `nom = valeur` sur une variable **jamais déclarée** donne une erreur claire :
```
score = 10   # ❌ La variable "score" n'existe pas. Déclarez-la d'abord avec : soit score = ...
```

### Portée
Chaque fonction a sa propre mémoire locale (ses paramètres et les `soit` faits à l'intérieur). Le script principal a sa mémoire globale, visible en lecture depuis les fonctions.

---

## 5. Opérateurs et priorités

Du **moins prioritaire** au **plus prioritaire** (comme en mathématiques : `*` avant `+`, parenthèses en premier) :

| Priorité | Opérateurs | Exemple |
|---|---|---|
| 1 (la plus basse) | `ou` | `a ou b` |
| 2 | `et` | `a et b` |
| 3 | `non` (ou `!`) | `non a` |
| 4 | `==` `!=` `<` `>` `<=` `>=` | `a == b` |
| 5 | `+` `-` | `a + b` |
| 6 | `*` `/` `%` | `a * b` |
| 7 | `-` unaire (nombre négatif) | `-a` |
| 8 (la plus haute) | `**` (puissance) | `a ** b` |

Les parenthèses `( )` forcent l'ordre d'évaluation, comme partout ailleurs :
```
afficher 2 + 3 * 4      # 14  (le * est prioritaire)
afficher (2 + 3) * 4    # 20  (les parenthèses forcent l'addition d'abord)
afficher -5 + 2         # -3
afficher 7 % 3          # 1   (reste de la division entière)
afficher 5 != 3         # vrai
```

**Puissance `**`** : `2 ** 10` donne `1024`. Elle s'associe **de droite à gauche** (`2 ** 3 ** 2` = `2 ** 9` = `512`), passe **avant** le `-` placé devant (`-2 ** 2` donne `-4`, comme en Python ; écrivez `(-2) ** 2` pour `4`) et accepte un exposant négatif ou décimal (`2 ** -1` = `0.5`, `16 ** 0.5` = `4`). Erreurs claires : `0 ** -1` → « Division par zéro. », `(-8) ** 0.5` → « Puissance impossible : une base négative demande un exposant entier. ». La fonction `puissance(a, b)` reste disponible.

L'opérateur `+` fonctionne aussi sur :
- **texte + n'importe quoi** → concaténation (conversion automatique en texte)
- **liste + liste** → fusion des deux listes : `[1,2] + [3,4]` donne `[1, 2, 3, 4]`

`==` et `!=` comparent le **contenu** des listes et structures (pas juste leur référence) :
```
afficher [1, 2] == [1, 2]              # vrai
afficher { a: 1 } == { a: 1 }          # vrai
```

---

## 6. Afficher des résultats

```
afficher "une seule valeur"
afficher "plusieurs", "valeurs", "séparées", "par", "des", "espaces"
afficher 1 + 1
```
`afficher` accepte plusieurs expressions séparées par des virgules ; elles sont affichées sur une seule ligne, séparées par un espace.

Les nombres, listes et structures sont automatiquement mis en forme lisible :
```
afficher [1, 2, 3]              # [1, 2, 3]
afficher { nom: "Awa", age: 20 } # { nom: "Awa", age: 20 }
afficher vrai                    # vrai
```

---

## 7. Structures de contrôle

### Condition : `si ... alors ... sinon ... fin`
```
soit note = 12

si note >= 10 alors
    afficher "Admis"
sinon
    afficher "Recalé"
fin
```

Enchaînement `sinon si` (sur la **même ligne** que `sinon`) :
```
si note >= 16 alors
    afficher "Très bien"
sinon si note >= 10 alors
    afficher "Admis"
sinon
    afficher "Recalé"
fin
```

### Boucle `tantque`
```
soit i = 0
tantque i < 5 faire
    afficher i
    i = i + 1
fin
```

### Boucle `pour ... allantde ... a ... faire`
```
pour i allantde 1 a 5 faire
    afficher i
fin
```
Avec un pas personnalisé (`pas`) — utile pour compter par 2, ou à l'envers avec un pas négatif :
```
pour i allantde 0 a 10 pas 2 faire
    afficher i          # 0, 2, 4, 6, 8, 10
fin

pour i allantde 10 a 1 pas -1 faire
    afficher i          # compte à rebours : 10, 9, 8, ... 1
fin
```

---

## 8. Fonctions

### Déclaration et appel
```
fonction carre(x)
    renvoyer x * x
fin

afficher carre(5)   # 25
```

### Plusieurs paramètres, ou aucun
```
fonction addition(a, b)
    renvoyer a + b
fin

fonction direBonjour()
    afficher "Bonjour !"
fin
```

### `renvoyer` sans valeur
```
fonction verifier(x)
    si x < 0 alors
        renvoyer   # sort de la fonction, valeur "rien"
    fin
    afficher "x est positif ou nul"
fin
```

### Une fonction peut être appelée avant sa déclaration
Contrairement à beaucoup de scripts, l'ordre des `fonction ... fin` dans le fichier n'a pas d'importance :
```
afficher double(21)   # fonctionne, même si "double" est défini plus bas

fonction double(x)
    renvoyer x * 2
fin
```

### Récursion
```
fonction factorielle(n)
    si n <= 1 alors
        renvoyer 1
    fin
    renvoyer n * factorielle(n - 1)
fin

afficher factorielle(5)   # 120
```
La récursion est limitée à 5000 appels imbriqués (protection contre les boucles infinies accidentelles).

---

## 9. Listes

Deux syntaxes équivalentes pour créer une liste :
```
soit fruits = ["pomme", "banane", "kiwi"]
soit fruits2 = liste("pomme", "banane", "kiwi")
```
Les listes peuvent contenir des valeurs de n'importe quel type, y compris d'autres listes (matrices) :
```
soit matrice = [[1, 2], [3, 4]]
```

| Opération | Syntaxe | Exemple |
|---|---|---|
| Créer | `[a, b, c]` ou `liste(a, b, c)` | `soit l = [1, 2, 3]` |
| Longueur | `taille(liste)` | `taille(l)` → `3` |
| Lire un élément (index commence à 0) | `lireElement(liste, index)` | `lireElement(l, 0)` → `1` |
| Ajouter à la fin | `ajouter(liste, valeur)` | `ajouter(l, 4)` |
| Modifier un élément | `modifierElement(liste, index, valeur)` | `modifierElement(l, 0, 99)` |

```
soit notes = [12, 15, 8]
ajouter(notes, 18)
afficher notes                       # [12, 15, 8, 18]
afficher lireElement(notes, 1)       # 15
modifierElement(notes, 0, 20)
afficher notes                       # [20, 15, 8, 18]
```
Un index invalide (négatif, non entier, ou trop grand) donne une erreur précise indiquant les indices valides.

---

## 10. Structures (enregistrements)

Une structure regroupe des champs nommés — l'équivalent d'un objet ou d'un dictionnaire.

```
structure personne = { nom: "Awa", age: 20, ville: "Dakar" }
afficher personne                    # { nom: "Awa", age: 20, ville: "Dakar" }
```

`{ ... }` fonctionne aussi comme une **expression**, utilisable directement en argument d'une fonction, sans passer par `structure` :
```
afficher lireCle({ x: 1, y: 2 }, "x")   # 1
```

| Opération | Syntaxe |
|---|---|
| Créer (nommée) | `structure nom = { cle: valeur, ... }` |
| Créer (en expression, anonyme) | `{ cle: valeur, ... }` |
| Lire un champ | `lireCle(structure, "cle")` |
| Modifier un champ | `modifierCle(structure, "cle", nouvelleValeur)` |
| Nombre de champs | `taille(structure)` |

```
structure voiture = { marque: "Toyota", annee: 2020 }
afficher lireCle(voiture, "marque")     # Toyota
modifierCle(voiture, "annee", 2024)
afficher voiture                         # { marque: "Toyota", annee: 2024 }
```
Lire une clé qui n'existe pas donne une erreur listant les clés disponibles.

---

## 11. Entrées / sorties

### Saisie utilisateur
```
soit nom = saisir("Quel est votre nom ? ")
afficher "Bonjour " + nom
```
- En CLI : lit une ligne au clavier. Si la réponse ressemble à un nombre, elle est automatiquement convertie en nombre.
- Dans Fadéra OS (web) : ouvre une fenêtre de saisie et **met le script en pause** jusqu'à la réponse.

### Fichiers (CLI uniquement)
```
ecrireFichier("resultat.txt", "Contenu à sauvegarder")
soit contenu = lireFichier("resultat.txt")
afficher contenu
```
Appeler `lireFichier`/`ecrireFichier`/`saisir` dans un environnement qui ne les supporte pas donne une erreur explicite plutôt qu'un plantage silencieux.

---

## 12. Gestion des erreurs

Fadéra vise à ce qu'**aucune erreur ne reste mystérieuse**. Chaque erreur indique :
- **La ligne concernée**
- **Une explication en français**, souvent avec une suggestion de correction

Exemples de messages réels :
```
❌ Ligne 4 : "si" est un mot réservé du langage : choisissez un autre nom.
❌ Ligne 7 : La variable "score" n'existe pas. Déclarez-la d'abord avec : soit score = ...
❌ Ligne 12 : Index 5 hors limites (indices valides : 0 à 2).
❌ Ligne 3 : Le bloc "si" ouvert ligne 3 n'est jamais fermé : il manque "fin".
❌ Ligne 9 : Fonction "calculerTotal" inconnue : elle n'est ni déclarée dans le script (mot-clé "fonction"), ni une fonction native.
```
### 12.1 L'affichage détaillé d'une erreur

Dans l'IDE, une erreur ne se limite plus à une phrase : le terminal montre **la ligne fautive, la colonne exacte** et, quand c'est possible, **ce que vous vouliez probablement écrire**.

```
💥 Erreur : Ligne 2 : La variable "totl" n'existe pas. Déclarez-la d'abord avec : soit totl = ...
  2 │ afficher totl
    │          ^^^^
💡 Tu voulais écrire « total » ?
```

- **Soulignement dans l'éditeur** : l'endroit fautif est souligné en rouge (ligne teintée, mot souligné en vague) et l'éditeur défile jusqu'à lui. Le repère disparaît dès que vous modifiez le texte ou relancez le programme.
- **« Tu voulais écrire… ? »** : pour une variable, une fonction (déclarée ou native) ou une instruction mal écrite, Fadéra propose le nom le plus proche. Les accents et les majuscules sont ignorés (`eleve` → `élève`), et une lettre manquante, en trop, fausse ou deux lettres échangées sont tolérées.
- **Variable déclarée plus bas** : si le nom existe plus loin dans le script, le message l'indique (« n'est déclarée qu'à la ligne 7 : place cette déclaration avant son utilisation »).
- **`=` au lieu de `==`** : dans un `si` ou un `tantque`, Fadéra rappelle que `==` sert à comparer.
- **Pile d'appels** : quand l'erreur survient dans une fonction, le terminal indique le chemin parcouru, de la fonction fautive jusqu'au programme :

```
📚 Pile d'appels :
     dans b (ligne 5)
     appelée depuis a (ligne 2)
     appelée depuis le programme (ligne 7)
```

Les appels récursifs identiques sont regroupés (`dans g (ligne 2) ×64`) ; seuls les 64 appels les plus récents sont gardés.

L'affichage est **le même avec le moteur JavaScript et avec le moteur Rust**. Pour la pile d'appels avec Rust, il faut recompiler le module avec la version fournie de `fadera-rs` ; avec un ancien module `.wasm`, l'IDE affiche quand même la ligne, la colonne et les suggestions (seule la pile manque).

### 12.2 Rattraper une erreur : `essayer / attraper`

Sans protection, une erreur arrête le script à l'endroit où elle survient. Pour continuer malgré une erreur possible, entourez le code risqué :

```
essayer
  afficher lireElement([1, 2], 5)
attraper e
  afficher "Problème : " + e
fin
afficher "le programme continue"
```

- `attraper e` donne le message de l'erreur (en texte) à la variable `e` ; `attraper` seul (sans nom) ignore le message.
- Le bloc fonctionne aussi dans une fonction, avec `renvoyer`, et attrape les erreurs venues d'appels imbriqués.

---

## 13. Catalogue des fonctions natives

> La liste **complète** (195 fonctions, chacune avec un exemple vérifié) est dans [docs/REFERENCE.md](docs/REFERENCE.md). Cette section en présente les familles principales.

Toutes ces fonctions s'utilisent comme des fonctions normales : `nomFonction(arguments)`, dans une expression (`soit x = fonction(...)`) ou avec `afficher`.

### Conversions et maths de base
| Fonction | Rôle | Exemple |
|---|---|---|
| `nombre(x)` | convertit un texte en nombre | `nombre("42")` → `42` |
| `texte(x)` | convertit n'importe quelle valeur en texte | `texte(42)` → `"42"` |
| `arrondir(x, decimales)` | arrondit `x` à `decimales` chiffres (0 par défaut) | `arrondir(3.14159, 2)` → `3.14` |
| `racine(x)` | racine carrée | `racine(16)` → `4` |
| `absolu(x)` | valeur absolue | `absolu(-5)` → `5` |
| `puissance(a, b)` | `a` puissance `b` | `puissance(2, 10)` → `1024` |
| `hasard(min, max)` | entier aléatoire entre `min` et `max` inclus | `hasard(1, 6)` (dé) |

### Statistiques
| Fonction | Rôle |
|---|---|
| `stats_somme(liste)` | somme des valeurs |
| `stats_moyenne(liste)` | moyenne arithmétique |
| `stats_min(liste)` / `stats_max(liste)` | minimum / maximum |
| `stats_mediane(liste)` | médiane |
| `stats_mode(liste)` | valeur(s) la/les plus fréquente(s) |
| `stats_variance(liste)` | variance |
| `stats_ecart_type(liste)` | écart-type |
| `stats_analyse_distribution(liste)` | quartiles Q1/Q3, écart interquartile, seuils d'anomalie |
| `stats_pearson_correlation(x, y)` | coefficient de corrélation entre deux listes (-1 à 1) |
| `stats_regression_lineaire(x, y)` | droite de régression `y = pente*x + ordonnee_origine` |
| `stats_supprimer_doublons(liste)` | enlève les valeurs en double |
| `stats_nettoyer_valeurs_manquantes(liste, strategie)` | `strategie` = `"supprimer"`, `"moyenne"` ou `"mediane"` |
| `stats_normalisation_ia(liste)` | ramène les valeurs entre 0 et 1 |
| `stats_query_engine(dataset, options)` | filtre/trie/sélectionne dans une liste de structures (voir exemple ci-dessous) |

```
soit notes = [12, 15, 8, 20, 12, 17]
afficher stats_moyenne(notes)                  # 14
afficher stats_mediane(notes)
afficher stats_ecart_type(notes)
afficher stats_analyse_distribution(notes)
```

Exemple de `stats_query_engine` (filtrer/trier une "base de données" faite de structures) :
```
soit eleves = [
    { nom: "Awa", note: 15 },
    { nom: "Ali", note: 9 },
    { nom: "Fatou", note: 18 }
]
structure options = {
    filtres: [{ champ: "note", operateur: ">=", valeur: 10 }],
    tri: { champ: "note", ordre: "desc" }
}
afficher stats_query_engine(eleves, options)
```

### Mathématiques avancées
| Fonction | Rôle |
|---|---|
| `math_pgcd_ppcm(a, b)` | renvoie `{ pgcd, ppcm }` |
| `math_facteurs_premiers(n)` | décomposition en facteurs premiers, ex. `60` → `[2,2,3,5]` |
| `math_resoudre_second_degre(a, b, c)` | racines de `ax² + bx + c = 0` |
| `math_integrale_trapeze(valeurs, dx)` | intégrale approchée (méthode des trapèzes) |
| `math_derivee_centree(valeurs, index)` | dérivée numérique centrée en un point |
| `math_resoudre_systeme_gauss(matriceAugmentee)` | résout un système d'équations linéaires (élimination de Gauss-Jordan) |

```
afficher math_resoudre_second_degre(1, -3, 2)     # racines de x² - 3x + 2
afficher math_resoudre_systeme_gauss([[2, 1, 5], [1, -1, 1]])  # système 2x+y=5, x-y=1
```

### Matrices
| Fonction | Rôle |
|---|---|
| `matrice_carree(m)` | `vrai` si la matrice est carrée |
| `matrice_triangulaire(m)` | `vrai` si triangulaire (haute ou basse) |
| `inverser_matrice(m)` | inverse d'une matrice 2x2 |

```
soit m = [[1, 2], [3, 4]]
afficher matrice_carree(m)        # vrai
afficher inverser_matrice(m)      # [[-2, 1], [1.5, -0.5]]
```

### Réseau / télécommunications
| Fonction | Rôle |
|---|---|
| `telecom_calculer_sous_reseau(ip, cidr)` | calcule masque, adresse réseau, broadcast, plage d'hôtes |
| `telecom_creer_topologie()` | crée un réseau vide (nœuds + liaisons) |
| `telecom_ajouter_liaison(topologie, liaison)` | ajoute une liaison bidirectionnelle ; `liaison` = `{ de, a, cout }` |
| `telecom_matrice_adjacence(topologie)` | matrice d'adjacence du réseau |
| `telecom_verifier_connexite(topologie)` | `vrai` si tous les nœuds sont reliés entre eux |
| `telecom_dijkstra_routage(topologie, requete)` | plus court chemin ; `requete` = `{ depart, arrivee }` |

```
afficher telecom_calculer_sous_reseau("192.168.1.45", 26)

soit reseau = telecom_creer_topologie()
telecom_ajouter_liaison(reseau, { de: "A", a: "B", cout: 5 })
telecom_ajouter_liaison(reseau, { de: "B", a: "C", cout: 3 })
afficher telecom_verifier_connexite(reseau)                       # vrai
afficher telecom_dijkstra_routage(reseau, { depart: "A", arrivee: "C" })
```

### Intelligence artificielle (réseaux de neurones)
| Fonction | Rôle |
|---|---|
| `ia_creer_neurone(nbEntrees, typeActivation)` | crée un neurone ; `typeActivation` = `"sigmoid"`, `"relu"` ou `"heaviside"` |
| `ia_activation_perceptron(neurone, entrees)` | calcule la sortie du neurone pour une liste d'entrées |
| `ia_descente_gradient(neurone, config)` | met à jour les poids ; `config` = `{ entrees, erreur }` |
| `ia_entrainer_reseau_xor(epoques)` | entraîne un petit réseau à résoudre le problème XOR, renvoie les résultats de test |
| `ia_reseau_multicouche(architecture, config)` | entraîne un réseau multicouche complet ; `architecture` = ex. `[2, 4, 1]`, `config` = `{ dataset, epoques, tauxApprentissage }` |

```
soit neurone1 = ia_creer_neurone(2, "sigmoid")
afficher ia_activation_perceptron(neurone1, [0.5, 0.8])
afficher ia_entrainer_reseau_xor(5000)
```

> Fadéra ne permet pas d'appeler une fonction stockée dans une variable (pas de `objet.methode()`). Ces fonctions renvoient donc directement des **données** (résultats calculés) plutôt que des objets "à appeler" plus tard.

### Chiffrement (vrai chiffrement, API Web Crypto standard)
| Fonction | Rôle |
|---|---|
| `crypto_hacher_sha256(texte)` | empreinte SHA-256 |
| `crypto_hacher_sha512(texte)` | empreinte SHA-512 |
| `crypto_hmac_sha256(texte, cle)` | signature HMAC-SHA256 |
| `crypto_aleatoire_hex(n)` | `n` octets aléatoires cryptographiquement sûrs, en hexadécimal |
| `crypto_chiffrer_symetrique(texte, motDePasse)` | chiffre avec AES-256-GCM (clé dérivée par PBKDF2, 600 000 itérations) |
| `crypto_dechiffrer_symetrique(chiffre, motDePasse)` | déchiffre (erreur claire si mauvais mot de passe) |
| `crypto_generer_cles_rsa()` | génère une paire de clés RSA-OAEP 2048 bits, renvoie `{ cle_publique, cle_privee }` |
| `crypto_chiffrer_asymetrique(texte, clePublique)` | chiffrement hybride RSA+AES (aucune limite de taille du message) |
| `crypto_dechiffrer_asymetrique(chiffre, clePrivee)` | déchiffre avec la clé privée correspondante |

```
soit empreinte = crypto_hacher_sha256("mot de passe")
afficher empreinte

soit c = crypto_chiffrer_symetrique("message secret", "motDePasseSolide")
afficher crypto_dechiffrer_symetrique(c, "motDePasseSolide")   # "message secret"

soit cles = crypto_generer_cles_rsa()
soit chiffre = crypto_chiffrer_asymetrique("confidentiel", lireCle(cles, "cle_publique"))
afficher crypto_dechiffrer_asymetrique(chiffre, lireCle(cles, "cle_privee"))
```

---

## 14. Bonnes pratiques et pièges à éviter

- **`fin` obligatoire** pour fermer `si`, `tantque`, `pour` et `fonction`. Un bloc non fermé donne une erreur indiquant la ligne où il a été ouvert.
- **Comparer avec `==`, pas `=`** : `=` sert uniquement à affecter une valeur.
- **Les index de liste commencent à 0**, pas à 1.
- **Ne réutilisez pas un mot réservé** comme nom de variable (voir la liste en section 2).
- **Les fonctions natives lèvent des erreurs précises** (mauvais type, mauvaise taille d'argument) — lisez le message, il indique quoi corriger.
- **Structures passées par référence** : si vous passez une structure à une fonction native qui la modifie (ex. `telecom_ajouter_liaison`, `ia_descente_gradient`), la variable d'origine est bien mise à jour, comme pour les listes.
- **Boucles infinies** : Fadéra les détecte après un grand nombre d'itérations et arrête le script avec une erreur, plutôt que de bloquer indéfiniment.

---

## 15. Programme complet commenté

```
# ===========================================
#  Carnet de notes — exemple complet Fadéra
# ===========================================

# Une structure par élève, regroupées dans une liste
soit eleves = [
    { nom: "Awa", notes: [15, 12, 18] },
    { nom: "Moussa", notes: [8, 9, 11] },
    { nom: "Fatou", notes: [20, 19, 17] }
]

# Fonction : calcule la moyenne d'une liste de notes
fonction moyenneListe(notes)
    soit somme = 0
    pour i allantde 0 a taille(notes) - 1 faire
        somme = somme + lireElement(notes, i)
    fin
    renvoyer somme / taille(notes)
fin

afficher "=== Bulletin de classe ==="

pour i allantde 0 a taille(eleves) - 1 faire
    soit eleve = lireElement(eleves, i)
    soit nom = lireCle(eleve, "nom")
    soit notes = lireCle(eleve, "notes")
    soit moyenne = moyenneListe(notes)

    afficher nom + " : moyenne = " + arrondir(moyenne, 2)

    si moyenne >= 10 alors
        afficher "  -> Admis(e)"
    sinon
        afficher "  -> À revoir"
    fin
fin

# Un peu de statistiques sur toutes les moyennes de la classe
soit moyennesClasse = liste()
pour i allantde 0 a taille(eleves) - 1 faire
    soit eleve = lireElement(eleves, i)
    ajouter(moyennesClasse, moyenneListe(lireCle(eleve, "notes")))
fin

afficher "=== Statistiques de la classe ==="
afficher "Moyenne générale : " + arrondir(stats_moyenne(moyennesClasse), 2)
afficher "Écart-type : " + arrondir(stats_ecart_type(moyennesClasse), 2)
```

---

## 16. Nouveautés : contrôle de flux, fonctions valeurs, texte, modules

### 16.1 `rompre` et `continuer`
Valables dans `pour`, `tantque` et `pour chaque`. `rompre` sort de la boucle, `continuer` passe à l'itération suivante. En dehors d'une boucle, c'est une erreur claire.
```
pour i allantde 1 a 10 faire
    si i % 2 == 0 alors
        continuer
    fin
    si i > 7 alors
        rompre
    fin
    afficher i        # 1 3 5 7
fin
```

### 16.2 `pour chaque`
Parcourt une liste, un texte (caractère par caractère) ou une structure (les clés).
```
pour chaque nom dans ["Awa", "Moussa"] faire
    afficher "Bonjour " + nom
fin
```

### 16.3 `selon / cas / defaut`
Pas de « chute » d'un cas à l'autre : seul le premier cas qui correspond s'exécute. Plusieurs valeurs possibles par cas, séparées par des virgules.
```
selon note faire
    cas 20 alors
        afficher "Parfait"
    cas 10, 11, 12 alors
        afficher "Passable"
    defaut
        afficher "Autre"
fin
```
> `selon`, `cas` et `defaut` sont des mots réservés : on ne peut pas nommer une variable `cas`.

### 16.4 Les fonctions sont des valeurs
```
soit double = x => x * 2                  # lambda courte
soit somme = (a, b) => a + b
soit f = fonction(x)                      # lambda longue
    renvoyer x + 1
fin

fonction ajouteur(n)                      # fermeture : retient n
    renvoyer x => x + n
fin
soit plus5 = ajouteur(5)
afficher plus5(10)                        # 15
```
On peut passer une fonction (la vôtre ou une native comme `racine`) en argument :
| Fonction | Rôle | Exemple |
|---|---|---|
| `transformer(liste, f)` | applique `f` à chaque élément | `transformer([1,2,3], x => x * x)` → `[1, 4, 9]` |
| `filtrer(liste, f)` | garde les éléments où `f` est vrai | `filtrer([1,2,3,4], x => x > 2)` → `[3, 4]` |
| `reduire(liste, f, depart)` | cumule les valeurs | `reduire([1,2,3], (a, b) => a + b, 0)` → `6` |
| `trier(liste)` / `trier(liste, comparer)` | tri croissant ou personnalisé | `trier([3,1,2], (a, b) => b - a)` → `[3, 2, 1]` |
| `trier_par(liste, cle)` | tri selon une clé calculée | `trier_par(mots, m => taille(m))` |
| `tous(liste, f)` / `existe(liste, f)` | tous / au moins un | `existe([1,3], x => x > 2)` |
| `trouver(liste, f)` | premier élément qui convient | |

### 16.5 Fonctions de texte
`majuscule`, `minuscule`, `nettoyer` (retire les espaces autour), `decouper(texte, sep)`, `joindre(liste, sep)`, `remplacer(texte, ancien, nouveau)`, `contient(texte_ou_liste, x)`, `commence_par`, `termine_par`, `sous_texte(texte, debut, fin)` (fin exclue), `position(texte, x)` (-1 si absent), `repeter(texte, n)`, `inverser(texte_ou_liste)`, `caractere(texte, i)`, `code_caractere`, `depuis_code`, `est_nombre`, `type_de(valeur)`.
Expressions régulières : `regex_test`, `regex_trouver`, `regex_remplacer`.

### 16.6 Listes, structures, JSON
- Listes : `plage(debut, fin[, pas])` (**fin incluse**, comme `pour`), `tranche(l, debut, fin)`, `unique`, `inserer(l, i, v)`, `retirer(l, i)`, `concatener`, `aplatir`, `compter(l, v)`.
- Structures (dictionnaires) : `cles`, `valeurs`, `paires`, `a_cle(s, k)`, `supprimer_cle(s, k)`, `fusionner(a, b)`, `structure_depuis_paires`.
- JSON : `json_texte(valeur)` et `json_lire(texte)`.
- Divers : `plancher`, `plafond`, `entier`, `minimum`, `maximum`, `maintenant()`, `date_iso()`, `formater_date()`.

### 16.7 Modules : `importer`
```
# fichier outils.fa
fonction double(x)
    renvoyer x * 2
fin
```
```
importer "outils"          # l'extension .fa est facultative
afficher double(21)        # 42
```
Un module n'est chargé qu'une fois (les cycles sont sans danger).

**Où Fadéra cherche le module** : un module qui en importe un autre le cherche **d'abord à côté de lui**, puis depuis le dossier courant. Un projet peut donc ranger ses fichiers en dossiers : si `lib/util.fa` contient `importer "base"`, c'est `lib/base.fa` qui est chargé.

**Sans alias, les noms d'un module sont communs à tout le programme** : si deux modules définissent la même fonction ou la même variable, le dernier importé l'emporte, sans avertissement. Pour l'éviter, utilisez un **alias**.

**Espaces de noms : `importer "…" comme alias`**
```
importer "outils" comme o
importer "autre" comme a
afficher o.double(3)       # 6
afficher a.double(3)       # la fonction « double » de autre.fa : aucun conflit
afficher o.PI2             # variables et structures du module : o.nom
importer "std/maths" comme m
afficher m.factorielle(4)  # 24
```
- Toutes les fonctions, variables et structures déclarées au **niveau supérieur** du module sont rangées sous `alias.`. À l'intérieur du module, elles s'appellent toujours par leur nom court : `quadruple` appelle `double` du **même** module, jamais celle d'un autre.
- Un paramètre ou une variable locale qui porte le même nom qu'un nom du module reste local. Les clés de structure (`{double: 1}`) ne sont pas modifiées.
- Un module peut être importé sous plusieurs alias, ou avec et sans alias. Un module qui en importe un autre avec alias (`importer "b" comme b`) garde ces noms à l'intérieur de son propre espace.
- `comme` n'est un mot spécial qu'après `importer "…"`, sur la même ligne : `soit comme = 3` reste valide.
- Un nom inconnu donne l'erreur habituelle avec l'alias : `Fonction "o.nope" inconnue`.
- Limites : l'alias lui-même n'est pas une valeur (`afficher o` ou `lireCle(o, "x")` ne marchent pas), et l'accès par point ne sert qu'aux modules : pour un champ de structure, on garde `lireCle(s, "champ")`. Une variable locale qui porte le nom d'un nom du module masque ce dernier dans **toute** sa fonction, même avant sa déclaration.

**Les erreurs situent le vrai problème.** Une erreur dans une fonction d'un module est rapportée à la **ligne de votre script où vous l'avez appelée**, et le message dit dans quel module et à quelle ligne elle est née :

```
Ligne 2 : [module std/listes.fa, ligne 9] Index 0 hors limites (la liste est vide).
```

La pile d'appels (§12.1) nomme le module : `dans premier [std/listes.fa] (ligne 9)`. Si l'erreur vient d'une fonction de **votre** script appelée par un module (par exemple la fonction passée à `compter_si`), elle n'est pas attribuée au module.

## 17. Le terminal à la Linux (dans l'IDE)
L'éditeur, le terminal de sortie et la ligne de commande partagent un système de fichiers unique, conservé dans le navigateur.
- Le texte de l'éditeur est le fichier `~/script.fa` ; la sortie du dernier ▶ est dans `~/sortie.log`.
- `ecrireFichier` / `lireFichier` / `importer` utilisent ces mêmes fichiers.
- Commandes : `ls -l -a`, `cd`, `pwd`, `mkdir -p`, `touch`, `cp -r`, `mv`, `rm -r -f`, `rmdir`, `tree`, `find`, `stat`, `chmod`, `cat`, `head`, `tail`, `wc`, `grep`, `sort`, `uniq`, `sed 's/a/b/g'`, `echo`, `history`, `aide`.
- Fadéra : `nano fichier.fa` (ouvre dans l'éditeur), `fadera fichier.fa` ou `./fichier.fa` (exécute).
- Pipes et redirections : `cat sortie.log | grep Bonjour`, `echo "x" >> notes.txt`, `cmd1 && cmd2`, jokers `*.fa`, variables `$HOME` et vos variables Fadéra.
- On peut aussi taper directement du Fadéra : `soit z = 4`, `z * 2.5`, `fnf hasard(10, 20)`.

---

Ce guide couvre l'intégralité du langage tel qu'implémenté. Pour toute fonction native dont le comportement n'est pas clair une fois testée, le message d'erreur (avec numéro de ligne) est en général suffisant pour corriger le script.


---

### 17.1 Commandes supplémentaires (toutes réelles, sur le système de fichiers virtuel)

- **Texte** : `awk` (sous-ensemble : `-F`, `$1…$NF`, `NR`, `NF`, `BEGIN/END`, motifs `/re/`), `cut`, `tr`, `tac`, `rev`, `nl`, `paste`, `fold`, `column -t`, `shuf`, `comm`, `diff [-u]`, `cmp`, `tee`, `xargs`, `strings`, `xxd`, `hexdump -C`.
- **Hachage / encodage** : `md5sum`, `sha1sum`, `sha256sum`, `sha384sum`, `sha512sum` (avec `-c`), `cksum`, `base64 [-d]`.
- **Système** : `date`, `cal`, `uname`, `whoami`, `id`, `hostname`, `uptime`, `df`, `du`, `file`, `ln`, `basename`, `dirname`, `realpath`, `mktemp`, `sleep`, `ps`/`top` (simulés), `sudo`, `time`, `man`, `type`.
- **Calcul** : `seq`, `printf`, `expr`, `bc`, `factor`, `$((1+2*3))`.
- **Shell** : `VAR=valeur`, `export`, `env`, `$?`, `$(commande)`, `cmd1 || cmd2`, `test` / `[ ... ]`, `alias`/`unalias` (persistants dans `~/.faderarc`), `find -perm`.
- **Scripts** : `sh script.sh [args]` ou `./script.sh` avec `if/elif/else/fi`, `for … in … do … done`, `while/until`, `$1 $# $@`. Mêmes constructions saisissables directement : `for f in *.fa; do cat $f; done`.

### 17.2 Les packs : importer des jeux de commandes

```
pkg list                         # packs disponibles / installés
pkg install securite             # pack sécurité
pkg install securite reseau dev  # plusieurs d'un coup (ou : pkg install tout)
pkg remove reseau
```

| Pack | Commandes |
|------|-----------|
| **securite** | `hashid` (identifie un hash), `entropie`, `pwcheck` (force d'un mot de passe), `pwgen`, `scansecrets` (clés/mots de passe en dur), `audit` (permissions dangereuses), `integrite init/check` (surveillance d'intégrité SHA-256), `logscan` (brute-force dans un auth.log), `hmac`, `chiffrer`/`dechiffrer` (AES-256-GCM), `shred`, `rot13`, `caesar`, `urlencode`/`urldecode`, `jwtdecode`, `portinfo` |
| **reseau** | `curl`, `wget`, `dig`/`nslookup` (DNS-over-HTTPS), `ipcalc`, `httping` |
| **dev** | `jq`, `uuidgen` |

**Vos propres packs** : `pkg install fichier` accepte
- un `.json` : `{"commandes": {"scan": "grep -rn $1 ."}}` → crée la commande `scan` ;
- un `.sh` : un script (alias, variables…) exécuté à l'import ;
- un `.fa` : le programme devient une commande (copié dans `~/bin`).

Les packs installés et les alias sont **conservés** d'une session à l'autre.

### 17.3 Ce qui est impossible dans un navigateur (dit franchement)

`nmap`, `nc`, `ping`, `traceroute`, `tcpdump`, `iptables`, `ssh`, `ifconfig`, `netstat`, `systemctl`… exigent des sockets bruts, des processus ou le noyau : un navigateur ne les donne pas. Ces commandes répondent par un message clair au lieu de simuler de faux résultats. `curl`/`wget` sont soumis aux règles CORS des sites. Pour de la vraie sécurité réseau, utilisez Kali/Linux ; l'EDI couvre l'analyse de fichiers, de journaux, de secrets, de hash, de mots de passe et d'intégrité.

## 18. Tester ses programmes

### 18.1 Tests intégrés au langage : `verifier` et `verifier_egal`

Deux fonctions natives permettent d'écrire des tests directement en Fadéra :

| Fonction | Rôle |
|---|---|
| `verifier(condition, message)` | Échoue si `condition` est fausse. Le `message` est facultatif. |
| `verifier_egal(obtenu, attendu, message)` | Échoue si les deux valeurs diffèrent (listes et structures comparées en profondeur). |

```
fonction double(x)
    renvoyer x * 2
fin

verifier_egal(double(4), 8, "double de 4")
verifier(taille([1, 2, 3]) == 3, "taille d'une liste")
verifier_egal(trier([3, 1, 2]), [1, 2, 3])
afficher "Tous les tests passent"
```

En cas d'échec, l'erreur est en français avec le numéro de ligne :

```
Ligne 1 : Échec du test (somme) : obtenu 3, attendu 4.
```

Pour qu'un test raté n'arrête pas tout le script, entourez-le d'`essayer … attraper` :

```
essayer
    verifier_egal(1 + 1, 3, "addition")
attraper e
    afficher "RATÉ : " + e
fin
```

### 18.2 Blocs `tester "nom" … fin` et `verifier_erreur`

Pour tester sans que les tests s'exécutent à chaque lancement, écrivez des blocs `tester` (voir la spécification, §11c) :

```
fonction double(x)
    renvoyer x * 2
fin

tester "le double de 4"
    verifier_egal(double(4), 8)
fin

tester "une division par zéro est signalée"
    verifier_erreur(() => 1 / 0, "zéro")
fin
```

Lancez-les avec le bouton **🧪 Lancer les tests du script** de l'IDE (moteur Rust si actif, sinon JavaScript), avec `tester fichier.fa` dans le terminal de l'IDE (un fichier ou un dossier : les fichiers `test_*.fa` y sont aussi lancés), ou `fadera tester fichier.fa` en ligne de commande. Chaque test affiche `✓` ou `✗` avec la ligne et la cause, puis un bilan ; le code de sortie est `1` si un test échoue. Un fichier `.fa` normal ignore ces blocs.

`verifier_erreur(fonction, texte)` appelle la fonction sans argument : elle **doit** lever une erreur, et si `texte` est donné, le message doit le contenir.

### 18.3 Suite de tests du moteur : `tests_fadera.js`

Le fichier `tests_fadera.js` vérifie le vrai moteur (lexer, parser, interpréteur, fonctions natives) et le terminal à la Linux. Il lit le code directement dans `index.html`, donc il teste toujours la version que vous livrez.

```
node tests_fadera.js                 # teste ./index.html (même dossier)
node tests_fadera.js chemin/index.html
```

Résultat attendu :

```
151 test(s) réussi(s), 0 échoué(s).
```

Le code de sortie est `0` si tout passe, `1` si un test échoue (utilisable dans une intégration continue). Chaque échec affiche le test, la valeur obtenue et la valeur attendue.

Pour ajouter un test, ajoutez une ligne dans la fonction `moteur()` :

```js
await t("nom du test", 'afficher 2 + 3', "5");          // sortie exacte
await t("erreur", 'afficher foo(1)', /Fonction "foo" inconnue/);  // expression régulière
```

Règle d'or : une nouvelle fonction se déclare uniquement dans `FONCTIONS_NATIVES`, jamais comme mot réservé avec une règle de grammaire. Ajoutez toujours un test avec elle.

---

## 19. Modules standard et paquets partageables

### 19.1 Fonctions standard ajoutées

| Domaine | Fonctions |
|---|---|
| **Dates** (en millisecondes, comme `maintenant()`) | `date_creer(an, mois, jour, h, min, s)`, `date_composants(ts)` (structure : annee, mois, jour, heure, minute, seconde, jour_semaine, nom_mois), `date_ajouter(ts, n, "jours")`, `date_ajouter_mois(ts, n)`, `date_difference(a, b, "jours")`, `date_analyser("2026-10-14")`, `date_formater(ts, "JJ/MM/AAAA hh:mm:ss")`, `date_debut(ts, "jour")`, `date_jours_dans_mois(an, mois)`, `date_semaine(ts)`, `date_age(naissance)`, `est_bissextile(an)` |
| **JSON** | `json_lire(texte)`, `json_texte(valeur)` ou `json_texte(valeur, 2)` (mis en forme, indentation de 2), `json_valide(texte)`, `json_chemin(valeur, "a.b[2].c")` ou avec valeur par défaut, `json_fusionner(a, b)` |
| **Expressions régulières** (le motif suit le texte) | `regex_test(texte, motif)`, `regex_trouver(texte, motif)` (liste de toutes les correspondances), `regex_groupes(texte, motif)` (première correspondance : `[tout, groupe 1, groupe 2…]`), `regex_remplacer(texte, motif, remplacement)` (`$1`, `$2`… pour les groupes), `regex_decouper(texte, motif)`, `regex_echapper(texte)` |
| **Texte** | `texte_remplir_gauche/droite(t, n, car)`, `texte_capitaliser`, `texte_titre`, `texte_lignes`, `texte_mots`, `texte_sans_accents`, `texte_slug`, `texte_est_palindrome`, `texte_distance(a, b)` (Levenshtein), `url_encoder`, `url_decoder` |
| **Maths étendues** | `pi()`, `sinus(x)`, `cosinus(x)`, `tangente(x)`, `arc_sinus(x)`, `arc_cosinus(x)`, `arc_tangente(x)`, `arc_tangente2(y, x)` (angles en **radians**), `radians(deg)`, `degres(rad)`, `exponentielle(x)`, `logarithme(x)` (népérien) ou `logarithme(x, base)`, `logarithme10(x)`, `signe(x)` (-1, 0 ou 1), `modulo(a, b)` |
| **Copie** | `copier(x)` : copie profonde d'une liste ou d'une structure (les listes imbriquées sont aussi recopiées) |
| **Texte à trous** | `texte_format(modèle, valeurs)` : `{}` (valeurs dans l'ordre), `{0}` `{1}` (par position), `{nom}` (clé d'une structure) ; `{{` et `}}` donnent une accolade |
| **Bases numériques** | `en_base(255, 16)` → `"ff"` ; `depuis_base("ff", 16)` → `255` (bases 2 à 36, entiers jusqu'à 9007199254740991) |
| **Aléatoire sécurisé** | `alea_entier(min, max)`, `alea_choisir(liste)`, `alea_melanger(liste)`, `alea_uuid()` |
| **Listes** | `liste_decouper(l, n)`, `liste_zip(a, b)`, `liste_cumul(l)`, `liste_frequences(l)` |
| **CSV** | `csv_lire(texte, séparateur)` (les nombres sont convertis), `csv_texte(lignes, séparateur)` |
| **Temps** | `pause(ms)` (max 10 s), `chrono()` |
| **HTTP** | `http_obtenir(url)`, `http_json(url)`, `http_envoyer(url, corps, entetes)` → structure `{statut, ok, corps}` |

Unités de durée acceptées : `ms`, `secondes`, `minutes`, `heures`, `jours`, `semaines`.

**Dates, précisions**
- `date_ajouter_mois(ts, n)` garde le même jour du mois, ou le dernier jour si le mois d'arrivée est plus court : le 31 janvier plus 1 mois donne le 28 février (29 en année bissextile).
- `date_debut(ts, unité)` ramène au début du `"jour"`, de la `"semaine"` (qui commence le **lundi**), du `"mois"` ou de l'`"annee"`.
- `date_semaine(ts)` donne le numéro de semaine **ISO** (la semaine 1 contient le premier jeudi de l'année ; certaines années ont 53 semaines).
- `date_age(naissance)` donne l'âge en années révolues aujourd'hui ; `date_age(naissance, ref)` à la date `ref`.
- Modèle de `date_formater` : `AAAA`, `MM`, `JJ`, `hh`, `mm`, `ss`, `jour` (« samedi ») et `mois` (« octobre »).

**JSON, précisions**
- `json_chemin` suit un chemin dans une structure ou une liste : `"client.adresses[1].ville"` (ou `client.adresses.1.ville`). Si le chemin n'existe pas, l'erreur dit où il s'arrête ; avec un 3e argument, celui-ci est renvoyé à la place (`json_chemin(d, "client.tel", "inconnu")`).
- `json_fusionner(a, b)` fusionne deux structures en profondeur ; en cas de conflit, la valeur de `b` l'emporte. Les originaux ne sont pas modifiés.

**Expressions régulières, précisions**
- Drapeaux facultatifs en dernier argument : `"i"` (ignorer majuscules/minuscules), `"m"` (`^` et `$` valent pour chaque ligne), `"s"` (le point couvre aussi les retours à la ligne), `"u"`. Exemple : `regex_test("ABC", "abc", "i")`.
- Un motif invalide donne une erreur claire plutôt qu'un plantage.
- `regex_groupes` renvoie une liste vide s'il n'y a aucune correspondance ; un groupe facultatif absent vaut `""`.

> ⚠️ **Fuseau horaire** : avec le moteur JavaScript, les dates sont en **heure locale** du navigateur. Le moteur Rust/WebAssembly les traite pour l'instant comme de l'heure **UTC** : les deux moteurs ne donnent la même heure affichée que si le fuseau du navigateur est UTC.

> ⚠️ HTTP depuis le navigateur dépend des règles **CORS** : beaucoup de sites refusent. L'erreur affichée le précise.

```
soit demain = date_ajouter(maintenant(), 1, "jours")
afficher date_formater(demain, "jour JJ mois AAAA")
afficher texte_slug("Été à Dakar : 2026 !")      # ete-a-dakar-2026
afficher texte_distance("kitten", "sitting")      # 3
```

**Maths étendues, précisions**

```
afficher pi()                                  # 3.14159265359
afficher sinus(pi() / 2)                       # 1
afficher arrondir(sinus(pi()), 10)             # 0
afficher logarithme(1024, 2)                   # 10
afficher modulo(-7, 3)                         # 2   (alors que  -7 % 3  donne -1)
afficher degres(arc_tangente2(1, 1))           # 45
```

- Les angles sont toujours en **radians** ; `radians(180)` convertit des degrés.
- `sinus(pi())` ne donne pas exactement 0 mais `1.22464679915e-16` (un nombre à virgule ne peut pas représenter π exactement) : utilisez `arrondir(valeur, 10)` avant de comparer à 0.
- `logarithme(x)` est le logarithme népérien (`ln`). `logarithme(8, 2)` et `logarithme(1000, 10)` donnent des entiers exacts.
- Hors domaine, l'erreur est claire : `logarithme(0)`, `arc_sinus(2)`, `modulo(5, 0)`.
- `modulo(a, b)` renvoie un résultat du **signe de `b`** (le « vrai » modulo) ; l'opérateur `%` garde le signe de `a`.
- **Précision : arrondi correct, identique dans les deux moteurs.** `sinus`, `cosinus`, `tangente`, `arc_*`, `exponentielle`, `logarithme*` et `**` renvoient le nombre à virgule flottante le plus proche de la vraie valeur (un seul arrondi, calculé en « double-double », ≈106 bits). Les moteurs Rust et JavaScript donnent donc **les mêmes 64 bits**, vérifiés sur près de 2 millions de calculs et 878 valeurs de référence calculées avec mpmath. La garantie vaut pour |x| < 8·10⁸ (trigonométrie), -700 < x < 709,7 (exponentielle) et les puissances à résultat normal ; au-delà, on retombe sur la bibliothèque de la plate-forme (erreur < 1 ULP).

**Copie : références et copies**

Affecter une liste ou une structure à une autre variable **ne la copie pas**, les deux noms désignent la même liste. Pour avoir une copie indépendante :

```
soit a = [1, [2, 3]]
soit b = copier(a)
ajouter(lireElement(b, 1), 99)
afficher a        # [1, [2, 3]]   (inchangée)
afficher b        # [1, [2, 3, 99]]
```

Les fonctions et les valeurs simples (nombres, textes) ne sont pas dupliquées, elles sont déjà indépendantes. Les listes ou structures qui se contiennent elles-mêmes sont copiées sans boucle infinie.

**Texte à trous : `texte_format`**

```
afficher texte_format("{} a {} ans", ["Awa", 30])                  # Awa a 30 ans
afficher texte_format("{1}-{0}-{1}", ["a", "b"])                   # b-a-b
afficher texte_format("{nom} ({age})", {nom: "Awa", age: 30})      # Awa (30)
afficher texte_format("{{ok}} {}", [7])                            # {ok} 7
```

Les valeurs sont écrites comme le ferait `afficher`. Une valeur manquante, une clé inconnue ou une accolade isolée donne une erreur qui le dit. Avec une liste, `{nom}` est refusé ; avec une structure, `{}` est refusé.

**Bases numériques**

```
afficher en_base(255, 16)           # ff
afficher en_base(10, 2)             # 1010
afficher depuis_base("ff", 16)      # 255
afficher depuis_base("-1010", 2)    # -10
```

`depuis_base` accepte majuscules et minuscules et refuse un chiffre trop grand pour la base (`depuis_base("12", 2)` est une erreur).

### 19.2 Paquets `.fapkg` : partager des modules

Un paquet est **un seul fichier JSON** contenant un dossier de modules Fadéra.

```
mkdir maths
nano maths/outils.fa                 # vos fonctions
pkg empaqueter maths maths 1.0.0     # crée maths.fapkg
pkg install maths.fapkg              # (chez vous ou chez quelqu'un d'autre)
pkg install https://site/maths.fapkg # depuis une adresse web (soumis à CORS)
pkg list                             # section « PAQUETS FADÉRA »
pkg remove maths
```

Le paquet s'installe dans `~/lib/maths/` et s'utilise dans le langage :

```
importer "lib/maths/outils"
afficher carre(7)
```

Sécurité : les chemins absolus ou contenant `..` sont refusés, un paquet est limité à 200 fichiers, et rien n'est exécuté à l'installation.

**Limites actuelles** : pas de numéros de version dans les dépendances, pas de dépôt central.

### 19.3 Modules standard : `importer "std/…"`

Douze modules sont **intégrés au moteur** (aucun fichier à créer ; les mêmes sources servent aux deux moteurs). Ils s'importent comme un module normal :

```
importer "std/maths"
afficher factorielle(5)          # 120
afficher premiers_jusqua(20)     # [2, 3, 5, 7, 11, 13, 17, 19]
```

| Module | Fonctions |
|---|---|
| `std/maths` | `factorielle(n)`, `fibonacci(n)` (0 → 0, 1 → 1), `est_premier(n)`, `premiers_jusqua(n)`, `somme_chiffres(n)` (signe ignoré), `limiter(x, bas, haut)` |
| `std/textes` | `est_vide(t)` (vrai aussi pour des espaces seuls), `centrer(t, largeur)`, `tronquer(t, n)` (ajoute `…` si coupé), `inverser_texte(t)`, `compter_occurrences(t, motif)` (sans chevauchement) |
| `std/listes` | `premier(l)`, `dernier(l)` (erreur si vide), `prendre(l, n)`, `sauter(l, n)`, `compter_si(l, f)`, `partitionner(l, f)` (renvoie `[éléments où f est vrai, les autres]`) |
| `std/dates` | `date_longue(ts)` (« samedi 17 octobre 2026 »), `est_week_end(ts)`, `jours_entre(a, b)` (négatif si `b` est avant `a`), `ajouter_jours(ts, n)`, `debut_du_mois(ts)`, `fin_du_mois(ts)`, `meme_jour(a, b)`, `est_jour_ouvre(ts)`, `jours_ouvres_entre(a, b)`, `ajouter_jours_ouvres(ts, n)`, `prochain_jour(ts, "lundi")`, `calendrier(an, mois)` (texte, semaine du lundi au dimanche), `duree_texte(ms)` (« 1 j 2 h 3 min 4 s ») |
| `std/maths` (suite) | `pgcd`, `ppcm`, `somme`, `produit`, `moyenne`, `mediane`, `variance`, `ecart_type`, `combinaisons(n, k)`, `arrangements(n, k)`, `puissance_mod(b, e, m)`, `racine_entiere`, `est_carre_parfait`, `diviseurs(n)`, `interpoler(a, b, t)`, `transposer(m)`, `produit_matrices(a, b)` |
| `std/tri` | `tri_fusion(l, comparer)` (stable), `tri_rapide`, `tri_insertion`, `trier_decroissant(l)`, `trier_multi(l, [f1, f2])` (tri stable sur plusieurs clés), `est_trie`, `recherche_binaire(l, x)` (position ou -1), `borne_inf`, `borne_sup`, `indice_de`, `minimum_par(l, f)`, `maximum_par(l, f)`, `plus_petits(l, n)`, `plus_grands(l, n)`, `grouper_par(l, f)`, `ordre_naturel(a, b)` |
| `std/json` | `json_joli(v)`, `json_compact(v)`, `json_lire_ou(texte, repli)`, `json_aplatir(v)` (`{"a.b[0]": …}`), `json_profondeur(v)`, `json_selectionner(v, champs)`, `json_egal(a, b)` |
| `std/regex` | `correspond_entierement(t, motif)`, `compter_motif`, `extraire_nombres`, `extraire_mots`, `extraire_courriels`, `extraire_urls`, `est_courriel`, `est_url`, `est_entier`, `est_telephone`, `reduire_espaces`, `remplacer_chaque(t, motif, f)` |
| `std/pile` | classe `Pile` : `empiler(x)`, `depiler()`, `sommet()`, `longueur()`, `est_vide()`, `vider()`, `en_liste()` |
| `std/file` | classe `File` : `enfiler(x)`, `defiler()`, `tete()`, `longueur()`, `est_vide()`, `vider()`, `en_liste()` |
| `std/dictionnaire` | classe `Dictionnaire` (ordre d'insertion conservé) : `definir(cle, valeur)`, `obtenir(cle)`, `obtenir_ou(cle, repli)`, `possede(cle)`, `supprimer(cle)`, `longueur()`, `est_vide()`, `liste_cles()`, `liste_valeurs()`, `liste_paires()`, `vider()`, `en_structure()` |
| `std/ensemble` | classe `Ensemble` : `inclure(x)`, `exclure(x)`, `possede(x)`, `longueur()`, `est_vide()`, `en_liste()`, `union(autre)`, `intersection(autre)`, `difference(autre)` ; `ensemble_depuis(liste)` |
| `std/tas` | classe `Tas(comparer)` (file de priorité) : `pousser(x)`, `extraire()`, `premier()`, `longueur()`, `est_vide()` ; `tas_min()`, `tas_max()` |

Les classes se manipulent comme toute classe (§22) et les appels s'enchaînent : `Pile().empiler(1).empiler(2).depiler()`. Avec un alias, la classe se crée par `p.Pile()`. Les erreurs (`Pile vide : rien à dépiler.`) sont rattrapables par `essayer`/`attraper`, et portent le préfixe `[module std/pile.fa, ligne N]`.

```
importer "std/tas"
soit t = tas_min()
t.pousser(5).pousser(1).pousser(3)
afficher t.extraire()      # 1
afficher t.extraire()      # 3
```

**Tests de la bibliothèque** : le menu **Applications → 📚 Tests de la bibliothèque standard** lance une suite de 12 blocs `tester` (un par module ou presque : pile, file, dictionnaire, ensemble, tas, tri, recherches, JSON, regex, dates, maths, textes/listes) sans toucher à votre éditeur. Ces blocs sont aussi des modèles : copiez-en un dans votre programme pour vérifier vos propres fonctions avec les mêmes `verifier`, `verifier_egal` et `verifier_erreur` (§11c de la spécification). Le texte de la suite est dans `src/std/tests_std.fa` du projet Rust.

**`lever(message)`** déclenche une erreur que `essayer`/`attraper` peut rattraper : `lever("Solde insuffisant")`. C'est ce qu'utilisent les modules ci-dessus pour signaler un mauvais usage.

Un nom de module standard inconnu donne la liste des modules disponibles. Comme pour tout module, les noms définis (`premier`, `centrer`…) sont communs à tout le programme (§16.7) : ne les redéfinissez pas avant l'import si vous comptez sur les versions standard.

---

## 20. Brancher le moteur Rust/WebAssembly dans l'IDE

L'IDE fonctionne seul avec le moteur JavaScript. Si vous ajoutez le moteur Rust compilé en WebAssembly, il l'utilise automatiquement pour les programmes calculatoires (plus rapide).

1. Installer l'outillage (une fois) : `rustup target add wasm32-unknown-unknown` puis `cargo install wasm-pack`.
2. Dans le dossier `fadera-rs` (dézippé) : `wasm-pack build --release --target web -- --features wasm` → crée `pkg/`.
3. Mettre `index.html` et le dossier `pkg/` côte à côte et **servir par HTTP** (un `.wasm` ne se charge pas depuis `file://`) : `python3 -m http.server 8000`, puis ouvrir `http://localhost:8000`.
4. Dans le terminal de l'IDE : `moteur` (état), `moteur bench` (compare les deux moteurs sur la même boucle).

| Commande | Effet |
|---|---|
| `moteur` | affiche le mode et si Rust est chargé |
| `moteur auto` | Rust si chargé **et** compatible, sinon JavaScript (défaut) |
| `moteur js` | toujours le moteur JavaScript (référence) |
| `moteur rust` | Rust ; avertit quand il faut basculer sur JavaScript |
| `moteur bench` | mesure JavaScript et Rust |

### 20.1 Le mode Worker : `saisir`, fichiers, réseau et bouton ■ Stop avec Rust

Avec le Worker, **tous** les programmes tournent en Rust, y compris ceux qui utilisent `saisir`, `importer`, `lireFichier`, `ecrireFichier`, `creer_fichier`, `ecrire_log`, `pause` et `http_*`. Le moteur tourne dans un fil séparé : la sortie s'affiche en direct, l'interface reste fluide, et le bouton rouge **■** (à côté de ▶) arrête une boucle infinie.

Il faut **servir l'IDE avec `serveur.py`** (fourni à côté de `index.html`) au lieu de `python -m http.server` : il ajoute les en-têtes d'isolation « cross-origin » qui autorisent `SharedArrayBuffer`.

1. Reconstruire le moteur (le binding a changé) : dans `fadera-rs`, `wasm-pack build --release --target web -- --features wasm`.
2. Remplacer le dossier `pkg/` à côté de `index.html` par le nouveau.
3. Arrêter l'ancien serveur (Ctrl+C) puis lancer : `python serveur.py` → ouvrir `http://localhost:8000`.
4. Tapez `moteur` : la ligne « Worker … : actif » doit apparaître. Un message vert 🧵 s'affiche aussi au chargement.

Sans `serveur.py`, **Rust exécute quand même tout** : il tourne alors dans la page, avec `saisir` (fenêtre du navigateur), les fichiers de l'IDE, `importer` et `http_*` (requête synchrone). Il ne manque que ce que seul un Worker permet : le bouton ■ Stop, une page qui reste fluide pendant un long calcul, et le débogage pas à pas avec Rust. Avec un vieux `pkg/` (sans `executer_hote`), l'IDE retombe sur l'ancien comportement : JavaScript pour les programmes qui ont besoin de `saisir`/fichiers.
Navigateurs : Chrome et Edge (récents) ou Firefox 119+ ; Safari n'est pas pris en charge pour ce mode.

**Limites actuelles :**
- les dates `date_*` sont en heure UTC avec Rust, en heure locale avec JavaScript ;
- `http_*` côté Rust utilise une requête synchrone du Worker : mêmes règles CORS que dans le navigateur ;
- un fichier lu ou écrit ne peut pas dépasser 8 Mo par appel ;
- le bouton ■ ne fonctionne qu'avec le moteur Rust (le moteur JavaScript ne peut pas être interrompu).

La compilation WebAssembly n'a pas pu être exécutée dans l'environnement où le code a été écrit ; le Worker a été testé avec un module simulé (saisie, fichiers, pause, Stop, erreurs), le vrai `wasm-pack` est donc la première vraie exécution. Si elle signale une erreur, elle vient de cette étape.

### 20.2 Performances : exécution « par tranches »

Le moteur Rust contient déjà un compilateur vers bytecode (`src/vm.rs`). Dans le navigateur, une fonction WebAssembly qui ne rend jamais la main reste compilée par le compilateur de base de Chrome (2 à 3 fois plus lent). Le moteur découpe donc l'exécution en tranches de 20 000 tours de boucle : les appels suivants utilisent le code optimisé. La sémantique ne change pas (357 programmes identiques entre JS et Rust). Pour en profiter : recompiler (`wasm-pack build --release --target web -- --features wasm`) et remplacer `pkg/`. `moteur bench` mesure le résultat.

### 20.3 Panneau des variables avec le moteur Rust

Après chaque exécution, le moteur Rust renvoie l'état final du programme (variables globales et fonctions), comme le moteur JavaScript, même quand le programme s'arrête sur une erreur (on voit alors les variables telles qu'elles étaient). Les commandes du terminal qui lisent cet état fonctionnent donc avec les deux moteurs :

| Commande | Effet |
|---|---|
| `variables` (ou `vars`) | liste les variables globales et les fonctions du dernier programme |
| `variables nom` | valeur d'une variable |
| `inspecter nom` | valeur d'une variable (avec le moniteur RAM) |
| `functions` / `inspecter_fonction f` | fonctions déclarées, paramètres, taille |

Remarques : les listes et structures très imbriquées (plus de 8 niveaux) ou qui se contiennent elles-mêmes sont abrégées en `[…]` / `{…}` ; pour une fonction, Rust indique le nombre d'instructions du corps. Il faut recompiler (`wasm-pack build --release --target web -- --features wasm`) et remplacer `pkg/` : avec un ancien `pkg/`, le panneau reste vide comme avant, sans erreur.

### 20.4 Mesurer sur des programmes réalistes : `moteur bench tout`

`moteur bench tout` exécute 9 programmes typiques (récursion, crible de nombres premiers, tri, textes, structures, matrices, boucles, fonctionnel, objets) avec le moteur JavaScript puis avec Rust/WebAssembly, vérifie que les résultats sont identiques et affiche le gain par programme. Les tailles sont réduites : la mesure dure une dizaine de secondes.

Mesures faites pendant le développement (Rust natif contre JavaScript dans V8, programmes de taille normale) : le moteur Rust est 5 à 60 fois plus rapide selon le programme (le plus fort gain : récursion ; le plus faible : matrices et objets avec tri). Un seul cas était plus lent que JavaScript : construire un très long texte par `s = s + …` dans une boucle (le texte était recopié à chaque tour). C'est corrigé : `s = s + a + b` ajoute maintenant en place (programme de test : 675 ms → 32 ms).

### 20.5 Worker « chaud »

Le Worker qui exécute le moteur Rust n'est plus recréé à chaque ▶ : il est préparé dès l'ouverture de la page (le module WebAssembly est chargé pendant que vous écrivez) puis réutilisé d'une exécution à l'autre. Les exécutions suivantes démarrent immédiatement et profitent du code déjà optimisé par le navigateur. Après un ■ Stop ou un plantage, un nouveau Worker est préparé automatiquement.

`moteur bench tout` affiche maintenant deux colonnes pour Rust : « Rust 1er » (premier passage, code pas encore optimisé) et « Rust 2e » (second passage, code chaud) ; le gain est calculé sur le second passage.

### 20.6 Erreurs détaillées avec Rust

Le module Rust renvoie maintenant, en plus du texte de l'erreur, son message seul, sa ligne, la pile d'appels et la ligne du programme d'où part l'appel (`executer` et `executer_hote`). L'IDE s'en sert pour afficher la même présentation qu'avec le moteur JavaScript (§12.1). Recompilez avec `wasm-pack build --release --target web -- --features wasm` pour en profiter ; sans recompilation tout continue de fonctionner, sans la pile d'appels.

---

### 20.7 Maths à arrondi correct (identiques partout)

Les fonctions mathématiques du moteur Rust (`src/mathfn.rs`) sont écrites en Rust pur, sans dépendre de la bibliothèque du système ni de celle du navigateur : le résultat est le même sur votre PC, sur Linux et dans WebAssembly. Le moteur JavaScript contient **le même algorithme** (`MathFD` dans `index.html`), si bien que les deux moteurs donnent les mêmes 64 bits.

Vérifier vous-même : `cargo build --release --example dump_maths` puis `node tests/verifier_maths.js ../index.html` compare les deux moteurs bit à bit sur des centaines de milliers d'entrées, et contre les valeurs de référence de `tests/maths_vecteurs.txt` (calculées avec mpmath en 300 bits).

### 20.8 Un moteur de référence, un moteur de secours

- **Rust/WebAssembly est le moteur de référence** : c'est lui qui exécute vos programmes dès que le dossier `pkg/` est présent et que vous lancez l'IDE avec `python serveur.py`.
- **JavaScript est le moteur de secours, et rien d'autre** : il prend le relais uniquement si `pkg/` est absent ou ne se charge pas (l'IDE vous le dit, avec la raison, une seule fois par session). Dès que `pkg/` est présent, **tout passe par Rust** : exécution, `saisir`, fichiers, `importer`, tests `tester`, vérification de syntaxe, ligne de commande interactive, évaluation pendant une pause. Une seule exception, structurelle : le débogage pas à pas avec Rust exige le Worker (le navigateur ne permet pas de bloquer la page pendant une pause) ; sans `serveur.py`, ou avec un `pkg/` ancien, le débogueur JavaScript prend le relais, et l'en-tête du terminal le dit.
- Les deux sont maintenus **identiques** : `node verifier_tout.js` (avec `fadera-rs/` dézippé à côté de `index.html`) compile, puis lance tous les tests, la parité sur plus de 500 programmes, la spécification exécutable et la comparaison bit à bit des maths.
- `moteur` affiche l'état ; `moteur js` force le moteur de secours (utile pour comparer).

## 21. Déboguer pas à pas

Le débogueur exécute le script **une ligne à la fois** : vous voyez la ligne en cours, les variables et la pile d'appels, et vous décidez quand avancer.

### 21.1 Lancer et poser des points d'arrêt

- **Lancer** : bouton **⏸ Déboguer** de la barre de l'éditeur (ou menu, ou commande `deboguer` / `debug`).
- **Point d'arrêt** : cliquez sur un **numéro de ligne** dans la marge (pastille rouge), ou placez le curseur sur la ligne et appuyez sur **F9**. Un second clic le retire.
- Sans point d'arrêt, le programme s'arrête **sur sa première instruction**. Avec des points d'arrêt, il s'arrête sur le premier atteint.
- Les points d'arrêt sont **mémorisés par fichier** dans le navigateur. En terminal : `pointarret 12 18` (poser), `pointarret -12` (retirer), `pointarret` (liste), `pointarret effacer`.

### 21.2 Avancer

| Bouton | Touche | Effet |
|---|---|---|
| ▶ Continuer | F8 | jusqu'au prochain point d'arrêt (ou la fin) |
| ⤼ Pas à pas | F10 | exécute la ligne **sans entrer** dans les fonctions appelées |
| ↴ Entrer | F11 | entre dans la fonction appelée sur la ligne |
| ↱ Sortir | Maj+F11 | termine la fonction en cours et s'arrête au retour |
| ■ Arrêter | Maj+F5 | arrête le programme |

### 21.3 Ce qu'on voit à chaque pause

- **La ligne en cours** : flèche ▶ dans la marge et bande surlignée dans l'éditeur.
- **Variables** : celles de la fonction en cours (« dans la fonction »), puis les globales, avec leur type et leur valeur. Une valeur qui vient de changer est mise en évidence.
- **Pile d'appels** : la fonction en cours en haut, puis ses appelantes avec la ligne d'où part chaque appel.
- **Le terminal reste utilisable** : tapez une expression (`total * 2`, `x + 100`) pour calculer avec les variables visibles à cet endroit, sans modifier le programme.

L'éditeur est verrouillé en lecture seule pendant le débogage ; il se déverrouille à la fin, à l'arrêt ou sur une erreur. Une erreur pendant le débogage s'affiche comme d'habitude (§12.1) et ferme la session.

### 21.4 À savoir

- Le débogueur utilise **le moteur Rust/WebAssembly** quand il est actif en mode Worker (il s'arrête aux mêmes lignes, avec les mêmes variables et la même pile que le moteur JavaScript : `node tests/parite_debogueur.js` le vérifie sur des dizaines de scénarios), et **le moteur JavaScript** sinon. L'en-tête du terminal dit lequel. Après une mise à jour du moteur Rust, refaites `wasm-pack build --release --target web -- --features wasm` pour que `pkg/` contienne le débogueur ; sans cela, l'IDE vous prévient et retombe sur JavaScript.
- Il n'y a **pas de pause dans le code d'un module** importé (`importer`) : ses numéros de ligne sont ceux d'un autre fichier.
- Le débogage est plus lent qu'une exécution normale : à réserver à la recherche d'un bug, pas aux calculs lourds.

---

## 22. Classes et méthodes

Une classe regroupe des données (les champs d'un objet) et les fonctions qui les utilisent (les méthodes). `classe` n'est pas un mot réservé : rien ne change pour vos programmes existants.

```
classe Compte
    fonction creer(titulaire, solde)
        soi.titulaire = titulaire
        soi.solde = solde
    fin
    fonction deposer(montant)
        soi.solde = soi.solde + montant
        renvoyer soi
    fin
    fonction resume()
        renvoyer soi.titulaire + " : " + soi.solde
    fin
fin

soit c = Compte("Awa", 100)
c.deposer(50)
afficher c.resume()     # Awa : 150
afficher c              # Compte { titulaire: "Awa", solde: 150 }
```

- **`soi`** désigne l'objet courant dans chaque méthode ; il est ajouté tout seul aux paramètres (ne l'écrivez pas).
- **`creer`** est le constructeur, facultatif. `Compte("Awa", 100)` crée l'objet, appelle `creer`, et renvoie l'objet.
- Un objet est une **structure** : `c.solde` se lit et s'écrit comme n'importe quel champ, `taille(c)` compte ses champs, `copier(c)` le duplique en gardant sa classe.
- **Héritage** : `classe Epargne herite Compte` reprend les méthodes de `Compte`. Le constructeur n'est pas hérité : écrivez `creer` et appelez celui du parent avec `Compte.creer(soi, titulaire, solde)`.
- `classe_de(c)` donne le nom de la classe (ou `rien`), `est_instance(c, "Compte")` dit si `c` est un `Compte` ou en hérite.
- Les méthodes s'appellent aussi sur une **structure ordinaire** si elle contient une fonction dans un champ : `{f: x => x * 2}.f(3)` s'écrit `s.f(3)`.
- **Enchaînement** : `Compte("Awa", 1).deposer(5).resume()` est accepté ; chaque point lit un champ ou appelle une méthode sur le résultat précédent. Le point doit suivre immédiatement la `)` ou le `]`.
- Les messages d'erreur sont explicites : `La méthode "nope" n'existe pas dans la classe "Compte".`, `Le champ "z" n'existe pas dans "c" (champs disponibles : titulaire, solde).`

Le débogueur et les tests marchent avec les classes : une pause s'arrête dans le corps des méthodes (la pile montre `Compte.deposer`), et `tester` vérifie vos classes comme le reste.


---

## 23. L'éditeur : onglets, recherche, coloration, complétion

**Onglets.** Chaque fichier ouvert a son onglet au-dessus de l'éditeur. Clic = passer à ce fichier (le curseur et le défilement sont retrouvés) ; `×` ou clic du milieu = fermer ; glisser-déposer = changer l'ordre ; `+` = nouveau fichier. Le dernier onglet ne se ferme pas. `Ctrl+Alt+→` / `Ctrl+Alt+←` changent d'onglet, `Ctrl+Alt+W` ferme, `Ctrl+Alt+N` crée un fichier. La commande `nano fichier.fa` du terminal ouvre aussi le fichier dans un onglet, et un fichier supprimé ou renommé dans le terminal disparaît ou suit dans la barre. L'exécution (▶), le débogage et les tests portent toujours sur l'onglet actif. Changer d'onglet repart d'un historique d'annulation vierge pour ce fichier (jamais d'annulation qui traverse deux fichiers).

**Recherche et remplacement.** `Ctrl+F` (recherche) et `Ctrl+H` (recherche et remplacement) ouvrent le panneau en haut à droite ; une sélection d'une ligne est reprise comme texte cherché. Toutes les occurrences sont surlignées et comptées. `Entrée` / `Maj+Entrée` (ou `F3` / `Maj+F3`) vont à la suivante / précédente en bouclant. Options : `Aa` respecter la casse (`Alt+C`), `ab` mot entier (`Alt+W`, les lettres accentuées comptent comme des lettres), `.*` expression régulière (`Alt+R`, avec `$1`, `$2`, `$&` dans le remplacement ; une expression invalide met le champ en rouge). **Remplacer** change l'occurrence courante, **Tout** remplace tout **en une seule étape d'annulation** (`Ctrl+Z`).

**Coloration.** Elle vient du vrai lexeur et du vrai analyseur du langage, pas d'expressions régulières : mots-clés, textes, nombres, commentaires, noms de fonctions et de classes (à la déclaration comme à l'appel), paramètres, champs (`soi.solde`), espaces de noms d'import (`o.double`), fonctions natives. Les mots contextuels (`classe`, `herite`, `tester`, `comme`, `chaque`, `dans`, `pas`) ne sont colorés comme mots-clés que là où l'analyseur les reconnaît : une variable nommée `pas` reste une variable. Un texte non fermé n'abîme pas la coloration des autres lignes. Après une courte pause de frappe, si l'analyseur refuse le code, l'endroit fautif est souligné en rouge et le message s'affiche dans la barre d'état (un clic y amène le curseur).

**Complétion.** Elle apparaît en tapant (ou avec `Ctrl+Espace`) ; `↑` `↓` choisissent, `Tab` ou `Entrée` valident, `Échap` ferme. `Entrée` sur un mot déjà tapé en entier fait simplement un retour à la ligne (et les raccourcis `si`, `pour`… + `Entrée` se développent comme avant). Les propositions viennent de l'analyseur :
- variables, compteurs de boucle, paramètres, fonctions et classes **visibles à l'endroit du curseur** (une variable déclarée plus bas, ou un paramètre d'une autre fonction, n'est pas proposé) ;
- après `objet.` : les champs et méthodes de la classe de l'objet (héritage compris), pour `soit c = Compte(…)` comme pour `Compte(…).` ; `soi.` dans une méthode ; les clés d'une `structure` ; les fonctions et classes d'un module importé `comme alias` ;
- après `importer "` : les modules `std/…` et vos propres fichiers `.fa` ;
- toutes les fonctions natives avec leur signature, les mots-clés et des modèles de blocs (`si`, `pour`, `fonction`, `classe`, `tester`…).
Aucune proposition dans un texte, dans un commentaire, ni après `soit` / `fonction` / `classe` (vous nommez quelque chose de nouveau).

## 24. Sauvegarde durable, export et import

Vos fichiers vivent dans le dossier personnel du système de fichiers de l'IDE (le même que celui du terminal) et sont enregistrés **dans IndexedDB**, la base du navigateur, quelques instants après chaque modification, puis immédiatement à la fermeture ou au passage en arrière-plan de la page. La barre d'état indique `💾 enregistré 14:32:05`. Les onglets ouverts (avec curseur et défilement) sont restaurés au rechargement. `Ctrl+S` ou **Applications → Enregistrer** force l'écriture. Au premier lancement avec cette version, l'ancien contenu (système de fichiers et projets enregistrés auparavant dans `localStorage`) est copié dans IndexedDB, sans rien supprimer de l'ancien.

Si IndexedDB est indisponible (navigation privée, stockage bloqué), l'IDE le dit (`⚠ stockage temporaire`) : exportez alors vos fichiers.

**Exporter** : *Applications → ⬇ Exporter tous les projets (.zip)* (tout le dossier personnel) ou *⬇ Exporter les onglets ouverts (.zip)*. L'archive est un `.zip` standard (noms UTF-8, compression deflate, sommes de contrôle).

**Importer** : *Applications → ⬆ Importer* (un `.zip` ou des fichiers `.fa`), ou glisser-déposer des fichiers sur l'éditeur. Si des fichiers existent déjà, l'IDE propose *Remplacer*, *Garder les deux* (le nouveau devient `nom (importé).fa`), *Ignorer les doublons* ou *Annuler*. Par prudence, les chemins absolus ou contenant `..` sont refusés, les archives de plus de 64 Mo ou de plus de 5 000 fichiers aussi, et les fichiers qui ne sont pas du texte UTF-8 sont ignorés (et comptés dans le message).

## 25. Formateur de code : `fadera fmt`

Le formateur remet un programme en forme : indentation (4 espaces, ou `--indentation N`), espaces autour des opérateurs et des virgules, lignes vides multiples réduites à une, commentaires conservés. Il s'appuie sur le vrai lexeur et sur l'analyseur : **un code qui ne s'analyse pas n'est jamais reformaté** (l'erreur de syntaxe est affichée), il ne déplace aucun mot d'une ligne à l'autre, et il vérifie à la fin que le texte produit contient exactement les mêmes éléments que l'original. Reformater un code déjà formaté ne change rien.

```
fadera fmt script.fa                 # affiche le code formaté
fadera fmt --ecrire script.fa autre.fa   # réécrit les fichiers (alias : -w)
fadera fmt --verifier src/*.fa       # ne modifie rien ; code de sortie 1 si un fichier n'est pas formaté
fadera fmt --indentation 2 script.fa
cat script.fa | fadera fmt           # entrée standard
```

Dans l'IDE : **Applications → ✨ Formater le script** ou `Maj+Alt+F` ; le curseur reste sur sa ligne et `Ctrl+Z` annule. Dans le terminal de l'IDE, `fmt script.fa` (`fmt -w` pour écrire) fait la même chose. Les deux moteurs (Rust et JavaScript) produisent exactement le même texte.

## 26. Distribution et partage

**Exécutable.** Chaque version publiée sur GitHub (page *Releases*) contient `fadera.exe` (Windows), `fadera` (Linux) et `fadera` (macOS, Apple Silicon), construits par la CI, avec le guide et la licence, plus un fichier `SHA256SUMS` pour vérifier les téléchargements. Windows peut avertir (SmartScreen) car l'exécutable n'est pas signé.

```
fadera run programme.fa       # exécuter un fichier (alias : fadera programme.fa) ; les « importer » cherchent à côté du fichier
fadera -e 'afficher 2 + 3'
fadera tester programme.fa    # blocs  tester "nom" … fin  (code de sortie 1 si un test échoue)
fadera --version
```

Codes de sortie de `fadera run` : 0 succès, 1 erreur du programme (message sur la sortie d'erreur), 2 fichier illisible.

**Playground en ligne.** `index.html` fonctionne tel quel sur n'importe quel hébergement statique ; le dépôt le publie sur GitHub Pages. Le bouton **🔗 Partager (lien)** crée une adresse `…#p=…` qui contient les onglets ouverts (compressés dans l'adresse elle-même : rien n'est envoyé à un serveur). Celui qui ouvre le lien voit la liste des fichiers et doit confirmer ; ils sont copiés dans le dossier `partage/` de son espace et **rien n'est exécuté automatiquement**.

**Versions.** Numérotation `MAJEUR.MINEUR.CORRECTIF`, notes dans `CHANGELOG.md`. À chaque modification du dépôt, la CI relance `node verifier_tout.js` (tests Rust et JavaScript, parité JS/Rust, formateur, débogueur, spécification, IDE dans Chromium), puis compile et teste sous Windows et macOS.

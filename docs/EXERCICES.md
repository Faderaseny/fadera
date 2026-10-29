# Exercices Fadéra

94 exercices corrigés, classés par domaine, avec leur niveau (★ facile, ★★ moyen, ★★★ plus long), couvrant tout ce que Fadéra sait faire :
calculs, décisions et boucles, textes, listes, structures, fonctions, erreurs, fichiers (CSV, JSON), dates, mathématiques et statistiques,
classes, modules standard, tests intégrés, réseau, chiffrement, intelligence artificielle, robotique et voix, jeux, mini-projets
et applications dans l'industrie et les métiers (qualité, maintenance, stocks, facturation, logistique, énergie, agriculture, finance, télécoms, sécurité).

## Comment s'en servir

1. Lisez l'énoncé et **essayez sans regarder le corrigé** : copiez-le dans l'éditeur, lancez, corrigez.
2. Cliquez sur « Corrigé » pour comparer. Il y a souvent plusieurs bonnes solutions : seul le résultat compte.
3. Chaque corrigé est aussi un fichier prêt à l'emploi dans [`docs/exercices/`](exercices/) ; lancez-le avec `fadera run docs/exercices/013_fizzbuzz.fa`.

Les corrigés sont **exécutés automatiquement sur les deux moteurs** de Fadéra à chaque modification du projet
(`node docs/verifier_exemples.js exercices`) : le résultat affiché ici est le vrai.

## Sommaire

- [Bases](#bases) — 4 exercices (1, 2, 3, 4)
- [Décisions et boucles](#decisions-et-boucles) — 7 exercices (10, 11, 12, 13, 14, 15, 16)
- [Texte](#texte) — 6 exercices (20, 21, 22, 23, 24, 25)
- [Listes](#listes) — 5 exercices (30, 31, 32, 33, 34)
- [Structures](#structures) — 5 exercices (40, 41, 42, 43, 44)
- [Fonctions et récursivité](#fonctions-et-recursivite) — 5 exercices (50, 51, 52, 53, 54)
- [Erreurs](#erreurs) — 3 exercices (60, 61, 62)
- [Fichiers](#fichiers) — 4 exercices (70, 71, 72, 73)
- [Dates](#dates) — 2 exercices (80, 81)
- [Mathématiques](#mathematiques) — 8 exercices (90, 91, 92, 93, 94, 95, 96, 97)
- [Classes](#classes) — 3 exercices (100, 101, 102)
- [Modules standard](#modules-standard) — 6 exercices (110, 111, 112, 113, 114, 115)
- [Tests intégrés](#tests-integres) — 1 exercice (120)
- [Réseaux](#reseaux) — 3 exercices (130, 131, 132)
- [Chiffrement](#chiffrement) — 3 exercices (140, 141, 142)
- [Intelligence artificielle](#intelligence-artificielle) — 3 exercices (150, 151, 152)
- [Robotique et voix](#robotique-et-voix) — 4 exercices (160, 161, 162, 163)
- [Jeux et simulations](#jeux-et-simulations) — 4 exercices (170, 171, 172, 173)
- [Mini-projets](#mini-projets) — 4 exercices (180, 181, 182, 183)
- [Industrie et métiers](#industrie-et-metiers) — 14 exercices (190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203)

## Bases

### Exercice 1 — Bonjour personnalisé

*Niveau : ★*

> Stockez votre prénom et votre âge dans deux variables, puis affichez la phrase :
> « Bonjour, je m'appelle Awa et j'ai 17 ans. » (avec vos valeurs).

<details>
<summary>Corrigé</summary>

```fadera
soit prenom = "Awa"
soit age = 17
afficher "Bonjour, je m'appelle " + prenom + " et j'ai " + age + " ans."
```

Résultat :

```
Bonjour, je m'appelle Awa et j'ai 17 ans.
```

</details>

### Exercice 2 — Celsius vers Fahrenheit

*Niveau : ★*

> Convertissez 0, 25, 37 et 100 degrés Celsius en Fahrenheit (F = C × 9 / 5 + 32)
> et affichez « 25 °C = 77 °F » pour chaque valeur.

<details>
<summary>Corrigé</summary>

```fadera
pour chaque c dans [0, 25, 37, 100] faire
  afficher c + " °C = " + (c * 9 / 5 + 32) + " °F"
fin
```

Résultat :

```
0 °C = 32 °F
25 °C = 77 °F
37 °C = 98.6 °F
100 °C = 212 °F
```

</details>

### Exercice 3 — Rendre la monnaie

*Niveau : ★★*

> Un client doit 3 650 FCFA et donne un billet de 10 000 FCFA. Calculez la monnaie à rendre
> puis le nombre de billets de 5 000 et de 1 000 puis de pièces de 500, 100 et 50 nécessaires (le moins possible).
> Indice : la division entière s'obtient avec plancher(a / b), le reste avec a % b.

<details>
<summary>Corrigé</summary>

```fadera
soit a_rendre = 10000 - 3650
afficher "Monnaie à rendre : " + a_rendre
soit reste = a_rendre
pour chaque valeur dans [5000, 1000, 500, 100, 50] faire
  soit n = plancher(reste / valeur)
  reste = reste % valeur
  afficher n + " x " + valeur
fin
afficher "Reste non rendu : " + reste
```

Résultat :

```
Monnaie à rendre : 6350
1 x 5000
1 x 1000
0 x 500
3 x 100
1 x 50
Reste non rendu : 0
```

</details>

### Exercice 4 — Surface et périmètre d'un cercle

*Niveau : ★*

> Pour un rayon de 7, affichez le périmètre et l'aire d'un cercle arrondis à 2 décimales
> (utilisez pi()).

<details>
<summary>Corrigé</summary>

```fadera
soit r = 7
afficher "Périmètre : " + arrondir(2 * pi() * r, 2)
afficher "Aire : " + arrondir(pi() * r * r, 2)
```

Résultat :

```
Périmètre : 43.98
Aire : 153.94
```

</details>

## Décisions et boucles

### Exercice 10 — Pair ou impair

*Niveau : ★*

> Pour chaque nombre de 1 à 8, affichez s'il est pair ou impair.

<details>
<summary>Corrigé</summary>

```fadera
pour n allantde 1 a 8 faire
  si n % 2 == 0 alors
    afficher n + " est pair"
  sinon
    afficher n + " est impair"
  fin
fin
```

Résultat :

```
1 est impair
2 est pair
3 est impair
4 est pair
5 est impair
6 est pair
7 est impair
8 est pair
```

</details>

### Exercice 11 — Mentions

*Niveau : ★★*

> Écrivez une fonction mention(moyenne) qui renvoie « Très bien » à partir de 16, « Bien » à partir de 14,
> « Assez bien » à partir de 12, « Passable » à partir de 10, sinon « Ajourné ». Testez-la sur 17, 14.5, 12, 10 et 7.

<details>
<summary>Corrigé</summary>

```fadera
fonction mention(moyenne)
  si moyenne >= 16 alors
    renvoyer "Très bien"
  sinon si moyenne >= 14 alors
    renvoyer "Bien"
  sinon si moyenne >= 12 alors
    renvoyer "Assez bien"
  sinon si moyenne >= 10 alors
    renvoyer "Passable"
  fin
  renvoyer "Ajourné"
fin

pour chaque m dans [17, 14.5, 12, 10, 7] faire
  afficher m + " → " + mention(m)
fin
```

Résultat :

```
17 → Très bien
14.5 → Bien
12 → Assez bien
10 → Passable
7 → Ajourné
```

</details>

### Exercice 12 — Jour de la semaine avec « selon »

*Niveau : ★★*

> Écrivez une fonction nom_jour(n) (1 = lundi … 7 = dimanche) avec la structure « selon ».
> Pour 6 et 7 elle renvoie « week-end », pour un numéro invalide « inconnu ». Affichez le résultat pour 1 à 8.

<details>
<summary>Corrigé</summary>

```fadera
fonction nom_jour(n)
  selon n faire
    cas 1 alors
      renvoyer "lundi"
    cas 2 alors
      renvoyer "mardi"
    cas 3 alors
      renvoyer "mercredi"
    cas 4 alors
      renvoyer "jeudi"
    cas 5 alors
      renvoyer "vendredi"
    cas 6, 7 alors
      renvoyer "week-end"
    defaut alors
      renvoyer "inconnu"
  fin
fin

pour n allantde 1 a 8 faire
  afficher n + " : " + nom_jour(n)
fin
```

Résultat :

```
1 : lundi
2 : mardi
3 : mercredi
4 : jeudi
5 : vendredi
6 : week-end
7 : week-end
8 : inconnu
```

</details>

### Exercice 13 — FizzBuzz

*Niveau : ★★*

> De 1 à 15 : affichez « Fizz » pour les multiples de 3, « Buzz » pour les multiples de 5,
> « FizzBuzz » pour les multiples des deux, sinon le nombre.

<details>
<summary>Corrigé</summary>

```fadera
pour n allantde 1 a 15 faire
  si n % 15 == 0 alors
    afficher "FizzBuzz"
  sinon si n % 3 == 0 alors
    afficher "Fizz"
  sinon si n % 5 == 0 alors
    afficher "Buzz"
  sinon
    afficher n
  fin
fin
```

Résultat :

```
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

</details>

### Exercice 14 — Somme des chiffres

*Niveau : ★★*

> Avec une boucle « tantque », calculez la somme des chiffres de 98 765 (utilisez % 10 et plancher(n / 10)).

<details>
<summary>Corrigé</summary>

```fadera
soit n = 98765
soit somme_chiffres = 0
tantque n > 0 faire
  somme_chiffres = somme_chiffres + n % 10
  n = plancher(n / 10)
fin
afficher "Somme des chiffres : " + somme_chiffres
```

Résultat :

```
Somme des chiffres : 35
```

</details>

### Exercice 15 — Suite de Syracuse

*Niveau : ★★★*

> Partez de 27 : si le nombre est pair, divisez-le par 2 ; sinon multipliez-le par 3 et ajoutez 1.
> Comptez le nombre d'étapes avant d'atteindre 1 et affichez aussi la valeur maximale atteinte.

<details>
<summary>Corrigé</summary>

```fadera
soit n = 27
soit etapes = 0
soit sommet = n
tantque n != 1 faire
  si n % 2 == 0 alors
    n = n / 2
  sinon
    n = 3 * n + 1
  fin
  si n > sommet alors
    sommet = n
  fin
  etapes = etapes + 1
fin
afficher "Étapes : " + etapes
afficher "Maximum : " + sommet
```

Résultat :

```
Étapes : 111
Maximum : 9232
```

</details>

### Exercice 16 — Triangle d'étoiles

*Niveau : ★★*

> Affichez un triangle de 5 lignes : 1 étoile sur la première, 2 sur la deuxième… (utilisez repeter).

<details>
<summary>Corrigé</summary>

```fadera
pour i allantde 1 a 5 faire
  afficher repeter("*", i)
fin
```

Résultat :

```
*
**
***
****
*****
```

</details>

## Texte

### Exercice 20 — Compter les voyelles

*Niveau : ★★*

> Comptez les voyelles (a, e, i, o, u, y) dans « Programmer en français est un plaisir ».

<details>
<summary>Corrigé</summary>

```fadera
soit phrase = minuscule("Programmer en français est un plaisir")
soit n = 0
pour i allantde 0 a taille(phrase) - 1 faire
  si contient("aeiouy", caractere(phrase, i)) alors
    n = n + 1
  fin
fin
afficher "Voyelles : " + n
```

Résultat :

```
Voyelles : 12
```

</details>

### Exercice 21 — Palindrome

*Niveau : ★★*

> Écrivez une fonction est_palindrome(t) qui ignore les majuscules, les espaces, les accents et la ponctuation.
> Testez « Esope reste ici et se repose » et « Fadéra ». (Indice : texte_sans_accents, regex_remplacer, inverser.)

<details>
<summary>Corrigé</summary>

```fadera
fonction est_palindrome(t)
  soit propre = regex_remplacer(minuscule(texte_sans_accents(t)), "[^a-z0-9]", "", "")
  renvoyer propre == inverser(propre)
fin

afficher est_palindrome("Esope reste ici et se repose")
afficher est_palindrome("Fadéra")
afficher est_palindrome("Engage le jeu que je le gagne")
```

Résultat :

```
vrai
faux
vrai
```

</details>

### Exercice 22 — Le mot le plus long

*Niveau : ★★*

> Trouvez le mot le plus long de « La programmation en français rend l'informatique accessible ».

<details>
<summary>Corrigé</summary>

```fadera
soit mots = decouper("La programmation en français rend l'informatique accessible", " ")
soit plus_long = ""
pour chaque m dans mots faire
  si taille(m) > taille(plus_long) alors
    plus_long = m
  fin
fin
afficher plus_long + " (" + taille(plus_long) + " lettres)"
```

Résultat :

```
l'informatique (14 lettres)
```

</details>

### Exercice 23 — Chiffre de César

*Niveau : ★★★*

> Écrivez cesar(texte, decalage) qui décale chaque lettre minuscule de l'alphabet (a→d pour un décalage de 3),
> sans toucher aux autres caractères. Chiffrez « bonjour fadera » avec 3 puis déchiffrez le résultat.
> Indice : code_caractere, depuis_code, modulo.

<details>
<summary>Corrigé</summary>

```fadera
fonction cesar(t, decalage)
  soit sortie = ""
  pour i allantde 0 a taille(t) - 1 faire
    soit c = caractere(t, i)
    soit code = code_caractere(c)
    si code >= 97 et code <= 122 alors
      sortie = sortie + depuis_code(97 + modulo(code - 97 + decalage, 26))
    sinon
      sortie = sortie + c
    fin
  fin
  renvoyer sortie
fin

soit secret = cesar("bonjour fadera", 3)
afficher secret
afficher cesar(secret, -3)
```

Résultat :

```
erqmrxu idghud
bonjour fadera
```

</details>

### Exercice 24 — Acronyme et adresse web

*Niveau : ★★*

> À partir de « Université Cheikh Anta Diop », fabriquez l'acronyme (initiales en majuscules)
> puis un « slug » utilisable dans une adresse web (texte_slug).

<details>
<summary>Corrigé</summary>

```fadera
soit nom = "Université Cheikh Anta Diop"
soit acronyme = ""
pour chaque mot dans decouper(nom, " ") faire
  acronyme = acronyme + majuscule(caractere(mot, 0))
fin
afficher acronyme
afficher texte_slug(nom)
```

Résultat :

```
UCAD
universite-cheikh-anta-diop
```

</details>

### Exercice 25 — Extraire des numéros avec une expression régulière

*Niveau : ★★★*

> Dans le texte « Appelez le 77 123 45 67 ou le 78 987 65 43, pas le 12 », extrayez les numéros sénégalais
> (deux chiffres, puis 3, 2 et 2 chiffres, séparés par des espaces) avec regex_trouver et affichez-les sans espaces.

<details>
<summary>Corrigé</summary>

```fadera
soit texte_source = "Appelez le 77 123 45 67 ou le 78 987 65 43, pas le 12"
soit numeros = regex_trouver(texte_source, "[0-9]{2} [0-9]{3} [0-9]{2} [0-9]{2}", "")
pour chaque n dans numeros faire
  afficher remplacer(n, " ", "")
fin
afficher taille(numeros) + " numéro(s) trouvé(s)"
```

Résultat :

```
771234567
789876543
2 numéro(s) trouvé(s)
```

</details>

## Listes

### Exercice 30 — Somme, minimum, maximum

*Niveau : ★*

> Pour la liste [12, 7, 25, 3, 18], affichez la somme, le plus petit, le plus grand et la moyenne,
> en parcourant la liste vous-même (sans stats_*).

<details>
<summary>Corrigé</summary>

```fadera
soit valeurs = [12, 7, 25, 3, 18]
soit total = 0
soit petit = lireElement(valeurs, 0)
soit grand = lireElement(valeurs, 0)
pour chaque v dans valeurs faire
  total = total + v
  petit = minimum(petit, v)
  grand = maximum(grand, v)
fin
afficher "Somme : " + total
afficher "Minimum : " + petit
afficher "Maximum : " + grand
afficher "Moyenne : " + total / taille(valeurs)
```

Résultat :

```
Somme : 65
Minimum : 3
Maximum : 25
Moyenne : 13
```

</details>

### Exercice 31 — Filtrer et transformer

*Niveau : ★★*

> Avec filtrer et transformer : à partir de plage(1, 10), gardez les nombres pairs puis élevez-les au carré.
> Affichez ensuite leur somme avec reduire.

<details>
<summary>Corrigé</summary>

```fadera
soit pairs = filtrer(plage(1, 10), x => x % 2 == 0)
soit carres = transformer(pairs, x => x * x)
afficher carres
afficher reduire(carres, (t, x) => t + x, 0)
```

Résultat :

```
[4, 16, 36, 64, 100]
220
```

</details>

### Exercice 32 — Tri à bulles

*Niveau : ★★★*

> Triez [64, 34, 25, 12, 22, 11, 90] par ordre croissant avec l'algorithme du tri à bulles
> (sans la fonction trier). Affichez la liste obtenue.

<details>
<summary>Corrigé</summary>

```fadera
soit l = [64, 34, 25, 12, 22, 11, 90]
soit n = taille(l)
pour i allantde 0 a n - 2 faire
  pour j allantde 0 a n - 2 - i faire
    si lireElement(l, j) > lireElement(l, j + 1) alors
      soit temp = lireElement(l, j)
      modifierElement(l, j, lireElement(l, j + 1))
      modifierElement(l, j + 1, temp)
    fin
  fin
fin
afficher l
```

Résultat :

```
[11, 12, 22, 25, 34, 64, 90]
```

</details>

### Exercice 33 — Doublons et fréquences

*Niveau : ★★*

> Pour [3, 1, 3, 2, 1, 3, 4], affichez la liste sans doublons, puis combien de fois chaque valeur apparaît
> (unique, liste_frequences ou compter).

<details>
<summary>Corrigé</summary>

```fadera
soit l = [3, 1, 3, 2, 1, 3, 4]
afficher unique(l)
pour chaque v dans unique(l) faire
  afficher v + " : " + compter(l, v) + " fois"
fin
```

Résultat :

```
[3, 1, 2, 4]
3 : 3 fois
1 : 2 fois
2 : 1 fois
4 : 1 fois
```

</details>

### Exercice 34 — Matrice transposée

*Niveau : ★★★*

> Une matrice est une liste de listes. Écrivez transposer(m) et affichez la transposée de [[1, 2, 3], [4, 5, 6]].

<details>
<summary>Corrigé</summary>

```fadera
fonction transposer(m)
  soit lignes = taille(m)
  soit colonnes = taille(lireElement(m, 0))
  soit resultat = []
  pour c allantde 0 a colonnes - 1 faire
    soit nouvelle = []
    pour r allantde 0 a lignes - 1 faire
      ajouter(nouvelle, lireElement(lireElement(m, r), c))
    fin
    ajouter(resultat, nouvelle)
  fin
  renvoyer resultat
fin

pour chaque ligne dans transposer([[1, 2, 3], [4, 5, 6]]) faire
  afficher ligne
fin
```

Résultat :

```
[1, 4]
[2, 5]
[3, 6]
```

</details>

## Structures

### Exercice 40 — Fiche d'élève

*Niveau : ★*

> Créez une structure { nom, classe, notes } pour un élève, ajoutez la note 15 à ses notes,
> puis affichez « Awa (3e A) : moyenne 13 » avec la moyenne calculée.

<details>
<summary>Corrigé</summary>

```fadera
soit eleve = { nom: "Awa", classe: "3e A", notes: [12, 14, 13] }
ajouter(lireCle(eleve, "notes"), 13)
afficher lireCle(eleve, "nom") + " (" + lireCle(eleve, "classe") + ") : moyenne " + stats_moyenne(lireCle(eleve, "notes"))
```

Résultat :

```
Awa (3e A) : moyenne 13
```

</details>

### Exercice 41 — Classement

*Niveau : ★★*

> Voici des élèves { nom, note }. Triez-les du meilleur au moins bon (trier_par) et affichez
> « 1. Fatou — 18 », « 2. Awa — 15 »… Indice : une clé négative inverse l'ordre.

<details>
<summary>Corrigé</summary>

```fadera
soit eleves = [
  { nom: "Awa", note: 15 },
  { nom: "Moussa", note: 12 },
  { nom: "Fatou", note: 18 },
  { nom: "Ibrahima", note: 9 }
]
soit rang = 1
pour chaque e dans trier_par(eleves, x => -lireCle(x, "note")) faire
  afficher rang + ". " + lireCle(e, "nom") + " — " + lireCle(e, "note")
  rang = rang + 1
fin
```

Résultat :

```
1. Fatou — 18
2. Awa — 15
3. Moussa — 12
4. Ibrahima — 9
```

</details>

### Exercice 42 — Inventaire d'une boutique

*Niveau : ★★*

> Un stock est une structure produit → quantité. Ajoutez 5 sacs de riz, retirez la clé « sucre »,
> puis affichez chaque produit avec sa quantité (cles, lireCle, supprimer_cle) et le total d'articles.

<details>
<summary>Corrigé</summary>

```fadera
soit stock = { riz: 20, sucre: 8, huile: 12 }
modifierCle(stock, "riz", lireCle(stock, "riz") + 5)
supprimer_cle(stock, "sucre")
soit total = 0
pour chaque produit dans cles(stock) faire
  afficher produit + " : " + lireCle(stock, produit)
  total = total + lireCle(stock, produit)
fin
afficher "Total : " + total
afficher "Du sucre ? " + a_cle(stock, "sucre")
```

Résultat :

```
riz : 25
huile : 12
Total : 37
Du sucre ? faux
```

</details>

### Exercice 43 — Compter les mots

*Niveau : ★★★*

> Comptez les occurrences de chaque mot de « le chat et le chien et le chat » dans une structure,
> puis affichez les mots du plus fréquent au moins fréquent (en cas d'égalité, par ordre alphabétique).
> Indice : la clé de tri peut être une liste [critère 1, critère 2] : Fadéra les compare élément par élément.

<details>
<summary>Corrigé</summary>

```fadera
soit comptes = {}
pour chaque mot dans decouper("le chat et le chien et le chat", " ") faire
  si a_cle(comptes, mot) alors
    modifierCle(comptes, mot, lireCle(comptes, mot) + 1)
  sinon
    modifierCle(comptes, mot, 1)
  fin
fin
soit classement = trier_par(cles(comptes), m => [-lireCle(comptes, m), m])
pour chaque m dans classement faire
  afficher m + " : " + lireCle(comptes, m)
fin
```

Résultat :

```
le : 3
chat : 2
et : 2
chien : 1
```

</details>

### Exercice 44 — Trier selon plusieurs critères

*Niveau : ★★★*

> Voici des élèves { nom, classe, note }. Triez-les par classe (ordre alphabétique), puis, dans chaque classe,
> par note décroissante, puis par nom. Faites-le deux fois : avec trier_par (une clé en liste)
> puis avec trier_multi du module std/tri (une fonction par critère). Vérifiez que les deux résultats sont identiques.

<details>
<summary>Corrigé</summary>

```fadera
importer "std/tri"

soit eleves = [
  { nom: "Awa", classe: "B", note: 15 },
  { nom: "Moussa", classe: "A", note: 12 },
  { nom: "Fatou", classe: "A", note: 18 },
  { nom: "Ibrahima", classe: "B", note: 15 },
  { nom: "Mariama", classe: "A", note: 12 }
]

# Façon 1 : une seule fonction qui renvoie la liste des critères
soit tri1 = trier_par(eleves, e => [e.classe, -e.note, e.nom])

# Façon 2 : une fonction par critère (le premier l'emporte)
soit tri2 = trier_multi(eleves, [e => e.classe, e => -e.note, e => e.nom])

pour chaque e dans tri1 faire
  afficher e.classe + "  " + texte_remplir_droite(e.nom, 9, " ") + e.note
fin
afficher "Mêmes résultats ? " + (json_texte(tri1, 0) == json_texte(tri2, 0))
```

Résultat :

```
A  Fatou    18
A  Mariama  12
A  Moussa   12
B  Awa      15
B  Ibrahima 15
Mêmes résultats ? vrai
```

</details>

## Fonctions et récursivité

### Exercice 50 — Factorielle

*Niveau : ★★*

> Écrivez factorielle(n) de façon récursive. Affichez les factorielles de 0 à 6.

<details>
<summary>Corrigé</summary>

```fadera
fonction factorielle(n)
  si n <= 1 alors
    renvoyer 1
  fin
  renvoyer n * factorielle(n - 1)
fin

pour n allantde 0 a 6 faire
  afficher n + "! = " + factorielle(n)
fin
```

Résultat :

```
0! = 1
1! = 1
2! = 2
3! = 6
4! = 24
5! = 120
6! = 720
```

</details>

### Exercice 51 — Suite de Fibonacci

*Niveau : ★★*

> Affichez les 12 premiers termes de la suite de Fibonacci (0, 1, 1, 2, 3, 5…) sans récursivité,
> avec une boucle et deux variables.

<details>
<summary>Corrigé</summary>

```fadera
soit a = 0
soit b = 1
soit termes = []
pour i allantde 1 a 12 faire
  ajouter(termes, a)
  soit suivant = a + b
  a = b
  b = suivant
fin
afficher joindre(termes, ", ")
```

Résultat :

```
0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89
```

</details>

### Exercice 52 — PGCD par l'algorithme d'Euclide

*Niveau : ★★*

> Écrivez pgcd(a, b) récursif : pgcd(a, 0) = a et pgcd(a, b) = pgcd(b, a % b).
> Calculez pgcd(84, 36) et simplifiez la fraction 84/36. Vérifiez avec math_pgcd_ppcm.

<details>
<summary>Corrigé</summary>

```fadera
fonction pgcd(a, b)
  si b == 0 alors
    renvoyer a
  fin
  renvoyer pgcd(b, a % b)
fin

soit d = pgcd(84, 36)
afficher "PGCD : " + d
afficher "Fraction simplifiée : " + 84 / d + "/" + 36 / d
afficher "Vérification : " + lireCle(math_pgcd_ppcm(84, 36), "pgcd")
```

Résultat :

```
PGCD : 12
Fraction simplifiée : 7/3
Vérification : 12
```

</details>

### Exercice 53 — Tours de Hanoï

*Niveau : ★★★*

> Écrivez hanoi(n, depart, arrivee, aide) qui affiche les déplacements pour déplacer n disques,
> et compte les coups. Pour 3 disques il faut 7 coups.

<details>
<summary>Corrigé</summary>

```fadera
soit coups = 0
fonction hanoi(n, depart, arrivee, aide)
  si n == 0 alors
    renvoyer 0
  fin
  hanoi(n - 1, depart, aide, arrivee)
  coups = coups + 1
  afficher "Disque " + n + " : " + depart + " → " + arrivee
  hanoi(n - 1, aide, arrivee, depart)
fin

hanoi(3, "A", "C", "B")
afficher "Total : " + coups + " coups"
```

Résultat :

```
Disque 1 : A → C
Disque 2 : A → B
Disque 1 : C → B
Disque 3 : A → C
Disque 1 : B → A
Disque 2 : B → C
Disque 1 : A → C
Total : 7 coups
```

</details>

### Exercice 54 — Fonctions en argument

*Niveau : ★★★*

> Écrivez appliquer_deux_fois(f, x) qui renvoie f(f(x)). Utilisez-la avec « doubler » (x => x * 2)
> et avec une fonction qui ajoute « ! » à un texte. Écrivez aussi composer(f, g) qui renvoie une fonction.

<details>
<summary>Corrigé</summary>

```fadera
fonction appliquer_deux_fois(f, x)
  renvoyer f(f(x))
fin

fonction composer(f, g)
  renvoyer x => f(g(x))
fin

afficher appliquer_deux_fois(x => x * 2, 5)
afficher appliquer_deux_fois(t => t + "!", "Fadéra")
soit plus_un_puis_double = composer(x => x * 2, x => x + 1)
afficher plus_un_puis_double(4)
```

Résultat :

```
20
Fadéra!!
10
```

</details>

## Erreurs

### Exercice 60 — Division sécurisée

*Niveau : ★★*

> Écrivez diviser(a, b) qui déclenche une erreur claire (lever) si b vaut 0. Appelez-la avec (10, 2) puis (1, 0)
> en rattrapant l'erreur avec essayer / attraper pour afficher son message.

<details>
<summary>Corrigé</summary>

```fadera
fonction diviser(a, b)
  si b == 0 alors
    lever("Division par zéro impossible.")
  fin
  renvoyer a / b
fin

pour chaque couple dans [[10, 2], [1, 0]] faire
  essayer
    afficher "Résultat : " + diviser(lireElement(couple, 0), lireElement(couple, 1))
  attraper erreur
    afficher "Erreur : " + erreur
  fin
fin
```

Résultat :

```
Résultat : 5
Erreur : Division par zéro impossible.
```

</details>

### Exercice 61 — Valider un âge

*Niveau : ★★*

> Écrivez lire_age(texte_saisi) : le texte doit représenter un nombre entier entre 0 et 120 (est_nombre).
> Elle renvoie l'âge ou lève une erreur explicite. Testez-la sur « 25 », « abc », « -4 » et « 130 ».

<details>
<summary>Corrigé</summary>

```fadera
fonction lire_age(t)
  si non est_nombre(t) alors
    lever("« " + t + " » n'est pas un nombre.")
  fin
  soit n = nombre(t)
  si n < 0 ou n > 120 alors
    lever(n + " n'est pas un âge réaliste.")
  fin
  renvoyer entier(n)
fin

pour chaque saisie dans ["25", "abc", "-4", "130"] faire
  essayer
    afficher "Âge valide : " + lire_age(saisie)
  attraper e
    afficher "Refusé : " + e
  fin
fin
```

Résultat :

```
Âge valide : 25
Refusé : « abc » n'est pas un nombre.
Refusé : -4 n'est pas un âge réaliste.
Refusé : 130 n'est pas un âge réaliste.
```

</details>

### Exercice 62 — Réessayer après une erreur

*Niveau : ★★★*

> Une opération échoue les deux premières fois puis réussit. Écrivez une boucle qui réessaie jusqu'à
> 5 fois et affiche le numéro de l'essai réussi. (L'opération incrémente un compteur global et lève une erreur tant qu'il est < 3.)

<details>
<summary>Corrigé</summary>

```fadera
soit tentatives = 0
fonction operation_capricieuse()
  tentatives = tentatives + 1
  si tentatives < 3 alors
    lever("échec n°" + tentatives)
  fin
  renvoyer "succès"
fin

soit resultat = ""
soit essai = 0
tantque essai < 5 et resultat == "" faire
  essai = essai + 1
  essayer
    resultat = operation_capricieuse()
  attraper e
    afficher "Essai " + essai + " : " + e
  fin
fin
afficher "Essai " + essai + " : " + resultat
```

Résultat :

```
Essai 1 : échec n°1
Essai 2 : échec n°2
Essai 3 : succès
```

</details>

## Fichiers

### Exercice 70 — Écrire puis relire un fichier

*Niveau : ★★*

> Écrivez trois lignes dans le fichier « courses.txt », relisez-le, découpez-le en lignes
> (texte_lignes) et affichez « 1. riz », « 2. huile »…

<details>
<summary>Corrigé</summary>

```fadera
ecrireFichier("courses.txt", "riz\nhuile\nsucre")
soit contenu = lireFichier("courses.txt")
soit numero = 1
pour chaque ligne dans texte_lignes(contenu) faire
  afficher numero + ". " + ligne
  numero = numero + 1
fin
```

Résultat :

```
1. riz
2. huile
3. sucre
```

</details>

### Exercice 71 — Moyenne par colonne dans un CSV

*Niveau : ★★★*

> Le texte CSV ci-dessous contient des élèves et leurs notes (séparateur « ; »). Avec csv_lire,
> calculez la moyenne de chaque matière et affichez l'élève qui a la meilleure moyenne générale.

<details>
<summary>Corrigé</summary>

```fadera
soit brut = "nom;maths;francais\nAwa;15;12\nMoussa;9;14\nFatou;18;16"
soit lignes = csv_lire(brut, ";")
soit entete = lireElement(lignes, 0)
soit donnees = tranche(lignes, 1, taille(lignes))
pour c allantde 1 a 2 faire
  soit colonne = transformer(donnees, l => nombre(lireElement(l, c)))
  afficher lireElement(entete, c) + " : " + arrondir(stats_moyenne(colonne), 2)
fin
soit meilleur = trier_par(donnees, l => -(nombre(lireElement(l, 1)) + nombre(lireElement(l, 2))))
afficher "Meilleure moyenne : " + lireElement(lireElement(meilleur, 0), 0)
```

Résultat :

```
maths : 14
francais : 14
Meilleure moyenne : Fatou
```

</details>

### Exercice 72 — Lire et modifier du JSON

*Niveau : ★★*

> À partir du JSON ci-dessous : affichez le prix du deuxième produit avec json_chemin, augmentez tous les prix de 10 %
> puis réécrivez le JSON en version compacte (json_texte).

<details>
<summary>Corrigé</summary>

```fadera
soit brut = "{\"boutique\": \"Awa\", \"produits\": [{\"nom\": \"thé\", \"prix\": 1000}, {\"nom\": \"café\", \"prix\": 2000}]}"
soit data = json_lire(brut)
afficher "Prix du 2e produit : " + json_chemin(data, "produits.1.prix", 0)
pour chaque p dans lireCle(data, "produits") faire
  modifierCle(p, "prix", lireCle(p, "prix") * 1.1)
fin
afficher json_texte(data, 0)
```

Résultat :

```
Prix du 2e produit : 2000
{"boutique":"Awa","produits":[{"nom":"thé","prix":1100},{"nom":"café","prix":2200}]}
```

</details>

### Exercice 73 — Journal d'événements

*Niveau : ★★★*

> Écrivez journaliser(message) qui ajoute une ligne « [n] message » à la fin du fichier « journal.log »
> (n = numéro de la ligne). Appelez-la 3 fois puis affichez le fichier.

<details>
<summary>Corrigé</summary>

```fadera
soit total = 0
fonction journaliser(message)
  total = total + 1
  soit ancien = ""
  essayer
    ancien = lireFichier("journal.log")
  attraper e
    ancien = ""
  fin
  ecrireFichier("journal.log", ancien + "[" + total + "] " + message + "\n")
fin

journaliser("démarrage")
journaliser("connexion de Awa")
journaliser("arrêt")
afficher lireFichier("journal.log")
```

Résultat :

```
[1] démarrage
[2] connexion de Awa
[3] arrêt
```

</details>

## Dates

### Exercice 80 — Âge et jours avant Noël

*Niveau : ★★*

> Pour une personne née le 15 mars 2008, calculez son âge le 24 octobre 2026 (date_age)
> et le nombre de jours jusqu'au 25 décembre 2026 (date_difference).

<details>
<summary>Corrigé</summary>

```fadera
soit naissance = date_creer(2008, 3, 15)
soit aujourdhui = date_creer(2026, 10, 24)
afficher "Âge : " + date_age(naissance, aujourdhui) + " ans"
afficher "Jours avant Noël : " + date_difference(aujourdhui, date_creer(2026, 12, 25), "jours")
```

Résultat :

```
Âge : 18 ans
Jours avant Noël : 62
```

</details>

### Exercice 81 — Planning de 4 semaines

*Niveau : ★★*

> À partir du 5 janvier 2026, affichez les dates de 4 réunions hebdomadaires au format JJ/MM/AAAA
> (date_ajouter, date_formater). Indiquez ensuite si 2028 est bissextile.

<details>
<summary>Corrigé</summary>

```fadera
soit debut = date_creer(2026, 1, 5)
pour i allantde 0 a 3 faire
  afficher "Réunion " + (i + 1) + " : " + date_formater(date_ajouter(debut, i * 7, "jours"), "JJ/MM/AAAA")
fin
afficher "2028 est bissextile ? " + est_bissextile(2028)
```

Résultat :

```
Réunion 1 : 05/01/2026
Réunion 2 : 12/01/2026
Réunion 3 : 19/01/2026
Réunion 4 : 26/01/2026
2028 est bissextile ? vrai
```

</details>

## Mathématiques

### Exercice 90 — Équation du second degré

*Niveau : ★★*

> Résolvez x² − 5x + 6 = 0 avec le discriminant (racine), puis vérifiez avec math_resoudre_second_degre.

<details>
<summary>Corrigé</summary>

```fadera
soit a = 1
soit b = -5
soit c = 6
soit delta = b * b - 4 * a * c
afficher "Discriminant : " + delta
si delta > 0 alors
  soit x1 = (-b - racine(delta)) / (2 * a)
  soit x2 = (-b + racine(delta)) / (2 * a)
  afficher "Solutions : " + x1 + " et " + x2
sinon si delta == 0 alors
  afficher "Solution double : " + (-b / (2 * a))
sinon
  afficher "Pas de solution réelle"
fin
afficher math_resoudre_second_degre(a, b, c)
```

Résultat :

```
Discriminant : 1
Solutions : 2 et 3
[2, 3]
```

</details>

### Exercice 91 — Crible d'Ératosthène

*Niveau : ★★★*

> Listez les nombres premiers jusqu'à 50 avec le crible d'Ératosthène (une liste de booléens).

<details>
<summary>Corrigé</summary>

```fadera
soit n = 50
soit premier = []
pour i allantde 0 a n faire
  ajouter(premier, i >= 2)
fin
pour i allantde 2 a entier(racine(n)) faire
  si lireElement(premier, i) alors
    pour j allantde i * i a n pas i faire
      modifierElement(premier, j, faux)
    fin
  fin
fin
soit resultat = []
pour i allantde 2 a n faire
  si lireElement(premier, i) alors
    ajouter(resultat, i)
  fin
fin
afficher resultat
```

Résultat :

```
[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
```

</details>

### Exercice 92 — Statistiques d'une classe

*Niveau : ★★*

> Pour les notes [12, 15, 8, 20, 15, 9, 14], affichez la moyenne (2 décimales), la médiane, le mode
> et l'écart-type (2 décimales) avec les fonctions stats_*.

<details>
<summary>Corrigé</summary>

```fadera
soit notes = [12, 15, 8, 20, 15, 9, 14]
afficher "Moyenne : " + arrondir(stats_moyenne(notes), 2)
afficher "Médiane : " + stats_mediane(notes)
afficher "Mode : " + stats_mode(notes)
afficher "Écart-type : " + arrondir(stats_ecart_type(notes), 2)
```

Résultat :

```
Moyenne : 13.29
Médiane : 14
Mode : [15]
Écart-type : 3.77
```

</details>

### Exercice 93 — Prévoir avec une droite

*Niveau : ★★★*

> Les ventes d'une boutique sur 5 mois sont [100, 120, 138, 161, 180]. Avec stats_regression_lineaire,
> affichez la droite obtenue puis prévoyez les ventes du mois 6. Affichez aussi le coefficient de corrélation.

<details>
<summary>Corrigé</summary>

```fadera
soit mois = [1, 2, 3, 4, 5]
soit ventes = [100, 120, 138, 161, 180]
soit r = stats_regression_lineaire(mois, ventes)
afficher r
soit pente = lireCle(r, "pente")
soit ordonnee = lireCle(r, "ordonnee_origine")
afficher "Prévision du mois 6 : " + arrondir(pente * 6 + ordonnee, 1)
afficher "Corrélation : " + arrondir(stats_pearson_correlation(mois, ventes), 3)
```

Résultat :

```
{ formule: "y = 20.1000 * x + 79.5000", pente: 20.1, ordonnee_origine: 79.5 }
Prévision du mois 6 : 200.1
Corrélation : 0.999
```

</details>

### Exercice 94 — Aire sous une courbe

*Niveau : ★★★*

> Calculez l'intégrale de x² entre 0 et 3 (valeur exacte : 9) en construisant 31 valeurs espacées de 0,1
> et en appelant math_integrale_trapeze. Arrondissez à 3 décimales.

<details>
<summary>Corrigé</summary>

```fadera
soit valeurs = []
pour i allantde 0 a 30 faire
  soit x = i * 0.1
  ajouter(valeurs, x * x)
fin
afficher arrondir(math_integrale_trapeze(valeurs, 0.1), 3)
```

Résultat :

```
9.005
```

</details>

### Exercice 95 — Résoudre un système d'équations

*Niveau : ★★★*

> Deux cahiers et un stylo coûtent 5 (en centaines de FCFA) ; un cahier coûte 1 de plus qu'un stylo.
> Avec x = prix d'un cahier et y = prix d'un stylo, résolvez 2x + y = 5 et x − y = 1 avec
> math_resoudre_systeme_gauss, puis vérifiez la solution en la remplaçant dans les deux équations.

<details>
<summary>Corrigé</summary>

```fadera
soit solution = math_resoudre_systeme_gauss([[2, 1, 5], [1, -1, 1]])
soit x = lireElement(solution, 0)
soit y = lireElement(solution, 1)
afficher "x = " + x + ", y = " + y
afficher "2x + y = " + (2 * x + y)
afficher "x − y = " + (x - y)
```

Résultat :

```
x = 2, y = 1
2x + y = 5
x − y = 1
```

</details>

### Exercice 96 — Inverser une matrice

*Niveau : ★★★*

> Calculez l'inverse de [[1, 2], [3, 4]] avec inverser_matrice, puis vérifiez en multipliant
> la matrice par son inverse à la main (on doit retrouver la matrice identité).

<details>
<summary>Corrigé</summary>

```fadera
soit m = [[1, 2], [3, 4]]
soit inv = inverser_matrice(m)
afficher inv
pour i allantde 0 a 1 faire
  soit ligne = []
  pour j allantde 0 a 1 faire
    soit somme_produits = 0
    pour k allantde 0 a 1 faire
      somme_produits = somme_produits + lireElement(lireElement(m, i), k) * lireElement(lireElement(inv, k), j)
    fin
    ajouter(ligne, arrondir(somme_produits, 6))
  fin
  afficher ligne
fin
```

Résultat :

```
[[-2, 1], [1.5, -0.5]]
[1, 0]
[0, 1]
```

</details>

### Exercice 97 — Repérer les valeurs aberrantes

*Niveau : ★★★*

> Pour les mesures [1, 2, 3, 4, 5, 6, 7, 8, 100], utilisez stats_analyse_distribution pour obtenir les seuils d'anomalie
> puis listez les valeurs en dehors de ces seuils.

<details>
<summary>Corrigé</summary>

```fadera
soit mesures = [1, 2, 3, 4, 5, 6, 7, 8, 100]
soit a = stats_analyse_distribution(mesures)
soit bas = lireCle(a, "seuil_anomalie_bas")
soit haut = lireCle(a, "seuil_anomalie_haut")
afficher "Seuils : " + bas + " à " + haut
afficher "Aberrantes : " + filtrer(mesures, x => x < bas ou x > haut)
```

Résultat :

```
Seuils : -3 à 13
Aberrantes : [100]
```

</details>

## Classes

### Exercice 100 — Une classe Rectangle

*Niveau : ★★*

> Écrivez une classe Rectangle (largeur, hauteur) avec les méthodes aire(), perimetre() et est_carre().
> Créez deux rectangles (3×4 et 5×5) et affichez leurs propriétés.

<details>
<summary>Corrigé</summary>

```fadera
classe Rectangle
  fonction creer(largeur, hauteur)
    soi.largeur = largeur
    soi.hauteur = hauteur
  fin

  fonction aire()
    renvoyer soi.largeur * soi.hauteur
  fin

  fonction perimetre()
    renvoyer 2 * (soi.largeur + soi.hauteur)
  fin

  fonction est_carre()
    renvoyer soi.largeur == soi.hauteur
  fin
fin

pour chaque r dans [Rectangle(3, 4), Rectangle(5, 5)] faire
  afficher r.largeur + "×" + r.hauteur + " : aire " + r.aire() + ", périmètre " + r.perimetre() + ", carré ? " + r.est_carre()
fin
```

Résultat :

```
3×4 : aire 12, périmètre 14, carré ? faux
5×5 : aire 25, périmètre 20, carré ? vrai
```

</details>

### Exercice 101 — Héritage et polymorphisme

*Niveau : ★★★*

> Écrivez une classe Animal (nom) avec la méthode parler() qui renvoie « … », puis Chien et Chat qui en héritent
> et redéfinissent parler(). Parcourez une liste mixte et affichez « Rex : Ouaf », « Mina : Miaou »…
> Affichez aussi la classe de chaque objet (classe_de) et si c'est un Animal (est_instance).

<details>
<summary>Corrigé</summary>

```fadera
classe Animal
  fonction creer(nom)
    soi.nom = nom
  fin
  fonction parler()
    renvoyer "…"
  fin
fin

classe Chien herite Animal
  fonction creer(nom)
    soi.nom = nom
  fin
  fonction parler()
    renvoyer "Ouaf"
  fin
fin

classe Chat herite Animal
  fonction creer(nom)
    soi.nom = nom
  fin
  fonction parler()
    renvoyer "Miaou"
  fin
fin

pour chaque a dans [Chien("Rex"), Chat("Mina"), Animal("Inconnu")] faire
  afficher a.nom + " (" + classe_de(a) + ") : " + a.parler() + " — animal ? " + est_instance(a, "Animal")
fin
```

Résultat :

```
Rex (Chien) : Ouaf — animal ? vrai
Mina (Chat) : Miaou — animal ? vrai
Inconnu (Animal) : … — animal ? vrai
```

</details>

### Exercice 102 — Un panier d'achats

*Niveau : ★★★*

> Écrivez une classe Panier avec ajouter_article(nom, prix, quantite), total() et une méthode qui affiche
> le ticket de caisse. Une quantité négative ou nulle doit déclencher une erreur. Testez avec deux articles
> puis un article invalide (rattrapé avec essayer).

<details>
<summary>Corrigé</summary>

```fadera
classe Panier
  fonction creer()
    soi.articles = []
  fin

  fonction ajouter_article(nom, prix, quantite)
    si quantite <= 0 alors
      lever("Quantité invalide pour " + nom)
    fin
    ajouter(soi.articles, { nom: nom, prix: prix, quantite: quantite })
  fin

  fonction total()
    soit t = 0
    pour chaque a dans soi.articles faire
      t = t + a.prix * a.quantite
    fin
    renvoyer t
  fin

  fonction ticket()
    pour chaque a dans soi.articles faire
      afficher texte_remplir_droite(a.nom, 10, " ") + texte_remplir_gauche(texte(a.quantite), 3, " ") + " x " + a.prix
    fin
    afficher "TOTAL : " + soi.total() + " FCFA"
  fin
fin

soit p = Panier()
p.ajouter_article("riz", 650, 2)
p.ajouter_article("huile", 1800, 1)
essayer
  p.ajouter_article("sucre", 700, 0)
attraper e
  afficher "Refusé : " + e
fin
p.ticket()
```

Résultat :

```
Refusé : Quantité invalide pour sucre
riz         2 x 650
huile       1 x 1800
TOTAL : 3100 FCFA
```

</details>

## Modules standard

### Exercice 110 — Ensembles : qui suit quoi ?

*Niveau : ★★*

> Avec le module std/ensemble : les élèves inscrits au club de maths sont Awa, Moussa, Fatou ; au club d'informatique
> Fatou, Ibrahima, Awa. Affichez ceux qui sont dans les deux clubs, dans au moins un, et seulement en maths.

<details>
<summary>Corrigé</summary>

```fadera
importer "std/ensemble"

soit maths = ensemble_depuis(["Awa", "Moussa", "Fatou"])
soit info = ensemble_depuis(["Fatou", "Ibrahima", "Awa"])
afficher "Les deux : " + trier_par(maths.intersection(info).en_liste(), x => x)
afficher "Au moins un : " + taille(maths.union(info).en_liste())
afficher "Seulement maths : " + maths.difference(info).en_liste()
```

Résultat :

```
Les deux : ["Awa", "Fatou"]
Au moins un : 4
Seulement maths : ["Moussa"]
```

</details>

### Exercice 111 — Un annuaire

*Niveau : ★★*

> Avec std/dictionnaire, créez un annuaire nom → numéro, ajoutez 3 contacts, changez un numéro,
> supprimez un contact et affichez ce qui reste (possede, obtenir_ou, liste_paires).

<details>
<summary>Corrigé</summary>

```fadera
importer "std/dictionnaire"

soit annuaire = Dictionnaire()
annuaire.definir("Awa", "77 111 11 11")
annuaire.definir("Moussa", "78 222 22 22")
annuaire.definir("Fatou", "70 333 33 33")
annuaire.definir("Awa", "77 999 99 99")
annuaire.supprimer("Moussa")
afficher "Contacts : " + annuaire.longueur()
afficher "Moussa existe ? " + annuaire.possede("Moussa")
afficher "Numéro de Moussa : " + annuaire.obtenir_ou("Moussa", "inconnu")
pour chaque paire dans annuaire.liste_paires() faire
  afficher lireElement(paire, 0) + " → " + lireElement(paire, 1)
fin
```

Résultat :

```
Contacts : 2
Moussa existe ? faux
Numéro de Moussa : inconnu
Awa → 77 999 99 99
Fatou → 70 333 33 33
```

</details>

### Exercice 112 — Tâches par priorité

*Niveau : ★★★*

> Avec le tas de std/tas (tas_min), traitez des tâches { priorite, nom } dans l'ordre de priorité croissante
> (1 = urgent). Le tas compare des nombres : poussez des paires [priorite, nom] sous forme de nombre
> priorite × 100 + numéro d'ordre d'arrivée, et gardez les noms dans une structure.

<details>
<summary>Corrigé</summary>

```fadera
importer "std/tas"

soit taches = [
  { priorite: 3, nom: "ranger le bureau" },
  { priorite: 1, nom: "réparer le serveur" },
  { priorite: 2, nom: "répondre aux courriels" },
  { priorite: 1, nom: "sauvegarder la base" }
]
soit tas = tas_min()
soit noms = {}
soit ordre = 0
pour chaque t dans taches faire
  soit cle = t.priorite * 100 + ordre
  modifierCle(noms, texte(cle), t.nom)
  tas.pousser(cle)
  ordre = ordre + 1
fin
tantque non tas.est_vide() faire
  soit cle = tas.extraire()
  afficher "P" + plancher(cle / 100) + " — " + lireCle(noms, texte(cle))
fin
```

Résultat :

```
P1 — réparer le serveur
P1 — sauvegarder la base
P2 — répondre aux courriels
P3 — ranger le bureau
```

</details>

### Exercice 113 — File d'impression

*Niveau : ★★*

> Une imprimante traite les documents dans l'ordre d'arrivée (std/file). Ajoutez 4 documents,
> traitez-en 2, ajoutez-en un cinquième puis affichez ce qui reste à imprimer.

<details>
<summary>Corrigé</summary>

```fadera
importer "std/file"

soit f = File()
pour chaque d dans ["rapport.pdf", "facture.pdf", "photo.png", "cv.docx"] faire
  f.enfiler(d)
fin
afficher "Impression : " + f.defiler()
afficher "Impression : " + f.defiler()
f.enfiler("menu.pdf")
afficher "En attente : " + f.longueur()
tantque non f.est_vide() faire
  afficher "Reste : " + f.defiler()
fin
```

Résultat :

```
Impression : rapport.pdf
Impression : facture.pdf
En attente : 3
Reste : photo.png
Reste : cv.docx
Reste : menu.pdf
```

</details>

### Exercice 114 — Recherche binaire

*Niveau : ★★★*

> Avec std/tri, triez [42, 7, 19, 3, 88, 56, 23] puis cherchez 19 et 20 avec recherche_binaire.
> Combien de comparaisons au maximum pour 1 000 000 d'éléments ? (logarithme(x, 2), plafond)

<details>
<summary>Corrigé</summary>

```fadera
importer "std/tri"

soit l = trier([42, 7, 19, 3, 88, 56, 23], (a, b) => a - b)
afficher l
afficher "Position de 19 : " + recherche_binaire(l, 19)
afficher "Position de 20 : " + recherche_binaire(l, 20)
afficher "Comparaisons pour un million d'éléments : " + plafond(logarithme(1000000, 2))
```

Résultat :

```
[3, 7, 19, 23, 42, 56, 88]
Position de 19 : 2
Position de 20 : -1
Comparaisons pour un million d'éléments : 20
```

</details>

### Exercice 115 — La boîte à outils std/maths

*Niveau : ★★*

> Avec std/maths : les nombres premiers jusqu'à 30, le PGCD et le PPCM de 12 et 18, le nombre de façons
> de choisir 3 élèves parmi 10 (combinaisons) et les diviseurs de 36.

<details>
<summary>Corrigé</summary>

```fadera
importer "std/maths"

afficher premiers_jusqua(30)
afficher "PGCD : " + pgcd(12, 18) + ", PPCM : " + ppcm(12, 18)
afficher "C(10, 3) = " + combinaisons(10, 3)
afficher "Diviseurs de 36 : " + diviseurs(36)
```

Résultat :

```
[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
PGCD : 6, PPCM : 36
C(10, 3) = 120
Diviseurs de 36 : [1, 2, 3, 4, 6, 9, 12, 18, 36]
```

</details>

## Tests intégrés

### Exercice 120 — Tester ses fonctions

*Niveau : ★★*

> Écrivez moyenne_ponderee(notes, coefficients) qui lève une erreur si les deux listes n'ont pas la même taille.
> Écrivez trois blocs « tester » : un calcul simple, un cas avec coefficients égaux et l'erreur de taille.
> Lancez-les avec :  fadera tester 120_tests_integres.fa

<details>
<summary>Corrigé</summary>

```fadera
fonction moyenne_ponderee(notes, coefficients)
  si taille(notes) != taille(coefficients) alors
    lever("Il faut autant de notes que de coefficients.")
  fin
  soit total = 0
  soit poids = 0
  pour i allantde 0 a taille(notes) - 1 faire
    total = total + lireElement(notes, i) * lireElement(coefficients, i)
    poids = poids + lireElement(coefficients, i)
  fin
  renvoyer total / poids
fin

tester "calcul simple"
  verifier_egal(moyenne_ponderee([10, 20], [1, 3]), 17.5)
fin

tester "coefficients égaux"
  verifier_egal(moyenne_ponderee([8, 12, 16], [2, 2, 2]), 12)
fin

tester "listes de tailles différentes"
  verifier_erreur(() => moyenne_ponderee([10], [1, 2]), "autant")
fin
```

Lancez les tests avec : `fadera tester docs/exercices/120_tests_integres.fa`

</details>

## Réseaux

### Exercice 130 — Plan d'adressage IP

*Niveau : ★★*

> Un réseau 192.168.10.0/24 doit être découpé : calculez, pour 192.168.10.0/26, 192.168.10.64/26 et 192.168.10.128/25,
> l'adresse de diffusion et le nombre d'hôtes utilisables (telecom_calculer_sous_reseau).

<details>
<summary>Corrigé</summary>

```fadera
pour chaque cidr dans [["192.168.10.0", 26], ["192.168.10.64", 26], ["192.168.10.128", 25]] faire
  soit r = telecom_calculer_sous_reseau(lireElement(cidr, 0), lireElement(cidr, 1))
  afficher lireElement(cidr, 0) + "/" + lireElement(cidr, 1) + " : diffusion " + lireCle(r, "adresse_diffusion_broadcast") + ", " + lireCle(r, "nombre_hotes_utilisables") + " hôtes"
fin
```

Résultat :

```
192.168.10.0/26 : diffusion 192.168.10.63, 62 hôtes
192.168.10.64/26 : diffusion 192.168.10.127, 62 hôtes
192.168.10.128/25 : diffusion 192.168.10.255, 126 hôtes
```

</details>

### Exercice 131 — Le chemin le plus court

*Niveau : ★★★*

> Construisez un réseau de 5 routeurs : A–B (4), A–C (1), C–B (2), B–D (5), C–D (8), D–E (3).
> Trouvez avec telecom_dijkstra_routage l'itinéraire le moins coûteux de A à E et son coût.
> Vérifiez aussi que le réseau est connexe.

<details>
<summary>Corrigé</summary>

```fadera
soit reseau = telecom_creer_topologie()
pour chaque l dans [["A", "B", 4], ["A", "C", 1], ["C", "B", 2], ["B", "D", 5], ["C", "D", 8], ["D", "E", 3]] faire
  reseau = telecom_ajouter_liaison(reseau, { de: lireElement(l, 0), a: lireElement(l, 1), cout: lireElement(l, 2) })
fin
afficher "Réseau connexe ? " + telecom_verifier_connexite(reseau)
soit chemin = telecom_dijkstra_routage(reseau, { depart: "A", arrivee: "E" })
afficher "Itinéraire : " + joindre(lireCle(chemin, "itineraire"), " → ")
afficher "Coût total : " + lireCle(chemin, "cout_total")
```

Résultat :

```
Réseau connexe ? vrai
Itinéraire : A → C → B → D → E
Coût total : 11
```

</details>

### Exercice 132 — Interroger une API sans planter

*Niveau : ★★★*

> Interrogez l'adresse https://api.github.com/repos/Faderaseny/fadera avec http_json et affichez le nom et
> la description du dépôt (json_chemin). Si le réseau est coupé, le programme doit afficher un message
> poli au lieu de s'arrêter (essayer / attraper).

<details>
<summary>Corrigé</summary>

```fadera
essayer
  soit depot = http_json("https://api.github.com/repos/Faderaseny/fadera")
  afficher "Dépôt : " + json_chemin(depot, "name", "?")
  afficher "Description : " + json_chemin(depot, "description", "(aucune)")
attraper e
  afficher "Réseau indisponible pour le moment."
fin
```

Cet exercice interroge internet : le résultat dépend de votre connexion (sans réseau, le message poli s'affiche).

</details>

## Chiffrement

### Exercice 140 — Ne jamais stocker un mot de passe en clair

*Niveau : ★★★*

> Écrivez enregistrer(mot_de_passe, sel) qui renvoie l'empreinte SHA-256 de sel + mot_de_passe,
> et verifier_connexion(saisie, sel, empreinte). Montrez qu'un bon mot de passe est accepté
> et qu'un mauvais est refusé.

<details>
<summary>Corrigé</summary>

```fadera
fonction enregistrer(mot_de_passe, sel)
  renvoyer crypto_hacher_sha256(sel + mot_de_passe)
fin

fonction verifier_connexion(saisie, sel, empreinte)
  renvoyer enregistrer(saisie, sel) == empreinte
fin

soit sel = "x7Qp"
soit empreinte = enregistrer("fadera2026", sel)
afficher "Empreinte : " + empreinte
afficher "Bon mot de passe : " + verifier_connexion("fadera2026", sel, empreinte)
afficher "Mauvais mot de passe : " + verifier_connexion("Fadera2026", sel, empreinte)
```

Résultat :

```
Empreinte : d0cd58da3c9d342433ccf491ecc19acf116767e24c040dbbe823b485b3baed3c
Bon mot de passe : vrai
Mauvais mot de passe : faux
```

</details>

### Exercice 141 — Chiffrer et déchiffrer

*Niveau : ★★*

> Chiffrez le message « Rendez-vous à 18h » avec un mot de passe, vérifiez que le texte chiffré est différent
> du message, puis déchiffrez-le avec le bon mot de passe. Que se passe-t-il avec un mauvais mot de passe ?

<details>
<summary>Corrigé</summary>

```fadera
soit message = "Rendez-vous à 18h"
soit secret = chiffrer(message, "baobab")
afficher "Différent du message ? " + (secret != message)
afficher "Déchiffré : " + dechiffrer(secret, "baobab")
essayer
  afficher "Avec un mauvais mot de passe : " + dechiffrer(secret, "mauvais")
attraper e
  afficher "Déchiffrement refusé avec un mauvais mot de passe."
fin
```

Résultat :

```
Différent du message ? vrai
Déchiffré : Rendez-vous à 18h
Déchiffrement refusé avec un mauvais mot de passe.
```

</details>

### Exercice 142 — Vérifier l'intégrité d'un message

*Niveau : ★★★*

> Deux services partagent la clé « cle-partagee ». L'un envoie un message et son HMAC-SHA256 ; l'autre
> recalcule le HMAC pour savoir si le message a été modifié en route. Montrez les deux cas.

<details>
<summary>Corrigé</summary>

```fadera
soit cle = "cle-partagee"
soit message = "virement:5000:vers:Awa"
soit signature = crypto_hmac_sha256(message, cle)
fonction authentique(recu, signature_recue)
  renvoyer crypto_hmac_sha256(recu, cle) == signature_recue
fin
afficher "Message intact : " + authentique(message, signature)
afficher "Message modifié : " + authentique("virement:50000:vers:Awa", signature)
```

Résultat :

```
Message intact : vrai
Message modifié : faux
```

</details>

## Intelligence artificielle

### Exercice 150 — Un neurone qui calcule ET et OU

*Niveau : ★★*

> Un neurone « tout ou rien » (activation heaviside) calcule ET avec les poids [1, 1] et le biais −1,5,
> et OU avec les mêmes poids et le biais −0,5. Affichez les tables de vérité avec ia_activation_perceptron.

<details>
<summary>Corrigé</summary>

```fadera
soit et_neurone = { poids: [1, 1], biais: -1.5, activation: "heaviside" }
soit ou_neurone = { poids: [1, 1], biais: -0.5, activation: "heaviside" }
afficher "a b | ET OU"
pour chaque a dans [0, 1] faire
  pour chaque b dans [0, 1] faire
    afficher a + " " + b + " |  " + ia_activation_perceptron(et_neurone, [a, b]) + "  " + ia_activation_perceptron(ou_neurone, [a, b])
  fin
fin
```

Résultat :

```
a b | ET OU
0 0 |  0  0
0 1 |  0  1
1 0 |  0  1
1 1 |  1  1
```

</details>

### Exercice 151 — Faire apprendre le ET à un perceptron

*Niveau : ★★★*

> Sans bibliothèque : entraînez un perceptron (poids w1, w2 et biais b, partis de 0) avec la règle
> « erreur = cible − sortie ; w = w + 0,1 × erreur × entrée ; b = b + 0,1 × erreur » pendant 20 passes
> sur les 4 cas du ET. Affichez les prédictions obtenues.

<details>
<summary>Corrigé</summary>

```fadera
soit exemples = [[0, 0, 0], [0, 1, 0], [1, 0, 0], [1, 1, 1]]
soit w1 = 0
soit w2 = 0
soit b = 0
fonction sortie(x1, x2)
  si w1 * x1 + w2 * x2 + b > 0 alors
    renvoyer 1
  fin
  renvoyer 0
fin
pour passe allantde 1 a 20 faire
  pour chaque c dans exemples faire
    soit x1 = lireElement(c, 0)
    soit x2 = lireElement(c, 1)
    soit erreur = lireElement(c, 2) - sortie(x1, x2)
    w1 = w1 + 0.1 * erreur * x1
    w2 = w2 + 0.1 * erreur * x2
    b = b + 0.1 * erreur
  fin
fin
pour chaque c dans exemples faire
  afficher lireElement(c, 0) + " ET " + lireElement(c, 1) + " = " + sortie(lireElement(c, 0), lireElement(c, 1))
fin
```

Résultat :

```
0 ET 0 = 0
0 ET 1 = 0
1 ET 0 = 0
1 ET 1 = 1
```

</details>

### Exercice 152 — Apprendre le OU exclusif

*Niveau : ★★★*

> Un seul neurone ne sait pas calculer le XOR, mais un petit réseau y arrive. Créez un réseau 2-6-1 (creer_reseau),
> entraînez-le avec ia_entrainer sur les 4 cas pendant 5000 passes (taux 0,5) puis affichez ses prédictions arrondies.
> (L'initialisation est aléatoire : le résultat peut varier d'une exécution à l'autre.)

<details>
<summary>Corrigé</summary>

```fadera
soit reseau = creer_reseau(2, 6, 1)
soit entrees = [[0, 0], [0, 1], [1, 0], [1, 1]]
soit cibles = [[0], [1], [1], [0]]
pour passe allantde 1 a 5000 faire
  pour k allantde 0 a 3 faire
    ia_entrainer(reseau, lireElement(entrees, k), lireElement(cibles, k), 0.5)
  fin
fin
pour chaque x dans entrees faire
  afficher lireElement(x, 0) + " XOR " + lireElement(x, 1) + " ≈ " + arrondir(lireElement(ia_predire(reseau, x), 0), 0)
fin
```

L'initialisation du réseau est aléatoire : le résultat attendu est 0, 1, 1, 0, mais l'apprentissage peut parfois échouer ; relancez-le.

</details>

## Robotique et voix

### Exercice 160 — Un robot sur une grille

*Niveau : ★★*

> Un robot part de (0, 0) tourné vers le nord. Les commandes sont « A » (avancer d'une case), « G » et « D »
> (tourner de 90°). Exécutez « AAD AA G A » (sans espaces) et affichez la position finale et l'orientation.
> Faites-le annoncer sa position à la fin avec parler.

<details>
<summary>Corrigé</summary>

```fadera
soit x = 0
soit y = 0
soit direction = 0   # 0 = nord, 1 = est, 2 = sud, 3 = ouest
soit noms = ["nord", "est", "sud", "ouest"]
pour chaque c dans decouper("AADAAGA", "") faire
  si c == "G" alors
    direction = modulo(direction - 1, 4)
  sinon si c == "D" alors
    direction = modulo(direction + 1, 4)
  sinon si c == "A" alors
    selon direction faire
      cas 0 alors
        y = y + 1
      cas 1 alors
        x = x + 1
      cas 2 alors
        y = y - 1
      cas 3 alors
        x = x - 1
    fin
  fin
fin
afficher "Position : (" + x + ", " + y + "), orienté vers le " + lireElement(noms, direction)
parler("Je suis arrivé en " + x + " " + y)
```

Résultat :

```
Position : (2, 3), orienté vers le nord
```

</details>

### Exercice 161 — S'arrêter devant un obstacle

*Niveau : ★★*

> Un robot avance de 0,5 m par pas vers un mur situé à 3 m. Il s'arrête dès que le capteur
> indique moins de 1 m, et le dit à voix haute. Affichez chaque pas.

<details>
<summary>Corrigé</summary>

```fadera
soit position = 0
soit mur = 3
soit pas = 0
tantque mur - position >= 1 faire
  position = position + 0.5
  pas = pas + 1
  afficher "Pas " + pas + " : distance au mur " + (mur - position) + " m"
fin
afficher "Obstacle proche : arrêt."
parler("Obstacle détecté, je m'arrête.")
```

Résultat :

```
Pas 1 : distance au mur 2.5 m
Pas 2 : distance au mur 2 m
Pas 3 : distance au mur 1.5 m
Pas 4 : distance au mur 1 m
Pas 5 : distance au mur 0.5 m
Obstacle proche : arrêt.
```

</details>

### Exercice 162 — Régler une vitesse

*Niveau : ★★★*

> Un robot doit atteindre la vitesse de 2 m/s. À chaque tour, la commande est 0,5 × (cible − vitesse)
> et la vitesse augmente de cette commande. Simulez 10 tours et affichez la vitesse arrondie à 3 décimales.

<details>
<summary>Corrigé</summary>

```fadera
soit cible = 2
soit vitesse = 0
pour tour allantde 1 a 10 faire
  soit commande = 0.5 * (cible - vitesse)
  vitesse = vitesse + commande
  afficher "Tour " + texte_remplir_gauche(texte(tour), 2, " ") + " : " + arrondir(vitesse, 3) + " m/s"
fin
```

Résultat :

```
Tour  1 : 1 m/s
Tour  2 : 1.5 m/s
Tour  3 : 1.75 m/s
Tour  4 : 1.875 m/s
Tour  5 : 1.938 m/s
Tour  6 : 1.969 m/s
Tour  7 : 1.984 m/s
Tour  8 : 1.992 m/s
Tour  9 : 1.996 m/s
Tour 10 : 1.998 m/s
```

</details>

### Exercice 163 — Lire un texte à voix haute

*Niveau : ★★*

> Un robot lecteur reçoit un texte, le découpe en phrases (séparées par « . ») et prononce chaque phrase
> avec parler, après l'avoir affichée. Il annonce à la fin le nombre de phrases lues.

<details>
<summary>Corrigé</summary>

```fadera
soit texte_lu = "Fadéra est un langage en français. Il fonctionne dans le navigateur. Il sait aussi parler."
soit lues = 0
pour chaque phrase dans decouper(texte_lu, ".") faire
  soit p = nettoyer(phrase)
  si taille(p) > 0 alors
    afficher p + "."
    parler(p)
    lues = lues + 1
  fin
fin
afficher lues + " phrases lues."
parler("J'ai lu " + lues + " phrases.")
```

Résultat :

```
Fadéra est un langage en français.
Il fonctionne dans le navigateur.
Il sait aussi parler.
3 phrases lues.
```

</details>

## Jeux et simulations

### Exercice 170 — Deviner un nombre par dichotomie

*Niveau : ★★*

> L'ordinateur choisit 73 entre 1 et 100. Un joueur « intelligent » propose toujours le milieu de l'intervalle.
> Affichez chaque proposition (« trop petit » / « trop grand ») et le nombre d'essais.

<details>
<summary>Corrigé</summary>

```fadera
soit secret = 73
soit bas = 1
soit haut = 100
soit essais = 0
soit trouve = faux
tantque non trouve faire
  soit proposition = plancher((bas + haut) / 2)
  essais = essais + 1
  si proposition == secret alors
    afficher "Essai " + essais + " : " + proposition + " — trouvé !"
    trouve = vrai
  sinon si proposition < secret alors
    afficher "Essai " + essais + " : " + proposition + " — trop petit"
    bas = proposition + 1
  sinon
    afficher "Essai " + essais + " : " + proposition + " — trop grand"
    haut = proposition - 1
  fin
fin
```

Résultat :

```
Essai 1 : 50 — trop petit
Essai 2 : 75 — trop grand
Essai 3 : 62 — trop petit
Essai 4 : 68 — trop petit
Essai 5 : 71 — trop petit
Essai 6 : 73 — trouvé !
```

</details>

### Exercice 171 — Feu tricolore

*Niveau : ★★*

> Simulez un feu qui passe de vert (4 s) à orange (1 s) puis rouge (3 s), en boucle, pendant 16 secondes.
> Affichez la couleur à chaque seconde sous la forme « t=0 vert ».

<details>
<summary>Corrigé</summary>

```fadera
soit couleurs = ["vert", "orange", "rouge"]
soit durees = [4, 1, 3]
soit etat = 0
soit reste = lireElement(durees, 0)
pour t allantde 0 a 15 faire
  afficher "t=" + t + " " + lireElement(couleurs, etat)
  reste = reste - 1
  si reste == 0 alors
    etat = modulo(etat + 1, 3)
    reste = lireElement(durees, etat)
  fin
fin
```

Résultat :

```
t=0 vert
t=1 vert
t=2 vert
t=3 vert
t=4 orange
t=5 rouge
t=6 rouge
t=7 rouge
t=8 vert
t=9 vert
t=10 vert
t=11 vert
t=12 orange
t=13 rouge
t=14 rouge
t=15 rouge
```

</details>

### Exercice 172 — Pierre, feuille, ciseaux

*Niveau : ★★★*

> Faites jouer deux ordinateurs 5 manches. Chacun tire son coup avec un générateur pseudo-aléatoire à graine fixe
> (xn+1 = (xn × 1103515245 + 12345) mod 2^31). Affichez chaque manche et le score final.

<details>
<summary>Corrigé</summary>

```fadera
soit graine = 2026
fonction tirer()
  graine = (graine * 1103515245 + 12345) % 2147483648
  renvoyer plancher(graine / 65536) % 3
fin

soit coups = ["pierre", "feuille", "ciseaux"]
soit score_a = 0
soit score_b = 0
pour manche allantde 1 a 5 faire
  soit a = tirer()
  soit b = tirer()
  soit issue = "égalité"
  si modulo(a - b, 3) == 1 alors
    issue = "A gagne"
    score_a = score_a + 1
  sinon si modulo(b - a, 3) == 1 alors
    issue = "B gagne"
    score_b = score_b + 1
  fin
  afficher "Manche " + manche + " : " + lireElement(coups, a) + " contre " + lireElement(coups, b) + " → " + issue
fin
afficher "Score : A " + score_a + " — B " + score_b
```

Résultat :

```
Manche 1 : feuille contre pierre → A gagne
Manche 2 : pierre contre feuille → B gagne
Manche 3 : pierre contre ciseaux → A gagne
Manche 4 : pierre contre feuille → B gagne
Manche 5 : ciseaux contre feuille → A gagne
Score : A 3 — B 2
```

</details>

### Exercice 173 — Une génération du jeu de la vie

*Niveau : ★★★*

> Sur une grille 5×5 (liste de listes de 0 et 1) contenant un « clignotant » (trois cellules en ligne au centre),
> calculez la génération suivante : une cellule vivante survit avec 2 ou 3 voisines vivantes,
> une cellule morte naît avec exactement 3. Affichez les deux grilles.

<details>
<summary>Corrigé</summary>

```fadera
fonction cellule(g, l, c)
  si l < 0 ou l > 4 ou c < 0 ou c > 4 alors
    renvoyer 0
  fin
  renvoyer lireElement(lireElement(g, l), c)
fin

fonction voisines(g, l, c)
  soit n = 0
  pour dl allantde -1 a 1 faire
    pour dc allantde -1 a 1 faire
      si dl != 0 ou dc != 0 alors
        n = n + cellule(g, l + dl, c + dc)
      fin
    fin
  fin
  renvoyer n
fin

fonction suivante(g)
  soit nouvelle = []
  pour l allantde 0 a 4 faire
    soit ligne = []
    pour c allantde 0 a 4 faire
      soit v = voisines(g, l, c)
      si cellule(g, l, c) == 1 et (v == 2 ou v == 3) alors
        ajouter(ligne, 1)
      sinon si cellule(g, l, c) == 0 et v == 3 alors
        ajouter(ligne, 1)
      sinon
        ajouter(ligne, 0)
      fin
    fin
    ajouter(nouvelle, ligne)
  fin
  renvoyer nouvelle
fin

fonction dessiner(g)
  pour chaque ligne dans g faire
    soit texte_ligne = ""
    pour chaque v dans ligne faire
      si v == 1 alors
        texte_ligne = texte_ligne + "#"
      sinon
        texte_ligne = texte_ligne + "."
      fin
    fin
    afficher texte_ligne
  fin
fin

soit grille = [[0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 1, 1, 1, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]]
dessiner(grille)
afficher ""
dessiner(suivante(grille))
```

Résultat :

```
.....
.....
.###.
.....
.....

.....
..#..
..#..
..#..
.....
```

</details>

## Mini-projets

### Exercice 180 — Projet : le bulletin de la classe

*Niveau : ★★★*

> À partir d'un CSV (nom;maths;francais;anglais), construisez pour chaque élève la moyenne,
> la mention (≥ 14 « Bien », ≥ 10 « Passable », sinon « Ajourné »), classez les élèves par moyenne décroissante
> et affichez un tableau aligné (texte_remplir_droite) avec la moyenne de la classe en bas.

<details>
<summary>Corrigé</summary>

```fadera
soit brut = "nom;maths;francais;anglais\nAwa;15;12;16\nMoussa;9;14;10\nFatou;18;16;17\nIbrahima;7;9;8"
soit lignes = tranche(csv_lire(brut, ";"), 1, 5)

fonction mention(m)
  si m >= 14 alors
    renvoyer "Bien"
  sinon si m >= 10 alors
    renvoyer "Passable"
  fin
  renvoyer "Ajourné"
fin

soit eleves = transformer(lignes, l => {
  nom: lireElement(l, 0),
  moyenne: stats_moyenne([nombre(lireElement(l, 1)), nombre(lireElement(l, 2)), nombre(lireElement(l, 3))])
})
soit classement = trier_par(eleves, e => -e.moyenne)
soit rang = 1
pour chaque e dans classement faire
  afficher rang + ". " + texte_remplir_droite(e.nom, 10, " ") + texte_remplir_gauche(texte(arrondir(e.moyenne, 2)), 6, " ") + "  " + mention(e.moyenne)
  rang = rang + 1
fin
afficher "Moyenne de la classe : " + arrondir(stats_moyenne(transformer(eleves, e => e.moyenne)), 2)
```

Résultat :

```
1. Fatou         17  Bien
2. Awa        14.33  Bien
3. Moussa        11  Passable
4. Ibrahima       8  Ajourné
Moyenne de la classe : 12.58
```

</details>

### Exercice 181 — Projet : la caisse du jour

*Niveau : ★★★*

> Écrivez une classe Caisse qui enregistre des ventes { article, montant } et sait donner le total,
> la plus grosse vente et le chiffre d'affaires par article. À la fin de la journée, enregistrez le résumé
> dans « caisse.json » (json_texte) et relisez-le pour vérifier.

<details>
<summary>Corrigé</summary>

```fadera
classe Caisse
  fonction creer()
    soi.ventes = []
  fin

  fonction vendre(article, montant)
    ajouter(soi.ventes, { article: article, montant: montant })
  fin

  fonction total()
    renvoyer reduire(soi.ventes, (t, v) => t + v.montant, 0)
  fin

  fonction plus_grosse()
    renvoyer lireElement(trier_par(soi.ventes, v => -v.montant), 0)
  fin

  fonction par_article()
    soit r = {}
    pour chaque v dans soi.ventes faire
      si a_cle(r, v.article) alors
        modifierCle(r, v.article, lireCle(r, v.article) + v.montant)
      sinon
        modifierCle(r, v.article, v.montant)
      fin
    fin
    renvoyer r
  fin
fin

soit c = Caisse()
c.vendre("riz", 1300)
c.vendre("huile", 1800)
c.vendre("riz", 650)
c.vendre("sucre", 700)
afficher "Total : " + c.total()
afficher "Plus grosse vente : " + lireCle(c.plus_grosse(), "article")
afficher c.par_article()
ecrireFichier("caisse.json", json_texte({ total: c.total(), par_article: c.par_article() }, 0))
afficher "Relu : " + lireFichier("caisse.json")
```

Résultat :

```
Total : 4450
Plus grosse vente : huile
{ riz: 1950, huile: 1800, sucre: 700 }
Relu : {"total":4450,"par_article":{"riz":1950,"huile":1800,"sucre":700}}
```

</details>

### Exercice 182 — Projet : un petit assistant à questions

*Niveau : ★★★*

> Écrivez repondre(question) : l'assistant met la question en minuscules sans accents, cherche des mots-clés
> (contient) dans une liste de règles { mots, reponse } et répond « Je n'ai pas compris » sinon.
> Testez-le sur 4 questions et faites lire les réponses à voix haute.

<details>
<summary>Corrigé</summary>

```fadera
soit regles = [
  { mot: "horaire", reponse: "Nous ouvrons de 8h à 18h." },
  { mot: "prix", reponse: "Les prix sont affichés à l'entrée." },
  { mot: "adresse", reponse: "Nous sommes au marché central." }
]

fonction repondre(question)
  soit q = minuscule(texte_sans_accents(question))
  pour chaque r dans regles faire
    si contient(q, r.mot) alors
      renvoyer r.reponse
    fin
  fin
  renvoyer "Je n'ai pas compris."
fin

pour chaque q dans ["Quels sont vos horaires ?", "Quel est le PRIX du riz ?", "Quelle est votre adresse ?", "Vendez-vous du poisson ?"] faire
  soit reponse = repondre(q)
  afficher "Q : " + q
  afficher "R : " + reponse
  parler(reponse)
fin
```

Résultat :

```
Q : Quels sont vos horaires ?
R : Nous ouvrons de 8h à 18h.
Q : Quel est le PRIX du riz ?
R : Les prix sont affichés à l'entrée.
Q : Quelle est votre adresse ?
R : Nous sommes au marché central.
Q : Vendez-vous du poisson ?
R : Je n'ai pas compris.
```

</details>

### Exercice 183 — Projet : un robot livreur

*Niveau : ★★★*

> Un robot doit livrer 3 colis à des adresses (x, y) d'une grille, en partant de (0, 0), en passant toujours
> par le colis le plus proche (distance de Manhattan : |dx| + |dy|). Affichez l'ordre de livraison, la distance
> parcourue et annoncez la fin de la tournée avec parler.

<details>
<summary>Corrigé</summary>

```fadera
soit position = [0, 0]
soit restants = [[4, 1], [1, 2], [5, 5]]
soit distance_totale = 0

fonction manhattan(a, b)
  renvoyer absolu(lireElement(a, 0) - lireElement(b, 0)) + absolu(lireElement(a, 1) - lireElement(b, 1))
fin

tantque taille(restants) > 0 faire
  soit proche = lireElement(trier_par(restants, p => manhattan(position, p)), 0)
  soit d = manhattan(position, proche)
  distance_totale = distance_totale + d
  afficher "Livraison en (" + lireElement(proche, 0) + ", " + lireElement(proche, 1) + ") — " + d + " cases"
  position = proche
  restants = filtrer(restants, p => p != proche)
fin
afficher "Distance totale : " + distance_totale + " cases"
parler("Tournée terminée, " + distance_totale + " cases parcourues.")
```

Résultat :

```
Livraison en (1, 2) — 3 cases
Livraison en (4, 1) — 4 cases
Livraison en (5, 5) — 5 cases
Distance totale : 12 cases
```

</details>

## Industrie et métiers

Fadéra n'est pas un langage pour piloter une machine en temps réel : c'est un outil pour **automatiser les calculs, les contrôles et les rapports** d'une petite ou moyenne entreprise, et pour **prototyper des règles métier** dans un français que le responsable de production, le comptable ou le technicien peut relire. Ces exercices reprennent des situations concrètes : qualité, maintenance, stocks, facturation, logistique, énergie, agriculture, finance, télécoms et sécurité. Les chiffres sont inventés, mais les calculs sont ceux du métier.

### Exercice 190 — Contrôle qualité sur une chaîne de production

*Niveau : ★★*

> Une machine remplit des bouteilles de 500 ml avec une tolérance de ± 5 ml. Voici 12 mesures.
> Comptez les bouteilles conformes et les rebuts, calculez le taux de rebut (en %), la moyenne et l'écart-type,
> et affichez « LOT ACCEPTÉ » si le taux de rebut est inférieur à 10 %, sinon « LOT REFUSÉ ».

<details>
<summary>Corrigé</summary>

```fadera
soit mesures = [500, 502, 497, 509, 501, 498, 494, 500, 503, 506, 499, 500]
soit conformes = filtrer(mesures, m => absolu(m - 500) <= 5)
soit rebuts = taille(mesures) - taille(conformes)
soit taux = arrondir(rebuts * 100 / taille(mesures), 1)
afficher "Conformes : " + taille(conformes) + " — rebuts : " + rebuts
afficher "Taux de rebut : " + taux + " %"
afficher "Moyenne : " + arrondir(stats_moyenne(mesures), 2) + " ml, écart-type : " + arrondir(stats_ecart_type(mesures), 2)
si taux < 10 alors
  afficher "LOT ACCEPTÉ"
sinon
  afficher "LOT REFUSÉ"
fin
```

Résultat :

```
Conformes : 9 — rebuts : 3
Taux de rebut : 25 %
Moyenne : 500.75 ml, écart-type : 3.81
LOT REFUSÉ
```

</details>

### Exercice 191 — Surveiller une machine

*Niveau : ★★★*

> Un capteur relève la température d'un moteur toutes les minutes. Calculez la moyenne glissante sur 3 relevés et
> déclenchez une alerte (affichée et prononcée avec parler) dès que cette moyenne dépasse 80 °C.
> Indiquez la minute de l'alerte et arrêtez la surveillance.

<details>
<summary>Corrigé</summary>

```fadera
soit releves = [62, 65, 70, 74, 79, 83, 88, 91]
soit alerte = faux
pour i allantde 2 a taille(releves) - 1 faire
  si non alerte alors
    soit moyenne = stats_moyenne(tranche(releves, i - 2, i + 1))
    afficher "Minute " + (i + 1) + " : moyenne glissante " + arrondir(moyenne, 1) + " °C"
    si moyenne > 80 alors
      alerte = vrai
      afficher "ALERTE : surchauffe à la minute " + (i + 1)
      parler("Alerte surchauffe moteur.")
    fin
  fin
fin
```

Résultat :

```
Minute 3 : moyenne glissante 65.7 °C
Minute 4 : moyenne glissante 69.7 °C
Minute 5 : moyenne glissante 74.3 °C
Minute 6 : moyenne glissante 78.7 °C
Minute 7 : moyenne glissante 83.3 °C
ALERTE : surchauffe à la minute 7
```

</details>

### Exercice 192 — Réapprovisionner un stock

*Niveau : ★★*

> Un entrepôt a un fichier CSV (article;stock;seuil;prix). Listez les articles dont le stock est inférieur au seuil,
> calculez pour chacun la quantité à commander pour atteindre 2 × le seuil et le coût de la commande,
> puis le coût total.

<details>
<summary>Corrigé</summary>

```fadera
soit brut = "article;stock;seuil;prix\nvis;120;200;15\nécrous;300;250;12\nrondelles;40;100;5\nboulons;10;50;40"
soit lignes = tranche(csv_lire(brut, ";"), 1, 5)
soit total = 0
pour chaque l dans lignes faire
  soit article = lireElement(l, 0)
  soit stock = nombre(lireElement(l, 1))
  soit seuil = nombre(lireElement(l, 2))
  soit prix = nombre(lireElement(l, 3))
  si stock < seuil alors
    soit quantite = 2 * seuil - stock
    afficher article + " : commander " + quantite + " (" + quantite * prix + " FCFA)"
    total = total + quantite * prix
  fin
fin
afficher "Coût total : " + total + " FCFA"
```

Résultat :

```
vis : commander 280 (4200 FCFA)
rondelles : commander 160 (800 FCFA)
boulons : commander 90 (3600 FCFA)
Coût total : 8600 FCFA
```

</details>

### Exercice 193 — Une facture avec remise et TVA

*Niveau : ★★*

> Établissez la facture de 3 lignes (désignation, quantité, prix unitaire HT). Appliquez une remise de 10 % si le
> total HT dépasse 100 000 FCFA, puis une TVA de 18 %. Affichez un tableau aligné et le net à payer.

<details>
<summary>Corrigé</summary>

```fadera
soit lignes = [
  { designation: "Pompe", quantite: 2, prix: 45000 },
  { designation: "Joint", quantite: 10, prix: 1500 },
  { designation: "Câble (m)", quantite: 50, prix: 800 }
]
soit total_ht = 0
pour chaque l dans lignes faire
  soit montant = l.quantite * l.prix
  total_ht = total_ht + montant
  afficher texte_remplir_droite(l.designation, 12, " ") + texte_remplir_gauche(texte(l.quantite), 4, " ") + texte_remplir_gauche(texte(montant), 10, " ")
fin
soit remise = 0
si total_ht > 100000 alors
  remise = total_ht * 0.10
fin
soit net_ht = total_ht - remise
soit tva = net_ht * 0.18
afficher "Total HT : " + total_ht
afficher "Remise : " + remise
afficher "TVA 18 % : " + tva
afficher "Net à payer : " + (net_ht + tva) + " FCFA"
```

Résultat :

```
Pompe          2     90000
Joint         10     15000
Câble (m)     50     40000
Total HT : 145000
Remise : 14500
TVA 18 % : 23490
Net à payer : 153990 FCFA
```

</details>

### Exercice 194 — Le trajet le moins coûteux pour une livraison

*Niveau : ★★★*

> Un transporteur relie des villes : Dakar–Thiès (70 km), Thiès–Touba (115 km), Dakar–Mbour (80 km),
> Mbour–Kaolack (110 km), Thiès–Kaolack (150 km), Kaolack–Touba (130 km). Avec le routage de plus court chemin,
> trouvez le meilleur trajet de Dakar à Touba et à Kaolack, puis le coût en carburant à 0,12 litre par km et 800 FCFA le litre.

<details>
<summary>Corrigé</summary>

```fadera
soit reseau = telecom_creer_topologie()
pour chaque t dans [["Dakar", "Thies", 70], ["Thies", "Touba", 115], ["Dakar", "Mbour", 80], ["Mbour", "Kaolack", 110], ["Thies", "Kaolack", 150], ["Kaolack", "Touba", 130]] faire
  reseau = telecom_ajouter_liaison(reseau, { de: lireElement(t, 0), a: lireElement(t, 1), cout: lireElement(t, 2) })
fin
pour chaque destination dans ["Touba", "Kaolack"] faire
  soit chemin = telecom_dijkstra_routage(reseau, { depart: "Dakar", arrivee: destination })
  soit km = lireCle(chemin, "cout_total")
  afficher "Dakar → " + destination + " : " + joindre(lireCle(chemin, "itineraire"), " → ") + ", " + km + " km, carburant " + arrondir(km * 0.12 * 800, 0) + " FCFA"
fin
```

Résultat :

```
Dakar → Touba : Dakar → Thies → Touba, 185 km, carburant 17760 FCFA
Dakar → Kaolack : Dakar → Mbour → Kaolack, 190 km, carburant 18240 FCFA
```

</details>

### Exercice 195 — Facture d'électricité par tranches

*Niveau : ★★★*

> Le prix du kWh dépend de la tranche : 0–150 kWh à 80 FCFA, 150–250 à 120 FCFA, au-delà à 160 FCFA.
> Calculez la facture pour 90, 200 et 320 kWh en parcourant un tableau de tranches { jusqua, prix }.

<details>
<summary>Corrigé</summary>

```fadera
soit tranches = [{ jusqua: 150, prix: 80 }, { jusqua: 250, prix: 120 }, { jusqua: 1000000, prix: 160 }]

fonction facture(kwh)
  soit total = 0
  soit debut = 0
  pour chaque t dans tranches faire
    si kwh > debut alors
      soit dans_tranche = minimum(kwh, t.jusqua) - debut
      total = total + dans_tranche * t.prix
    fin
    debut = t.jusqua
  fin
  renvoyer total
fin

pour chaque k dans [90, 200, 320] faire
  afficher k + " kWh → " + facture(k) + " FCFA"
fin
```

Résultat :

```
90 kWh → 7200 FCFA
200 kWh → 18000 FCFA
320 kWh → 35200 FCFA
```

</details>

### Exercice 196 — Irrigation automatique

*Niveau : ★★*

> Un système arrose un champ selon l'humidité du sol (en %) et la prévision de pluie. Règles : arroser 20 minutes si
> l'humidité est inférieure à 30 % et qu'il ne pleut pas ; 10 minutes si elle est entre 30 et 50 % sans pluie ;
> sinon ne pas arroser. Appliquez les règles à 5 relevés et calculez le total d'eau (débit : 15 litres par minute).

<details>
<summary>Corrigé</summary>

```fadera
soit releves = [
  { humidite: 22, pluie: faux },
  { humidite: 35, pluie: faux },
  { humidite: 28, pluie: vrai },
  { humidite: 55, pluie: faux },
  { humidite: 41, pluie: faux }
]
fonction minutes_arrosage(r)
  si r.pluie alors
    renvoyer 0
  fin
  si r.humidite < 30 alors
    renvoyer 20
  fin
  si r.humidite <= 50 alors
    renvoyer 10
  fin
  renvoyer 0
fin
soit total = 0
pour chaque r dans releves faire
  soit m = minutes_arrosage(r)
  afficher "Humidité " + r.humidite + " %, pluie : " + r.pluie + " → " + m + " min"
  total = total + m
fin
afficher "Eau consommée : " + total * 15 + " litres"
```

Résultat :

```
Humidité 22 %, pluie : faux → 20 min
Humidité 35 %, pluie : faux → 10 min
Humidité 28 %, pluie : vrai → 0 min
Humidité 55 %, pluie : faux → 0 min
Humidité 41 %, pluie : faux → 10 min
Eau consommée : 600 litres
```

</details>

### Exercice 197 — Rembourser un prêt

*Niveau : ★★★*

> Une entreprise emprunte 1 200 000 FCFA sur 6 mois à 12 % par an (1 % par mois). La mensualité constante est
> M = C × t / (1 − (1 + t)^(−n)). Calculez M puis affichez le tableau d'amortissement : intérêts, capital remboursé et
> capital restant dû chaque mois (arrondis à l'unité).

<details>
<summary>Corrigé</summary>

```fadera
soit capital = 1200000
soit t = 0.01
soit n = 6
soit mensualite = capital * t / (1 - puissance(1 + t, -n))
afficher "Mensualité : " + arrondir(mensualite, 0) + " FCFA"
soit restant = capital
pour mois allantde 1 a n faire
  soit interets = restant * t
  soit rembourse = mensualite - interets
  restant = restant - rembourse
  afficher "Mois " + mois + " : intérêts " + arrondir(interets, 0) + ", capital " + arrondir(rembourse, 0) + ", reste " + arrondir(restant, 0)
fin
```

Résultat :

```
Mensualité : 207058 FCFA
Mois 1 : intérêts 12000, capital 195058, reste 1004942
Mois 2 : intérêts 10049, capital 197009, reste 807933
Mois 3 : intérêts 8079, capital 198979, reste 608955
Mois 4 : intérêts 6090, capital 200968, reste 407986
Mois 5 : intérêts 4080, capital 202978, reste 205008
Mois 6 : intérêts 2050, capital 205008, reste 0
```

</details>

### Exercice 198 — Facturer un forfait mobile

*Niveau : ★★*

> Un forfait comprend 120 minutes et 5 Go pour 10 000 FCFA. Au-delà : 50 FCFA par minute et 2 000 FCFA par Go entamé
> (utilisez plafond). Calculez la facture de trois clients et repérez celui qui paie le plus de dépassement.

<details>
<summary>Corrigé</summary>

```fadera
soit clients = [
  { nom: "Awa", minutes: 90, donnees: 4.2 },
  { nom: "Moussa", minutes: 180, donnees: 5.5 },
  { nom: "Fatou", minutes: 130, donnees: 8.1 }
]
soit pire = ""
soit pire_depassement = -1
pour chaque c dans clients faire
  soit depassement = maximum(0, c.minutes - 120) * 50 + plafond(maximum(0, c.donnees - 5)) * 2000
  afficher c.nom + " : " + (10000 + depassement) + " FCFA (dépassement " + depassement + ")"
  si depassement > pire_depassement alors
    pire_depassement = depassement
    pire = c.nom
  fin
fin
afficher "Plus gros dépassement : " + pire
```

Résultat :

```
Awa : 10000 FCFA (dépassement 0)
Moussa : 15000 FCFA (dépassement 5000)
Fatou : 18500 FCFA (dépassement 8500)
Plus gros dépassement : Fatou
```

</details>

### Exercice 199 — Journal de sécurité d'un atelier

*Niveau : ★★★*

> Écrivez une classe Atelier qui enregistre des incidents { niveau, texte } (niveau 1 = information, 2 = avertissement,
> 3 = danger). Un incident de niveau 3 déclenche une alarme prononcée avec parler. La méthode bilan() renvoie le
> nombre d'incidents par niveau. Un niveau invalide doit lever une erreur, rattrapée par le programme.

<details>
<summary>Corrigé</summary>

```fadera
classe Atelier
  fonction creer()
    soi.incidents = []
  fin

  fonction signaler(niveau, texte_incident)
    si niveau < 1 ou niveau > 3 alors
      lever("Niveau invalide : " + niveau)
    fin
    ajouter(soi.incidents, { niveau: niveau, texte: texte_incident })
    si niveau == 3 alors
      afficher "ALARME : " + texte_incident
      parler("Danger. " + texte_incident)
    fin
  fin

  fonction bilan()
    soit r = { info: 0, avertissement: 0, danger: 0 }
    pour chaque i dans soi.incidents faire
      selon i.niveau faire
        cas 1 alors
          modifierCle(r, "info", r.info + 1)
        cas 2 alors
          modifierCle(r, "avertissement", r.avertissement + 1)
        cas 3 alors
          modifierCle(r, "danger", r.danger + 1)
      fin
    fin
    renvoyer r
  fin
fin

soit atelier = Atelier()
atelier.signaler(1, "Prise de poste")
atelier.signaler(2, "Casque manquant")
atelier.signaler(3, "Fuite d'huile sur la presse 2")
atelier.signaler(1, "Fin de poste")
essayer
  atelier.signaler(7, "test")
attraper e
  afficher "Refusé : " + e
fin
afficher atelier.bilan()
```

Résultat :

```
ALARME : Fuite d'huile sur la presse 2
Refusé : Niveau invalide : 7
{ info: 2, avertissement: 1, danger: 1 }
```

</details>

### Exercice 200 — Santé : indice de masse corporelle

*Niveau : ★★*

> Écrivez imc(poids, taille_m) puis categorie(valeur) (moins de 18,5 « maigreur », jusqu'à 25 « normal »,
> jusqu'à 30 « surpoids », sinon « obésité »). Affichez-les pour trois personnes.
> (Exercice de programmation : ce n'est pas un avis médical.)

<details>
<summary>Corrigé</summary>

```fadera
fonction imc(poids, taille_m)
  renvoyer poids / (taille_m * taille_m)
fin

fonction categorie(v)
  si v < 18.5 alors
    renvoyer "maigreur"
  sinon si v < 25 alors
    renvoyer "normal"
  sinon si v < 30 alors
    renvoyer "surpoids"
  fin
  renvoyer "obésité"
fin

pour chaque p dans [["Awa", 58, 1.65], ["Moussa", 92, 1.75], ["Fatou", 47, 1.70]] faire
  soit v = imc(lireElement(p, 1), lireElement(p, 2))
  afficher lireElement(p, 0) + " : IMC " + arrondir(v, 1) + " (" + categorie(v) + ")"
fin
```

Résultat :

```
Awa : IMC 21.3 (normal)
Moussa : IMC 30 (obésité)
Fatou : IMC 16.3 (maigreur)
```

</details>

### Exercice 201 — Transport : horaires d'un bus

*Niveau : ★★*

> Un bus part à 06:30 et dessert des arrêts espacés de 12, 9, 15 et 7 minutes. Affichez l'heure d'arrivée à chaque arrêt
> (date_ajouter, date_formater) et la durée totale du trajet.

<details>
<summary>Corrigé</summary>

```fadera
soit depart = date_creer(2026, 10, 29, 6, 30, 0)
soit arrets = [["Marché", 12], ["Gare", 9], ["Université", 15], ["Hôpital", 7]]
soit maintenant_bus = depart
soit total = 0
afficher "Départ : " + date_formater(depart, "hh:mm")
pour chaque a dans arrets faire
  maintenant_bus = date_ajouter(maintenant_bus, lireElement(a, 1), "minutes")
  total = total + lireElement(a, 1)
  afficher texte_remplir_droite(lireElement(a, 0), 12, " ") + date_formater(maintenant_bus, "hh:mm")
fin
afficher "Durée totale : " + total + " minutes"
```

Résultat :

```
Départ : 06:30
Marché      06:42
Gare        06:51
Université  07:06
Hôpital     07:13
Durée totale : 43 minutes
```

</details>

### Exercice 202 — Sécurité informatique : repérer une attaque

*Niveau : ★★★*

> Un journal de connexions contient des lignes « heure utilisateur résultat ». Repérez les utilisateurs
> qui ont 3 échecs de suite (un succès remet le compteur à zéro) et affichez une alerte pour chacun.

<details>
<summary>Corrigé</summary>

```fadera
soit journal = "08:01 awa ok\n08:03 root echec\n08:03 root echec\n08:04 moussa ok\n08:05 root echec\n08:06 awa echec\n08:07 awa ok"
soit serie = {}
soit alertes = []
pour chaque ligne dans texte_lignes(journal) faire
  soit morceaux = decouper(ligne, " ")
  soit qui = lireElement(morceaux, 1)
  si lireElement(morceaux, 2) == "echec" alors
    soit n = 1
    si a_cle(serie, qui) alors
      n = lireCle(serie, qui) + 1
    fin
    modifierCle(serie, qui, n)
    si n == 3 alors
      ajouter(alertes, qui)
      afficher "ALERTE : 3 échecs de suite pour « " + qui + " » à " + lireElement(morceaux, 0)
    fin
  sinon
    modifierCle(serie, qui, 0)
  fin
fin
afficher "Comptes à surveiller : " + alertes
```

Résultat :

```
ALERTE : 3 échecs de suite pour « root » à 08:05
Comptes à surveiller : ["root"]
```

</details>

### Exercice 203 — Logistique : charger un camion

*Niveau : ★★★*

> Un camion porte 100 kg au maximum. Des colis { nom, poids, priorite } (1 = urgent) attendent. Chargez-les par priorité
> (puis du plus léger au plus lourd) tant qu'ils tiennent, et listez ceux qui restent pour le prochain voyage.

<details>
<summary>Corrigé</summary>

```fadera
soit colis = [
  { nom: "médicaments", poids: 12, priorite: 1 },
  { nom: "pièces moteur", poids: 45, priorite: 2 },
  { nom: "vêtements", poids: 30, priorite: 3 },
  { nom: "riz", poids: 50, priorite: 2 },
  { nom: "vaccins", poids: 8, priorite: 1 }
]
soit charge = 0
soit restants = []
pour chaque c dans trier_par(colis, c => [c.priorite, c.poids]) faire
  si charge + c.poids <= 100 alors
    charge = charge + c.poids
    afficher "Chargé : " + c.nom + " (" + c.poids + " kg), total " + charge + " kg"
  sinon
    ajouter(restants, c.nom)
  fin
fin
afficher "Pour le prochain voyage : " + restants
```

Résultat :

```
Chargé : vaccins (8 kg), total 8 kg
Chargé : médicaments (12 kg), total 20 kg
Chargé : pièces moteur (45 kg), total 65 kg
Chargé : vêtements (30 kg), total 95 kg
Pour le prochain voyage : ["riz"]
```

</details>

## Pour aller plus loin : exercices en ligne de commande

Ces trois exercices utilisent le terminal ou internet ; ils ne sont donc pas exécutés automatiquement.

**A. Installer un paquet.** Installez le paquet `geometrie` (`fadera paquet installer Faderaseny/geometrie`), puis écrivez un programme qui l'importe (`importer "paquets/geometrie/principal" comme g`) et calcule l'aire d'un triangle de côtés 3, 4 et 5 avec `g.aire_triangle_cotes`.
Voir [PAQUETS.md](PAQUETS.md).

**B. Créer son paquet.** Avec `fadera paquet creer mon-outil`, écrivez deux fonctions utiles dans `principal.fa`, testez-les avec des blocs `tester`, puis publiez le dossier dans un dépôt GitHub.

**C. Parler à un service.** Écrivez un programme qui interroge une API publique avec `http_json`, affiche trois champs et gère l'absence de réseau (voir l'exercice 132).

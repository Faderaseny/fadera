# Exercices de mathématiques en Fadéra

10 exercices progressifs pour s'entraîner à programmer en Fadéra, du calcul simple jusqu'aux systèmes d'équations. Chaque exercice a : un **énoncé**, un **indice** (fonctions/notions à utiliser) et une **solution complète**, testée sur le vrai moteur Fadéra.

Essayez d'écrire votre propre solution avant de regarder celle proposée !

---

## Exercice 1 — Mini calculatrice
**Énoncé** : Avec deux nombres `a` et `b` de votre choix, affichez leur somme, différence, produit, quotient (arrondi à 2 décimales) et reste de la division.

**Indice** : `+`, `-`, `*`, `/`, `%`, `arrondir(x, decimales)`.

**Solution :**
```
soit a = 15
soit b = 4
afficher "Somme : " + (a + b)
afficher "Différence : " + (a - b)
afficher "Produit : " + (a * b)
afficher "Quotient : " + arrondir(a / b, 2)
afficher "Reste : " + (a % b)
```
**Résultat attendu :**
```
Somme : 19
Différence : 11
Produit : 60
Quotient : 3.75
Reste : 3
```

---

## Exercice 2 — Pair ou impair
**Énoncé** : Demandez un nombre entier `n` et affichez s'il est pair ou impair.

**Indice** : `%` (modulo), `si/alors/sinon/fin`.

**Solution :**
```
soit n = 17
si n % 2 == 0 alors
    afficher n + " est pair"
sinon
    afficher n + " est impair"
fin
```
**Résultat attendu :** `17 est impair`

---

## Exercice 3 — Somme de 1 à N
**Énoncé** : Calculez la somme des entiers de 1 à `n` (avec `n = 100`), sans formule toute faite — avec une boucle.

**Indice** : `pour i allantde 1 a n faire ... fin`, un accumulateur (`soit somme = 0`).

**Solution :**
```
soit n = 100
soit somme = 0
pour i allantde 1 a n faire
    somme = somme + i
fin
afficher "Somme de 1 à " + n + " = " + somme
```
**Résultat attendu :** `Somme de 1 à 100 = 5050`

*Vérification : la formule mathématique n(n+1)/2 = 100×101/2 = 5050. ✓*

---

## Exercice 4 — Table de multiplication
**Énoncé** : Affichez la table de multiplication d'un nombre (par exemple 7), de ×1 à ×10.

**Indice** : `pour i allantde 1 a 10 faire ... fin`.

**Solution :**
```
soit table = 7
pour i allantde 1 a 10 faire
    afficher table + " x " + i + " = " + (table * i)
fin
```
**Résultat attendu (extrait) :**
```
7 x 1 = 7
7 x 2 = 14
...
7 x 10 = 70
```

---

## Exercice 5 — Nombres premiers
**Énoncé** : Écrivez une fonction `estPremier(n)` qui renvoie `vrai` si `n` est premier, puis affichez tous les nombres premiers entre 2 et 30.

**Indice** : un nombre est premier s'il n'est divisible par aucun entier entre 2 et lui-même exclu. Utilisez `renvoyer` pour sortir dès qu'un diviseur est trouvé.

**Solution :**
```
fonction estPremier(n)
    si n < 2 alors
        renvoyer faux
    fin
    pour i allantde 2 a n - 1 faire
        si n % i == 0 alors
            renvoyer faux
        fin
    fin
    renvoyer vrai
fin

pour n allantde 2 a 30 faire
    si estPremier(n) alors
        afficher n + " est premier"
    fin
fin
```
**Résultat attendu :** `2, 3, 5, 7, 11, 13, 17, 19, 23, 29` (chacun sur une ligne).

---

## Exercice 6 — Suite de Fibonacci
**Énoncé** : Écrivez une fonction récursive `fibonacci(n)` (0, 1, 1, 2, 3, 5, 8, 13...) et affichez les 11 premiers termes.

**Indice** : un appel récursif utilise `fibonacci(n-1) + fibonacci(n-2)`, avec un cas de base pour `n <= 1`.

**Solution :**
```
fonction fibonacci(n)
    si n <= 1 alors
        renvoyer n
    fin
    renvoyer fibonacci(n - 1) + fibonacci(n - 2)
fin

pour i allantde 0 a 10 faire
    afficher fibonacci(i)
fin
```
**Résultat attendu :** `0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55`

> ⚠️ Cette version récursive recalcule beaucoup de valeurs (elle devient lente au-delà de `fibonacci(30)` environ). Défi bonus : réécrivez-la avec une boucle `tantque` et deux variables au lieu de la récursion, pour qu'elle reste rapide même pour de grands `n`.

---

## Exercice 7 — Équation du second degré
**Énoncé** : Trouvez les racines de `x² - 5x + 6 = 0` avec la fonction native dédiée.

**Indice** : `math_resoudre_second_degre(a, b, c)` résout `ax² + bx + c = 0`.

**Solution :**
```
soit a = 1
soit b = -5
soit c = 6
afficher "Racines de x² - 5x + 6 = 0 :"
afficher math_resoudre_second_degre(a, b, c)
```
**Résultat attendu :** `[2, 3]`

*Vérification : 2² - 5×2 + 6 = 4-10+6 = 0 ✓, et 3² - 5×3 + 6 = 9-15+6 = 0 ✓.*

---

## Exercice 8 — Statistiques d'une classe
**Énoncé** : À partir d'une liste de notes, affichez la moyenne, la médiane, la note minimale, la note maximale et l'écart-type.

**Indice** : `stats_moyenne`, `stats_mediane`, `stats_min`, `stats_max`, `stats_ecart_type` — toutes prennent une liste en argument.

**Solution :**
```
soit notes = [12, 8, 15, 20, 9, 14, 17, 11]
afficher "Notes : " + notes
afficher "Moyenne : " + arrondir(stats_moyenne(notes), 2)
afficher "Médiane : " + stats_mediane(notes)
afficher "Note min : " + stats_min(notes)
afficher "Note max : " + stats_max(notes)
afficher "Écart-type : " + arrondir(stats_ecart_type(notes), 2)
```
**Résultat attendu :**
```
Moyenne : 13.25
Médiane : 13
Note min : 8
Note max : 20
Écart-type : 3.8
```

---

## Exercice 9 — PGCD et PPCM
**Énoncé** : Calculez le PGCD et le PPCM de deux nombres (60 et 48).

**Indice** : `math_pgcd_ppcm(a, b)` renvoie une structure `{ pgcd, ppcm }` ; utilisez `lireCle(...)` pour en extraire les valeurs.

**Solution :**
```
soit a = 60
soit b = 48
soit resultat = math_pgcd_ppcm(a, b)
afficher "PGCD(" + a + ", " + b + ") = " + lireCle(resultat, "pgcd")
afficher "PPCM(" + a + ", " + b + ") = " + lireCle(resultat, "ppcm")
```
**Résultat attendu :**
```
PGCD(60, 48) = 12
PPCM(60, 48) = 240
```

---

## Exercice 10 — Système d'équations linéaires
**Énoncé** : Résolvez le système à trois inconnues :
```
2x +  y -  z = 3
 x -  y + 2z = 1
3x + 2y -  z = 5
```

**Indice** : `math_resoudre_systeme_gauss(matriceAugmentee)` prend une matrice où chaque ligne est `[coeff_x, coeff_y, coeff_z, résultat]`, et renvoie la liste `[x, y, z]`.

**Solution :**
```
soit systeme = [[2, 1, -1, 3], [1, -1, 2, 1], [3, 2, -1, 5]]
soit solution = math_resoudre_systeme_gauss(systeme)
afficher "x = " + arrondir(lireElement(solution, 0), 4)
afficher "y = " + arrondir(lireElement(solution, 1), 4)
afficher "z = " + arrondir(lireElement(solution, 2), 4)
```
**Résultat attendu :**
```
x = 1.25
y = 0.75
z = 0.25
```
*Vérification (ligne 1) : 2×1.25 + 0.75 - 0.25 = 2.5 + 0.75 - 0.25 = 3 ✓*

---

## Pour aller plus loin

Une fois ces 10 exercices maîtrisés, essayez par vous-même :
- **Trieur de nombres** : demandez 5 nombres avec `saisir(...)`, stockez-les dans une liste, triez-les à la main (sans fonction native de tri) avec une double boucle.
- **Calcul de π par Monte-Carlo** : utilisez `hasard(0, 1000)` pour tirer des points aléatoires dans un carré et estimer π à partir de la proportion tombée dans le cercle inscrit.
- **Corrélation** : construisez deux listes de nombres liées (ex. taille et poids fictifs) et utilisez `stats_pearson_correlation(x, y)` et `stats_regression_lineaire(x, y)` pour analyser leur relation.
- **Matrice inversible** : avec `matrice_carree(m)` et `inverser_matrice(m)`, écrivez un script qui vérifie si une matrice 2×2 est inversible avant de tenter de l'inverser.

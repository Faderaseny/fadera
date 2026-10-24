# Exercices avancés de mathématiques en Fadéra

10 exercices plus costauds : analyse numérique, théorie des nombres, statistiques appliquées, algèbre linéaire, réseaux de neurones et séries infinies. Chaque solution a été exécutée pour de vrai sur le moteur Fadéra et vérifiée.

Fadéra n'a pas de fonctions trigonométriques natives (`sin`, `cos`...) : les exercices s'appuient donc sur l'algèbre, les suites, les statistiques et l'analyse numérique par différences finies.

---

## A — Intégration numérique
**Énoncé** : Approximez ∫₀⁴ x² dx (valeur exacte connue : 4³/3 ≈ 21.333) par la méthode des trapèzes, en générant vous-même les points de la fonction.

**Indice** : générez une liste de valeurs `y = x²` par pas de 0.5 avec une boucle `tantque`, puis `math_integrale_trapeze(valeurs, pas)`.

**Solution :**
```
soit pas = 0.5
soit valeursY = liste()
soit x = 0
tantque x <= 4 faire
    ajouter(valeursY, x * x)
    x = x + pas
fin
soit approx = math_integrale_trapeze(valeursY, pas)
afficher "Approximation (trapèzes) : " + arrondir(approx, 4)
afficher "Valeur exacte : " + arrondir(puissance(4, 3) / 3, 4)
afficher "Erreur : " + arrondir(absolu(approx - puissance(4,3)/3), 4)
```
**Résultat attendu :**
```
Approximation (trapèzes) : 21.5
Valeur exacte : 21.3333
Erreur : 0.1667
```
*Plus le pas est petit, plus l'erreur diminue — essayez avec `pas = 0.1` pour vérifier.*

---

## B — Dérivée numérique
**Énoncé** : Approximez la dérivée de f(x) = x³ en x = 2 par différence centrée (dérivée exacte : 3x² = 12).

**Indice** : `math_derivee_centree(valeurs, index)` renvoie `(y[index+1] - y[index-1]) / 2` — il faut donc diviser le résultat par le pas utilisé pour obtenir la vraie pente.

**Solution :**
```
soit pas = 0.01
soit valeurs = [puissance(2 - pas, 3), puissance(2, 3), puissance(2 + pas, 3)]
soit approx = math_derivee_centree(valeurs, 1) / pas
afficher "Dérivée approchée en x=2 : " + arrondir(approx, 4)
afficher "Dérivée exacte (3x²) : " + (3 * puissance(2, 2))
```
**Résultat attendu :**
```
Dérivée approchée en x=2 : 12.0001
Dérivée exacte (3x²) : 12
```

---

## C — Nombres parfaits
**Énoncé** : Un nombre parfait est égal à la somme de ses diviseurs propres (ex. 6 = 1+2+3). Trouvez tous les nombres parfaits inférieurs à 1000.

**Indice** : fonction `sommeDiviseurs(n)` avec une boucle testant chaque diviseur possible via `%`.

**Solution :**
```
fonction sommeDiviseurs(n)
    soit somme = 0
    pour i allantde 1 a n - 1 faire
        si n % i == 0 alors
            somme = somme + i
        fin
    fin
    renvoyer somme
fin

pour n allantde 2 a 1000 faire
    si sommeDiviseurs(n) == n alors
        afficher n + " est un nombre parfait"
    fin
fin
```
**Résultat attendu :** `6`, `28`, `496` (les trois premiers nombres parfaits, connus depuis l'Antiquité).

> ⚠️ Cette approche est en `O(n²)` — elle devient lente au-delà de quelques milliers. Défi bonus : ne tester les diviseurs que jusqu'à `racine(n)` pour accélérer.

---

## D — Conjecture de Goldbach
**Énoncé** : Vérifiez que tout nombre pair entre 4 et 30 peut s'écrire comme la somme de deux nombres premiers (conjecture non démontrée mais vérifiée jusqu'à des nombres immenses).

**Indice** : réutilisez `estPremier(n)` (exercice 5 du premier cahier) puis cherchez `a` premier tel que `n - a` soit aussi premier.

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

fonction decomposerGoldbach(n)
    pour a allantde 2 a n faire
        si estPremier(a) et estPremier(n - a) alors
            renvoyer [a, n - a]
        fin
    fin
    renvoyer faux
fin

pour n allantde 4 a 30 pas 2 faire
    afficher n + " = " + decomposerGoldbach(n)
fin
```
**Résultat attendu (extrait) :**
```
4 = [2, 2]
6 = [3, 3]
8 = [3, 5]
...
30 = [7, 23]
```

---

## E — Détection d'anomalies statistiques
**Énoncé** : Dans un relevé de températures, détectez automatiquement les valeurs aberrantes (méthode de l'écart interquartile — IQR).

**Indice** : `stats_analyse_distribution(liste)` renvoie déjà les seuils `seuil_anomalie_bas` et `seuil_anomalie_haut` (règle classique : Q1 - 1.5×IQR et Q3 + 1.5×IQR).

**Solution :**
```
soit temperatures = [21, 22, 20, 23, 21, 19, 45, 22, 20, -15, 21, 22]
soit dist = stats_analyse_distribution(temperatures)
soit seuilBas = lireCle(dist, "seuil_anomalie_bas")
soit seuilHaut = lireCle(dist, "seuil_anomalie_haut")
afficher "Seuils normaux : [" + arrondir(seuilBas,2) + ", " + arrondir(seuilHaut,2) + "]"
pour i allantde 0 a taille(temperatures) - 1 faire
    soit t = lireElement(temperatures, i)
    si t < seuilBas ou t > seuilHaut alors
        afficher "⚠️ Anomalie détectée : " + t + "°C (index " + i + ")"
    fin
fin
```
**Résultat attendu :**
```
Seuils normaux : [17, 25]
⚠️ Anomalie détectée : 45°C (index 6)
⚠️ Anomalie détectée : -15°C (index 9)
```
Les deux capteurs défectueux (45°C et -15°C, manifestement hors norme au milieu de relevés autour de 20°C) sont repérés automatiquement.

---

## F — Régression linéaire et prédiction
**Énoncé** : À partir d'un historique heures de révision / notes obtenues, mesurez la corrélation, calculez la droite de régression, et prédisez la note pour 10h de révision.

**Indice** : `stats_pearson_correlation`, `stats_regression_lineaire` (renvoie une structure avec `pente` et `ordonnee_origine`), puis appliquez vous-même la formule `y = pente*x + ordonnee_origine`.

**Solution :**
```
soit heuresRevision = [1, 2, 3, 4, 5, 6, 7]
soit noteObtenue = [8, 10, 11, 13, 15, 16, 18]
afficher "Corrélation : " + arrondir(stats_pearson_correlation(heuresRevision, noteObtenue), 4)
soit reg = stats_regression_lineaire(heuresRevision, noteObtenue)
afficher "Modèle : " + lireCle(reg, "formule")
soit pente = lireCle(reg, "pente")
soit ordonnee = lireCle(reg, "ordonnee_origine")
soit predictionA10h = pente * 10 + ordonnee
afficher "Note prédite pour 10h de révision : " + arrondir(predictionA10h, 2)
```
**Résultat attendu :**
```
Corrélation : 0.9972
Modèle : y = 1.6429 * x + 6.4286
Note prédite pour 10h de révision : 22.86
```
*(La prédiction dépasse 20 — signe que le modèle linéaire ne doit pas être extrapolé trop loin des données observées, une limite classique de la régression linéaire à connaître !)*

---

## G — Système linéaire appliqué : problème d'alliage
**Énoncé** : Un fondeur veut produire 100 kg d'un alliage à partir de trois métaux de prix différents (10€/kg, 25€/kg, 40€/kg), pour une valeur totale de 2200€, avec 10 kg de métal A de plus que de métal B. Quelle quantité de chaque métal utiliser ?

**Indice** : posez le système `x+y+z=100`, `10x+25y+40z=2200`, `x-y=10`, puis `math_resoudre_systeme_gauss`.

**Solution :**
```
soit systeme = [[1, 1, 1, 100], [10, 25, 40, 2200], [1, -1, 0, 10]]
soit solution = math_resoudre_systeme_gauss(systeme)
afficher "Métal A : " + arrondir(lireElement(solution, 0), 2) + " kg"
afficher "Métal B : " + arrondir(lireElement(solution, 1), 2) + " kg"
afficher "Métal C : " + arrondir(lireElement(solution, 2), 2) + " kg"
```
**Résultat attendu :**
```
Métal A : 43.33 kg
Métal B : 33.33 kg
Métal C : 23.33 kg
```
*Vérification : 43.33+33.33+23.33=100 kg ✓, et 43.33-33.33=10 ✓.*

---

## H — Entraîner un perceptron à la main (porte logique ET)
**Énoncé** : Sans utiliser `ia_entrainer_reseau_xor` (boîte noire), écrivez vous-même la boucle d'apprentissage d'un neurone pour qu'il apprenne la porte logique **ET** (0 ET 0 = 0, 1 ET 1 = 1, etc.).

**Indice** : à chaque exemple, calculez la sortie avec `ia_activation_perceptron`, l'erreur (`attendu - obtenu`), puis corrigez les poids avec `ia_descente_gradient`. Répétez sur plusieurs milliers d'époques.

**Solution :**
```
soit neurone1 = ia_creer_neurone(2, "sigmoid")
soit dataset = [
    { entrees: [0, 0], attendu: 0 },
    { entrees: [0, 1], attendu: 0 },
    { entrees: [1, 0], attendu: 0 },
    { entrees: [1, 1], attendu: 1 }
]

pour epoque allantde 1 a 4000 faire
    pour i allantde 0 a taille(dataset) - 1 faire
        soit exemple = lireElement(dataset, i)
        soit entrees = lireCle(exemple, "entrees")
        soit sortie = ia_activation_perceptron(neurone1, entrees)
        soit erreur = lireCle(exemple, "attendu") - sortie
        structure config = { entrees: entrees, erreur: erreur }
        ia_descente_gradient(neurone1, config)
    fin
fin

afficher "Résultats après entraînement (porte ET) :"
afficher "0 ET 0 = " + arrondir(ia_activation_perceptron(neurone1, [0, 0]), 3)
afficher "0 ET 1 = " + arrondir(ia_activation_perceptron(neurone1, [0, 1]), 3)
afficher "1 ET 0 = " + arrondir(ia_activation_perceptron(neurone1, [1, 0]), 3)
afficher "1 ET 1 = " + arrondir(ia_activation_perceptron(neurone1, [1, 1]), 3)
```
**Résultat attendu (les valeurs exactes varient légèrement, poids initiaux aléatoires) :**
```
0 ET 0 = 0.001
0 ET 1 = 0.093
1 ET 0 = 0.093
1 ET 1 = 0.89
```
Le neurone a bien appris : proche de 0 partout sauf pour `[1,1]`, proche de 1.

> Contrairement au XOR (exercice précédent dans le premier cahier), le ET est apprenable par un **seul** neurone — c'est justement parce que le XOR ne l'est pas qu'il faut une couche cachée (`ia_reseau_multicouche`), une découverte historique importante en IA (1969, Minsky & Papert).

---

## I — Approximation de π par la série de Leibniz
**Énoncé** : Approchez π avec la formule `π/4 = 1 - 1/3 + 1/5 - 1/7 + 1/9 - ...`, en sommant 200 000 termes.

**Indice** : boucle `pour`, un accumulateur `somme`, un `signe` qui alterne entre 1 et -1 à chaque tour.

**Solution :**
```
soit termes = 200000
soit somme = 0
soit signe = 1
pour i allantde 0 a termes - 1 faire
    somme = somme + signe / (2 * i + 1)
    signe = signe * -1
fin
soit piApprox = somme * 4
afficher "π approché après " + termes + " termes : " + arrondir(piApprox, 6)
afficher "Erreur par rapport à 3.14159265 : " + arrondir(absolu(piApprox - 3.14159265), 6)
```
**Résultat attendu :**
```
π approché après 200000 termes : 3.141588
Erreur par rapport à 3.14159265 : 0.000005
```
*Cette série converge très lentement (l'erreur ne diminue qu'en 1/termes) — comparez avec l'exercice suivant, qui converge beaucoup plus vite.*

---

## J — Approximation du nombre e par sa série
**Énoncé** : Approchez *e* (base du logarithme népérien, ≈ 2.71828...) avec `e = 1/0! + 1/1! + 1/2! + 1/3! + ...`.

**Indice** : une fonction récursive `factorielle(n)`, puis une boucle qui somme `1 / factorielle(n)`.

**Solution :**
```
fonction factorielle(n)
    si n <= 1 alors
        renvoyer 1
    fin
    renvoyer n * factorielle(n - 1)
fin

soit somme = 0
pour n allantde 0 a 15 faire
    somme = somme + 1 / factorielle(n)
fin
afficher "e approché : " + arrondir(somme, 10)
afficher "Valeur de référence : 2.7182818285"
```
**Résultat attendu :**
```
e approché : 2.7182818285
Valeur de référence : 2.7182818285
```
Avec seulement 16 termes (0! à 15!), la précision est déjà parfaite à 10 décimales — contrairement à Leibniz, cette série converge extrêmement vite (factoriellement).

---

## Pour aller encore plus loin

- **Crible d'Ératosthène** : réécrivez l'exercice D (Goldbach) en pré-calculant d'abord la primalité de tous les nombres jusqu'à N dans une liste, pour éviter de retester `estPremier` inutilement.
- **Descente de gradient sur une fonction 1D** : sans passer par `ia_*`, minimisez `f(x) = (x-3)²` à la main : partez de `x = 0`, et répétez `x = x - taux * derivee` où `derivee = 2*(x-3)`, en utilisant `math_derivee_centree` pour l'estimer numériquement à chaque étape.
- **Réseau multicouche sur un jeu de données personnalisé** : utilisez `ia_reseau_multicouche([2, 4, 1], config)` sur un problème non linéairement séparable de votre invention (comme XOR, mais avec vos propres règles).
- **Nombres amiables** : deux nombres sont amiables si la somme des diviseurs de l'un donne l'autre, et vice versa (ex. 220 et 284). Adaptez `sommeDiviseurs` de l'exercice C pour les retrouver.

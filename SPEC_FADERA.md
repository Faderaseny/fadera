# Spécification du langage Fadéra

Version de la spécification : **1.0** — décrit le comportement des deux moteurs (JavaScript dans `index.html`, Rust dans `fadera-rs`).

**Cette spécification est exécutable.** Chaque bloc `fa-test` est suivi d'un bloc `sortie` : le programme, et ce qu'il doit afficher. Le script `verifier_spec.js` exécute tous les exemples avec **les deux moteurs** et vérifie que chacun produit exactement la sortie annoncée. Un écart entre le texte et un moteur, ou entre les deux moteurs, est un défaut : soit du moteur, soit de ce document.

**Statut des moteurs.** Le moteur **Rust** (natif et WebAssembly) est le moteur de **référence** : en cas de désaccord entre les deux, c'est lui qui a raison tant que la présente spécification n'en dit pas autrement. Le moteur **JavaScript** est un moteur de **secours** (il sert quand le moteur Rust n'est pas chargé) et un second moteur du **débogueur pas à pas** (le débogueur Rust s'arrête aux mêmes lignes, avec les mêmes variables et la même pile : cela est vérifié). Les deux sont maintenus identiques : la parité est vérifiée sur plusieurs centaines de programmes et sur les calculs mathématiques bit à bit.

Convention : une erreur d'exécution est écrite `ERR:Ligne N : message` dans les blocs `sortie`.

Les mots **doit**, **ne doit pas** et **peut** ont leur sens habituel de spécification : ce qui « doit » est garanti, ce qui « peut » est laissé libre.

---

## 1. Lexique

### 1.1 Espaces, lignes, commentaires
- Les espaces, tabulations et sauts de ligne séparent les éléments ; ils n'ont **aucune autre signification** (l'indentation est libre). Un saut de ligne ne termine pas une instruction par lui-même, mais le numéro de ligne sert aux messages d'erreur.
- `#` et `//` commencent un commentaire qui va jusqu'à la fin de la ligne.
- Le point-virgule `;` est **ignoré** partout : il est facultatif et sans effet.

```fa-test
afficher 1 # commentaire
// autre commentaire
afficher 2;
```
```sortie
1
2
```

### 1.2 Nombres
Entiers et décimaux avec un point : `42`, `3.14`. La notation scientifique est admise : `1e7`, `2.5E-3`, `4e+2`. Le `e` doit suivre immédiatement les chiffres et être suivi d'un chiffre (avec un signe facultatif). Tous les nombres sont des flottants 64 bits (IEEE 754).

```fa-test
afficher 1e7
afficher 2.5E-3
afficher 4e+2
afficher 2e3 + 1
```
```sortie
10000000
0.0025
400
2001
```

### 1.3 Textes
Entre guillemets doubles ou simples. Échappements : `\n` (saut de ligne), `\t` (tabulation), `\"`, `\'`, `\\`.

Un texte est une suite de **points de code Unicode** : `taille`, l'indexation et `sous_texte` comptent un emoji pour 1.

```fa-test
afficher "a\nb\t|\"q\""
afficher 'simple'
afficher taille("😀a")
afficher lireElement("😀a", 1)
```
```sortie
a
b	|"q"
simple
2
a
```

### 1.4 Identifiants
Une lettre (Unicode) ou `_`, puis des lettres, chiffres ou `_`. Les identifiants sont **sensibles à la casse**. Un identifiant peut contenir des points suivis d'une lettre : `o.double` est **un seul** identifiant (espaces de noms de modules, §11).

### 1.5 Mots réservés
`soit fnf si alors sinon fin afficher tantque faire fonction renvoyer saisir liste ajouter taille lireElement modifierElement modifierCle hasard structure lireCle pour allantde lireFichier ecrireFichier vrai faux non essayer attraper selon cas defaut rompre continuer importer`, ainsi que `et` et `ou`. On ne peut pas les utiliser comme noms de variables ou de fonctions.

`a`, `pas`, `chaque`, `dans` et `comme` ne sont **pas** réservés : ils n'ont un sens spécial qu'à l'endroit précis de la grammaire où ils sont attendus.

```fa-test
soit comme = 3
soit pas = 4
afficher comme + pas
soit taille = 3
```
```sortie
ERR:Ligne 4 : "taille" est un mot réservé du langage : choisissez un autre nom.
```

---

## 2. Valeurs et types

| Type | Exemples | Nom donné par `type_de` |
|---|---|---|
| nombre | `42`, `-3.5` | `nombre` |
| texte | `"salut"` | `texte` |
| booléen | `vrai`, `faux` | `booléen` |
| liste | `[1, "a", [2]]` | `liste` |
| structure | `{ nom: "Awa", age: 20 }` | `structure` |
| fonction | `x => x * 2` | `fonction` |
| rien | résultat d'une fonction qui ne renvoie rien | `rien` |

`rien` n'a pas de littéral : on l'obtient en appelant une fonction sans `renvoyer`.

```fa-test
fonction f()
fin
afficher f()
```
```sortie
rien
```

### 2.1 Affichage
`afficher a, b, c` écrit les valeurs séparées par **une espace**, puis un saut de ligne.

- Un nombre non entier est arrondi à **12 chiffres significatifs**.
- Les textes **à l'intérieur** d'une liste ou d'une structure sont affichés entre guillemets ; au premier niveau ils ne le sont pas.
- Une liste s'écrit `[a, b]`, une structure `{ clé: valeur, … }` (les clés dans l'ordre d'insertion), une structure vide `{  }`.
- Une liste ou structure qui se **contient elle-même** s'affiche `[...]` ou `{...}` à l'endroit du cycle.

```fa-test
afficher "a", 1, [1]
afficher 1 / 3
afficher 0.1 + 0.2
afficher [1, "a", [2, vrai], { k: "v" }]
afficher {}
afficher {b: 1, a: 2}
soit a = []
ajouter(a, a)
afficher a
```
```sortie
a 1 [1]
0.333333333333
0.3
[1, "a", [2, vrai], { k: "v" }]
{  }
{ b: 1, a: 2 }
[[...]]
```

---

## 3. Grammaire

Notation : `x*` zéro ou plus, `x+` un ou plus, `[x]` facultatif, `|` ou, les terminaux entre guillemets.

```
programme    = instruction*
instruction  = "soit" ID "=" expr
             | "structure" ID "=" expr
             | ID "=" expr                       (affectation)
             | "afficher" expr ("," expr)*
             | "si" expr "alors" instruction* {"sinon" "si" expr "alors" instruction*} ["sinon" instruction*] "fin"
             | "tantque" expr "faire" instruction* "fin"
             | "pour" ID "allantde" expr "a" expr ["pas" expr] "faire" instruction* "fin"
             | "pour" "chaque" ID "dans" expr "faire" instruction* "fin"
             | "fonction" ID "(" [ID ("," ID)*] ")" instruction* "fin"
             | "renvoyer" [expr]
             | "rompre" | "continuer"
             | "selon" expr "faire" {"cas" expr ("," expr)* "alors" instruction*} ["defaut" instruction*] "fin"
             | "essayer" instruction* "attraper" ID instruction* "fin"
             | "importer" expr ["comme" ID]
             | "ecrireFichier" "(" expr "," expr ")"
             | appel | "ajouter" "(" … ")" | "modifierElement" "(" … ")" | "modifierCle" "(" … ")"
expr         = ou
ou           = et {"ou" et}
et           = non {"et" non}
non          = "non" non | comparaison
comparaison  = somme {("==" | "!=" | "<" | ">" | "<=" | ">=") somme}
somme        = produit {("+" | "-") produit}
produit      = unaire {("*" | "/" | "%") unaire}
unaire       = "-" unaire | puissance
puissance    = primaire ["**" unaire]
primaire     = NOMBRE | TEXTE | "vrai" | "faux" | ID | appel
             | "(" expr ")" | liste | structure | lambda
appel        = ["fnf"] ID "(" [expr ("," expr)*] ")"
liste        = "[" [expr ("," expr)*] "]"  |  "liste" "(" [expr ("," expr)*] ")"
structure    = "{" [ID ":" expr ("," ID ":" expr)*] "}"
lambda       = ID "=>" expr  |  "(" [ID ("," ID)*] ")" "=>" expr  |  "fonction" "(" … ")" instruction* "fin"
```

`sinon si` sur **la même ligne** n'ouvre pas un nouveau bloc : un seul `fin` ferme toute la chaîne.

Une instruction peut être une expression seule (un appel). Le préfixe `fnf` devant un appel est facultatif et sans effet sur le résultat.

### 3.1 Priorités (de la plus faible à la plus forte)

| Niveau | Opérateurs | Associativité |
|---|---|---|
| 1 | `ou` | gauche |
| 2 | `et` | gauche |
| 3 | `non` | préfixe |
| 4 | `==` `!=` `<` `>` `<=` `>=` | gauche |
| 5 | `+` `-` | gauche |
| 6 | `*` `/` `%` | gauche |
| 7 | `-` (négatif) | préfixe |
| 8 | `**` | **droite** |

Conséquences à connaître :
- `non` est **moins** prioritaire que les comparaisons : `non 1 < 2` signifie `non (1 < 2)`.
- `**` est **plus** prioritaire que le signe moins : `-2 ** 2` vaut `-4`.
- L'exposant peut être négatif : `2 ** -1` vaut `0.5`.
- Les comparaisons ne se **chaînent pas** utilement : `1 < 2 < 3` calcule `(1 < 2) < 3`, ce qui compare un booléen et un nombre, et **échoue** (§5.3).

```fa-test
afficher 1 + 2 * 3 - 4 / 2
afficher 10 - 2 - 3
afficher 100 / 10 / 5
afficher 2 ** 3 ** 2
afficher -2 ** 2
afficher (-2) ** 2
afficher 2 * 3 ** 2
afficher 2 ** -1
afficher non 1 < 2
```
```sortie
5
5
2
512
-4
4
18
0.5
faux
```

```fa-test
afficher 1 < 2 < 3
```
```sortie
ERR:Ligne 1 : Impossible de comparer booléen et nombre avec "<".
```

---

## 4. Vérité, `et`, `ou`, `non`

Sont **faux** : `faux`, `0`, le texte vide `""`, et `rien`. **Tout le reste est vrai**, y compris la liste vide `[]` et la structure vide.

`et` et `ou` sont **court-circuités** et **renvoient l'un de leurs opérandes** (pas forcément un booléen), comme en JavaScript : `a et b` renvoie `a` si `a` est faux, sinon `b` ; `a ou b` renvoie `a` si `a` est vrai, sinon `b`. `non` renvoie toujours un booléen.

```fa-test
si 0 alors afficher "v" sinon afficher "f" fin
si "" alors afficher "v" sinon afficher "f" fin
si [] alors afficher "v" sinon afficher "f" fin
si "a" alors afficher "v" sinon afficher "f" fin
afficher non 0
afficher non "x"
afficher 1 et "a"
afficher 0 ou "b"
afficher faux ou 5
si faux et 1 / 0 alors afficher 1 fin
afficher "ok"
```
```sortie
f
f
v
v
vrai
faux
a
b
5
ok
```

---

## 5. Opérateurs

### 5.1 Arithmétique
`-`, `*`, `/`, `%`, `**` n'acceptent que des **nombres**. La division et le modulo par zéro sont des **erreurs** (pas d'infini). Le reste `%` prend le signe du dividende (comme en JavaScript).

```fa-test
afficher 7 % 3
afficher -7 % 3
afficher 7 % -3
afficher 5 / 2
afficher 4 / 2
afficher 1 / 0
```
```sortie
1
-1
1
2.5
2
ERR:Ligne 6 : Division par zéro.
```

```fa-test
afficher "a" - 1
```
```sortie
ERR:Ligne 1 : L'opérateur "-" attend deux nombres (reçu : texte et nombre).
```

### 5.2 `+`
- nombre + nombre : addition.
- si **l'un** des deux est un texte et l'autre un nombre ou un booléen : **concaténation** (le nombre ou le booléen est écrit comme `afficher` l'écrit).
- toute autre combinaison est une erreur.

```fa-test
afficher 1 + "2"
afficher "a" + 1
afficher "a" + vrai
```
```sortie
12
a1
avrai
```

```fa-test
afficher vrai + 1
```
```sortie
ERR:Ligne 1 : Impossible d'additionner booléen et nombre.
```

### 5.3 Comparaisons
- `==` et `!=` ne convertissent **jamais** : `1 == "1"` est faux, `vrai == 1` est faux.
- `<`, `>`, `<=`, `>=` comparent deux nombres, ou deux textes. Les textes sont comparés **unité UTF-16 par unité UTF-16** : `"B" < "a"`, et `"10" < "9"`. Toute autre combinaison est une erreur.

```fa-test
afficher 1 == "1"
afficher vrai == 1
afficher "a" < "b"
afficher "B" < "a"
afficher "10" < "9"
afficher 10 < 9
afficher 1 < "2"
```
```sortie
faux
faux
vrai
vrai
vrai
faux
ERR:Ligne 7 : Impossible de comparer nombre et texte avec "<".
```

### 5.4 Fonctions mathématiques : arrondi correct
`sinus`, `cosinus`, `tangente`, `arc_sinus`, `arc_cosinus`, `arc_tangente`, `arc_tangente2`, `exponentielle`, `logarithme`, `logarithme10` et l'opérateur `**` donnent **le nombre à virgule flottante le plus proche de la vraie valeur mathématique** (un seul arrondi, calculé avec ≈106 bits). Le résultat **ne dépend donc ni de la machine, ni du navigateur, ni du moteur** (Rust, WebAssembly ou JavaScript) : les deux moteurs donnent les mêmes 64 bits.

Cette garantie vaut pour : les fonctions trigonométriques quand |x| < 8·10⁸ ; `exponentielle` pour -700 < x < 709,7 ; `**` quand le résultat est normal (entre 10⁻³⁰⁰ et 10³⁰⁰ environ). Hors de ces domaines, le résultat est celui de la bibliothèque mathématique de la plate-forme, exact à moins d'une unité du dernier chiffre.

```fa-test
afficher sinus(1) == 0.8414709848078965
afficher cosinus(1) == 0.5403023058681398
afficher exponentielle(1) == 2.718281828459045
afficher logarithme(10) == 2.302585092994046
afficher 2 ** 0.5 == 1.4142135623730951
afficher arc_tangente(1) == 0.7853981633974483
afficher 10 ** -2 == 0.01
afficher sinus(0)
afficher logarithme(0)
```
```sortie
vrai
vrai
vrai
vrai
vrai
vrai
vrai
0
ERR:Ligne 9 : logarithme : l'argument doit être strictement positif.
```

---

## 6. Égalité

`==` compare **le contenu** des listes et des structures, récursivement :
- deux listes sont égales si elles ont la même longueur et des éléments égaux deux à deux ;
- deux structures sont égales si elles ont les mêmes clés (**l'ordre ne compte pas**) avec des valeurs égales ;
- une valeur est égale à **elle-même** (même référence), y compris si elle se contient ;
- deux structures cycliques distinctes sont comparées sans boucler à l'infini.

```fa-test
afficher [1, 2] == [1, 2]
afficher [1, 2] == [1, 2, 3]
afficher {a: 1, b: 2} == {b: 2, a: 1}
afficher {a: 1} == {a: 1, b: 2}
afficher 0 == 0
afficher 1 / 3 == 0.333333333333
soit a = []
ajouter(a, a)
afficher a == a
soit b = []
ajouter(b, b)
afficher a == b
```
```sortie
vrai
faux
vrai
faux
vrai
faux
vrai
vrai
```

---

## 7. Copie et référence

- **Nombres, textes et booléens** sont des valeurs : on ne peut pas les modifier en place.
- **Listes et structures** sont des **références** partagées. L'affectation `soit b = a` ne copie pas : `a` et `b` désignent la même liste. `ajouter`, `modifierElement` et `modifierCle` modifient la liste ou la structure **en place** et la **renvoient**.
- Passer une liste à une fonction passe la référence : la fonction peut la modifier.
- Pour obtenir une copie indépendante : `copier(x)` (copie profonde, qui supporte les cycles).

```fa-test
soit a = [1]
soit b = a
ajouter(b, 2)
afficher a
soit s = {k: 1}
soit t = s
modifierCle(t, "k", 2)
afficher s
soit l = [1, 2]
soit f = x => modifierElement(x, 0, 9)
f(l)
afficher l
soit c = copier(a)
ajouter(c, 3)
afficher a
afficher c
soit u = "x"
soit v = u
v = v + "y"
afficher u
```
```sortie
[1, 2]
{ k: 2 }
[9, 2]
[1, 2]
[1, 2, 3]
x
```

```fa-test
soit l = [1, 2]
afficher ajouter(l, 3)
afficher modifierElement(l, 0, 9)
soit s = {a: 1}
afficher modifierCle(s, "b", 2)
```
```sortie
[1, 2, 3]
[9, 2, 3]
{ a: 1, b: 2 }
```

---

## 8. Variables et portée

- `soit x = …` crée la variable dans **la portée courante** : le script (portée globale) ou le corps de la fonction. Re-déclarer le même nom dans la même portée **remplace** la valeur.
- **Il n'y a pas de portée de bloc** : un `soit` dans un `si`, un `tantque` ou un `pour` reste visible après le bloc.
- `x = …` (sans `soit`) modifie une variable **existante**, en remontant les portées ; si elle n'existe pas, c'est une erreur.
- La variable d'une boucle `pour` appartient à la portée courante et **reste visible** après la boucle, avec sa dernière valeur. L'affecter dans le corps change la valeur vue par le corps, mais pas le nombre de tours.

```fa-test
soit x = 1
si vrai alors
  soit x = 2
  afficher x
fin
afficher x
si vrai alors
  soit y = 5
fin
afficher y
soit x = 3
afficher x
```
```sortie
2
2
5
3
```

```fa-test
x = 1
afficher x
```
```sortie
ERR:Ligne 1 : La variable "x" n'existe pas. Déclarez-la d'abord avec : soit x = ...
```

```fa-test
fonction f()
  soit x = 5
fin
f()
afficher x
```
```sortie
ERR:Ligne 5 : La variable "x" n'existe pas. Déclarez-la d'abord avec : soit x = ...
```

```fa-test
pour i allantde 1 a 3 faire
  i = 10
  afficher i
fin
afficher i
```
```sortie
10
10
10
10
```

---

## 9. Fonctions

### 9.1 Déclaration et appel
- `fonction f(a, b) … fin` déclare une fonction **globale**. Les déclarations sont « remontées » : on peut appeler `f` **avant** sa déclaration.
- Déclarer deux fois le même nom : **la dernière** déclaration l'emporte.
- Le nombre d'arguments doit être exactement celui des paramètres.
- Sans `renvoyer`, la fonction renvoie `rien`. `renvoyer` sans valeur renvoie `rien` (voir §9.5 pour la place où il est permis).
- À l'intérieur d'une fonction, les noms qui ne sont ni des paramètres ni des variables locales sont cherchés dans la **portée globale au moment de l'appel** (pas au moment de la déclaration).
- La récursion est limitée à **5000 appels imbriqués**.

```fa-test
afficher f(2)
fonction f(x)
  renvoyer x + 1
fin
fonction g(x)
  renvoyer x
fin
fonction g(x)
  renvoyer x * 2
fin
afficher g(2)
soit x = 1
fonction h()
  renvoyer x
fin
soit x = 2
afficher h()
```
```sortie
3
4
2
```

```fa-test
fonction f(a, b)
  renvoyer a
fin
afficher f(1)
```
```sortie
ERR:Ligne 4 : La fonction "f" attend 2 argument(s), 1 reçu(s).
```

```fa-test
fonction r(n)
  renvoyer r(n + 1)
fin
r(0)
```
```sortie
ERR:Ligne 2 : Récursion trop profonde (plus de 5000 appels imbriqués).
```

### 9.2 Lambdas et fermetures
`x => x * 2`, `(a, b) => a + b` et `fonction(x) … fin` sont des **valeurs** : on peut les ranger dans une variable, les passer en argument, les renvoyer. Une lambda **capture** les variables locales de la fonction qui la crée (fermeture).

Une lambda ne contient qu'**une expression** : elle ne peut pas contenir d'affectation. Pour une fermeture qui garde un état modifiable, on capture une liste ou une structure.

```fa-test
soit f = x => x * 2
afficher f(4)
afficher f
fonction faire_additionneur(n)
  renvoyer x => x + n
fin
soit plus5 = faire_additionneur(5)
afficher plus5(1)
soit a = 5
soit g = fonction(x)
  renvoyer x + a
fin
afficher g(1)
fonction compteur()
  soit c = [0]
  renvoyer () => modifierElement(c, 0, lireElement(c, 0) + 1)
fin
soit k = compteur()
k()
afficher k()
```
```sortie
8
<fonction anonyme>
6
6
[2]
```

### 9.3 Limite connue : fonctions nommées imbriquées
Une fonction **déclarée avec `fonction nom(...)` à l'intérieur d'une autre** est globale comme les autres : elle **ne voit pas** les variables locales de la fonction englobante. Pour capturer des variables locales, utilisez une lambda (§9.2).

```fa-test
fonction compteur()
  soit n = 0
  fonction inc()
    n = n + 1
    renvoyer n
  fin
  renvoyer inc
fin
soit c = compteur()
c()
```
```sortie
ERR:Ligne 4 : La variable "n" n'existe pas. Déclarez-la d'abord avec : soit n = ...
```

### 9.4 `renvoyer` hors d'une fonction
Au niveau du script, `renvoyer` **arrête le programme** sans erreur.

```fa-test
afficher 1
renvoyer 5
afficher 2
```
```sortie
1
```

### 9.5 `renvoyer` sans valeur
`renvoyer` seul n'est reconnu que s'il est **immédiatement suivi** de `fin`, `sinon`, `cas`, `defaut`, `attraper` ou de la fin du programme. Partout ailleurs, ce qui le suit est lu comme la valeur à renvoyer.

```fa-test
fonction f(x)
  si x > 0 alors
    renvoyer
  fin
  renvoyer 1
fin
afficher f(5)
afficher f(-5)
```
```sortie
rien
1
```

```fa-test
fonction g()
  renvoyer
  afficher 2
fin
```
```sortie
ERR:Ligne 3 : Valeur ou expression attendue, mais "afficher" trouvé.
```

---

## 10. Structures de contrôle

### 10.1 `pour`
`pour i allantde a a b [pas p]` : `i` prend les valeurs `a`, `a+p`, `a+2p`… tant que `i <= b` (si `p > 0`) ou `i >= b` (si `p < 0`). Le pas par défaut est `1`. Les bornes sont **incluses**. Le pas `0` est une erreur. Les bornes peuvent être décimales.

```fa-test
pour i allantde 5 a 1 pas -2 faire
  afficher i
fin
pour i allantde 1 a 0 faire
  afficher i
fin
pour i allantde 1 a 2.5 faire
  afficher i
fin
pour i allantde 1 a 3 pas 0 faire
  afficher i
fin
```
```sortie
5
3
1
1
2
ERR:Ligne 10 : Le pas de la boucle "pour" ne peut pas être 0.
```

### 10.2 `pour chaque`
Parcourt une liste (ses éléments), un texte (ses caractères, au sens des points de code) ou une structure (ses **clés**, dans l'ordre d'insertion).

```fa-test
pour chaque c dans "😀a" faire
  afficher c
fin
pour chaque k dans {a: 1, b: 2} faire
  afficher k
fin
```
```sortie
😀
a
a
b
```

### 10.3 `tantque`, `rompre`, `continuer`
`rompre` quitte la boucle la plus proche ; `continuer` passe au tour suivant. Hors d'une boucle, ils sont une erreur de **syntaxe** (détectée avant l'exécution). Une boucle qui dépasse **20 millions de pas** d'exécution est interrompue par une erreur (protection contre les boucles infinies).

```fa-test
soit i = 0
tantque i < 3 faire
  i = i + 1
  si i == 2 alors continuer fin
  afficher i
fin
tantque vrai faire
fin
```
```sortie
1
3
ERR:Ligne 7 : Limite d'exécution dépassée : boucle infinie probable.
```

### 10.4 `selon`
Compare la valeur de l'expression à chaque `cas` **dans l'ordre**, avec la même égalité que `==` (donc sans conversion : `"2"` ne correspond pas à `2`). Le premier qui correspond est exécuté, **sans enchaînement** vers les suivants. `defaut` est exécuté si aucun cas ne correspond. Un `cas` peut lister plusieurs valeurs séparées par des virgules.

```fa-test
selon 2 faire
  cas 1 alors
    afficher "un"
  cas 2, 3 alors
    afficher "deux ou trois"
  defaut
    afficher "autre"
fin
selon 2 faire
  cas 2 alors
    afficher "a"
  cas 2 alors
    afficher "b"
fin
selon "2" faire
  cas 2 alors
    afficher "nombre"
  defaut
    afficher "autre"
fin
```
```sortie
deux ou trois
a
autre
```

### 10.5 `essayer` / `attraper`
Exécute le premier bloc. Si une **erreur d'exécution** s'y produit, le bloc s'arrête, l'erreur est convertie en **texte** (son message, sans « Ligne N : »), rangé dans la variable de `attraper`, et le second bloc s'exécute. Sinon le second bloc est ignoré. Les `essayer` peuvent s'imbriquer ; une erreur levée dans un `attraper` est transmise au `essayer` englobant. `renvoyer`, `rompre` et `continuer` traversent `essayer` normalement. Les erreurs de **syntaxe** et la limite d'exécution ne sont **pas** attrapables.

```fa-test
essayer
  afficher 1
  afficher 1 / 0
  afficher 2
attraper e
  afficher "erreur : " + e
fin
afficher "suite"
essayer
  essayer
    afficher 1 / 0
  attraper e
    afficher "interne"
    afficher lireElement([], 0)
  fin
attraper e2
  afficher "externe : " + e2
fin
fonction f()
  essayer
    renvoyer 1
  attraper e
    renvoyer 2
  fin
fin
afficher f()
pour i allantde 1 a 5 faire
  essayer
    si i == 2 alors afficher 1 / 0 fin
    si i == 4 alors rompre fin
    afficher i
  attraper e
    afficher "x"
  fin
fin
```
```sortie
1
erreur : Division par zéro.
suite
interne
externe : Index 0 hors limites (la liste est vide).
1
1
x
3
```

---

## 11. Modules

`importer "chemin"` lit le fichier `chemin.fa` (l'extension est facultative), l'exécute **une seule fois** (les cycles sont sans danger) et rend ses fonctions et variables disponibles. Un module qui en importe un autre le cherche **d'abord à côté de lui**, puis depuis le dossier courant. Les modules standard s'importent par `importer "std/nom"`.

Sans alias, les noms du module sont **communs à tout le programme** : le dernier importé l'emporte.

`importer "chemin" comme o` range tout ce que le module déclare **au niveau supérieur** (fonctions, variables, structures, alias d'imports) sous le préfixe `o.` : on l'utilise comme `o.double(3)`. À l'intérieur du module, les noms restent courts, et un paramètre ou une variable locale de même nom reste local. Les clés d'une structure littérale ne sont pas modifiées. Un même module peut être importé sous plusieurs alias.

```fa-test
importer "std/maths" comme m
afficher m.factorielle(4)
soit factorielle = 5
afficher factorielle
afficher m.factorielle(3)
```
```sortie
24
5
6
```

---

## 11b. Classes et méthodes

`classe` et `herite` ne sont **pas** des mots réservés : `classe` suivi d'un nom ouvre une classe, `herite` n'a de sens que sur la ligne de `classe`, après le nom. Une variable nommée `classe` reste donc possible.

```
classe Nom [herite Parent]
    fonction creer(a, b)      # constructeur (facultatif)
        soi.a = a
    fin
    fonction methode(x)
        renvoyer soi.a + x
    fin
fin
```

Une classe ne contient que des méthodes (`fonction … fin`). Chaque méthode reçoit **implicitement** le paramètre `soi` (l'objet) en première position ; écrire `soi` dans la liste des paramètres est une erreur. `Nom(args)` crée une instance, appelle `creer(args)` si elle existe, et renvoie l'instance.

Une instance est une **structure ordinaire** portant une étiquette cachée : le nom de sa classe. L'étiquette est invisible pour `taille`, `pour chaque`, les clés et le JSON ; elle apparaît à l'affichage (`Point { x: 3, y: 4 }`) et dans l'égalité (deux structures de classes différentes ne sont jamais égales). `type_de` reste `"structure"`.

```fa-test
classe Point
    fonction creer(x, y)
        soi.x = x
        soi.y = y
    fin
    fonction norme()
        renvoyer racine(soi.x ** 2 + soi.y ** 2)
    fin
    fonction deplacer(dx, dy)
        soi.x = soi.x + dx
        soi.y = soi.y + dy
        renvoyer soi
    fin
fin
soit p = Point(3, 4)
afficher p.norme()
afficher p
p.x = 10
afficher p.x, p.y
afficher p.deplacer(1, 1) == Point(11, 5)
afficher type_de(p), taille(p), classe_de(p), classe_de(5)
afficher copier(p)
```
```sortie
5
Point { x: 3, y: 4 }
10 4
vrai
structure 2 Point rien
Point { x: 11, y: 5 }
```

### 11b.1 Héritage

`herite Parent` fait remonter la recherche d'une méthode au parent (puis au parent du parent, 100 niveaux au plus). Le constructeur n'est **pas** hérité : une classe fille écrit son `creer` et appelle, si elle le veut, celui du parent par `Parent.creer(soi, …)`. `est_instance(x, "C")` est vrai si la classe de `x` est `C` ou en hérite.

```fa-test
classe Forme
    fonction creer(nom)
        soi.nom = nom
    fin
    fonction decrire()
        renvoyer "Forme " + soi.nom
    fin
fin
classe Cercle herite Forme
    fonction creer(r)
        Forme.creer(soi, "cercle")
        soi.r = r
    fin
    fonction aire()
        renvoyer 3 * soi.r * soi.r
    fin
fin
soit c = Cercle(2)
afficher c, c.aire(), c.decrire()
afficher est_instance(c, "Forme"), est_instance(c, "Cercle"), est_instance(Forme("x"), "Cercle")
```
```sortie
Cercle { nom: "cercle", r: 2 } 12 Forme cercle
vrai vrai faux
```

### 11b.2 Accès pointés

Un nom `a.b.c` s'écrit sans espace : c'est un seul identifiant (comme les espaces de noms de modules, §11). Quand ce nom exact n'existe pas, le moteur cherche le **plus long préfixe** qui est une variable, puis lit les champs restants. Les espaces de noms de modules gardent donc la priorité. Cela marche en lecture (`a.b.c`), en écriture (`a.b.c = v`, qui crée le champ s'il n'existe pas) et en appel (`a.b.m(args)`) sur toute structure, instance ou non.

Quand `a.b.m(args)` est appelé : si l'objet a un **champ** `m`, c'est lui (il doit être une fonction) ; sinon, pour une instance, la méthode `m` de sa classe ou d'un ancêtre ; sinon c'est une erreur.

```fa-test
classe Point
    fonction creer(x)
        soi.x = x
    fin
    fonction norme()
        renvoyer soi.x
    fin
fin
soit p = Point(3)
soit s = {a: 1, f: x => x * 2}
s.b = 2
afficher s.a, s.b, s.f(21)
essayer afficher p.z attraper e afficher e fin
essayer p.norme(1) attraper e afficher e fin
essayer p.nope() attraper e afficher e fin
essayer
    soit n = 5
    n.x = 3
attraper e
    afficher e
fin
essayer afficher zz.foo attraper e afficher e fin
```
```sortie
1 2 42
Le champ "z" n'existe pas dans "p" (champs disponibles : x).
La méthode "norme" de la classe "Point" attend 0 argument(s), 1 reçu(s).
La méthode "nope" n'existe pas dans la classe "Point".
Impossible d'écrire le champ "x" : "n" est de type nombre (une structure ou une instance est attendue).
La variable "zz.foo" n'existe pas. Déclarez-la d'abord avec : soit zz.foo = ...
```

### 11b.3 Enchaînements

Après un appel ou un index, un point enchaîne : `Compte(1).deposer(2).solde`, `f().a.b`, `(x + 1).champ`. Le point doit suivre **immédiatement** la parenthèse ou le crochet fermant. Chaque maillon lit un champ ou appelle une méthode, avec les mêmes règles et les mêmes erreurs qu'en §11b.2 ; dans les messages, la valeur intermédiaire est nommée `(expression)`.

```fa-test
classe Compte
    fonction creer(s)
        soi.s = s
    fin
    fonction plus(m)
        soi.s = soi.s + m
        renvoyer soi
    fin
    fonction val()
        renvoyer soi.s
    fin
fin
fonction f()
    renvoyer {a: {b: 7}}
fin
afficher Compte(1).plus(2).plus(3).val()
afficher Compte(1).plus(1).s
afficher f().a.b
essayer afficher f().z attraper e afficher e fin
```
```sortie
6
2
7
Le champ "z" n'existe pas dans "(expression)" (champs disponibles : a).
```

### 11b.4 Erreurs d'analyse

- Autre chose qu'une méthode dans une classe : `Dans une classe, seules les méthodes (fonction ... fin) sont permises (reçu : …).`
- Méthode définie deux fois : `La méthode "m" est déjà définie dans la classe "C".`
- `soi` dans les paramètres : `"soi" est réservé dans une méthode : il désigne déjà l'objet courant.`
- `fin` manquant : `Le bloc "classe C" ouvert ligne N n'est jamais fermé : il manque "fin".`

## 11c. Tests intégrés : `tester`

`tester "nom" … fin` (où `tester` n'est pas non plus un mot réservé : il faut un texte juste après) déclare un test. Une exécution normale **ne l'exécute pas** : le bloc est une fonction sans paramètre, rangée sous le nom `#test:nom`. Le lanceur (`tester` dans le terminal de l'IDE, `fadera tester fichier.fa`, `lancerTests` côté JavaScript) exécute d'abord le programme entier, puis chaque test du niveau supérieur dans l'ordre du source, et en rapporte le résultat. Les variables d'un test lui sont locales.

Natives d'assertion (échec = erreur `Échec du test : …`) : `verifier(cond [, message])`, `verifier_egal(obtenu, attendu [, message])`, `verifier_erreur(fonction [, texte])` — la fonction, appelée sans argument, doit lever une erreur ; si `texte` est donné, le message de l'erreur doit le **contenir**.

```fa-test
fonction double(x)
    renvoyer x * 2
fin
tester "le double de 4"
    verifier_egal(double(4), 8)
fin
tester "ne s'exécute jamais en exécution normale"
    verifier(faux)
fin
afficher "programme terminé"
afficher verifier_erreur(() => 1 / 0, "zéro")
```
```sortie
programme terminé
rien
```

Rapport du lanceur (une ligne par test, puis le bilan) :

```
✓ le double de 4
✗ ne s'exécute jamais en exécution normale
    Ligne 8 : Échec du test.
2 test(s) : 1 réussi(s), 1 échoué(s).
```

Une erreur **hors** d'un test (pendant l'exécution du programme) interrompt tout, comme d'habitude. Sans aucun test : `Aucun test (écrivez : tester "nom" ... fin).`

## 11d. Bibliothèque standard et `lever`

Les modules `std/dates`, `std/dictionnaire`, `std/ensemble`, `std/file`, `std/json`, `std/listes`, `std/maths`, `std/pile`, `std/regex`, `std/tas`, `std/textes` et `std/tri` sont écrits en Fadéra ; les deux moteurs embarquent les mêmes sources. Les structures de données (`Pile`, `File`, `Dictionnaire`, `Ensemble`, `Tas`) sont des classes (§11b). Un dictionnaire ordonné garde l'ordre d'insertion de ses clés, comparées par leur texte.

Une suite de 12 tests (`tests_std.fa`, un bloc `tester` par module) vérifie cette bibliothèque ; l'IDE la lance depuis Applications → « Tests de la bibliothèque standard ».

`lever(message)` interrompt le programme par une erreur d'exécution portant `message` ; elle se rattrape comme toute erreur (§10.5).

```fa-test
importer "std/pile"
importer "std/dictionnaire"
soit p = Pile()
p.empiler(1).empiler(2)
afficher p.depiler()
soit d = Dictionnaire()
d.definir("z", 1).definir(10, "dix")
afficher d.liste_cles()
essayer
  p.depiler()
  p.depiler()
attraper e
  afficher e
fin
```
```sortie
2
["z", "10"]
[module std/pile.fa, ligne 22] Pile vide : rien à dépiler.
```

---

## 12. Erreurs

### 12.1 Deux familles
- **Erreurs de syntaxe** : détectées **avant** toute exécution (le programme n'affiche rien). Elles ne sont pas attrapables.
- **Erreurs d'exécution** : détectées pendant l'exécution, après les affichages déjà faits. Elles sont attrapables par `essayer`/`attraper`.

```fa-test
afficher 1
afficher "abc
```
```sortie
ERR:Ligne 2 : Texte non fermé : il manque le guillemet de fermeture.
```

```fa-test
afficher 1
afficher 1 / 0
```
```sortie
1
ERR:Ligne 2 : Division par zéro.
```

### 12.2 Forme d'un message
Un message commence par `Ligne N :` (numéro de la ligne du script où l'erreur est détectée) et est rédigé en français. Une erreur dans un module est rapportée à la ligne du script **qui a appelé**, avec le module et la ligne d'origine entre crochets : `Ligne 2 : [module outils.fa, ligne 9] …`.

Les moteurs ajoutent à cela la colonne, un extrait souligné, une suggestion (« Tu voulais écrire … ? ») et la pile d'appels ; ce sont des **présentations** qui ne font pas partie du message ci-dessus.

### 12.3 Quelques messages normatifs
```fa-test
afficher lireElement([1, 2, 3], 3)
```
```sortie
ERR:Ligne 1 : Index 3 hors limites (indices valides : 0 à 2).
```

```fa-test
afficher lireElement([1, 2, 3], 1.5)
```
```sortie
ERR:Ligne 1 : L'index doit être un nombre entier (reçu : 1.5).
```

```fa-test
afficher lireCle({a: 1}, "b")
```
```sortie
ERR:Ligne 1 : La clé "b" n'existe pas dans la structure (clés disponibles : a).
```

```fa-test
afficher taille(5)
```
```sortie
ERR:Ligne 1 : taille attend une liste, un texte ou une structure (reçu : nombre).
```

```fa-test
afficher a
```
```sortie
ERR:Ligne 1 : La variable "a" n'existe pas. Déclarez-la d'abord avec : soit a = ...
```

---

## 13. Indexation

Les indices commencent à **0**. Un indice négatif ou ≥ la taille est une erreur : il n'y a **pas** d'indice négatif « depuis la fin ». L'indice doit être un entier. `lireElement` fonctionne sur les listes et sur les textes (le résultat est un texte d'un caractère).

```fa-test
afficher lireElement([10, 20, 30], 0)
afficher lireElement("abc", 1)
afficher lireElement([1, 2, 3], -1)
```
```sortie
10
b
ERR:Ligne 3 : Index -1 hors limites (indices valides : 0 à 2).
```

---

## 14. Ce que cette spécification ne fixe pas

- Le résultat de `hasard`, de la date/heure, des fonctions réseau et du chiffrement aléatoire (les formats, eux, sont fixés par la documentation de chaque fonction).
- Les valeurs des fonctions mathématiques **hors** des domaines d'arrondi correct du §5.4 (angles énormes, exponentielles proches du dépassement) : elles peuvent différer au dernier chiffre entre moteurs.
- La mise en forme des messages d'erreur détaillés (colonne, extrait, pile).

## 15. Points de conception à trancher


Des comportements **actuels et identiques** dans les deux moteurs, mais que la spécification signale comme discutables, pour qu'un changement futur soit décidé en connaissance de cause plutôt que découvert par accident :

1. La liste vide `[]` est **vraie** (alors que `0` et `""` sont faux).
2. Pas de portée de bloc : un `soit` dans un `si` fuit dans la portée englobante.
3. Les fonctions nommées imbriquées ne capturent pas les variables locales (§9.3).
4. `1 < 2 < 3` est une erreur au lieu d'une comparaison chaînée.
5. `renvoyer` au niveau du script arrête le programme sans le dire, et `renvoyer` sans valeur n'est permis que juste avant `fin`, `sinon`, `cas`, `defaut`, `attraper` ou la fin du programme (§9.5).
6. Pas d'indice négatif (`lireElement(l, -1)`).
7. Les comparaisons de textes se font par unités UTF-16, pas par points de code ni par ordre alphabétique localisé.

# Apprendre Fadéra pas à pas

Fadéra est un langage de programmation **écrit en français** : on y lit `si … alors … sinon … fin`, `pour chaque … dans … faire`, `afficher`. Ce tutoriel vous mène de votre premier programme jusqu'à des jeux, des fichiers et des appels réseau. Aucune connaissance préalable n'est nécessaire.

> Tous les résultats montrés ci-dessous sont **les vrais résultats** : ce fichier est généré, chaque exemple est exécuté avant d'être publié. Les 30 programmes complets du dossier [`exemples/`](exemples/) sont vérifiés de la même façon.

## Chapitre 0 — Comment lancer un programme

Vous avez deux façons d'exécuter du Fadéra, et les deux donnent **exactement les mêmes résultats**.

1. **Dans le navigateur** : ouvrez le playground `https://faderaseny.github.io/fadera/`, tapez votre programme à gauche, cliquez sur ▶ Exécuter le script. Le bouton « 🔗 Partager (lien) » fabrique un lien que vous pouvez envoyer à quelqu'un : il ouvrira votre programme tel quel.
2. **Sur votre ordinateur** : avec `fadera.exe` (Windows) ou `fadera` (Linux). Enregistrez votre programme dans un fichier qui se termine par `.fa`, par exemple `bonjour.fa`, puis tapez dans un terminal :

```text
fadera run bonjour.fa
```

Quelques autres commandes utiles : `fadera fmt bonjour.fa` (met le code en forme), `fadera tester bonjour.fa` (lance les tests du fichier), `fadera deboguer bonjour.fa` (pas à pas) et `fadera` seul pour une session interactive où l'on tape une ligne et voit le résultat tout de suite.

## Chapitre 1 — Bonjour le monde

Un programme est une suite d'instructions, exécutées de haut en bas. `afficher` écrit quelque chose à l'écran.

```fadera
afficher "Bonjour le monde !"
afficher "Je m'appelle Fadéra."
```

Résultat :

```
Bonjour le monde !
Je m'appelle Fadéra.
```

Tout ce qui suit un `#` est un **commentaire** : l'ordinateur l'ignore, il sert aux humains.

```fadera
# Ceci est un commentaire
afficher "Visible"   # celui-ci est à la fin d'une ligne
```

Résultat :

```
Visible
```

On peut afficher des nombres et même faire des calculs :

```fadera
afficher 42
afficher 6 * 7
afficher 10 / 4
```

Résultat :

```
42
42
2.5
```

## Chapitre 2 — Des variables pour retenir des valeurs

Une **variable** est une boîte qui porte un nom. On la crée avec `soit`.

```fadera
soit prenom = "Awa"
soit age = 20
afficher prenom
afficher age
```

Résultat :

```
Awa
20
```

Pour changer sa valeur plus tard, on écrit son nom **sans** `soit` :

```fadera
soit score = 0
score = score + 10
score = score + 5
afficher score
```

Résultat :

```
15
```

Avec `+`, un texte et un nombre se collent ensemble :

```fadera
soit nom = "Moussa"
soit annees = 17
afficher nom + " a " + annees + " ans."
```

Résultat :

```
Moussa a 17 ans.
```

Il existe cinq sortes de valeurs, qu'on découvrira au fil des chapitres : les **nombres** (`42`, `3.14`), les **textes** (`"bonjour"`), les **booléens** (`vrai`, `faux`), les **listes** (`[1, 2, 3]`) et les **structures** (`{ nom: "Awa" }`). La fonction `type_de` donne le type d'une valeur :

```fadera
afficher type_de(42)
afficher type_de("bonjour")
afficher type_de(vrai)
afficher type_de([1, 2])
afficher type_de({ a: 1 })
```

Résultat :

```
nombre
texte
booléen
liste
structure
```

> **Attention aux noms réservés.** Certains mots appartiennent au langage (`soit`, `si`, `fin`, `liste`, `taille`, `pour`, `vrai`…) : on ne peut pas s'en servir comme noms de variables. Écrire `soit liste = 3` provoque une erreur. Choisissez `ma_liste`, `notes`, etc. La liste complète est dans le [guide](../GUIDE_FADERA.md).

## Chapitre 3 — Calculer

Les opérations habituelles : `+ - * /`, et `%` qui donne le **reste de la division** (très utile pour savoir si un nombre est pair).

```fadera
afficher 7 + 3
afficher 7 - 3
afficher 7 * 3
afficher 7 / 2
afficher 7 % 2
afficher 2 + 3 * 4
afficher (2 + 3) * 4
```

Résultat :

```
10
4
21
3.5
1
14
20
```

Quelques fonctions mathématiques toutes prêtes :

```fadera
afficher racine(81)
afficher puissance(2, 8)
afficher absolu(-12)
afficher arrondir(3.14159, 2)
afficher plancher(7.9)
afficher minimum(4, 9)
afficher maximum(4, 9)
```

Résultat :

```
9
256
12
3.14
7
4
9
```

Pour convertir entre texte et nombre : `nombre("12")` et `texte(12)`.

```fadera
soit saisie = "12"
afficher nombre(saisie) + 1
afficher saisie + 1
```

Résultat :

```
13
121
```

Dans le premier cas c'est une addition (13), dans le second les deux morceaux de texte sont collés (« 121 »).

## Chapitre 4 — Prendre des décisions

`si … alors … sinon … fin` exécute un bloc ou un autre selon une condition.

```fadera
soit temperature = 31
si temperature > 30 alors
  afficher "Il fait chaud !"
sinon
  afficher "Température agréable."
fin
```

Résultat :

```
Il fait chaud !
```

On compare avec `==` (égal), `!=` (différent), `<`, `>`, `<=`, `>=`. On combine les conditions avec `et`, `ou`, `non`. Pour plusieurs cas, on enchaîne des `sinon si`.

```fadera
soit note = 14
si note >= 16 alors
  afficher "Très bien"
sinon si note >= 12 alors
  afficher "Bien"
sinon si note >= 10 alors
  afficher "Passable"
sinon
  afficher "Insuffisant"
fin

soit age = 25
soit a_un_billet = vrai
si age >= 18 et a_un_billet alors
  afficher "Entrée autorisée"
fin
```

Résultat :

```
Bien
Entrée autorisée
```

Quand on teste **une même valeur** contre plusieurs possibilités, `selon` est plus lisible :

```fadera
soit jour = 6
selon jour faire
  cas 1, 2, 3, 4, 5 alors
    afficher "Semaine"
  cas 6, 7 alors
    afficher "Week-end"
  defaut
    afficher "Jour inconnu"
fin
```

Résultat :

```
Week-end
```

## Chapitre 5 — Répéter des actions

**`pour … allantde … a …`** compte d'un nombre à un autre (bornes incluses). `pas` change l'incrément.

```fadera
pour i allantde 1 a 5 faire
  afficher "Tour numéro " + i
fin

pour i allantde 10 a 0 pas -5 faire
  afficher i
fin
```

Résultat :

```
Tour numéro 1
Tour numéro 2
Tour numéro 3
Tour numéro 4
Tour numéro 5
10
5
0
```

**`tantque`** répète tant qu'une condition est vraie. À vous de faire changer la condition, sinon la boucle ne s'arrête jamais !

```fadera
soit n = 1
tantque n < 100 faire
  n = n * 2
fin
afficher n
```

Résultat :

```
128
```

`rompre` sort de la boucle immédiatement ; `continuer` saute au tour suivant.

```fadera
pour i allantde 1 a 10 faire
  si i % 2 == 0 alors
    continuer
  fin
  si i > 7 alors
    rompre
  fin
  afficher i
fin
```

Résultat :

```
1
3
5
7
```

Une table de multiplication :

```fadera
pour i allantde 1 a 10 faire
  afficher "7 x " + i + " = " + (7 * i)
fin
```

Résultat :

```
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
7 x 6 = 42
7 x 7 = 49
7 x 8 = 56
7 x 9 = 63
7 x 10 = 70
```

## Chapitre 6 — Les listes

Une **liste** range plusieurs valeurs dans l'ordre. Les positions commencent à **0**.

```fadera
soit fruits = ["mangue", "banane", "orange"]
afficher taille(fruits)
afficher lireElement(fruits, 0)
afficher lireElement(fruits, 2)
```

Résultat :

```
3
mangue
orange
```

On ajoute avec `ajouter`, on remplace avec `modifierElement` :

```fadera
soit courses = ["riz", "huile"]
ajouter(courses, "sel")
modifierElement(courses, 0, "riz parfumé")
afficher courses
afficher taille(courses)
```

Résultat :

```
["riz parfumé", "huile", "sel"]
3
```

Pour parcourir une liste, on utilise `pour chaque` (et **non** `pour x dans`) :

```fadera
soit notes = [12, 15, 9, 18]
soit total = 0
pour chaque n dans notes faire
  total = total + n
fin
afficher "Moyenne : " + total / taille(notes)
```

Résultat :

```
Moyenne : 13.5
```

Beaucoup de fonctions travaillent sur les listes :

```fadera
soit nombres = [5, 3, 8, 1]
afficher trier(nombres, (a, b) => a - b)
afficher inverser(nombres)
afficher stats_max(nombres)
afficher joindre(nombres, " < ")
afficher plage(1, 10, 3)
```

Résultat :

```
[1, 3, 5, 8]
[1, 8, 3, 5]
8
5 < 3 < 8 < 1
[1, 4, 7, 10]
```

## Chapitre 7 — Les structures

Une **structure** regroupe des informations nommées : une fiche. On la lit avec `lireCle`, on la modifie avec `modifierCle`.

```fadera
soit eleve = { nom: "Fatou", classe: "3e B", moyenne: 15.5 }
afficher lireCle(eleve, "nom")
modifierCle(eleve, "moyenne", 16)
modifierCle(eleve, "delegue", vrai)
afficher eleve
afficher a_cle(eleve, "classe")
afficher cles(eleve)
```

Résultat :

```
Fatou
{ nom: "Fatou", classe: "3e B", moyenne: 16, delegue: vrai }
vrai
["nom", "classe", "moyenne", "delegue"]
```

On les combine avec les listes pour fabriquer une petite base de données :

```fadera
soit eleves = [
  { nom: "Awa", note: 15 },
  { nom: "Moussa", note: 9 },
  { nom: "Fatou", note: 18 }
]
pour chaque e dans eleves faire
  afficher lireCle(e, "nom") + " : " + lireCle(e, "note")
fin
```

Résultat :

```
Awa : 15
Moussa : 9
Fatou : 18
```

## Chapitre 8 — Écrire ses propres fonctions

Une **fonction** donne un nom à une suite d'instructions réutilisable. `renvoyer` indique le résultat.

```fadera
fonction carre(x)
  renvoyer x * x
fin

afficher carre(7)
afficher carre(carre(2))
```

Résultat :

```
49
16
```

Une fonction peut avoir plusieurs paramètres, ou aucun :

```fadera
fonction saluer(prenom, politesse)
  renvoyer politesse + " " + prenom + " !"
fin

fonction bonjour()
  afficher "Bonjour à tous"
fin

afficher saluer("Awa", "Bonsoir")
bonjour()
```

Résultat :

```
Bonsoir Awa !
Bonjour à tous
```

Une fonction peut s'appeler elle-même : c'est la **récursivité**.

```fadera
fonction factorielle(n)
  si n <= 1 alors
    renvoyer 1
  fin
  renvoyer n * factorielle(n - 1)
fin
afficher factorielle(6)
```

Résultat :

```
720
```

Fadéra est strict : appeler une fonction avec le mauvais nombre d'arguments est une erreur, et le message dit exactement ce qui manque.

```fadera
fonction addition(a, b)
  renvoyer a + b
fin
afficher addition(1)
```

Résultat :

```
Erreur : Ligne 4 : La fonction "addition" attend 2 argument(s), 1 reçu(s).
```

### Fonctions anonymes

Une petite fonction sans nom s'écrit avec une flèche : `x => x * 2`. On la range dans une variable ou on la donne directement à une autre fonction.

```fadera
soit double = x => x * 2
afficher double(21)

soit nombres = [1, 2, 3, 4, 5, 6]
afficher filtrer(nombres, n => n % 2 == 0)
afficher transformer(nombres, n => n * n)
afficher reduire(nombres, (total, n) => total + n, 0)
afficher trouver(nombres, n => n > 3)
```

Résultat :

```
42
[2, 4, 6]
[1, 4, 9, 16, 25, 36]
21
4
```

`filtrer` garde les éléments qui passent un test, `transformer` change chaque élément, `reduire` les résume en une seule valeur. Pour trier des structures, `trier_par` prend une fonction qui donne **la clé de tri** :

```fadera
soit gens = [{ nom: "Awa", age: 34 }, { nom: "Ibra", age: 19 }, { nom: "Fatou", age: 41 }]
pour chaque g dans trier_par(gens, p => lireCle(p, "age")) faire
  afficher lireCle(g, "nom") + " (" + lireCle(g, "age") + ")"
fin
```

Résultat :

```
Ibra (19)
Awa (34)
Fatou (41)
```

## Chapitre 9 — Travailler avec du texte

```fadera
soit phrase = "  Fadéra est un langage français  "
soit propre = nettoyer(phrase)
afficher propre
afficher majuscule(propre)
afficher taille(propre)
afficher contient(propre, "langage")
afficher remplacer(propre, "français", "libre")
afficher decouper(propre, " ")
afficher sous_texte(propre, 0, 6)
```

Résultat :

```
Fadéra est un langage français
FADÉRA EST UN LANGAGE FRANÇAIS
30
vrai
Fadéra est un langage libre
["Fadéra", "est", "un", "langage", "français"]
Fadéra
```

Pour aligner des colonnes, on complète avec des espaces :

```fadera
pour chaque ligne dans [["Riz", 500], ["Huile", 1200], ["Sel", 150]] faire
  afficher texte_remplir_droite(lireElement(ligne, 0), 8, ".") + texte_remplir_gauche(texte(lireElement(ligne, 1)), 6, " ") + " F"
fin
```

Résultat :

```
Riz.....   500 F
Huile...  1200 F
Sel.....   150 F
```

Et `texte_format` remplit un modèle :

```fadera
afficher texte_format("{} a {} ans et habite à {}", ["Awa", 20, "Dakar"])
afficher texte_format("{nom} : {note}/20", { nom: "Moussa", note: 14 })
```

Résultat :

```
Awa a 20 ans et habite à Dakar
Moussa : 14/20
```

## Chapitre 10 — Quand ça se passe mal : les erreurs

Une erreur arrête le programme et affiche un message qui indique la ligne.

```fadera
soit x = 10
afficher x / 0 + inconnue
```

Résultat :

```
Erreur : Ligne 2 : Division par zéro.
```

On peut **prévoir** les problèmes avec `essayer … attraper … fin` : si une erreur survient dans le bloc `essayer`, le programme saute au bloc `attraper` au lieu de s'arrêter.

```fadera
essayer
  soit donnees = json_lire("{pas du json")
  afficher donnees
attraper erreur
  afficher "Le texte n'est pas du JSON valide."
fin
afficher "Le programme continue."
```

Résultat :

```
Le texte n'est pas du JSON valide.
Le programme continue.
```

Vous pouvez aussi **déclencher** une erreur vous-même avec `lever`, pour refuser une valeur absurde :

```fadera
fonction racine_carree(x)
  si x < 0 alors
    lever("Pas de racine réelle pour un nombre négatif.")
  fin
  renvoyer racine(x)
fin

essayer
  afficher racine_carree(25)
  afficher racine_carree(-4)
attraper erreur
  afficher "Erreur : " + erreur
fin
```

Résultat :

```
5
Erreur : Pas de racine réelle pour un nombre négatif.
```

## Chapitre 11 — Fichiers, CSV et JSON

`ecrireFichier` et `lireFichier` lisent et écrivent du texte. Avec la commande `fadera`, le fichier est créé dans le dossier courant ; dans le playground, il vit dans un espace de fichiers propre à la page.

```fadera
ecrireFichier("notes.txt", "Awa;15\nMoussa;9\nFatou;18")
soit contenu = lireFichier("notes.txt")
afficher contenu
```

Résultat :

```
Awa;15
Moussa;9
Fatou;18
```

Les tableurs échangent des fichiers **CSV** : `csv_lire` les transforme en listes, `csv_texte` fait l'inverse.

```fadera
ecrireFichier("notes.txt", "Awa;15\nMoussa;9\nFatou;18")
soit lignes = csv_lire(lireFichier("notes.txt"), ";")
soit total = 0
pour chaque l dans lignes faire
  total = total + nombre(lireElement(l, 1))
fin
afficher "Moyenne de la classe : " + arrondir(total / taille(lignes), 1)
```

Résultat :

```
Moyenne de la classe : 14
```

Les programmes et les sites web échangent du **JSON** : `json_lire` le transforme en listes et structures, `json_texte` fait l'inverse, `json_chemin` va chercher une valeur profonde sans risque d'erreur.

```fadera
soit config = json_lire("{\"ville\": \"Dakar\", \"habitants\": {\"2020\": 3.9, \"2025\": 4.4}}")
afficher json_chemin(config, "habitants.2025", 0)
afficher json_chemin(config, "habitants.1990", "inconnu")
modifierCle(config, "ville", "Thiès")
afficher json_texte(config)
```

Résultat :

```
4.4
inconnu
{"ville":"Thiès","habitants":{"2020":3.9,"2025":4.4}}
```

Le programme complet [`exemples/18_carnet_csv.fa`](exemples/18_carnet_csv.fa) lit un carnet d'adresses CSV, [`19_json_configuration.fa`](exemples/19_json_configuration.fa) manipule du JSON et [`20_journal_fichier.fa`](exemples/20_journal_fichier.fa) tient un journal de bord.

## Chapitre 12 — Le réseau : aller chercher des informations sur internet

`http_obtenir(adresse)` télécharge une page, `http_json(adresse)` télécharge et convertit du JSON, `http_envoyer(adresse, donnees)` envoie des données. Comme internet peut être coupé, on les entoure **toujours** d'un `essayer … attraper`.

```fadera
essayer
  soit reponse = http_json("https://api.open-meteo.com/v1/forecast?latitude=14.69&longitude=-17.44&current_weather=true")
  afficher "Température à Dakar : " + json_chemin(reponse, "current_weather.temperature", "?") + " °C"
attraper erreur
  afficher "Pas de connexion : " + erreur
fin
```

> Dans le playground, le navigateur applique une règle de sécurité (CORS) : seuls les sites qui autorisent les appels depuis une page web répondent. Avec la commande `fadera`, tous les sites sont accessibles.

Fadéra sait aussi calculer sur les réseaux : adresses IP, sous-réseaux, plus court chemin.

```fadera
soit sous_reseau = telecom_calculer_sous_reseau("192.168.1.130", 26)
afficher lireCle(sous_reseau, "adresse_reseau_exacte")
afficher lireCle(sous_reseau, "adresse_diffusion_broadcast")
afficher lireCle(sous_reseau, "nombre_hotes_utilisables")
```

Résultat :

```
192.168.1.128
192.168.1.191
62
```

Voir [`22_sous_reseau_ip.fa`](exemples/22_sous_reseau_ip.fa), [`23_routage_dijkstra.fa`](exemples/23_routage_dijkstra.fa), [`24_meteo_http_json.fa`](exemples/24_meteo_http_json.fa) et [`25_verificateur_sites.fa`](exemples/25_verificateur_sites.fa).

## Chapitre 13 — Les classes : des objets qui savent faire des choses

Une **classe** décrit un modèle d'objet : ses données et ses actions. `creer` s'exécute à la fabrication, `soi` désigne l'objet lui-même.

```fadera
classe Compteur
  fonction creer(depart)
    soi.valeur = depart
  fin
  fonction incrementer()
    soi.valeur = soi.valeur + 1
    renvoyer soi.valeur
  fin
fin

soit c = Compteur(10)
c.incrementer()
c.incrementer()
afficher c.valeur
```

Résultat :

```
12
```

Une classe peut **hériter** d'une autre : elle reprend ses actions et en ajoute. Le constructeur, lui, n'est pas hérité : la classe fille écrit son propre `creer` et appelle celui du parent avec `Parent.creer(soi, …)`.

```fadera
classe Animal
  fonction creer(nom)
    soi.nom = nom
  fin
  fonction presentation()
    renvoyer "Je suis " + soi.nom
  fin
fin

classe Chien herite Animal
  fonction creer(nom)
    Animal.creer(soi, nom)
  fin
  fonction aboyer()
    renvoyer soi.nom + " dit : Ouaf !"
  fin
fin

soit rex = Chien("Rex")
afficher rex.presentation()
afficher rex.aboyer()
afficher est_instance(rex, "Animal")
```

Résultat :

```
Je suis Rex
Rex dit : Ouaf !
vrai
```

Un exemple complet avec compte bancaire et compte épargne : [`26_compte_bancaire.fa`](exemples/26_compte_bancaire.fa).

## Chapitre 14 — Les modules : ne pas tout réécrire

`importer` charge des fonctions déjà écrites. Fadéra fournit 12 modules **standard** (`std/maths`, `std/dates`, `std/textes`, `std/listes`, `std/tri`, `std/pile`, `std/file`, `std/tas`, `std/ensemble`, `std/dictionnaire`, `std/json`, `std/regex`), décrits dans la [référence](REFERENCE.md#modules-standard).

```fadera
importer "std/maths"
importer "std/pile"

afficher fibonacci(12)
afficher premiers_jusqua(30)

soit p = Pile()
p.empiler("a").empiler("b").empiler("c")
afficher p.depiler()
afficher p.longueur()
```

Résultat :

```
144
[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
c
2
```

Pour organiser **votre** projet en plusieurs fichiers, mettez des fonctions dans `outils.fa` et chargez-les depuis `principal.fa` avec `importer "outils.fa"`.

```fadera
# outils.fa
fonction double(x)
  renvoyer x * 2
fin

# principal.fa
importer "outils.fa"
afficher double(21)
```

Si deux modules ont des fonctions de même nom, `importer "outils.fa" comme o` les range sous un préfixe : on écrit alors `o.double(21)`.

## Chapitre 15 — Tester ses programmes

Vous modifiez un programme, et soudain ce qui marchait ne marche plus. Les **tests** évitent cela : on décrit ce qui doit être vrai, et on le revérifie à volonté. Un bloc `tester "nom" … fin` contient des vérifications :

```fadera
fonction est_pair(n)
  renvoyer n % 2 == 0
fin

tester "parité"
  verifier(est_pair(4))
  verifier(non est_pair(7))
  verifier_egal(filtrer([1, 2, 3, 4], est_pair), [2, 4])
fin

tester "refus des textes"
  verifier_erreur(() => racine("abc"), "nombre")
fin
```

Lancez-les avec `fadera tester monfichier.fa` : chaque test affiche ✓ ou ✗ avec la raison. Le programme [`29_tests_integres.fa`](exemples/29_tests_integres.fa) en est un exemple complet.

## Chapitre 16 — Jouer

Fadéra permet d'écrire de vrais jeux, avec un hasard « répétable » (une graine) pour qu'ils soient testables. Les exemples [`09`](exemples/09_devine_le_nombre.fa) à [`15`](exemples/15_labyrinthe.fa) contiennent :

| Exemple | Ce qu'il montre |
|---|---|
| 09 Devine le nombre | fonctions passées en argument, recherche par dichotomie |
| 10 Pierre-feuille-ciseaux | `selon`, tournoi automatique |
| 11 Pendu | textes, listes de lettres |
| 12 Morpion | tableau à deux dimensions, détection de victoire |
| 13 Jeu de la vie | simulation, grilles |
| 14 Blackjack | structures, tirage de cartes |
| 15 Labyrinthe | recherche du plus court chemin (BFS) |

Pour jouer **au clavier** plutôt que contre l'ordinateur, remplacez les réponses automatiques par `saisir("Votre choix ? ")` :

```fadera
soit prenom = saisir("Comment vous appelez-vous ? ")
afficher "Enchanté, " + prenom + " !"
```

## Chapitre 17 — Les pièges classiques

Voici les erreurs que tout le monde fait au début, et comment les reconnaître.

**1. Utiliser un mot réservé comme nom.** `soit liste = [1, 2]` ou `soit fin = 3` échouent : ces mots appartiennent au langage. Écrivez `ma_liste`, `dernier`…

**2. Oublier `fin`.** Chaque `si`, `tantque`, `pour`, `fonction`, `classe`, `essayer`, `selon`, `tester` se ferme par un `fin`. Si le message parle de « fin du fichier », il manque un `fin` quelque part.

**3. Écrire `pour x dans liste`.** Le bon mot est `pour chaque x dans liste faire`.

**4. Se tromper de nombre d'arguments.** Une fonction à deux paramètres s'appelle avec exactement deux arguments.

**5. Compter à partir de 1.** Le premier élément d'une liste est à la position **0**, le dernier à la position `taille(liste) - 1`.

**6. Utiliser `=` pour comparer.** `=` range une valeur (`x = 5`), `==` compare (`si x == 5 alors`).

**7. Boucle infinie.** Une boucle `tantque` dont la condition ne change jamais ne s'arrête pas : dans le navigateur, le bouton « Arrêter » (ou Maj+F5) l'interrompt ; dans un terminal, `Ctrl + C`.

## Pour aller plus loin

- La [**référence**](REFERENCE.md) liste toutes les fonctions avec un exemple vérifié pour chacune.
- Le [**guide**](../GUIDE_FADERA.md) détaille la grammaire complète du langage.
- Les [**30 exemples**](exemples/) sont des programmes complets, de 5 à 80 lignes, classés du plus simple au plus riche.
- La [**spécification**](../SPEC_FADERA.md) décrit précisément le comportement du langage, pour qui voudrait écrire un autre moteur.

Bonne programmation !

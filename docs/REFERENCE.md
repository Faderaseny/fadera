# Référence des fonctions Fadéra

> Ce fichier est **généré** par `node docs/generer_reference.js` : chaque exemple a été exécuté sur les deux moteurs (navigateur et `fadera`) et le résultat affiché est le résultat réel. Ne le modifiez pas à la main : éditez `docs/reference/fonctions.txt`.

195 fonctions natives, 12 fonctions du langage, 12 modules standard. Pour une première approche, lisez plutôt le [tutoriel](TUTORIEL.md).

## Sommaire

- [Entrées, sorties et fichiers](#entrees-sorties-et-fichiers)
- [Listes et structures du langage](#listes-et-structures-du-langage)
- [Listes : outils](#listes-outils)
- [Nombres et mathématiques](#nombres-et-mathematiques)
- [Statistiques](#statistiques)
- [Textes](#textes)
- [Dates](#dates)
- [Réseau et télécommunications](#reseau-et-telecommunications)
- [Chiffrement](#chiffrement)
- [Intelligence artificielle](#intelligence-artificielle)
- [Tests](#tests)
- [Autres noms (écriture en majuscules internes)](#autres-noms-ecriture-en-majuscules-internes)
- [Modules standard (`importer "std/…"`)](#modules-standard)

## Entrées, sorties et fichiers

Ces fonctions font partie du langage : elles s'écrivent comme les autres (nom suivi de parenthèses).

**`afficher valeur`** — Écrit la valeur dans la console (mot-clé : sans parenthèses).

**`saisir(message)`** — Demande une réponse à l'utilisateur et la renvoie sous forme de texte.

**`hasard(min, max)`** — Entier aléatoire entre min et max inclus.

**`lireFichier(chemin)`** — Renvoie le contenu d'un fichier texte ; erreur s'il n'existe pas.

**`ecrireFichier(chemin, texte)`** — Écrit (remplace) le contenu d'un fichier texte.

**`creer_fichier(chemin, texte)`** — Crée un fichier dans l'espace de fichiers de l'IDE (le navigateur). Avec la commande fadera, utilisez ecrireFichier.

**`ecrire_log(chemin, ligne)`** — Ajoute une ligne horodatée à un journal dans l'IDE. Avec la commande fadera, utilisez ecrireFichier.

**`pause(ms)`** — Attend ms millisecondes (10 000 au maximum) et renvoie ms.

```fadera
afficher pause(0)
# → 0
```

**`chrono()`** — Heure actuelle en millisecondes ; sert à mesurer une durée en faisant la différence de deux appels.

**`maintenant()`** — Date et heure actuelles sous forme de nombre (millisecondes depuis 1970).

## Listes et structures du langage

**`liste(a, b, …)`** — Fabrique une liste à partir de ses arguments (équivalent de [a, b, …]).

```fadera
afficher liste(1, 2, 3)
# → [1, 2, 3]
```

**`taille(x)`** — Nombre d'éléments d'une liste, de caractères d'un texte ou de clés d'une structure.

```fadera
afficher taille([4, 5, 6])
# → 3
```

**`ajouter(liste, valeur)`** — Ajoute la valeur à la fin de la liste (modifie la liste).

```fadera
afficher taille(ajouter([1, 2], 3))
# → 3
```

**`lireElement(liste, position)`** — Élément à la position donnée (la première position est 0).

```fadera
afficher lireElement(["a", "b", "c"], 1)
# → b
```

**`modifierElement(liste, position, valeur)`** — Remplace l'élément à la position donnée (modifie la liste).

```fadera
afficher modifierElement([1, 2, 3], 0, 9)
# → [9, 2, 3]
```

**`lireCle(structure, cle)`** — Valeur associée à la clé d'une structure.

```fadera
afficher lireCle({ nom: "Awa" }, "nom")
# → Awa
```

**`modifierCle(structure, cle, valeur)`** — Crée ou remplace une clé d'une structure (modifie la structure).

```fadera
afficher modifierCle({ a: 1 }, "b", 2)
# → { a: 1, b: 2 }
```

**`a_cle(structure, cle)`** — Vrai si la structure possède cette clé.

```fadera
afficher a_cle({ a: 1 }, "a")
# → vrai
```

**`supprimer_cle(structure, cle)`** — Retire une clé d'une structure et renvoie la valeur retirée.

```fadera
afficher supprimer_cle({ a: 1, b: 2 }, "a")
# → 1
```

**`cles(structure)`** — Liste des clés.

```fadera
afficher cles({ a: 1, b: 2 })
# → ["a", "b"]
```

**`valeurs(structure)`** — Liste des valeurs.

```fadera
afficher valeurs({ a: 1, b: 2 })
# → [1, 2]
```

**`paires(structure)`** — Liste de paires [clé, valeur].

```fadera
afficher paires({ a: 1, b: 2 })
# → [["a", 1], ["b", 2]]
```

**`structure_depuis_paires(paires)`** — Reconstruit une structure à partir d'une liste de paires [clé, valeur].

```fadera
afficher structure_depuis_paires([["a", 1], ["b", 2]])
# → { a: 1, b: 2 }
```

**`fusionner(s1, s2)`** — Nouvelle structure réunissant les clés de s1 et s2 (s2 l'emporte).

```fadera
afficher fusionner({ a: 1, b: 2 }, { b: 9, c: 3 })
# → { a: 1, b: 9, c: 3 }
```

**`copier(valeur)`** — Copie profonde : modifier la copie ne change pas l'original.

```fadera
afficher copier([1, [2, 3]])
# → [1, [2, 3]]
```

**`type_de(valeur)`** — Nom du type : nombre, texte, booléen, liste, structure, fonction…

```fadera
afficher type_de([1])
# → liste
```

**`est_nombre(valeur)`** — Vrai si le texte ou la valeur représente un nombre.

```fadera
afficher est_nombre("12.5")
# → vrai
```

**`classe_de(objet)`** — Nom de la classe d'un objet.

**`est_instance(objet, "Classe")`** — Vrai si l'objet est de cette classe ou d'une classe dérivée.

**`lever(message)`** — Déclenche une erreur que essayer / attraper peut intercepter.

## Listes : outils

**`plage(debut, fin, pas)`** — Liste de nombres de debut à fin inclus, par pas (1 par défaut).

```fadera
afficher plage(1, 9, 2)
# → [1, 3, 5, 7, 9]
```

**`tranche(liste, debut, fin)`** — Sous-liste de la position debut (incluse) à fin (exclue).

```fadera
afficher tranche([1, 2, 3, 4, 5], 1, 3)
# → [2, 3]
```

**`inserer(liste, position, valeur)`** — Insère la valeur à la position (modifie la liste).

```fadera
afficher inserer([1, 3], 1, 2)
# → [1, 2, 3]
```

**`retirer(liste, position)`** — Retire l'élément à la position et le renvoie (modifie la liste).

```fadera
afficher retirer([7, 8, 9], 0)
# → 7
```

**`concatener(l1, l2)`** — Nouvelle liste : l1 suivie de l2.

```fadera
afficher concatener([1, 2], [3])
# → [1, 2, 3]
```

**`aplatir(liste)`** — Met à plat les listes imbriquées.

```fadera
afficher aplatir([[1], [2, [3]]])
# → [1, 2, 3]
```

**`unique(liste)`** — Liste sans doublons, dans l'ordre d'apparition.

```fadera
afficher unique([1, 1, 2, 1, 3])
# → [1, 2, 3]
```

**`compter(liste, valeur)`** — Combien de fois la valeur apparaît.

```fadera
afficher compter([1, 2, 1, 1], 1)
# → 3
```

**`inverser(x)`** — Inverse l'ordre d'une liste ou d'un texte.

```fadera
afficher inverser("abc")
# → cba
```

**`trier(liste, comparer)`** — Nouvelle liste triée ; comparer(a, b) renvoie un nombre négatif, nul ou positif.

```fadera
afficher trier([3, 1, 2], (a, b) => a - b)
# → [1, 2, 3]
```

**`trier_par(liste, cle)`** — Nouvelle liste triée selon la valeur renvoyée par la fonction cle(x) pour chaque élément.

```fadera
afficher trier_par(["ccc", "a", "bb"], x => taille(x))
# → ["a", "bb", "ccc"]
```

**`filtrer(liste, test)`** — Garde les éléments pour lesquels test(x) est vrai.

```fadera
afficher filtrer([1, 2, 3, 4], x => x % 2 == 0)
# → [2, 4]
```

**`transformer(liste, f)`** — Applique f à chaque élément.

```fadera
afficher transformer([1, 2, 3], x => x * x)
# → [1, 4, 9]
```

**`reduire(liste, f, depart)`** — Combine les éléments en une seule valeur avec f(cumul, x).

```fadera
afficher reduire([1, 2, 3, 4], (t, x) => t + x, 0)
# → 10
```

**`trouver(liste, test)`** — Premier élément pour lequel test(x) est vrai.

```fadera
afficher trouver([1, 2, 3], x => x > 1)
# → 2
```

**`tous(liste, test)`** — Vrai si test(x) est vrai pour tous les éléments.

```fadera
afficher tous([2, 4], x => x % 2 == 0)
# → vrai
```

**`existe(liste, test)`** — Vrai si test(x) est vrai pour au moins un élément.

```fadera
afficher existe([1, 2], x => x > 1)
# → vrai
```

**`appliquer(f, arguments)`** — Appelle la fonction f avec les éléments de la liste comme arguments.

```fadera
afficher appliquer((a, b) => a + b, [3, 4])
# → 7
```

**`liste_decouper(liste, taille)`** — Découpe en morceaux de la taille donnée.

```fadera
afficher liste_decouper([1, 2, 3, 4, 5], 2)
# → [[1, 2], [3, 4], [5]]
```

**`liste_zip(l1, l2)`** — Associe les éléments deux à deux.

```fadera
afficher liste_zip([1, 2], ["a", "b"])
# → [[1, "a"], [2, "b"]]
```

**`liste_cumul(liste)`** — Sommes cumulées.

```fadera
afficher liste_cumul([1, 2, 3, 4])
# → [1, 3, 6, 10]
```

**`liste_frequences(liste)`** — Structure qui compte les occurrences de chaque valeur.

```fadera
afficher liste_frequences(["a", "b", "a"])
# → { a: 2, b: 1 }
```

**`alea_choisir(liste)`** — Un élément au hasard.

**`alea_melanger(liste)`** — Nouvelle liste mélangée au hasard.

**`alea_entier(min, max)`** — Entier au hasard entre min et max inclus.

**`alea_uuid()`** — Identifiant unique aléatoire (UUID).

## Nombres et mathématiques

**`nombre(x)`** — Convertit un texte en nombre.

```fadera
afficher nombre("42") + 1
# → 43
```

**`texte(x)`** — Convertit n'importe quelle valeur en texte.

```fadera
afficher texte(42) + "!"
# → 42!
```

**`arrondir(x, decimales)`** — Arrondit à un nombre de décimales (0 par défaut).

```fadera
afficher arrondir(3.14159, 2)
# → 3.14
```

**`plancher(x)`** — Plus grand entier inférieur ou égal.

```fadera
afficher plancher(3.9)
# → 3
```

**`plafond(x)`** — Plus petit entier supérieur ou égal.

```fadera
afficher plafond(3.1)
# → 4
```

**`entier(x)`** — Partie entière (coupe les décimales).

```fadera
afficher entier(-3.9)
# → -3
```

**`racine(x)`** — Racine carrée.

```fadera
afficher racine(16)
# → 4
```

**`absolu(x)`** — Valeur absolue.

```fadera
afficher absolu(-5)
# → 5
```

**`puissance(a, b)`** — a puissance b.

```fadera
afficher puissance(2, 10)
# → 1024
```

**`minimum(a, b)`** — Le plus petit des deux.

```fadera
afficher minimum(3, 8)
# → 3
```

**`maximum(a, b)`** — Le plus grand des deux.

```fadera
afficher maximum(3, 8)
# → 8
```

**`signe(x)`** — -1, 0 ou 1 selon le signe.

```fadera
afficher signe(-7)
# → -1
```

**`modulo(a, b)`** — Reste de la division, toujours positif pour b positif.

```fadera
afficher modulo(-1, 5)
# → 4
```

**`pi()`** — Le nombre π.

```fadera
afficher arrondir(pi(), 5)
# → 3.14159
```

**`sinus(x)`** — Sinus (x en radians).

```fadera
afficher arrondir(sinus(pi() / 2), 3)
# → 1
```

**`cosinus(x)`** — Cosinus (x en radians).

```fadera
afficher arrondir(cosinus(0), 3)
# → 1
```

**`tangente(x)`** — Tangente (x en radians).

```fadera
afficher arrondir(tangente(pi() / 4), 3)
# → 1
```

**`arc_sinus(x)`** — Arc sinus (résultat en radians).

```fadera
afficher arrondir(arc_sinus(1), 4)
# → 1.5708
```

**`arc_cosinus(x)`** — Arc cosinus (résultat en radians).

```fadera
afficher arrondir(arc_cosinus(1), 4)
# → 0
```

**`arc_tangente(x)`** — Arc tangente (résultat en radians).

```fadera
afficher arrondir(arc_tangente(1), 4)
# → 0.7854
```

**`arc_tangente2(y, x)`** — Angle du point (x, y) en radians.

```fadera
afficher arrondir(arc_tangente2(1, 1), 4)
# → 0.7854
```

**`exponentielle(x)`** — e puissance x.

```fadera
afficher arrondir(exponentielle(1), 4)
# → 2.7183
```

**`logarithme(x, base)`** — Logarithme ; népérien si la base est omise.

```fadera
afficher logarithme(8, 2)
# → 3
```

**`logarithme10(x)`** — Logarithme décimal.

```fadera
afficher logarithme10(1000)
# → 3
```

**`radians(degres)`** — Convertit des degrés en radians.

```fadera
afficher arrondir(radians(180), 4)
# → 3.1416
```

**`degres(radians)`** — Convertit des radians en degrés.

```fadera
afficher degres(pi())
# → 180
```

**`en_base(n, base)`** — Écrit un entier dans une base de 2 à 36.

```fadera
afficher en_base(255, 2)
# → 11111111
```

**`depuis_base(texte, base)`** — Lit un entier écrit dans une base de 2 à 36.

```fadera
afficher depuis_base("ff", 16)
# → 255
```

**`math_pgcd_ppcm(a, b)`** — Structure { pgcd, ppcm }.

```fadera
afficher math_pgcd_ppcm(12, 18)
# → { pgcd: 6, ppcm: 36 }
```

**`math_facteurs_premiers(n)`** — Décomposition en facteurs premiers.

```fadera
afficher math_facteurs_premiers(60)
# → [2, 2, 3, 5]
```

**`math_resoudre_second_degre(a, b, c)`** — Racines de ax² + bx + c = 0.

```fadera
afficher math_resoudre_second_degre(1, -3, 2)
# → [1, 2]
```

**`math_integrale_trapeze(valeurs, dx)`** — Intégrale approchée (méthode des trapèzes) à partir de valeurs régulièrement espacées de dx.

```fadera
afficher math_integrale_trapeze([0, 1, 4, 9], 1)
# → 9.5
```

**`math_derivee_centree(valeurs, position)`** — Dérivée numérique centrée en un point (valeurs espacées de 1).

```fadera
afficher math_derivee_centree([0, 1, 4, 9, 16], 2)
# → 4
```

**`math_resoudre_systeme_gauss(matrice)`** — Résout un système linéaire donné par sa matrice augmentée.

```fadera
afficher math_resoudre_systeme_gauss([[2, 1, 5], [1, -1, 1]])
# → [2, 1]
```

**`matrice_carree(m)`** — Vrai si la matrice est carrée.

```fadera
afficher matrice_carree([[1, 2], [3, 4]])
# → vrai
```

**`matrice_triangulaire(m)`** — Vrai si la matrice est triangulaire.

```fadera
afficher matrice_triangulaire([[1, 2], [0, 4]])
# → vrai
```

**`inverser_matrice(m)`** — Inverse d'une matrice 2×2.

```fadera
afficher inverser_matrice([[1, 2], [3, 4]])
# → [[-2, 1], [1.5, -0.5]]
```

## Statistiques

**`stats_somme(liste)`** — Somme des valeurs.

```fadera
afficher stats_somme([12, 15, 8])
# → 35
```

**`stats_moyenne(liste)`** — Moyenne arithmétique.

```fadera
afficher stats_moyenne([12, 15, 8, 20])
# → 13.75
```

**`stats_min(liste)`** — Plus petite valeur.

```fadera
afficher stats_min([12, 15, 8])
# → 8
```

**`stats_max(liste)`** — Plus grande valeur.

```fadera
afficher stats_max([12, 15, 8])
# → 15
```

**`stats_mediane(liste)`** — Médiane.

```fadera
afficher stats_mediane([12, 15, 8, 20])
# → 13.5
```

**`stats_mode(liste)`** — Liste des valeurs les plus fréquentes (plusieurs en cas d'égalité).

```fadera
afficher stats_mode([1, 2, 2, 3])
# → [2]
```

**`stats_variance(liste)`** — Variance.

```fadera
afficher stats_variance([2, 4, 4, 4, 5, 5, 7, 9])
# → 4
```

**`stats_ecart_type(liste)`** — Écart-type.

```fadera
afficher stats_ecart_type([2, 4, 4, 4, 5, 5, 7, 9])
# → 2
```

**`stats_analyse_distribution(liste)`** — Quartiles, écart interquartile et seuils de valeurs aberrantes.

```fadera
afficher stats_analyse_distribution([1, 2, 3, 4, 5, 6, 7, 8, 100])
# → { premier_quartile_q1: 3, troisieme_quartile_q3: 7, intervalle_interquartile_iqr: 4, seuil_anomalie_bas: -3, seuil_anomalie_haut: 13 }
```

**`stats_pearson_correlation(x, y)`** — Coefficient de corrélation entre deux listes (de -1 à 1).

```fadera
afficher stats_pearson_correlation([1, 2, 3], [2, 4, 6])
# → 1
```

**`stats_regression_lineaire(x, y)`** — Droite de régression : pente et ordonnée à l'origine.

```fadera
afficher stats_regression_lineaire([1, 2, 3], [3, 5, 7])
# → { formule: "y = 2.0000 * x + 1.0000", pente: 2, ordonnee_origine: 1 }
```

**`stats_supprimer_doublons(liste)`** — Enlève les valeurs en double.

```fadera
afficher stats_supprimer_doublons([1, 2, 2, 3])
# → [1, 2, 3]
```

**`stats_nettoyer_valeurs_manquantes(liste, strategie)`** — Traite les valeurs manquantes : "supprimer", "moyenne" ou "mediane".

```fadera
afficher stats_nettoyer_valeurs_manquantes([1, 2, 3], "supprimer")
# → [1, 2, 3]
```

**`stats_normalisation_ia(liste)`** — Ramène les valeurs entre 0 et 1.

```fadera
afficher stats_normalisation_ia([10, 20, 30])
# → [0, 0.5, 1]
```

**`stats_query_engine(donnees, options)`** — Filtre, trie et sélectionne dans une liste de structures (voir le guide, section 13).

```fadera
afficher stats_query_engine([{ n: 1 }, { n: 5 }], { filtres: [{ champ: "n", operateur: ">", valeur: 2 }] })
# → [{ n: 5 }]
```

**`somme(liste)`** — Autre nom court pour stats_somme.

```fadera
afficher somme([1, 2, 3])
# → 6
```

**`moyenne(liste)`** — Autre nom court pour stats_moyenne.

```fadera
afficher moyenne([1, 2, 3])
# → 2
```

## Textes

**`majuscule(t)`** — Met en majuscules.

```fadera
afficher majuscule("bonjour")
# → BONJOUR
```

**`minuscule(t)`** — Met en minuscules.

```fadera
afficher minuscule("BONJOUR")
# → bonjour
```

**`nettoyer(t)`** — Retire les espaces au début et à la fin.

```fadera
afficher "[" + nettoyer("  salut  ") + "]"
# → [salut]
```

**`decouper(t, separateur)`** — Coupe un texte en liste (séparateur vide : lettre par lettre).

```fadera
afficher decouper("a,b,c", ",")
# → ["a", "b", "c"]
```

**`joindre(liste, separateur)`** — Assemble une liste en texte.

```fadera
afficher joindre(["a", "b", "c"], "-")
# → a-b-c
```

**`remplacer(t, ancien, nouveau)`** — Remplace toutes les occurrences.

```fadera
afficher remplacer("a-b-c", "-", "+")
# → a+b+c
```

**`contient(t, morceau)`** — Vrai si le texte (ou la liste) contient le morceau.

```fadera
afficher contient("bonjour", "jou")
# → vrai
```

**`commence_par(t, debut)`** — Vrai si le texte commence par debut.

```fadera
afficher commence_par("bonjour", "bon")
# → vrai
```

**`termine_par(t, fin)`** — Vrai si le texte se termine par fin.

```fadera
afficher termine_par("bonjour", "jour")
# → vrai
```

**`sous_texte(t, debut, fin)`** — Morceau du texte de debut (inclus) à fin (exclu).

```fadera
afficher sous_texte("bonjour", 0, 3)
# → bon
```

**`position(t, morceau)`** — Position de la première occurrence, ou -1.

```fadera
afficher position("bonjour", "j")
# → 3
```

**`repeter(t, n)`** — Répète le texte n fois.

```fadera
afficher repeter("ab", 3)
# → ababab
```

**`caractere(t, position)`** — Le caractère à la position donnée.

```fadera
afficher caractere("bonjour", 1)
# → o
```

**`code_caractere(c)`** — Code Unicode du caractère.

```fadera
afficher code_caractere("A")
# → 65
```

**`depuis_code(code)`** — Caractère correspondant à un code Unicode.

```fadera
afficher depuis_code(66)
# → B
```

**`texte_remplir_gauche(t, largeur, c)`** — Complète à gauche avec c jusqu'à la largeur.

```fadera
afficher texte_remplir_gauche("7", 3, "0")
# → 007
```

**`texte_remplir_droite(t, largeur, c)`** — Complète à droite avec c jusqu'à la largeur.

```fadera
afficher texte_remplir_droite("ab", 5, ".") + "|"
# → ab...|
```

**`texte_capitaliser(t)`** — Première lettre en majuscule.

```fadera
afficher texte_capitaliser("fadéra")
# → Fadéra
```

**`texte_titre(t)`** — Majuscule à chaque mot.

```fadera
afficher texte_titre("le petit prince")
# → Le Petit Prince
```

**`texte_lignes(t)`** — Découpe en lignes.

```fadera
afficher texte_lignes("a\nb\nc")
# → ["a", "b", "c"]
```

**`texte_mots(t)`** — Découpe en mots (la casse est conservée).

```fadera
afficher texte_mots("Le chat, le chien")
# → ["Le", "chat", "le", "chien"]
```

**`texte_sans_accents(t)`** — Retire les accents.

```fadera
afficher texte_sans_accents("éléphant à Thiès")
# → elephant a Thies
```

**`texte_slug(t)`** — Texte utilisable dans une adresse web : minuscules, sans accents, tirets.

```fadera
afficher texte_slug("Mon Premier Article !")
# → mon-premier-article
```

**`texte_est_palindrome(t)`** — Vrai si le texte se lit pareil dans les deux sens (en ignorant casse et ponctuation).

```fadera
afficher texte_est_palindrome("Esope reste ici et se repose")
# → vrai
```

**`texte_distance(a, b)`** — Distance de Levenshtein : nombre minimal de modifications pour passer de a à b.

```fadera
afficher texte_distance("chat", "chien")
# → 3
```

**`texte_format(modele, valeurs)`** — Remplit {} (dans l'ordre), {0} (par position) ou {nom} (liste ou structure).

```fadera
afficher texte_format("{} a {} ans", ["Awa", 20])
# → Awa a 20 ans
```

**`url_encoder(t)`** — Encode un texte pour une adresse web.

```fadera
afficher url_encoder("café & thé")
# → caf%C3%A9%20%26%20th%C3%A9
```

**`url_decoder(t)`** — Décode un texte d'adresse web.

```fadera
afficher url_decoder("caf%C3%A9%20%26%20th%C3%A9")
# → café & thé
```

**`regex_test(t, motif, options)`** — Vrai si le texte correspond au motif (expression régulière).

```fadera
afficher regex_test("abc123", "[0-9]+", "")
# → vrai
```

**`regex_trouver(t, motif, options)`** — Liste de toutes les correspondances.

```fadera
afficher regex_trouver("a1 b22 c333", "[0-9]+", "")
# → ["1", "22", "333"]
```

**`regex_remplacer(t, motif, nouveau, options)`** — Remplace les correspondances.

```fadera
afficher regex_remplacer("a1 b22", "[0-9]+", "#", "")
# → a# b#
```

**`regex_groupes(t, motif, options)`** — Valeurs des groupes entre parenthèses de la première correspondance.

```fadera
afficher regex_groupes("2026-10-24", "([0-9]+)-([0-9]+)-([0-9]+)", "")
# → ["2026-10-24", "2026", "10", "24"]
```

**`regex_decouper(t, motif, options)`** — Découpe le texte à chaque correspondance.

```fadera
afficher regex_decouper("a1b22c", "[0-9]+", "")
# → ["a", "b", "c"]
```

**`regex_echapper(t)`** — Neutralise les caractères spéciaux pour les chercher tels quels.

```fadera
afficher regex_echapper("1+1=2?")
# → 1\+1=2\?
```

**`csv_lire(t, separateur)`** — Transforme un texte CSV en liste de lignes (listes de textes).

```fadera
afficher csv_lire("a;b\n1;2", ";")
# → [["a", "b"], [1, 2]]
```

**`csv_texte(lignes, separateur)`** — Transforme une liste de lignes en texte CSV.

```fadera
afficher csv_texte([["a", "b"], [1, 2]], ";")
```

Affiche :

```
a;b
1;2
```

**`json_lire(t)`** — Transforme du texte JSON en listes et structures.

```fadera
afficher json_lire("{\"a\": [1, 2]}")
# → { a: [1, 2] }
```

**`json_texte(valeur, indentation)`** — Transforme une valeur en texte JSON (indentation facultative).

```fadera
afficher json_texte({ a: [1, 2] }, 0)
# → {"a":[1,2]}
```

**`json_valide(t)`** — Vrai si le texte est du JSON valide.

```fadera
afficher json_valide("{\"a\": 1}")
# → vrai
```

**`json_chemin(valeur, chemin, defaut)`** — Va chercher une valeur profonde : "a.b.0.c"; renvoie defaut si le chemin n'existe pas.

```fadera
afficher json_chemin({ a: { b: [10, 20] } }, "a.b.1", 0)
# → 20
```

**`json_fusionner(a, b)`** — Fusionne deux structures en profondeur.

```fadera
afficher json_fusionner({ a: { x: 1 } }, { a: { y: 2 } })
# → { a: { x: 1, y: 2 } }
```

## Dates

Une date est un nombre (millisecondes depuis le 1er janvier 1970, heure UTC). Les exemples ci-dessous sont en heure UTC.

**`date_creer(an, mois, jour, h, min, s)`** — Fabrique une date (heures, minutes, secondes facultatives).

```fadera
afficher date_creer(2026, 10, 24, 0, 0, 0)
# → 1792800000000
```

**`date_composants(date)`** — Structure : annee, mois, jour, heure, minute, seconde, jour de la semaine…

```fadera
afficher date_composants(date_creer(2026, 10, 24, 15, 30, 0))
# → { annee: 2026, mois: 10, jour: 24, heure: 15, minute: 30, seconde: 0, jour_semaine: "samedi", nom_mois: "octobre", numero_semaine_jour: 6 }
```

**`date_formater(date, modele)`** — Écrit une date. Modèle : AAAA MM JJ hh mm ss jour mois.

```fadera
afficher date_formater(date_creer(2026, 10, 24), "JJ/MM/AAAA")
# → 24/10/2026
```

**`date_analyser(texte)`** — Lit une date écrite en texte (format ISO ou JJ/MM/AAAA).

```fadera
afficher date_analyser("2026-10-24")
# → 1792800000000
```

**`date_iso(date)`** — Écrit la date au format ISO 8601.

```fadera
afficher date_iso(date_creer(2026, 10, 24))
# → 2026-10-24T00:00:00.000Z
```

**`formater_date(date)`** — Écrit la date au format JJ/MM/AAAA hh:mm:ss.

```fadera
afficher formater_date(date_creer(2026, 10, 24, 8, 5, 0))
# → 24/10/2026 08:05:00
```

**`date_ajouter(date, quantite, unite)`** — Ajoute une durée ("secondes", "minutes", "heures", "jours").

```fadera
afficher date_formater(date_ajouter(date_creer(2026, 10, 24), 10, "jours"), "JJ/MM/AAAA")
# → 03/11/2026
```

**`date_ajouter_mois(date, n)`** — Ajoute n mois.

```fadera
afficher date_formater(date_ajouter_mois(date_creer(2026, 1, 31), 1), "JJ/MM/AAAA")
# → 28/02/2026
```

**`date_difference(a, b, unite)`** — Durée de a à b dans l'unité demandée.

```fadera
afficher date_difference(date_creer(2026, 10, 24), date_creer(2026, 12, 25), "jours")
# → 62
```

**`date_debut(date, unite)`** — Début du jour, du mois ou de l'année contenant la date.

```fadera
afficher date_formater(date_debut(date_creer(2026, 10, 24, 15, 0, 0), "mois"), "JJ/MM/AAAA hh:mm")
# → 01/10/2026 00:00
```

**`date_jours_dans_mois(an, mois)`** — Nombre de jours du mois.

```fadera
afficher date_jours_dans_mois(2028, 2)
# → 29
```

**`date_semaine(date)`** — Numéro de la semaine dans l'année.

```fadera
afficher date_semaine(date_creer(2026, 10, 24))
# → 43
```

**`date_age(naissance, date)`** — Âge en années complètes à la date donnée.

```fadera
afficher date_age(date_creer(2008, 3, 15), date_creer(2026, 10, 24))
# → 18
```

**`est_bissextile(an)`** — Vrai si l'année est bissextile.

```fadera
afficher est_bissextile(2028)
# → vrai
```

## Réseau et télécommunications

**`telecom_calculer_sous_reseau(ip, prefixe)`** — Masque, adresse du réseau, diffusion et plage d'hôtes.

```fadera
afficher telecom_calculer_sous_reseau("192.168.1.130", 26)
# → { adresse_saisie: "192.168.1.130/26", masque_sous_reseau: "255.255.255.192", masque_inverse_wildcard: "0.0.0.63", adresse_reseau_exacte: "192.168.1.128", adresse_diffusion_broadcast: "192.168.1.191", premiere_ip_disponible: "192.168.1.129", derniere_ip_disponible: "192.168.1.190", nombre_hotes_utilisables: 62 }
```

**`masque_reseau(prefixe)`** — Masque de sous-réseau pour un préfixe.

```fadera
afficher masque_reseau(24)
# → 255.255.255.0
```

**`plage_reseau(ip, prefixe)`** — Première et dernière adresse du réseau.

```fadera
afficher plage_reseau("192.168.1.0", 30)
# → 192.168.1.0 - 192.168.1.3
```

**`telecom_creer_topologie()`** — Réseau vide (nœuds et liaisons).

```fadera
afficher telecom_creer_topologie()
# → { noeuds: {  }, listeNoeuds: [] }
```

**`telecom_ajouter_liaison(reseau, liaison)`** — Ajoute une liaison { de, a, cout } (dans les deux sens).

```fadera
afficher telecom_ajouter_liaison(telecom_creer_topologie(), { de: "A", a: "B", cout: 4 })
# → { noeuds: { A: { B: 4 }, B: { A: 4 } }, listeNoeuds: ["A", "B"] }
```

**`telecom_matrice_adjacence(reseau)`** — Matrice d'adjacence du réseau.

```fadera
afficher telecom_matrice_adjacence(telecom_ajouter_liaison(telecom_creer_topologie(), { de: "A", a: "B", cout: 4 }))
# → { index_noeuds: ["A", "B"], matrice: [[0, 4], [4, 0]] }
```

**`telecom_verifier_connexite(reseau)`** — Vrai si tous les nœuds sont reliés.

```fadera
afficher telecom_verifier_connexite(telecom_ajouter_liaison(telecom_creer_topologie(), { de: "A", a: "B", cout: 4 }))
# → vrai
```

**`telecom_dijkstra_routage(reseau, requete)`** — Plus court chemin : requête { depart, arrivee }.

```fadera
afficher telecom_dijkstra_routage(telecom_ajouter_liaison(telecom_creer_topologie(), { de: "A", a: "B", cout: 4 }), { depart: "A", arrivee: "B" })
# → { cout_total: 4, itineraire: ["A", "B"] }
```

**`http_obtenir(url, entetes)`** — Télécharge une page ou un fichier texte ; « entetes » (facultatif) est une structure d'en-têtes, par exemple { Authorization: "Bearer …" } (a besoin d'internet).

**`http_json(url, entetes)`** — Télécharge une réponse JSON et la convertit en valeur Fadéra ; « entetes » est facultatif, comme pour http_obtenir (a besoin d'internet).

**`http_envoyer(url, donnees, entetes)`** — Envoie des données (POST) ; renvoie { statut, ok, corps } (a besoin d'internet).

## Chiffrement

**`crypto_hacher_sha256(t)`** — Empreinte SHA-256.

```fadera
afficher crypto_hacher_sha256("a")
# → ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
```

**`sha256(t)`** — Autre nom de crypto_hacher_sha256.

```fadera
afficher sha256("a")
# → ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
```

**`hacher_sha256(t)`** — Autre nom de crypto_hacher_sha256.

```fadera
afficher hacher_sha256("a")
# → ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
```

**`crypto_hacher_sha512(t)`** — Empreinte SHA-512.

```fadera
afficher taille(crypto_hacher_sha512("a"))
# → 128
```

**`crypto_hmac_sha256(t, cle)`** — Signature HMAC-SHA256.

```fadera
afficher crypto_hmac_sha256("message", "cle")
# → 21b40c043183816f3b637522a26a5880cdf5091a1216a0bb0619d45a76a0e10e
```

**`crypto_aleatoire_hex(n)`** — n octets aléatoires sûrs, écrits en hexadécimal.

**`crypto_chiffrer_symetrique(t, mot_de_passe)`** — Chiffre avec AES-256-GCM ; le résultat change à chaque appel.

**`crypto_dechiffrer_symetrique(chiffre, mot_de_passe)`** — Déchiffre ; erreur claire si le mot de passe est faux.

```fadera
afficher crypto_dechiffrer_symetrique(crypto_chiffrer_symetrique("secret", "mdp"), "mdp")
# → secret
```

**`crypto_generer_cles_rsa()`** — Paire de clés RSA 2048 bits : { cle_publique, cle_privee }.

**`crypto_chiffrer_asymetrique(t, cle_publique)`** — Chiffre avec la clé publique.

**`crypto_dechiffrer_asymetrique(chiffre, cle_privee)`** — Déchiffre avec la clé privée.

**`chiffrer(t, mot_de_passe)`** — Autre nom de crypto_chiffrer_symetrique.

```fadera
afficher chiffrer("abc", "mdp") != "abc"
# → vrai
```

**`dechiffrer(chiffre, mot_de_passe)`** — Autre nom de crypto_dechiffrer_symetrique.

```fadera
afficher dechiffrer(chiffrer("abc", "mdp"), "mdp")
# → abc
```

**`chiffrer_base64(t)`** — Encode un texte en Base64 (ce n'est pas un chiffrement secret).

```fadera
afficher chiffrer_base64("Fadéra")
# → RmFkw6lyYQ==
```

**`dechiffrer_base64(t)`** — Décode du Base64.

```fadera
afficher dechiffrer_base64("RmFkw6lyYQ==")
# → Fadéra
```

## Intelligence artificielle

**`ia_creer_neurone(nb_entrees, activation)`** — Crée un neurone ("sigmoid", "relu" ou "heaviside").

```fadera
afficher type_de(ia_creer_neurone(2, "relu"))
# → structure
```

**`ia_activation_perceptron(neurone, entrees)`** — Sortie du neurone pour une liste d'entrées.

```fadera
afficher ia_activation_perceptron({ poids: [1, 1], biais: -1, activation: "heaviside" }, [1, 1])
# → 1
```

**`ia_descente_gradient(neurone, config)`** — Met à jour les poids : config = { entrees, erreur }.

**`ia_entrainer_reseau_xor(epoques)`** — Entraîne un petit réseau sur le problème XOR.

**`ia_reseau_multicouche(architecture, config)`** — Entraîne un réseau multicouche : config = { dataset, epoques, tauxApprentissage }.

**`creer_reseau(entrees, caches, sorties)`** — Crée un réseau de neurones à une couche cachée.

```fadera
afficher type_de(creer_reseau(2, 3, 1))
# → réseau
```

**`ia_entrainer(reseau, entrees, cibles, taux)`** — Fait un pas d'entraînement et renvoie l'erreur.

**`ia_predire(reseau, entrees)`** — Prédiction du réseau pour une liste d'entrées.

## Tests

**`verifier(condition, message)`** — Échoue si la condition est fausse (dans un bloc tester).

**`verifier_egal(obtenu, attendu, message)`** — Échoue si les deux valeurs diffèrent (dans un bloc tester).

**`verifier_erreur(fonction, texte)`** — Échoue si la fonction ne déclenche pas d'erreur contenant ce texte (dans un bloc tester).

## Autres noms (écriture en majuscules internes)

**`masqueReseau(prefixe)`** — Autre nom de masque_reseau.

```fadera
afficher masqueReseau(24)
# → 255.255.255.0
```

**`plageReseau(ip, prefixe)`** — Autre nom de plage_reseau.

```fadera
afficher plageReseau("192.168.1.0", 30)
# → 192.168.1.0 - 192.168.1.3
```

**`creerReseau(entrees, caches, sorties)`** — Autre nom de creer_reseau.

```fadera
afficher type_de(creerReseau(2, 3, 1))
# → réseau
```

**`iaEntrainer(reseau, entrees, cibles, taux)`** — Autre nom de ia_entrainer.

**`iaPredire(reseau, entrees)`** — Autre nom de ia_predire.

## Modules standard

Ces modules sont écrits en Fadéra et inclus dans l'exécutable et dans l'IDE : `importer "std/nom"`. Ils s'utilisent avec `importer "std/nom" comme alias` (puis `alias.fonction(…)`) ou directement.

### std/dates

```
date_longue(ts)           « samedi 17 octobre 2026 »
est_week_end(ts)          vrai pour un samedi ou un dimanche
jours_entre(a, b)         nombre de jours de la date a à la date b (négatif si b est avant a)
ajouter_jours(ts, n)      la date n jours plus tard (n peut être négatif)
debut_du_mois(ts)         le 1er du mois à 00:00
fin_du_mois(ts)           le dernier jour du mois à 00:00
meme_jour(a, b)           vrai si les deux dates tombent le même jour
est_jour_ouvre(ts)        vrai du lundi au vendredi
jours_ouvres_entre(a, b)  nombre de jours ouvrés dans [a, b[ (négatif si b est avant a)
ajouter_jours_ouvres(ts, n)  avance de n jours ouvrés (n >= 0)
prochain_jour(ts, nom)    prochaine date strictement après ts qui tombe un « lundi », « mardi »…
calendrier(an, mois)      texte du calendrier du mois, semaine du lundi au dimanche
duree_texte(ms)           « 1 j 2 h 3 min 4 s » à partir d'une durée en millisecondes
```

Exemple :

```fadera
importer "std/dates"

soit d = date_creer(2026, 10, 24, 0, 0, 0)
afficher date_longue(d)
afficher est_week_end(d)
afficher jours_entre(d, date_creer(2026, 12, 25))
afficher date_longue(ajouter_jours_ouvres(d, 5))
afficher date_longue(fin_du_mois(d))
afficher duree_texte(93784000)
```

Résultat :

```
samedi 24 octobre 2026
vrai
62
vendredi 30 octobre 2026
samedi 31 octobre 2026
1 j 2 h 3 min 4 s
```

### std/dictionnaire

```
Dictionnaire ordonné : les clés gardent leur ordre d'insertion (même les clés qui ressemblent
à des nombres). Les clés sont comparées par leur texte : 1 et "1" désignent la même entrée.
  soit d = Dictionnaire()
  d.definir(cle, valeur)      ajoute ou remplace (renvoie le dictionnaire : les appels s'enchaînent)
  d.obtenir(cle)              valeur (erreur si la clé est absente)
  d.obtenir_ou(cle, repli)   valeur, ou repli si la clé est absente
  d.possede(cle)              vrai si la clé existe
  d.supprimer(cle)            retire la clé et renvoie sa valeur (erreur si absente)
  d.longueur()                nombre d'entrées
  d.est_vide()                vrai s'il n'y a aucune entrée
  d.liste_cles()              clés dans l'ordre d'insertion
  d.liste_valeurs()           valeurs dans le même ordre
  d.liste_paires()            [[cle, valeur], …]
  d.vider()                   retire tout
  d.en_structure()            structure ordinaire { cle: valeur }
```

Exemple :

```fadera
importer "std/dictionnaire"

soit d = Dictionnaire()
d.definir("pomme", 3).definir("poire", 5).definir("kiwi", 2)
afficher d.obtenir("poire")
afficher d.obtenir_ou("mangue", 0)
afficher d.liste_cles()
afficher d.supprimer("pomme")
afficher d.longueur()
```

Résultat :

```
5
0
["pomme", "poire", "kiwi"]
3
2
```

### std/ensemble

```
Ensemble d'éléments sans doublon, dans l'ordre d'insertion. Deux éléments sont égaux s'ils ont
la même représentation JSON (1 et "1" sont donc différents).
  soit e = Ensemble()
  e.inclure(x)           ajoute x (sans effet s'il est déjà là) ; renvoie l'ensemble
  e.exclure(x)           retire x (sans effet s'il est absent) ; renvoie l'ensemble
  e.possede(x)           vrai si x est dans l'ensemble
  e.longueur()           nombre d'éléments
  e.est_vide()           vrai si l'ensemble est vide
  e.en_liste()           éléments dans l'ordre d'insertion
  e.union(autre)         nouvel ensemble : éléments de l'un ou de l'autre
  e.intersection(autre)  nouvel ensemble : éléments communs
  e.difference(autre)    nouvel ensemble : éléments de e absents de autre
ensemble_depuis(l)       construit un ensemble à partir d'une liste
```

Exemple :

```fadera
importer "std/ensemble"

soit a = ensemble_depuis([1, 2, 3, 3, 2])
soit b = ensemble_depuis([3, 4])
afficher a.longueur()
afficher a.union(b).en_liste()
afficher a.intersection(b).en_liste()
afficher a.difference(b).en_liste()
afficher a.possede(2)
```

Résultat :

```
3
[1, 2, 3, 4]
[3]
[1, 2]
vrai
```

### std/file

```
File (premier entré, premier sorti) bâtie sur une classe.
  soit f = File()
  f.enfiler(x)        ajoute à la fin (renvoie la file : les appels s'enchaînent)
  f.defiler()         retire et renvoie le premier élément (erreur si la file est vide)
  f.tete()            lit le premier élément sans le retirer (erreur si vide)
  f.longueur()        nombre d'éléments
  f.est_vide()        vrai si la file est vide
  f.vider()           retire tout
  f.en_liste()        copie des éléments, du premier au dernier
```

Exemple :

```fadera
importer "std/file"

soit f = File()
f.enfiler("premier").enfiler("deuxième").enfiler("troisième")
afficher f.defiler()
afficher f.tete()
afficher f.longueur()
afficher f.en_liste()
```

Résultat :

```
premier
deuxième
2
["deuxième", "troisième"]
```

### std/json

```
Compléments à json_lire, json_texte et json_chemin.
json_joli(valeur)                 texte JSON indenté de 2 espaces
json_compact(valeur)              texte JSON sur une seule ligne
json_lire_ou(texte, repli)       lit le JSON, ou renvoie repli si le texte est invalide
json_aplatir(valeur)              structure { "a.b[0]": valeur, … } de toutes les feuilles
json_profondeur(valeur)           niveaux d'imbrication (0 pour une valeur simple)
json_selectionner(valeur, champs) garde seulement les champs donnés (structure, ou liste de structures)
json_egal(a, b)                   vrai si les deux valeurs ont le même contenu
```

Exemple :

```fadera
importer "std/json"

soit v = { nom: "Awa", adresse: { ville: "Dakar", rue: ["Plateau", 12] } }
afficher json_compact(v)
afficher json_aplatir(v)
afficher json_profondeur(v)
afficher json_selectionner(v, ["nom"])
afficher json_lire_ou("pas du json", "repli")
```

Résultat :

```
{"nom":"Awa","adresse":{"ville":"Dakar","rue":["Plateau",12]}}
{ nom: "Awa", adresse.ville: "Dakar", adresse.rue[0]: "Plateau", adresse.rue[1]: 12 }
3
{ nom: "Awa" }
repli
```

### std/listes

```
premier(l), dernier(l)      premier / dernier élément (erreur si la liste est vide)
prendre(l, n)               les n premiers éléments
sauter(l, n)                la liste sans ses n premiers éléments
compter_si(l, f)            nombre d'éléments pour lesquels f(x) est vrai
partitionner(l, f)          [éléments où f est vrai, autres éléments]
```

Exemple :

```fadera
importer "std/listes"

soit l = [5, 8, 1, 9, 4]
afficher premier(l) + " " + dernier(l)
afficher prendre(l, 2)
afficher sauter(l, 3)
afficher compter_si(l, x => x > 4)
afficher partitionner(l, x => x % 2 == 0)
```

Résultat :

```
5 4
[5, 8]
[9, 4]
3
[[8, 4], [5, 1, 9]]
```

### std/maths

```
factorielle(n)            n!  (1 si n <= 1)
fibonacci(n)              n-ième terme de la suite (fibonacci(0) = 0, fibonacci(1) = 1)
est_premier(n)            vrai si n est un nombre premier
premiers_jusqua(n)        liste des nombres premiers de 2 à n
somme_chiffres(n)         somme des chiffres de n (signe ignoré)
limiter(x, bas, haut)     ramène x entre bas et haut
pgcd(a, b), ppcm(a, b)    plus grand commun diviseur / plus petit commun multiple (entiers)
somme(l), produit(l)      somme / produit des éléments d'une liste
moyenne(l), mediane(l)    moyenne / médiane (erreur si la liste est vide)
variance(l), ecart_type(l)   variance et écart type de la population
combinaisons(n, k)        nombre de façons de choisir k éléments parmi n
arrangements(n, k)        nombre de listes ordonnées de k éléments parmi n
puissance_mod(b, e, m)    b puissance e modulo m (e entier >= 0), sans dépassement
racine_entiere(n)         plus grand entier r tel que r * r <= n
est_carre_parfait(n)      vrai si n est le carré d'un entier
diviseurs(n)              liste triée des diviseurs positifs de n
interpoler(a, b, t)       a + (b - a) * t
transposer(m)             transposée d'une matrice (liste de listes)
produit_matrices(a, b)    produit de deux matrices
```

Exemple :

```fadera
importer "std/maths"

afficher factorielle(6)
afficher fibonacci(10)
afficher premiers_jusqua(30)
afficher pgcd(84, 36) + " " + ppcm(4, 6)
afficher combinaisons(5, 2)
afficher diviseurs(28)
afficher transposer([[1, 2, 3], [4, 5, 6]])
afficher puissance_mod(2, 100, 13)
```

Résultat :

```
720
55
[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
12 12
10
[1, 2, 4, 7, 14, 28]
[[1, 4], [2, 5], [3, 6]]
3
```

### std/pile

```
Pile (dernier entré, premier sorti) bâtie sur une classe.
  soit p = Pile()
  p.empiler(x)        ajoute au sommet (renvoie la pile : les appels s'enchaînent)
  p.depiler()         retire et renvoie le sommet (erreur si la pile est vide)
  p.sommet()          lit le sommet sans le retirer (erreur si vide)
  p.longueur()        nombre d'éléments
  p.est_vide()        vrai si la pile est vide
  p.vider()           retire tout
  p.en_liste()        copie des éléments, du fond vers le sommet
```

Exemple :

```fadera
importer "std/pile"

soit p = Pile()
p.empiler("a").empiler("b").empiler("c")
afficher p.sommet()
afficher p.depiler() + p.depiler()
afficher p.longueur()
afficher p.en_liste()
```

Résultat :

```
c
cb
1
["a"]
```

### std/regex

```
Expressions régulières toutes prêtes, bâties sur regex_test / regex_trouver / regex_remplacer.
correspond_entierement(t, motif)   vrai si le texte entier correspond au motif
compter_motif(t, motif)            nombre de correspondances
extraire_nombres(t)                liste des nombres (entiers ou décimaux, négatifs compris) du texte
extraire_mots(t)                   liste des mots (lettres, accents compris)
extraire_courriels(t)              adresses électroniques trouvées
extraire_urls(t)                   adresses http(s) trouvées
est_courriel(t)                    vrai si t est une adresse électronique
est_url(t)                         vrai si t est une adresse http(s)
est_entier(t)                      vrai si t est un entier écrit en chiffres (signe permis)
est_telephone(t)                   vrai pour un numéro de 8 à 15 chiffres (espaces, points, tirets, +, parenthèses permis)
reduire_espaces(t)                 remplace toute suite d'espaces par une seule espace, sans espaces aux bords
remplacer_chaque(t, motif, f)      remplace chaque correspondance par f(correspondance)
```

Exemple :

```fadera
importer "std/regex"

soit t = "Écrivez à awa@exemple.sn ou moussa@fadera.org avant le 24 octobre (12 places, 7,5 €)."
afficher extraire_courriels(t)
afficher extraire_nombres(t)
afficher est_courriel("awa@exemple.sn")
afficher est_courriel("awa@")
afficher reduire_espaces("  trop    d'espaces   ici ")
afficher remplacer_chaque("a1b22", "[0-9]+", n => "<" + n + ">")
```

Résultat :

```
["awa@exemple.sn", "moussa@fadera.org"]
[24, 12, 7, 5]
vrai
faux
trop d'espaces ici
a<1>b<22>
```

### std/tas

```
Tas : file de priorité. L'élément « le plus petit » selon la fonction de comparaison sort en premier.
  soit t = Tas((a, b) => a.age - b.age)    comparer(a, b) < 0 quand a doit sortir avant b
  soit t = tas_min()                       tas de nombres ou de textes, plus petit d'abord
  soit t = tas_max()                       idem, plus grand d'abord
  t.pousser(x)       ajoute x (renvoie le tas : les appels s'enchaînent)
  t.extraire()       retire et renvoie l'élément prioritaire (erreur si le tas est vide)
  t.premier()        lit l'élément prioritaire sans le retirer (erreur si vide)
  t.longueur()       nombre d'éléments
  t.est_vide()       vrai si le tas est vide
```

Exemple :

```fadera
importer "std/tas"

soit t = tas_min()
pour chaque x dans [42, 7, 19, 3, 25] faire
  t.pousser(x)
fin
soit ordre = []
tantque non t.est_vide() faire
  ajouter(ordre, t.extraire())
fin
afficher ordre

# Une file de priorité avec sa propre règle : les plus âgés d'abord
soit urgences = Tas((a, b) => lireCle(b, "age") - lireCle(a, "age"))
urgences.pousser({ nom: "Awa", age: 30 }).pousser({ nom: "Ali", age: 71 }).pousser({ nom: "Bineta", age: 55 })
afficher lireCle(urgences.extraire(), "nom")
```

Résultat :

```
[3, 7, 19, 25, 42]
Ali
```

### std/textes

```
est_vide(t)                      vrai si le texte est vide ou ne contient que des espaces
centrer(t, largeur)              ajoute des espaces des deux côtés pour centrer
tronquer(t, n)                   coupe à n caractères au plus (« … » à la fin si le texte est coupé)
inverser_texte(t)                texte à l'envers
compter_occurrences(t, motif)    nombre de fois où motif apparaît (sans chevauchement)
```

Exemple :

```fadera
importer "std/textes"

afficher "[" + centrer("Fadéra", 14) + "]"
afficher tronquer("Programmer en français", 12)
afficher inverser_texte("bonjour")
afficher compter_occurrences("banane", "an")
afficher est_vide("   ")
```

Résultat :

```
[    Fadéra    ]
Programmer …
ruojnob
2
vrai
```

### std/tri

```
Tri et recherche avancés. Les fonctions ne modifient jamais la liste reçue.
comparer(a, b) renvoie un nombre négatif, 0 ou positif (comme pour trier).

tri_fusion(l, comparer)        tri stable en O(n log n)
tri_rapide(l, comparer)        tri rapide (non stable)
tri_insertion(l, comparer)     tri stable, rapide sur les petites listes
trier_decroissant(l)           tri décroissant naturel
trier_multi(l, cles)           tri stable sur plusieurs clés : cles = [f1, f2, …] ; une clé est
                               une fonction x => valeur (le premier critère l'emporte)
est_trie(l)                    vrai si la liste est croissante
recherche_binaire(l, x)        position de x dans une liste triée croissante, ou -1
borne_inf(l, x)                première position où x pourrait s'insérer en gardant l'ordre
borne_sup(l, x)                dernière position où x pourrait s'insérer
indice_de(l, x)                position de la première occurrence de x (liste quelconque), ou -1
minimum_par(l, f)              élément dont f(x) est le plus petit (erreur si vide)
maximum_par(l, f)              élément dont f(x) est le plus grand (erreur si vide)
plus_petits(l, n)              les n plus petits éléments, triés
plus_grands(l, n)              les n plus grands éléments, triés du plus grand au plus petit
grouper_par(l, f)              structure { clé: [éléments] } selon f(x) (clés en texte)
```

Exemple :

```fadera
importer "std/tri"

soit l = [30, 10, 20, 10]
afficher tri_fusion(l, (a, b) => a - b)
afficher trier_decroissant(l)
afficher recherche_binaire([10, 20, 30, 40], 30)
afficher borne_inf([10, 20, 20, 30], 20) + " " + borne_sup([10, 20, 20, 30], 20)
afficher est_trie([1, 2, 2, 3])
```

Résultat :

```
[10, 10, 20, 30]
[30, 20, 10, 10]
2
1 3
vrai
```

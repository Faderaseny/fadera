# Documentation de Fadéra

| Document | Pour qui ? |
|---|---|
| [**TUTORIEL.md**](TUTORIEL.md) | Vous découvrez la programmation ou Fadéra : 17 chapitres pas à pas, avec le vrai résultat de chaque exemple. |
| [**exemples/**](exemples/) | Vous voulez des programmes complets à lire, lancer et modifier : 40 exemples (calculs, jeux, textes, fichiers, réseau, objets, robotique). |
| [**EXERCICES.md**](EXERCICES.md) | Vous voulez vous entraîner : 94 exercices corrigés dans 20 domaines (du calcul aux robots, au chiffrement, à l'IA et aux métiers de l'industrie), avec le vrai résultat de chaque corrigé. |
| [**REFERENCE.md**](REFERENCE.md) | Vous cherchez une fonction précise : les 196 fonctions natives, les 12 du langage et les 12 modules standard, avec un exemple vérifié pour chacune. |
| [**PAQUETS.md**](PAQUETS.md) | Vous voulez installer du code écrit par d'autres, ou partager le vôtre : `fadera paquet …`. |
| [**PAQUETS_CONNUS.md**](PAQUETS_CONNUS.md) | La liste des paquets déjà publiés (géométrie, notes…) et comment ajouter le vôtre. |
| [**SERVICES.md**](SERVICES.md) | Vous voulez interroger une API, une base de données hébergée ou un service en ligne. |
| [Guide du langage](../GUIDE_FADERA.md) | Vous voulez la grammaire complète, l'IDE, la distribution. |
| [Spécification](../SPEC_FADERA.md) | Vous voulez le comportement exact du langage (elle est exécutable et testée). |

## Lancer un exemple

```
fadera run docs/exemples/01_bonjour.fa
```

ou copiez son contenu dans le [playground](https://faderaseny.github.io/fadera/).

## Les 40 exemples

| Fichier | Sujet | Notions |
|---|---|---|
| [`01_bonjour.fa`](exemples/01_bonjour.fa) | Bonjour, Fadéra ! | afficher, variables (soit), texte, nombres, concaténation avec + |
| [`02_tables_multiplication.fa`](exemples/02_tables_multiplication.fa) | Tables de multiplication | boucle « pour », boucle imbriquée, pas (pas) |
| [`03_temperatures.fa`](exemples/03_temperatures.fa) | Convertisseur de températures | fonctions, paramètres, renvoyer, arrondir, listes de structures |
| [`04_nombres_premiers.fa`](exemples/04_nombres_premiers.fa) | Nombres premiers : le crible d'Ératosthène | listes, tantque, lireElement / modifierElement, racine |
| [`05_recursivite.fa`](exemples/05_recursivite.fa) | Récursivité : factorielle, Fibonacci, tours de Hanoï | une fonction qui s'appelle elle-même, cas de base |
| [`06_notes_statistiques.fa`](exemples/06_notes_statistiques.fa) | Statistiques d'une classe | listes de structures, stats_*, selon, transformer, filtrer, trier_par |
| [`07_second_degre.fa`](exemples/07_second_degre.fa) | Équation du second degré  ax² + bx + c = 0 | discriminant, racine, fonctions de la bibliothèque (math_resoudre_second_degre) |
| [`08_finances.fa`](exemples/08_finances.fa) | Épargne et prêt : intérêts composés | boucles, arrondi, fonction qui renvoie une liste, structures |
| [`09_devine_le_nombre.fa`](exemples/09_devine_le_nombre.fa) | Devine le nombre | fonctions passées en argument, générateur pseudo-aléatoire, dichotomie |
| [`10_pierre_feuille_ciseaux.fa`](exemples/10_pierre_feuille_ciseaux.fa) | Pierre, feuille, ciseaux : un tournoi entre deux ordinateurs | selon, structures modifiables, générateur pseudo-aléatoire |
| [`11_pendu.fa`](exemples/11_pendu.fa) | Le pendu | texte (caractere, taille), listes, contient, boucle tantque |
| [`12_morpion.fa`](exemples/12_morpion.fa) | Morpion (tic-tac-toe) : deux ordinateurs s'affrontent | listes comme grille, fonctions, boucles, tableau des lignes gagnantes |
| [`13_jeu_de_la_vie.fa`](exemples/13_jeu_de_la_vie.fa) | Le jeu de la vie de Conway | grille à deux dimensions (liste de listes), voisins, génération suivante |
| [`14_blackjack.fa`](exemples/14_blackjack.fa) | Blackjack : le joueur contre la banque | paquet de cartes, mélange de Fisher-Yates avec une graine, fonctions, listes |
| [`15_labyrinthe.fa`](exemples/15_labyrinthe.fa) | Sortir d'un labyrinthe : le plus court chemin | parcours en largeur (file), grille de texte, structures comme clés |
| [`16_mots_frequents.fa`](exemples/16_mots_frequents.fa) | Les mots les plus fréquents d'un texte | texte, découper, structures comme tableau de comptage, trier_par, liste_frequences |
| [`17_palindromes_anagrammes.fa`](exemples/17_palindromes_anagrammes.fa) | Palindromes et anagrammes | normaliser un texte (accents, majuscules, espaces), comparer, trier des lettres |
| [`18_carnet_csv.fa`](exemples/18_carnet_csv.fa) | Un carnet d'adresses au format CSV | csv_lire, csv_texte, listes de structures, filtrer, trier_par, recherche |
| [`19_json_configuration.fa`](exemples/19_json_configuration.fa) | Lire, modifier et écrire du JSON | json_lire, json_texte, json_valide, json_chemin, essayer / attraper |
| [`20_journal_fichier.fa`](exemples/20_journal_fichier.fa) | Un journal de bord enregistré dans un fichier | ecrireFichier, lireFichier, découper en lignes, gestion d'erreur |
| [`21_calendrier_dates.fa`](exemples/21_calendrier_dates.fa) | Dates et calendrier | importer "std/dates", date_creer, date_formater, calculs de durées |
| [`22_sous_reseau_ip.fa`](exemples/22_sous_reseau_ip.fa) | Calculer un sous-réseau IP | telecom_calculer_sous_reseau, lireCle, boucle sur plusieurs préfixes |
| [`23_routage_dijkstra.fa`](exemples/23_routage_dijkstra.fa) | Le plus court chemin dans un réseau | telecom_creer_topologie, telecom_ajouter_liaison, telecom_dijkstra_routage |
| [`24_meteo_http_json.fa`](exemples/24_meteo_http_json.fa) | Interroger une API web (réseau) | http_json, essayer / attraper, json_chemin |
| [`25_verificateur_sites.fa`](exemples/25_verificateur_sites.fa) | Vérifier que des sites répondent | http_obtenir, chrono (mesurer une durée), tableau de résultats, essayer / attraper |
| [`26_compte_bancaire.fa`](exemples/26_compte_bancaire.fa) | Un compte bancaire avec des classes | classe, creer, soi, héritage, erreurs avec lever |
| [`27_parentheses_pile.fa`](exemples/27_parentheses_pile.fa) | Vérifier les parenthèses avec une pile | importer "std/pile", empiler / dépiler, algorithme classique |
| [`28_file_guichet.fa`](exemples/28_file_guichet.fa) | La file d'attente d'un guichet | importer "std/file", enfiler / défiler, simulation avec une graine fixe |
| [`29_tests_integres.fa`](exemples/29_tests_integres.fa) | Tester ses propres fonctions | tester "nom" … fin, verifier, verifier_egal, verifier_erreur |
| [`30_tri_et_recherche.fa`](exemples/30_tri_et_recherche.fa) | Trier et chercher efficacement | importer "std/tri", tri_fusion, trier_multi, recherche_binaire |
| [`31_robot_marche_parle.fa`](exemples/31_robot_marche_parle.fa) | Un robot qui marche en parlant | parler, classe, soi, repeter, texte_format |
| [`32_robot_slogan_fadera.fa`](exemples/32_robot_slogan_fadera.fa) | Le robot qui présente Fadéra (slogan, forces) | parler, listes de structures, pour chaque, texte_remplir_droite |
| [`33_robot_coureur.fa`](exemples/33_robot_coureur.fa) | Un robot coureur : vitesse, fatigue, batterie | simulation par pas de temps, minimum / maximum, barre de progression |
| [`34_robot_lecteur.fa`](exemples/34_robot_lecteur.fa) | Un robot qui lit une histoire à voix haute | parler, decouper, compter les mots |
| [`35_robot_menager.fa`](exemples/35_robot_menager.fa) | Un robot qui fait plein de choses : liste de tâches, batterie | trier_par, structures, essayer / attraper, lever |
| [`36_robot_navigation.fa`](exemples/36_robot_navigation.fa) | Simulation : le robot trouve son chemin et l'annonce | parcours en largeur, cap (nord, est, sud, ouest), tourner / avancer |
| [`37_robot_capteurs_regulation.fa`](exemples/37_robot_capteurs_regulation.fa) | Capteur de distance bruité et régulation proportionnelle | simulation, capteur, régulateur, graine fixe |
| [`38_robot_bras_articule.fa`](exemples/38_robot_bras_articule.fa) | Bras de robot à deux segments (cinématique inverse) | cosinus, arc_cosinus, arc_tangente, degres, essayer / attraper |
| [`39_robot_machine_etats.fa`](exemples/39_robot_machine_etats.fa) | Le cerveau d'un robot gardien : machine à états | selon, états, batterie, graine fixe |
| [`40_essaim_de_robots.fa`](exemples/40_essaim_de_robots.fa) | Un essaim de robots qui se partagent une mission | classe, héritage, polymorphisme, filtrer, est_instance |

Chaque `.fa` a un fichier `.sortie` voisin : le résultat attendu. Les exemples 31 à 40 (robotique) utilisent `parler` : dans le terrain de jeu ou dans `fadera.exe` ils se font entendre, et dans les tests la voix est simplement ignorée. Les exemples 24 et 25 ont besoin d'internet ; hors ligne ils affichent un message d'erreur propre au lieu de planter.

## Comment cette documentation reste juste

Rien n'est écrit « de tête » :

- `node docs/verifier_exemples.js` exécute les 40 exemples sur le moteur JS **et** sur le moteur Rust et compare avec les `.sortie` ;
- `node docs/generer_reference.js` régénère `REFERENCE.md` en exécutant chaque exemple, et refuse de produire le fichier si une fonction native n'est pas documentée ;
- `node docs/generer_tutoriel.js` régénère `TUTORIEL.md` à partir de `tutoriel/TUTORIEL.src.md` en exécutant chaque exemple.

Ces trois contrôles font partie de `node verifier_tout.js` et donc de l'intégration continue : ajouter une fonction à Fadéra sans la documenter fait échouer la CI.

**Pour modifier** : éditez `docs/reference/fonctions.txt` (référence) ou `docs/tutoriel/TUTORIEL.src.md` (tutoriel), puis relancez les générateurs ; ne modifiez jamais `REFERENCE.md` ni `TUTORIEL.md` à la main.

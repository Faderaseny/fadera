// Contenu des leçons de « Apprendre Fadéra ». Chaque exercice est vérifié automatiquement :
// pour chaque test, on exécute le code de l'élève avec les `saisies` données et on compare les lignes affichées à `sortie`.
// `node site/apprendre/verifier_cours.js` contrôle que chaque `solution` réussit ses tests et que chaque `depart` échoue.
(function (racine) {
  const COURS = [
    {
      id: "afficher", chapitre: "1. Premiers pas", titre: "Afficher un message",
      lecon: `<p>Un programme est une suite d'ordres que l'ordinateur exécute <strong>ligne par ligne</strong>. Le premier ordre à connaître est <code>afficher</code> : il écrit quelque chose à l'écran.</p>
<p>Un texte se met entre guillemets <code>" "</code>. Les nombres s'écrivent tels quels.</p>`,
      exemple: `afficher "Bonjour, monde !"\nafficher 2026`,
      consigne: `Écris un programme qui affiche exactement : <code>Salut, Fadéra !</code>`,
      depart: `# Écris ton code ici\n`,
      indice: `Utilise <code>afficher</code> suivi du texte entre guillemets, sans oublier la ponctuation et les majuscules.`,
      solution: `afficher "Salut, Fadéra !"`,
      tests: [{ saisies: [], sortie: ["Salut, Fadéra !"] }],
    },
    {
      id: "variables", chapitre: "1. Premiers pas", titre: "Les variables",
      lecon: `<p>Une <strong>variable</strong> est une boîte qui porte un nom et contient une valeur. On la crée avec <code>soit</code>.</p>
<p>Pour mettre plusieurs choses dans un même <code>afficher</code>, on les <strong>assemble avec <code>+</code></strong>. Pour <em>demander</em> quelque chose à la personne, on utilise <code>saisir("question")</code>.</p>`,
      exemple: `soit prenom = "Fatima"\nsoit age = 14\nafficher "Je m'appelle " + prenom + " et j'ai " + age + " ans."`,
      consigne: `Demande le prénom avec <code>saisir</code>, puis affiche <code>Bonjour, </code> suivi du prénom et d'un point d'exclamation. Pour « Fatima » : <code>Bonjour, Fatima !</code>`,
      depart: `soit prenom = saisir("Ton prénom ? ")\n# affiche maintenant le message\n`,
      indice: `Assemble les morceaux : <code>"Bonjour, " + prenom + " !"</code>`,
      solution: `soit prenom = saisir("Ton prénom ? ")\nafficher "Bonjour, " + prenom + " !"`,
      tests: [{ saisies: ["Fatima"], sortie: ["Bonjour, Fatima !"] }, { saisies: ["Ansou"], sortie: ["Bonjour, Ansou !"] }],
    },
    {
      id: "calculs", chapitre: "1. Premiers pas", titre: "Faire des calculs",
      lecon: `<p>Fadéra calcule avec <code>+</code> <code>-</code> <code>*</code> <code>/</code>. Les priorités sont celles des maths : la multiplication passe avant l'addition, et les parenthèses <code>( )</code> permettent de changer l'ordre.</p>
<p>Quand une réponse saisie ressemble à un nombre, Fadéra la convertit toute seule : tu peux donc calculer directement avec.</p>`,
      exemple: `soit longueur = 8\nsoit largeur = 5\nafficher "Aire : " + longueur * largeur\nafficher "Périmètre : " + 2 * (longueur + largeur)`,
      consigne: `Demande la longueur puis la largeur d'un rectangle, puis affiche l'aire sous la forme <code>Aire : 40</code> (pour 8 et 5).`,
      depart: `soit longueur = saisir("Longueur ? ")\nsoit largeur = saisir("Largeur ? ")\n`,
      indice: `L'aire est <code>longueur * largeur</code>. Assemble-la au texte avec <code>+</code>.`,
      solution: `soit longueur = saisir("Longueur ? ")\nsoit largeur = saisir("Largeur ? ")\nafficher "Aire : " + longueur * largeur`,
      tests: [{ saisies: ["8", "5"], sortie: ["Aire : 40"] }, { saisies: ["3", "7"], sortie: ["Aire : 21"] }, { saisies: ["2.5", "4"], sortie: ["Aire : 10"] }],
    },
    {
      id: "si", chapitre: "2. Décider", titre: "Les conditions : si … sinon",
      lecon: `<p>Pour que le programme <strong>choisisse</strong>, on utilise <code>si … alors … sinon … fin</code>. La condition utilise des comparaisons : <code>&gt;</code> <code>&lt;</code> <code>&gt;=</code> <code>&lt;=</code> <code>==</code> (égal) <code>!=</code> (différent).</p>
<p>Les lignes à l'intérieur sont décalées (indentées) pour que le code reste lisible, et le bloc se termine toujours par <code>fin</code>.</p>`,
      exemple: `soit age = 15\nsi age >= 18 alors\n    afficher "Majeur"\nsinon\n    afficher "Mineur"\nfin`,
      consigne: `Demande l'âge. Affiche <code>Majeur</code> s'il est de 18 ou plus, sinon <code>Mineur</code>.`,
      depart: `soit age = saisir("Ton âge ? ")\n`,
      indice: `<code>si age >= 18 alors</code> … <code>sinon</code> … <code>fin</code>`,
      solution: `soit age = saisir("Ton âge ? ")\nsi age >= 18 alors\n    afficher "Majeur"\nsinon\n    afficher "Mineur"\nfin`,
      tests: [{ saisies: ["15"], sortie: ["Mineur"] }, { saisies: ["18"], sortie: ["Majeur"] }, { saisies: ["40"], sortie: ["Majeur"] }, { saisies: ["0"], sortie: ["Mineur"] }],
    },
    {
      id: "sinonsi", chapitre: "2. Décider", titre: "Plusieurs cas : sinon si",
      lecon: `<p>Quand il y a plus de deux possibilités, on enchaîne avec <code>sinon si</code> (sur la <strong>même ligne</strong>). Fadéra teste les conditions dans l'ordre et s'arrête à la première vraie.</p>`,
      exemple: `soit note = 13\nsi note >= 16 alors\n    afficher "Très bien"\nsinon si note >= 12 alors\n    afficher "Bien"\nsinon\n    afficher "À améliorer"\nfin`,
      consigne: `Demande une note sur 20 et affiche : <code>Très bien</code> (16 et plus), <code>Bien</code> (12 à 15), <code>Passable</code> (10 ou 11), <code>Insuffisant</code> (moins de 10).`,
      depart: `soit note = saisir("Note sur 20 ? ")\n`,
      indice: `Teste du plus haut au plus bas : d'abord <code>note >= 16</code>, puis <code>sinon si note >= 12</code>, puis <code>sinon si note >= 10</code>, puis un <code>sinon</code> final.`,
      solution: `soit note = saisir("Note sur 20 ? ")\nsi note >= 16 alors\n    afficher "Très bien"\nsinon si note >= 12 alors\n    afficher "Bien"\nsinon si note >= 10 alors\n    afficher "Passable"\nsinon\n    afficher "Insuffisant"\nfin`,
      tests: [{ saisies: ["18"], sortie: ["Très bien"] }, { saisies: ["16"], sortie: ["Très bien"] }, { saisies: ["13"], sortie: ["Bien"] }, { saisies: ["12"], sortie: ["Bien"] }, { saisies: ["10"], sortie: ["Passable"] }, { saisies: ["9"], sortie: ["Insuffisant"] }],
    },
    {
      id: "pour", chapitre: "3. Répéter", titre: "La boucle pour",
      lecon: `<p>Pour répéter des instructions, on utilise une <strong>boucle</strong>. <code>pour i allantde 1 a 5 faire … fin</code> exécute le bloc 5 fois, avec <code>i</code> qui vaut 1, puis 2, … puis 5.</p>
<p>On peut compter autrement avec <code>pas</code> : <code>pour i allantde 0 a 10 pas 2 faire</code> donne 0, 2, 4, …</p>`,
      exemple: `pour i allantde 1 a 5 faire\n    afficher i\nfin`,
      consigne: `Demande un nombre <code>n</code> et affiche la table de multiplication de 7 jusqu'à <code>n</code>. Pour n = 3 : <code>7 x 1 = 7</code>, <code>7 x 2 = 14</code>, <code>7 x 3 = 21</code> (une ligne chacune).`,
      depart: `soit n = saisir("Jusqu'à combien ? ")\n`,
      indice: `<code>pour i allantde 1 a n faire</code> puis <code>afficher "7 x " + i + " = " + 7 * i</code>`,
      solution: `soit n = saisir("Jusqu'à combien ? ")\npour i allantde 1 a n faire\n    afficher "7 x " + i + " = " + 7 * i\nfin`,
      tests: [{ saisies: ["3"], sortie: ["7 x 1 = 7", "7 x 2 = 14", "7 x 3 = 21"] }, { saisies: ["1"], sortie: ["7 x 1 = 7"] }, { saisies: ["5"], sortie: ["7 x 1 = 7", "7 x 2 = 14", "7 x 3 = 21", "7 x 4 = 28", "7 x 5 = 35"] }],
    },
    {
      id: "somme", chapitre: "3. Répéter", titre: "Accumuler : la somme",
      lecon: `<p>Une boucle devient puissante quand elle <strong>fait évoluer une variable</strong>. On crée un « compteur » avant la boucle, puis on le modifie à chaque tour : <code>total = total + i</code>.</p>
<p>Attention : <code>soit</code> sert à <em>créer</em> une variable ; pour la modifier ensuite, on écrit simplement <code>nom = …</code>.</p>`,
      exemple: `soit total = 0\npour i allantde 1 a 4 faire\n    total = total + i\nfin\nafficher total   # 1+2+3+4 = 10`,
      consigne: `Demande un nombre <code>n</code> et affiche la somme des entiers de 1 à <code>n</code>. Pour n = 4 : <code>10</code>.`,
      depart: `soit n = saisir("n ? ")\nsoit total = 0\n`,
      indice: `Dans la boucle <code>pour i allantde 1 a n</code>, écris <code>total = total + i</code>. Affiche <code>total</code> <strong>après</strong> la boucle (après son <code>fin</code>).`,
      solution: `soit n = saisir("n ? ")\nsoit total = 0\npour i allantde 1 a n faire\n    total = total + i\nfin\nafficher total`,
      tests: [{ saisies: ["4"], sortie: ["10"] }, { saisies: ["10"], sortie: ["55"] }, { saisies: ["1"], sortie: ["1"] }, { saisies: ["100"], sortie: ["5050"] }],
    },
    {
      id: "tantque", chapitre: "3. Répéter", titre: "La boucle tantque",
      lecon: `<p>Quand on ne sait pas à l'avance combien de tours il faudra, on utilise <code>tantque condition faire … fin</code> : le bloc se répète <strong>tant que</strong> la condition est vraie.</p>
<p>⚠ Il faut que quelque chose, dans la boucle, finisse par rendre la condition fausse — sinon le programme tourne sans fin !</p>`,
      exemple: `soit i = 3\ntantque i > 0 faire\n    afficher i\n    i = i - 1\nfin\nafficher "Décollage !"`,
      consigne: `Demande un nombre <code>n</code> et fais un compte à rebours de <code>n</code> jusqu'à 1, puis affiche <code>Décollage !</code>. Pour n = 3 : <code>3</code>, <code>2</code>, <code>1</code>, <code>Décollage !</code>.`,
      depart: `soit n = saisir("Départ du compte à rebours ? ")\n`,
      indice: `<code>tantque n > 0 faire</code>, affiche <code>n</code>, puis <code>n = n - 1</code>. Le message final va après la boucle.`,
      solution: `soit n = saisir("Départ du compte à rebours ? ")\ntantque n > 0 faire\n    afficher n\n    n = n - 1\nfin\nafficher "Décollage !"`,
      tests: [{ saisies: ["3"], sortie: ["3", "2", "1", "Décollage !"] }, { saisies: ["1"], sortie: ["1", "Décollage !"] }, { saisies: ["0"], sortie: ["Décollage !"] }],
    },
    {
      id: "listes", chapitre: "4. Les listes", titre: "Créer une liste",
      lecon: `<p>Une <strong>liste</strong> range plusieurs valeurs dans une seule variable : <code>[12, 15, 8]</code>. Chaque valeur a une place numérotée, <strong>à partir de 0</strong>.</p>
<p>On connaît sa taille avec <code>taille(liste)</code>, on lit une valeur avec <code>lireElement(liste, place)</code>, et on en ajoute une à la fin avec <code>ajouter(liste, valeur)</code>.</p>`,
      exemple: `soit notes = [12, 15, 8]\najouter(notes, 18)\nafficher notes\nafficher taille(notes)\nafficher lireElement(notes, 1)   # la 2e valeur : 15`,
      consigne: `Crée une liste vide <code>[]</code>, demande trois nombres avec <code>saisir</code> et ajoute-les dans l'ordre, puis affiche la liste. Avec 4, 8 et 15 : <code>[4, 8, 15]</code>.`,
      depart: `soit nombres = []\n`,
      indice: `<code>ajouter(nombres, saisir("Un nombre ? "))</code>, trois fois, puis <code>afficher nombres</code>.`,
      solution: `soit nombres = []\najouter(nombres, saisir("Un nombre ? "))\najouter(nombres, saisir("Un nombre ? "))\najouter(nombres, saisir("Un nombre ? "))\nafficher nombres`,
      tests: [{ saisies: ["4", "8", "15"], sortie: ["[4, 8, 15]"] }, { saisies: ["1", "1", "1"], sortie: ["[1, 1, 1]"] }, { saisies: ["30", "20", "10"], sortie: ["[30, 20, 10]"] }],
    },
    {
      id: "parcourir", chapitre: "4. Les listes", titre: "Parcourir une liste",
      lecon: `<p>Pour traiter chaque valeur d'une liste, on combine une boucle <code>pour</code> et les places de la liste : elles vont de <code>0</code> à <code>taille(liste) - 1</code>.</p>
<p>À chaque tour, <code>i</code> est la place, et <code>lireElement(liste, i)</code> donne la valeur.</p>`,
      exemple: `soit prix = [5, 12, 3]\nsoit total = 0\npour i allantde 0 a taille(prix) - 1 faire\n    total = total + lireElement(prix, i)\nfin\nafficher "Total : " + total`,
      consigne: `Demande d'abord combien de notes il y a (<code>n</code>), puis lis les <code>n</code> notes. Affiche la moyenne sous la forme <code>Moyenne : 12</code> (pour 10, 14 et 12).`,
      depart: `soit n = saisir("Combien de notes ? ")\nsoit notes = []\n`,
      indice: `Une première boucle <code>pour i allantde 1 a n</code> ajoute chaque note. Une seconde boucle (ou la même) fait la somme, puis divise par <code>n</code>.`,
      solution: `soit n = saisir("Combien de notes ? ")\nsoit notes = []\npour i allantde 1 a n faire\n    ajouter(notes, saisir("Note ? "))\nfin\nsoit total = 0\npour i allantde 0 a taille(notes) - 1 faire\n    total = total + lireElement(notes, i)\nfin\nafficher "Moyenne : " + total / n`,
      tests: [{ saisies: ["3", "10", "14", "12"], sortie: ["Moyenne : 12"] }, { saisies: ["2", "10", "15"], sortie: ["Moyenne : 12.5"] }, { saisies: ["1", "17"], sortie: ["Moyenne : 17"] }, { saisies: ["4", "8", "8", "8", "8"], sortie: ["Moyenne : 8"] }],
    },
    {
      id: "trier", chapitre: "4. Les listes", titre: "Trier et chercher",
      lecon: `<p>Fadéra sait déjà trier : <code>trier(liste)</code> range du plus petit au plus grand, <code>inverser(liste)</code> renverse l'ordre, et <code>contient(liste, valeur)</code> répond <code>vrai</code> ou <code>faux</code>.</p>
<p>Ces fonctions <strong>renvoient une nouvelle liste</strong> : la liste d'origine ne change pas. On peut les emboîter : <code>inverser(trier(l))</code>.</p>`,
      exemple: `soit l = [15, 8, 18, 12]\nafficher trier(l)\nafficher inverser(trier(l))\nafficher contient(l, 18)\nafficher contient(l, 5)`,
      consigne: `Demande trois notes, puis affiche-les du <strong>plus grand au plus petit</strong>. Avec 12, 18 et 15 : <code>[18, 15, 12]</code>.`,
      depart: `soit notes = []\najouter(notes, saisir("Note 1 ? "))\najouter(notes, saisir("Note 2 ? "))\najouter(notes, saisir("Note 3 ? "))\n`,
      indice: `Trie d'abord, puis inverse : <code>afficher inverser(trier(notes))</code>.`,
      solution: `soit notes = []\najouter(notes, saisir("Note 1 ? "))\najouter(notes, saisir("Note 2 ? "))\najouter(notes, saisir("Note 3 ? "))\nafficher inverser(trier(notes))`,
      tests: [{ saisies: ["12", "18", "15"], sortie: ["[18, 15, 12]"] }, { saisies: ["1", "2", "3"], sortie: ["[3, 2, 1]"] }, { saisies: ["9", "9", "4"], sortie: ["[9, 9, 4]"] }],
    },
    {
      id: "textes", chapitre: "5. Les textes", titre: "Travailler avec du texte",
      lecon: `<p>Un texte a des outils bien à lui : <code>taille(texte)</code> compte les caractères, <code>majuscule(texte)</code> et <code>minuscule(texte)</code> changent la casse, <code>nettoyer(texte)</code> retire les espaces autour.</p>
<p>Comme pour les listes, ces fonctions <strong>renvoient un nouveau texte</strong>.</p>`,
      exemple: `soit t = "  Bonjour Fadéra  "\nafficher nettoyer(t)\nafficher majuscule(nettoyer(t))\nafficher taille(nettoyer(t))`,
      consigne: `Demande un nom. Affiche-le <strong>nettoyé</strong> et en majuscules, puis, sur la ligne suivante, <code>Taille : </code> et le nombre de caractères. Pour « Ansou » : <code>ANSOU</code> puis <code>Taille : 5</code>.`,
      depart: `soit nom = saisir("Ton nom ? ")\n`,
      indice: `<code>soit propre = nettoyer(nom)</code>, puis <code>afficher majuscule(propre)</code> et <code>afficher "Taille : " + taille(propre)</code>.`,
      solution: `soit nom = saisir("Ton nom ? ")\nsoit propre = nettoyer(nom)\nafficher majuscule(propre)\nafficher "Taille : " + taille(propre)`,
      tests: [{ saisies: ["Ansou"], sortie: ["ANSOU", "Taille : 5"] }, { saisies: ["  fatima "], sortie: ["FATIMA", "Taille : 6"] }, { saisies: ["Li"], sortie: ["LI", "Taille : 2"] }],
    },
    {
      id: "decouper", chapitre: "5. Les textes", titre: "Découper et assembler",
      lecon: `<p><code>decouper(texte, séparateur)</code> casse un texte en <strong>liste</strong> de morceaux ; <code>joindre(liste, séparateur)</code> fait l'inverse. <code>remplacer(texte, ancien, nouveau)</code> change un morceau de texte.</p>
<p>Découper une phrase sur les espaces donne la liste de ses mots : on peut les compter avec <code>taille</code>.</p>`,
      exemple: `soit phrase = "le chat dort"\nsoit mots = decouper(phrase, " ")\nafficher mots\nafficher taille(mots)\nafficher joindre(mots, "-")`,
      consigne: `Demande une phrase. Affiche d'abord le <strong>nombre de mots</strong>, puis la phrase avec des tirets <code>-</code> à la place des espaces. Pour « le chat dort » : <code>3</code> puis <code>le-chat-dort</code>.`,
      depart: `soit phrase = saisir("Une phrase ? ")\n`,
      indice: `<code>soit mots = decouper(phrase, " ")</code>, puis <code>afficher taille(mots)</code> et <code>afficher joindre(mots, "-")</code>.`,
      solution: `soit phrase = saisir("Une phrase ? ")\nsoit mots = decouper(phrase, " ")\nafficher taille(mots)\nafficher joindre(mots, "-")`,
      tests: [{ saisies: ["le chat dort"], sortie: ["3", "le-chat-dort"] }, { saisies: ["bonjour"], sortie: ["1", "bonjour"] }, { saisies: ["Fadéra parle français aux élèves"], sortie: ["5", "Fadéra-parle-français-aux-élèves"] }],
    },
    {
      id: "voyelles", chapitre: "5. Les textes", titre: "Lire un texte lettre par lettre",
      lecon: `<p><code>caractere(texte, place)</code> donne la lettre à une place donnée (la première est la place <code>0</code>), exactement comme pour les listes. On peut donc <strong>parcourir un texte</strong> avec une boucle.</p>
<p><code>contient("aeiou", lettre)</code> répond <code>vrai</code> si la lettre fait partie de ces voyelles.</p>`,
      exemple: `soit mot = "bonjour"\npour i allantde 0 a taille(mot) - 1 faire\n    afficher caractere(mot, i)\nfin`,
      consigne: `Demande un mot (en minuscules, sans accent) et affiche <code>Voyelles : </code> suivi de leur nombre. Pour « bonjour » : <code>Voyelles : 3</code>.`,
      depart: `soit mot = saisir("Un mot ? ")\nsoit compte = 0\n`,
      indice: `Dans la boucle, <code>si contient("aeiou", caractere(mot, i)) alors</code> ajoute 1 à <code>compte</code>.`,
      solution: `soit mot = saisir("Un mot ? ")\nsoit compte = 0\npour i allantde 0 a taille(mot) - 1 faire\n    si contient("aeiou", caractere(mot, i)) alors\n        compte = compte + 1\n    fin\nfin\nafficher "Voyelles : " + compte`,
      tests: [{ saisies: ["bonjour"], sortie: ["Voyelles : 3"] }, { saisies: ["fadera"], sortie: ["Voyelles : 3"] }, { saisies: ["rythme"], sortie: ["Voyelles : 1"] }, { saisies: ["strt"], sortie: ["Voyelles : 0"] }, { saisies: ["aeiou"], sortie: ["Voyelles : 5"] }],
    },
    {
      id: "structures", chapitre: "6. Les structures", titre: "Regrouper avec une structure",
      lecon: `<p>Une <strong>structure</strong> regroupe des informations <em>nommées</em> sur une même chose, par exemple un élève : <code>{ nom: "Fatima", note: 16 }</code>.</p>
<p>On crée avec <code>structure nom = { champ: valeur, … }</code>, on lit avec <code>lireCle(structure, "champ")</code> et on change avec <code>modifierCle(structure, "champ", valeur)</code>.</p>`,
      exemple: `structure eleve = { nom: "Fatima", note: 16 }\nafficher lireCle(eleve, "nom")\nmodifierCle(eleve, "note", 17)\nafficher eleve`,
      consigne: `Demande un nom puis une note. Crée une structure <code>eleve</code> avec ces deux champs, ajoute <strong>1 point bonus</strong> à la note, puis affiche la structure. Pour « Ansou » et 14 : <code>{ nom: "Ansou", note: 15 }</code>.`,
      depart: `soit nom = saisir("Nom ? ")\nsoit note = saisir("Note ? ")\n`,
      indice: `<code>structure eleve = { nom: nom, note: note }</code>, puis <code>modifierCle(eleve, "note", lireCle(eleve, "note") + 1)</code>, puis <code>afficher eleve</code>.`,
      solution: `soit nom = saisir("Nom ? ")\nsoit note = saisir("Note ? ")\nstructure eleve = { nom: nom, note: note }\nmodifierCle(eleve, "note", lireCle(eleve, "note") + 1)\nafficher eleve`,
      tests: [{ saisies: ["Ansou", "14"], sortie: ['{ nom: "Ansou", note: 15 }'] }, { saisies: ["Fatima", "19"], sortie: ['{ nom: "Fatima", note: 20 }'] }, { saisies: ["Li", "0"], sortie: ['{ nom: "Li", note: 1 }'] }],
    },
    {
      id: "classe", chapitre: "6. Les structures", titre: "Une liste de structures",
      lecon: `<p>On peut mettre des structures <strong>dans une liste</strong> : c'est ainsi qu'on représente un tableau de données (une classe d'élèves, un stock, des factures…).</p>
<p>Pour lire un champ du 3<sup>e</sup> élément : <code>lireCle(lireElement(eleves, 2), "nom")</code>.</p>`,
      exemple: `soit eleves = [{ nom: "Ansou", note: 14 }, { nom: "Fatima", note: 17 }]\npour i allantde 0 a taille(eleves) - 1 faire\n    soit e = lireElement(eleves, i)\n    afficher lireCle(e, "nom") + " : " + lireCle(e, "note")\nfin`,
      consigne: `Demande combien d'élèves, puis, pour chacun, son nom et sa note. Affiche à la fin <code>Major : </code> suivi du nom de l'élève qui a la <strong>meilleure note</strong>. (Les notes seront toutes différentes.)`,
      depart: `soit n = saisir("Combien d'élèves ? ")\nsoit eleves = []\n`,
      indice: `Une boucle <code>pour i allantde 1 a n</code> ajoute <code>{ nom: …, note: … }</code> à la liste. Une seconde boucle garde le nom et la note du meilleur vus jusque-là.`,
      solution: `soit n = saisir("Combien d'élèves ? ")\nsoit eleves = []\npour i allantde 1 a n faire\n    soit nom = saisir("Nom ? ")\n    soit note = saisir("Note ? ")\n    ajouter(eleves, { nom: nom, note: note })\nfin\nsoit major = lireElement(eleves, 0)\npour i allantde 1 a taille(eleves) - 1 faire\n    soit e = lireElement(eleves, i)\n    si lireCle(e, "note") > lireCle(major, "note") alors\n        major = e\n    fin\nfin\nafficher "Major : " + lireCle(major, "nom")`,
      tests: [{ saisies: ["2", "Ansou", "14", "Fatima", "17"], sortie: ["Major : Fatima"] }, { saisies: ["3", "Li", "9", "Fatima", "18", "Kofi", "12"], sortie: ["Major : Fatima"] }, { saisies: ["1", "Solo", "10"], sortie: ["Major : Solo"] }, { saisies: ["3", "A", "20", "B", "5", "C", "7"], sortie: ["Major : A"] }],
    },
    {
      id: "fonctions", chapitre: "7. Organiser", titre: "Les fonctions",
      lecon: `<p>Une <strong>fonction</strong> est un petit programme qu'on nomme pour le réutiliser. On la définit avec <code>fonction nom(paramètres) … fin</code> et elle <strong>rend</strong> un résultat avec <code>renvoyer</code>.</p>
<p>On l'appelle ensuite par son nom, autant de fois qu'on veut.</p>`,
      exemple: `fonction carre(x)\n    renvoyer x * x\nfin\n\nafficher carre(4)\nafficher carre(10)`,
      consigne: `Écris une fonction <code>double(x)</code> qui renvoie le double de <code>x</code>. Le programme fourni appelle ta fonction : il doit afficher le double du nombre saisi.`,
      depart: `# Écris ta fonction ici\n\nsoit n = saisir("Un nombre ? ")\nafficher double(n)\n`,
      indice: `<code>fonction double(x)</code> puis <code>renvoyer x * 2</code> puis <code>fin</code>.`,
      solution: `fonction double(x)\n    renvoyer x * 2\nfin\n\nsoit n = saisir("Un nombre ? ")\nafficher double(n)`,
      tests: [{ saisies: ["4"], sortie: ["8"] }, { saisies: ["0"], sortie: ["0"] }, { saisies: ["-3"], sortie: ["-6"] }, { saisies: ["2.5"], sortie: ["5"] }],
    },
    {
      id: "projet", chapitre: "7. Organiser", titre: "Mini-projet : nombres pairs",
      lecon: `<p>Tu connais maintenant l'essentiel : <strong>afficher, variables, conditions, boucles, fonctions</strong>. Mettons-les ensemble.</p>
<p>Astuce : le reste d'une division s'obtient avec <code>%</code>. <code>n % 2 == 0</code> est vrai quand <code>n</code> est pair.</p>`,
      exemple: `fonction estPair(n)\n    renvoyer n % 2 == 0\nfin\n\nafficher estPair(8)\nafficher estPair(7)`,
      consigne: `Demande un nombre <code>n</code>. Affiche, une par ligne, tous les nombres <strong>pairs</strong> de 1 à <code>n</code>, puis <code>Somme : </code> suivi de leur somme. Pour n = 7 : <code>2</code>, <code>4</code>, <code>6</code>, <code>Somme : 12</code>.`,
      depart: `soit n = saisir("n ? ")\n`,
      indice: `Boucle <code>pour i allantde 1 a n</code> ; dedans, <code>si i % 2 == 0 alors</code> affiche <code>i</code> et ajoute-le à un total. Affiche la somme après la boucle.`,
      solution: `soit n = saisir("n ? ")\nsoit somme = 0\npour i allantde 1 a n faire\n    si i % 2 == 0 alors\n        afficher i\n        somme = somme + i\n    fin\nfin\nafficher "Somme : " + somme`,
      tests: [{ saisies: ["7"], sortie: ["2", "4", "6", "Somme : 12"] }, { saisies: ["10"], sortie: ["2", "4", "6", "8", "10", "Somme : 30"] }, { saisies: ["1"], sortie: ["Somme : 0"] }],
    },
  ];
  if (typeof module !== "undefined" && module.exports) module.exports = COURS;
  else racine.COURS = COURS;
})(typeof window !== "undefined" ? window : globalThis);

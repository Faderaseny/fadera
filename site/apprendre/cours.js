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
      id: "fonctions", chapitre: "4. Organiser", titre: "Les fonctions",
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
      id: "projet", chapitre: "4. Organiser", titre: "Mini-projet : nombres pairs",
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

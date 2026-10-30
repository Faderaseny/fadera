# Journal des versions

Le format suit [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/) et la numérotation [SemVer](https://semver.org/lang/fr/) :
`MAJEUR.MINEUR.CORRECTIF`. La version publiée est celle de `fadera-rs/Cargo.toml` ; l'étiquette Git est `v` + ce numéro.

## [Non publié]

## [0.8.0] — 2026-10-29

### Ajouté
- Signature du code par SignPath, prête mais inactive tant que la variable `SIGNPATH_ORGANIZATION_ID` n'existe pas : étapes du workflow de publication et [procédure complète](docs/SIGNATURE.md) (politique de signature comprise).
- **Installateur Windows** `Fadera-Installateur-0.8.0.exe` (Inno Setup, sans droits administrateur par défaut) : choix du dossier, raccourci sur le Bureau et dans le menu Démarrer, ajout au PATH, association des fichiers `.fa` (icône Fadéra, double-clic = ouverture dans l'éditeur, clic droit = « Exécuter avec Fadéra »), lancement de l'éditeur à la fin de l'installation, désinstallation propre. Construit automatiquement par le workflow de publication.
- **Fadéra IDE en fenêtre native** (`Fadera-IDE.exe`, dossier `fadera-ide/`) : l'éditeur s'ouvre dans sa propre fenêtre Windows (WebView2), sans barre d'adresse ni onglets, avec l'icône Fadéra et sans console. Si WebView2 manque, repli automatique sur une fenêtre d'application Edge/Chrome.
- **`fadera ide [fichier.fa]`** : sert l'éditeur sur `127.0.0.1:4747` (jamais accessible depuis le réseau), l'ouvre dans une fenêtre d'application et lui transmet le fichier. Une seconde commande réutilise le serveur déjà lancé ; le serveur s'arrête seul quelques minutes après la fermeture de la fenêtre.
- CI : une tâche Windows compile `Fadera-IDE.exe` et vérifie que le serveur de l'éditeur répond et transmet le fichier.

### Amélioré
- Windows : `fadera.ico` reconstruit en format classique (images BMP de 16 à 128 px, PNG pour 256 px) et `associer_fadera.ps1` écrit la valeur par défaut des clés du registre avec l'API .NET (le nom « Programme Fadera » et l'icône n'étaient pas lus) et rafraîchit le cache d'icônes, pour que l'icône s'affiche bien sur les fichiers `.fa`.
- Terminal du navigateur : quand on lui donne un chemin Windows (`docs\exercices\…`) ou une lettre de lecteur introuvable, il ajoute une astuce (« ce terminal ne voit que les fichiers de l'éditeur ; utilisez Fichier → Importer »).

## [0.7.7] — 2026-10-29

### Corrigé
- Moteur Rust : `trier` et `trier_par` comparaient les listes comme du texte (`[-1, "chien"]` passait avant `[-3, "le"]`), alors que le moteur JavaScript les comparait élément par élément. Les deux moteurs comparent maintenant les listes élément par élément (nombres entre eux, textes entre eux, la liste la plus courte d'abord à égalité de début). Cela rend possible un tri sur plusieurs critères avec une clé en liste : `trier_par(l, x => [critère1, critère2])`.

### Ajouté
- Exercice 44 : trier selon plusieurs critères de deux façons (`trier_par` avec une clé en liste, `trier_multi` du module `std/tri`) et vérifier qu'elles donnent le même résultat.
- Un 20e domaine d'exercices, « Industrie et métiers » (exercices 190 à 203) : contrôle qualité, surveillance d'une machine, réapprovisionnement de stock, facture avec remise et TVA, livraison au moindre coût, facture d'électricité par tranches, irrigation, prêt et amortissement, forfait mobile, journal de sécurité d'un atelier, indice de masse corporelle, horaires d'un bus, repérage d'attaques dans un journal de connexions, chargement d'un camion. La collection compte désormais 94 exercices.
- La page des exercices présente honnêtement l'usage visé : automatiser des calculs, contrôles et rapports, et prototyper des règles métier lisibles en français (et non piloter une machine en temps réel).
- Tests de non-régression (JS et parité Rust) : clés de tri en liste avec `trier_par`, et listes comparées élément par élément avec `trier`.

## [0.7.6] — 2026-10-28

### Modifié
- Le menu « Applications » de la barre du haut, devenu trop long (les dernières lignes n'étaient plus visibles), est coupé en deux : « Fichier » (nouveau, enregistrer, ouvrir, importer, exporter, partager) et « Applications » (exécuter, déboguer, tests, formater, rechercher, paquets). Un seul menu s'ouvre à la fois, et chacun défile s'il dépasse la fenêtre.

### Ajouté
- `docs/EXERCICES.md` : 79 exercices corrigés dans 19 domaines (bases, décisions et boucles, texte, listes, structures, fonctions et récursivité, erreurs, fichiers, dates, mathématiques et statistiques, classes, modules standard, tests intégrés, réseaux, chiffrement, intelligence artificielle, robotique et voix, jeux, mini-projets). Chaque corrigé est un fichier de `docs/exercices/`, exécuté sur les deux moteurs par `node docs/verifier_exemples.js exercices` ; la page est produite par `docs/generer_exercices.js` et vérifiée dans la CI.
- `docs/verifier_exemples.js` accepte un nom de dossier (`exemples` ou `exercices`) et lance `fadera tester` sur tout fichier qui contient des blocs `tester`.

## [0.7.5] — 2026-10-28

### Corrigé
- Le workflow de publication utilise le dossier `publication/` (et non `assets/`, désormais occupé par les logos du dépôt).

### Ajouté
- Nouveau logo Fadéra (monogramme F doré avec curseur, bleu nuit) : écran de démarrage, barre du haut, favicon de l'onglet du navigateur, et petite icône devant chaque fichier `.fa` dans les onglets de l'éditeur.
- `assets/logo/` : icône et logos (SVG, PNG, ICO) à fond sombre et à fond clair, lettres converties en tracés.
- `windows/` : `associer_fadera.ps1` donne l'icône Fadéra aux fichiers `.fa` dans l'Explorateur et les ouvre avec `fadera.exe` au double-clic (sans droits administrateur) ; `retirer_association.ps1` annule.

## [0.7.4] — 2026-10-26

### Corrigé
- L'écran de démarrage de l'éditeur affichait « Démarrage de l'IDE v0.2… » : il affiche maintenant la vraie version (« Démarrage de Fadéra v0.7.4… »), et le bouton 🌿 (infos du langage) l'indique aussi.

### Amélioré
- Un test (`verifier_ide.js`) vérifie que la version affichée dans l'éditeur est toujours celle de `fadera-rs/Cargo.toml` : on ne peut plus publier une version avec un ancien numéro à l'écran.

## [0.7.3] — 2026-10-26

### Corrigé
- **Boutons 📄 Nouveau et 💾 Enregistrer** de la barre latérale : leurs petits menus étaient coupés par le bord de la fenêtre de l'éditeur (une moitié de menu seulement à l'écran). Ils s'affichent maintenant par-dessus la page, toujours en entier, même dans une petite fenêtre.
- « Avec modèle de base » remplaçait le contenu du fichier ouvert sans prévenir ; il crée maintenant un nouveau fichier, comme « Fichier vierge ».
- Un nom de fichier déjà pris affichait en plus « créé » ; le message dit désormais que le fichier existe déjà et que rien n'a été remplacé.
- Un caractère « > » parasite s'affichait dans la barre de titre de l'éditeur.

### Amélioré
- Menus utilisables au clavier (Tab, Entrée, Espace, flèches, Échap) et fermés par un clic ailleurs ; raccourcis affichés (Ctrl+Alt+N, Ctrl+S, Ctrl+Maj+S).
- Nouveau raccourci **Ctrl+Maj+S** (Enregistrer sous) et entrée « Enregistrer sous… » dans le menu Applications.
- La fenêtre de saisie du nom explique que l'extension `.fa` est ajoutée ; un nom vide est signalé au lieu d'être ignoré.

## [0.7.2] — 2026-10-26

### Ajouté
- **Installer un paquet dans l'éditeur** : menu Applications → 📦 Paquets… (installer depuis GitHub ou une adresse https, utiliser, retirer ; paquets connus en un clic). Mêmes règles de sécurité que `fadera paquet`.

## [0.7.1] — 2026-10-26

### Ajouté
- **`parler(texte)`** : le programme parle en français et renvoie le texte. Voix du système dans `fadera.exe` (Windows, macOS, Linux),
  voix du navigateur dans le terrain de jeu (après un clic sur la page). Sans voix installée, le programme continue en silence.
  Le bouton ■ Stop coupe la parole. Aucun nouveau mot-clé.
- **Dix exemples de robotique** (31 à 40) : robot qui marche en parlant, qui présente Fadéra, qui court, qui lit, qui fait le ménage, navigation dans un labyrinthe, capteur et régulation, bras articulé, machine à états, essaim de robots.

### Modifié
- Les messages vus par les utilisateurs parlent de « moteur Fadéra » et de « moteur de secours » ; les noms Rust et JavaScript ne restent que
  dans la commande `moteur` (vue développeur) et la documentation technique. `fadera --version` affiche « Fadéra 0.7.1 (moteur Fadéra) ».

### Ajouté (suite)
- `docs/PAQUETS_CONNUS.md` : la liste des paquets publiés (`geometrie`, `notes`) et comment ajouter le sien.
- Banc d'essai de vitesse du moteur Rust (`fadera-rs/tests/vitesse.js`), lancé par `node verifier_tout.js` et donc par la CI :
  12 programmes avec des limites très larges, pour détecter une régression catastrophique de performance.

## [0.7.0] — 2026-10-25

### Ajouté
- **Gestionnaire de paquets** : `fadera paquet installer | lister | retirer | creer`. Un paquet est un dossier de fichiers Fadéra avec un
  `fadera-paquet.json` ; il s'installe depuis GitHub (`utilisateur/depot@version`), une adresse web ou un dossier, dans `paquets/<nom>/`,
  est noté dans `fadera.json` et s'utilise avec `importer`. Aucun nouveau mot-clé. Installation sûre : noms et chemins contrôlés, extensions
  limitées, tout est vérifié avant la première écriture (voir `docs/PAQUETS.md`).
- **Accès réseau dans la ligne de commande** : `http_obtenir`, `http_json` et `http_envoyer` fonctionnent dans `fadera.exe` (jusqu'ici, seul
  le terrain de jeu en navigateur y avait accès). Délai de 20 s, réponse limitée à 20 Mo, prise en charge du proxy d'entreprise.
- **En-têtes facultatifs** pour `http_obtenir(adresse, entetes)` et `http_json(adresse, entetes)`, comme pour `http_envoyer` : on peut
  désormais envoyer une clé d'accès (`Authorization`) avec une simple lecture. Identique dans les deux moteurs.
- Guide `docs/SERVICES.md` : interroger une API, une base de données hébergée (Supabase, Airtable…) ou tout service en ligne.
- Test `fadera-rs/tests/verifier_paquets.js` (29 vérifications, sans internet) branché dans `node verifier_tout.js` et donc dans la CI.

### Modifié
- Nouvelle dépendance du binaire en ligne de commande : `ureq` (client HTTP, absent de la version WebAssembly).

## [0.6.0] — 2026-10-24

Documentation complète.

### Ajouté
- Documentation (`docs/`) : tutoriel en français en 17 chapitres (`docs/TUTORIEL.md`), 30 exemples complets avec leur sortie attendue
  (calculs, jeux, textes, CSV et JSON, dates, réseau, classes, tests), référence de toutes les fonctions natives et des 12 modules
  standard (`docs/REFERENCE.md`). Les exemples et les résultats affichés sont exécutés sur les deux moteurs ; la CI échoue si une
  fonction native n'est pas documentée ou si un exemple donne des résultats différents en JS et en Rust.

### Corrigé
- Test de l'IDE : ne plante plus quand la clé de stockage du navigateur est absente.

## [0.5.0] — 2026-10-23

Première version publiée du dépôt.

### Ajouté
- Langage : modules (`importer "fichier"`, `importer "m" comme o`), classes (`classe C herite P … fin`, `soi`, `creer`), appels chaînés,
  blocs `tester "nom" … fin` avec `verifier`, `verifier_egal`, `verifier_erreur`.
- Bibliothèque standard en Fadéra, `importer "std/…"` : dates, JSON, expressions régulières, maths, tri et recherche,
  pile, file, dictionnaire ordonné, ensemble, tas. Natif `lever(message)`.
- IDE (`index.html`) : onglets, recherche et remplacement, sauvegarde durable (IndexedDB), export et import .zip,
  coloration et complétion fondées sur l'analyseur du langage, formateur (`Maj+Alt+F`), partage par lien (`#p=…`).
- Ligne de commande : `fadera run fichier.fa`, `fadera fmt`, `fadera tester`, `fadera deboguer`, `fadera -e "code"`, mode interactif.
- Distribution : exécutables Linux, Windows (`fadera.exe`) et macOS construits par la CI, playground sur GitHub Pages,
  intégration continue (tests JS et Rust, parité JS/Rust, formateur, débogueur, spécification exécutable).

### Corrigé
- Formateur : `dans [1, 2]` et `pas -1` écrits de la même façon par les deux moteurs.

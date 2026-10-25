# Paquets connus

Les paquets que vous pouvez installer dès maintenant avec `fadera paquet installer`. Comment ça marche : voir [PAQUETS.md](PAQUETS.md).

| Paquet | Pour quoi faire | Installation |
|---|---|---|
| [**geometrie**](https://github.com/Faderaseny/geometrie) | Aires, périmètres et distances : rectangle, cercle, triangle (formule de Héron, Pythagore), polygones. | `fadera paquet installer Faderaseny/geometrie` |
| [**notes**](https://github.com/Faderaseny/notes) | Moyennes pondérées, mentions, classement avec ex aequo, bulletins mis en forme et statistiques de classe, pour des notes sur 20. | `fadera paquet installer Faderaseny/notes` |

Chaque paquet a son mode d'emploi (`LISEZMOI.md`) et ses tests (`fadera tester paquets/<nom>/tests.fa`). Lisez le code d'un paquet avant de l'utiliser : il s'exécute avec vos droits.

## Ajouter le vôtre à cette liste

1. Créez votre paquet : `fadera paquet creer mon-paquet`.
2. Vérifiez qu'il passe ses tests : `fadera tester mon-paquet/tests.fa`.
3. Publiez-le dans un dépôt GitHub public (le contenu du dossier à la racine du dépôt).
4. Testez l'installation depuis un dossier vide : `fadera paquet installer votre-nom/mon-paquet`.
5. Proposez son ajout à cette page : ouvrez une « demande d'extraction » (pull request) ou un « problème » (issue) sur le dépôt de Fadéra, avec son adresse et une phrase de description.

Un bon paquet : un nom clair, un `LISEZMOI.md` avec un exemple qui marche, des tests qui passent, et une licence.

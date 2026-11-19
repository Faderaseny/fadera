# Mini-site : une liste de tâches en Fadéra

Un site web complet en deux fichiers : `site.fa` (le serveur, environ 70 lignes) et `index.html` (la page).

```
fadera site.fa
```
Puis ouvrez http://localhost:3000. Les tâches sont enregistrées dans `taches.json` à côté du programme et retrouvées au redémarrage.

Variables facultatives : `PORT` (port, 3000 par défaut) et `DONNEES_TACHES` (chemin du fichier de sauvegarde).

Ce programme montre : `serveur_http`, un gestionnaire qui aiguille selon la méthode et le chemin, des réponses JSON, l'envoi d'un fichier, la lecture d'un corps de requête JSON, une écriture « tout ou rien » (`fichier_ecrire_sur`) et la lecture des variables d'environnement. Le chapitre 18 du tutoriel l'explique pas à pas. Pour aller plus loin (comptes, mots de passe, plusieurs utilisateurs), voir les fonctions `crypto_scrypt` et `crypto_hmac_base64url` dans `docs/SERVEUR.md`.

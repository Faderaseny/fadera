# Écrire un serveur web (et manipuler des dossiers) en Fadéra

Ces fonctions n'existent que dans la **ligne de commande** (`fadera programme.fa`) : un navigateur ne peut ni ouvrir un port ni lire un dossier, et répond alors par une erreur claire. Elles servent à écrire une vraie application réseau ; l'exemple fourni est `applications/mini-site` (chapitre 18 du tutoriel).

## Un serveur en 10 lignes

```
fonction gestionnaire(req)
  si req.chemin == "/" alors
    renvoyer { statut: 200, entetes: { "Content-Type": "text/html; charset=utf-8" }, corps: "<h1>Bonjour !</h1>" }
  fin
  renvoyer { statut: 404, corps: "Introuvable" }
fin

afficher "Ouvrez http://localhost:3000"
serveur_http(3000, gestionnaire)
```

`serveur_http(port, gestionnaire, options)` écoute sur le port et ne rend jamais la main. Les requêtes sont traitées **l'une après l'autre**. Le gestionnaire reçoit `{ methode, chemin, requete, entetes, corps, ip }` (`requete` : les paramètres de l'adresse, `entetes` : en-têtes) et renvoie `{ statut, entetes, corps }`. Pour envoyer un fichier tel quel (image, page, feuille de style), renvoyez `{ statut: 200, entetes: {…}, fichier: "chemin" }`. Le compteur d'étapes est remis à zéro à chaque requête : une requête trop longue ne bloque pas les suivantes. Un gestionnaire qui plante ne tue pas le serveur : le visiteur reçoit une erreur 400 avec le message en JSON, et l’erreur s’affiche dans la console. `options` (facultatif) : `{ limite_corps: octets }`, 100 Mo par défaut.

## Dossiers et fichiers

| Fonction | Rôle |
|---|---|
| `fichier_existe(chemin)`, `dossier_existe(chemin)` | vrai / faux |
| `dossier_creer(chemin)` | crée le dossier et ses parents (sans erreur s'il existe) |
| `dossier_lister(chemin)` | noms triés des éléments d'un dossier |
| `fichier_supprimer(chemin)` | supprime un fichier |
| `dossier_supprimer(chemin, tout)` | supprime un dossier ; avec `vrai`, aussi son contenu |
| `fichier_renommer(ancien, nouveau)` | renomme ou déplace |
| `fichier_copier(source, destination)` | copie un fichier |
| `fichier_ecrire_sur(chemin, texte)` | écriture « tout ou rien » : une coupure de courant ne laisse jamais un fichier à moitié écrit |

## Environnement

| Fonction | Rôle |
|---|---|
| `environnement(nom)` | valeur d'une variable d'environnement, ou « rien » |
| `arguments()` | arguments après le nom du programme : `fadera serveur.fa --aide` → `["--aide"]` |
| `dossier_programme()` | dossier qui contient le programme en cours |
| `adresses_reseau()` | adresses IPv4 de la machine sur le réseau local |
| `arreter(code)` | quitte le programme avec un code de sortie |

## Mots de passe et jetons

| Fonction | Rôle |
|---|---|
| `crypto_scrypt(mot_de_passe, sel)` | empreinte scrypt (N=16384, r=8, p=1) en hexadécimal ; le sel est lu comme du texte, comme `crypto.scryptSync` de Node |
| `crypto_hmac_base64url(texte, cle)` | HMAC-SHA-256 en base64url sans remplissage (jetons signés) |
| `crypto_egal_constant(a, b)` | compare deux textes sans révéler la position de la première différence |
| `crypto_totp(secret_base32)` ou `crypto_totp(secret_base32, temps_ms)` | code à 6 chiffres à usage unique (TOTP, RFC 6238 : pas de 30 s, HMAC-SHA-1), celui que calculent Google Authenticator, Microsoft Authenticator ou Aegis ; le secret est du base32 (16 caractères ou plus) ; sans second argument, l'heure actuelle est utilisée |
| `crypto_aleatoire_entier(n)` | entier aléatoire cryptographique dans [0, n[ |
| `base64url_encoder(texte)`, `base64url_decoder(texte)` | encode / décode ; le décodage renvoie « rien » si l'entrée est invalide |

Ne stockez jamais un mot de passe en clair : gardez `crypto_scrypt(mot_de_passe, sel)` avec un sel aléatoire (`crypto_aleatoire_hex(16)`), et comparez avec `crypto_egal_constant`.

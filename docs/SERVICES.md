# Parler à des services en ligne

Fadéra n'a pas de pilote pour chaque base de données, mais presque tous les services modernes (bases hébergées, météo, cartes, paiement, messagerie, intelligence artificielle…) proposent une **API web** : des adresses qu'on interroge en HTTP et qui répondent en JSON. Trois fonctions suffisent :

| Fonction | Usage |
|---|---|
| `http_json(adresse, entetes)` | lire des données JSON (GET) |
| `http_obtenir(adresse, entetes)` | lire du texte brut ou une page (GET) |
| `http_envoyer(adresse, donnees, entetes)` | envoyer des données (POST) ; renvoie `{ statut, ok, corps }` |

`entetes` est facultatif : une structure comme `{ Authorization: "Bearer MON_JETON" }`. Ces fonctions marchent dans la ligne de commande **et** dans le terrain de jeu (où le site visé doit autoriser les appels depuis un navigateur, règle CORS).

Comme internet peut être coupé, entourez toujours les appels d'un `essayer … attraper`.

## Lire des données

```
essayer
  soit meteo = http_json("https://api.open-meteo.com/v1/forecast?latitude=14.69&longitude=-17.44&current_weather=true")
  afficher lireCle(lireCle(meteo, "current_weather"), "temperature") + " °C à Dakar"
attraper erreur
  afficher "Service injoignable : " + erreur
fin
```

## Avec une clé d'accès

```
soit jeton = saisir("Votre jeton : ")
soit entetes = { Authorization: "Bearer " + jeton, "X-Version": "2" }
soit reponse = http_json("https://api.exemple.org/moi", entetes)
```

Ne mettez jamais une clé secrète en dur dans un programme que vous partagez : demandez-la avec `saisir`, ou lisez-la dans un fichier non publié avec `lireFichier`.

## Envoyer des données

```
soit rep = http_envoyer("https://api.exemple.org/messages", { texte: "Bonjour" }, { Authorization: "Bearer " + jeton })
si rep.ok alors
  afficher "Envoyé."
sinon
  afficher "Refusé (code " + rep.statut + ") : " + rep.corps
fin
```

Une liste ou une structure est envoyée en JSON automatiquement.

## Utiliser une base de données

Les bases hébergées (Supabase, Airtable, Firebase, Directus, PocketBase…) exposent leurs tables par une API web : lire une table, c'est un `http_json`, ajouter une ligne, c'est un `http_envoyer`. Exemple avec une API de type PostgREST (Supabase) :

```
soit base = "https://MON-PROJET.supabase.co/rest/v1"
soit cles = { apikey: jeton, Authorization: "Bearer " + jeton }

# lire les 10 premiers clients de Dakar
soit clients = http_json(base + "/clients?ville=eq.Dakar&limit=10", cles)
pour chaque c dans clients faire
  afficher lireCle(c, "nom")
fin

# ajouter un client
http_envoyer(base + "/clients", { nom: "Awa", ville: "Dakar" }, cles)
```

## Ce que Fadéra ne fait pas (encore)

- Pas de pilote natif SQL (PostgreSQL, MySQL, SQLite) : il faudrait un module compilé pour chaque base. Passez par l'API web de la base, ou par un petit service intermédiaire.
- Seulement les méthodes GET et POST : pas encore PUT, PATCH ni DELETE (certaines API les exigent).
- Pas de connexions longues (WebSocket) ni de serveur : Fadéra n'écoute rien : il appelle.
- Le texte de la réponse est limité à 20 Mo, et une requête à 20 secondes.

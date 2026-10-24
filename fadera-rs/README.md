# Fadéra — moteur Rust

Port Rust du moteur de référence (celui de `index.html`) : lexer, parser, interpréteur, ~130 fonctions natives,
vrai chiffrement (AES-256-GCM, RSA-OAEP 2048, SHA-256/512, HMAC), terminal en ligne de commande.

## Compiler et utiliser (Linux / macOS / Windows)

```
cargo build --release
./target/release/fadera programme.fa      # exécuter un fichier
./target/release/fadera -e 'afficher 2+3' # exécuter du code
./target/release/fadera                   # mode interactif
```

## Vérifier que Rust et JavaScript donnent les mêmes résultats

```
cargo test --release                      # tests Rust (moteur, CLI, chiffrement, bibliothèque standard, formateur)
TZ=UTC node tests/parite.js ../index.html # compare les deux moteurs sur ~380 programmes (TZ=UTC : le JS lit l'heure locale)
node tests/crypto_interop.js ../index.html# ce que JS chiffre, Rust le déchiffre (et inversement)
```
`parite.js` utilise les tests de `tests_fadera.js` et les exercices des fichiers `EXERCICES_*.md` :
placez-les dans le dossier parent de `fadera-rs`.

## Pour le navigateur (WebAssembly) — à faire sur votre machine

Deux points d'entrée : `executer(code, saisies)` (simple, synchrone, sans fichiers) et
`executer_hote(code, hote)` pour un **Web Worker** : `hote` est un objet JS avec `afficher`, `saisir`, `lire`, `ecrire`,
`pause`, `http` (synchrones, bloquants via `Atomics.wait`). L'IDE les branche (voir GUIDE §20.1, `serveur.py`).

```
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build --release --target web -- --features wasm
```
Cela produit `pkg/` avec la fonction `executer(code, saisies)` qui renvoie du JSON
`{"sortie": [...], "erreur": null | "Ligne N : ...", "source": "..."}`. Le code de liaison (`src/wasm.rs`) vérifie
sa compilation de type (`cargo check --features wasm`), mais la compilation WebAssembly elle-même n'a pas pu être
exécutée dans l'environnement où il a été écrit (la cible wasm32 n'y était pas téléchargeable).

## Modules standard (`src/modules.rs`)
Dates, texte, aléatoire, listes, CSV, temps, HTTP : 34 fonctions portées du moteur JS, mêmes noms et mêmes messages d'erreur.
Dépendance ajoutée : `unicode-normalization` (pour `texte_sans_accents` / `texte_slug`).

## Différences connues avec le moteur JavaScript
- Expressions régulières : bibliothèque Rust (pas de lookahead / références arrière).
- Dates : `formater_date` et tous les `date_*` utilisent l'heure UTC (le moteur JS, l'heure locale du navigateur).
- `date_analyser` : sous-ensemble ISO 8601 (`2026-10-14`, `2026-10-14T10:30:00Z`, `…+02:00`, `2026-10-14 08:00`) ; le JS accepte en plus
  d'autres écritures de V8 (« Oct 14 2026 »…).
- `alea_*` : octets aléatoires du système (comme le JS) ; repli sur le générateur interne si la fonctionnalité `crypto` est absente.
- `http_obtenir`, `http_json`, `http_envoyer` : le binaire en ligne de commande n'a pas de client HTTP (aucune dépendance réseau ajoutée) ;
  ils répondent « réseau indisponible ». L'hôte WebAssembly/embarqué peut les fournir en implémentant `Hote::http`.
- `pause` : bloque le thread (10 s maximum) ; sans effet en WebAssembly.
- `trier` sur du texte : ordre alphabétique français approché (accents, casse), pas ICU complet.
- Longueurs de texte comptées en caractères Unicode (le JS compte en unités UTF-16 : différence uniquement pour les émojis).
- Messages d'erreur de `json_lire` : texte propre à Rust.

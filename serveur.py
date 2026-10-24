#!/usr/bin/env python3
"""Serveur local pour l'IDE Fadéra : comme « python -m http.server », mais avec les en-têtes
d'isolation « cross-origin » qui activent le Worker (saisir / fichiers / réseau / bouton Stop avec le moteur Rust).

Usage :  python serveur.py [port]      puis ouvrir  http://localhost:8000
"""
import http.server
import os
import socketserver
import sys

# Sert toujours le dossier où se trouve serveur.py (quel que soit le dossier d'où on le lance).
DOSSIER = os.path.dirname(os.path.abspath(__file__))
os.chdir(DOSSIER)

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8000


class Handler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".wasm": "application/wasm", ".js": "text/javascript", ".mjs": "text/javascript"}

    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "credentialless")
        self.send_header("Cross-Origin-Resource-Policy", "same-origin")
        self.send_header("Cache-Control", "no-cache")
        super().end_headers()


class Serveur(socketserver.ThreadingTCPServer):
    # Sous Windows, SO_REUSEADDR autorise un 2e serveur sur le même port (et l'ancien continue de répondre) : on l'évite.
    allow_reuse_address = os.name != "nt"
    daemon_threads = True


if __name__ == "__main__":
    try:
        httpd = Serveur(("", PORT), Handler)
    except OSError:
        print("Le port %d est déjà utilisé (un ancien serveur tourne encore ?). Fermez l'autre fenêtre PowerShell, ou essayez : python serveur.py %d" % (PORT, PORT + 1))
        sys.exit(1)
    with httpd:
        print("Dossier servi : " + DOSSIER)
        print("Moteur Rust (pkg/fadera.js) : " + ("trouvé" if os.path.exists(os.path.join(DOSSIER, "pkg", "fadera.js")) else "INTROUVABLE — copiez le dossier pkg à côté de index.html"))
        print("Fadéra : http://localhost:%d  (isolation cross-origin activée — Ctrl+C pour arrêter)" % PORT)
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nArrêt.")

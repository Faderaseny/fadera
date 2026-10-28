# Icône et ouverture des fichiers `.fa` sous Windows

1. Placez `fadera.ico` et `associer_fadera.ps1` dans le même dossier (celui de ce dépôt : `windows\`).
2. Dans PowerShell :

       powershell -ExecutionPolicy Bypass -File .\windows\associer_fadera.ps1

   Par défaut `fadera.exe` est cherché dans `C:\Outils\`. Ailleurs : `-Exe "D:\chemin\fadera.exe"`.
3. Les fichiers `.fa` affichent l'icône Fadéra ; le double-clic les exécute dans un terminal.

Aucun droit administrateur : tout est écrit pour votre compte. Pour annuler : `retirer_association.ps1`.

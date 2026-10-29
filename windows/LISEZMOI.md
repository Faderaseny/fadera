# Fadéra sous Windows

## Le plus simple : l'installateur
Téléchargez `Fadera-Installateur-<version>.exe` (page *Releases* du dépôt) et double-cliquez dessus. Il propose le dossier d'installation,
un raccourci sur le Bureau, l'ajout au PATH et l'association des fichiers `.fa`, puis ouvre Fadéra IDE.
Aucun droit administrateur n'est nécessaire (installation pour votre compte).

## Sans installateur : icône et ouverture des `.fa`
1. Placez `fadera.ico` et `associer_fadera.ps1` dans le même dossier (celui de ce dépôt : `windows\`).
2. Dans PowerShell :

       powershell -ExecutionPolicy Bypass -File .\windows\associer_fadera.ps1

   Par défaut `fadera.exe` est cherché dans `C:\Outils\`. Ailleurs : `-Exe "D:\chemin\fadera.exe"`.
3. Les fichiers `.fa` affichent l'icône Fadéra ; le double-clic les exécute dans un terminal.

Pour annuler : `retirer_association.ps1`.

## Fichiers de ce dossier
- `installateur\fadera.iss` : script Inno Setup de l'installateur (construit par le workflow de publication) ;
- `fadera.ico` : icône ; `associer_fadera.ps1` / `retirer_association.ps1` : association manuelle.

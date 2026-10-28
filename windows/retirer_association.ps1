# Retire l'association .fa creee par associer_fadera.ps1 (utilisateur courant).
Remove-Item -Path "HKCU:\Software\Classes\.fa" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path "HKCU:\Software\Classes\Fadera.Programme" -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "Association .fa retiree."

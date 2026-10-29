# Associe les fichiers .fa a Fadera (icone + double-clic) pour l'utilisateur courant.
# Aucun droit administrateur requis : tout est ecrit dans HKCU.
# Usage :  powershell -ExecutionPolicy Bypass -File associer_fadera.ps1 [-Exe "C:\Outils\fadera.exe"]
param([string]$Exe = "C:\Outils\fadera.exe")

if (-not (Test-Path $Exe)) { Write-Host "fadera.exe introuvable : $Exe  (utilisez -Exe pour indiquer son emplacement)"; exit 1 }
$dossier = Split-Path -Parent $Exe
$ico = Join-Path $dossier "fadera.ico"
$source = Join-Path $PSScriptRoot "fadera.ico"
if (Test-Path $source) { Copy-Item $source $ico -Force }
if (-not (Test-Path $ico)) { Write-Host "fadera.ico introuvable (placez-le a cote de ce script)."; exit 1 }

$cle = "HKEY_CURRENT_USER\Software\Classes"
function Def($chemin, $valeur) { [Microsoft.Win32.Registry]::SetValue("$cle\$chemin", "", $valeur) }
Def ".fa" "Fadera.Programme"
Def "Fadera.Programme" "Programme Fadera"
Def "Fadera.Programme\DefaultIcon" "$ico,0"
Def "Fadera.Programme\shell\open\command" "cmd.exe /k `"`"$Exe`" run `"%1`"`""
# Retirer un ancien choix utilisateur qui masquerait l'association
Remove-Item "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\.fa" -Recurse -Force -ErrorAction SilentlyContinue

# Prevenir l'Explorateur que les associations ont change
Add-Type -Namespace Win32 -Name Shell -MemberDefinition '[DllImport("shell32.dll")] public static extern void SHChangeNotify(int e, uint f, IntPtr a, IntPtr b);'
[Win32.Shell]::SHChangeNotify(0x08000000, 0, [IntPtr]::Zero, [IntPtr]::Zero)
try { & ie4uinit.exe -show } catch {}
Write-Host "OK : les fichiers .fa ont maintenant l'icone Fadera et s'ouvrent avec Fadera."
Write-Host "Si l'icone n'apparait pas tout de suite : fermez/rouvrez l'Explorateur ou redemarrez la session."

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

$base = "HKCU:\Software\Classes"
New-Item -Path "$base\.fa" -Force | Out-Null
Set-ItemProperty -Path "$base\.fa" -Name "(default)" -Value "Fadera.Programme"
New-Item -Path "$base\Fadera.Programme" -Force | Out-Null
Set-ItemProperty -Path "$base\Fadera.Programme" -Name "(default)" -Value "Programme Fadera"
New-Item -Path "$base\Fadera.Programme\DefaultIcon" -Force | Out-Null
Set-ItemProperty -Path "$base\Fadera.Programme\DefaultIcon" -Name "(default)" -Value "`"$ico`""
New-Item -Path "$base\Fadera.Programme\shell\open\command" -Force | Out-Null
Set-ItemProperty -Path "$base\Fadera.Programme\shell\open\command" -Name "(default)" -Value "cmd.exe /k `"`"$Exe`" run `"%1`"`""

# Prevenir l'Explorateur que les associations ont change
Add-Type -Namespace Win32 -Name Shell -MemberDefinition '[DllImport("shell32.dll")] public static extern void SHChangeNotify(int e, uint f, IntPtr a, IntPtr b);'
[Win32.Shell]::SHChangeNotify(0x08000000, 0, [IntPtr]::Zero, [IntPtr]::Zero)
Write-Host "OK : les fichiers .fa ont maintenant l'icone Fadera et s'ouvrent avec Fadera."
Write-Host "Si l'icone n'apparait pas tout de suite : fermez/rouvrez l'Explorateur ou redemarrez la session."

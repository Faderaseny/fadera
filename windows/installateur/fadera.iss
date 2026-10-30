; Installateur Windows de Fadéra (Inno Setup 6) — produit « Fadera-Installateur-<version>.exe ».
; Construction (voir le workflow « Publication d'une version ») :
;   iscc /DVersion=0.7.8 /DDossierSource=dist\installation windows\installateur\fadera.iss
; Le dossier source contient : fadera.exe, fadera.ico, LICENSE, README.md, GUIDE_FADERA.md, CHANGELOG.md,
; examples\ (facultatif) et ide\ (index.html + pkg\ : l'éditeur et le moteur WebAssembly).

#ifndef Version
  #define Version "0.0.0"
#endif
#ifndef DossierSource
  #define DossierSource "..\..\dist\installation"
#endif

[Setup]
AppId={{6B2F7C1E-4D0A-4B7A-9C2E-FADE5A0001A7}
AppName=Fadéra
AppVersion={#Version}
AppVerName=Fadéra {#Version}
AppPublisher=Faderaseny
AppPublisherURL=https://github.com/Faderaseny/fadera
AppSupportURL=https://github.com/Faderaseny/fadera/issues
AppUpdatesURL=https://github.com/Faderaseny/fadera/releases
DefaultDirName={autopf}\Fadera
DefaultGroupName=Fadéra
DisableProgramGroupPage=yes
DisableDirPage=no
; Sans droits administrateur par défaut (installation pour l'utilisateur) ; la boîte de lancement permet de choisir « pour tous ».
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputBaseFilename=Fadera-Installateur-{#Version}
OutputDir={#SourcePath}\..\..\dist\installateur
SetupIconFile={#DossierSource}\fadera.ico
UninstallDisplayIcon={app}\fadera.ico
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
; Images aux couleurs de Fadéra (bleu nuit #0F1B33, orange #F2A93B), une par niveau de zoom de l'écran
WizardImageFile=assistant-grand-100.bmp,assistant-grand-125.bmp,assistant-grand-150.bmp,assistant-grand-200.bmp
WizardSmallImageFile=assistant-petit-100.bmp,assistant-petit-125.bmp,assistant-petit-150.bmp,assistant-petit-200.bmp
ChangesAssociations=yes
ChangesEnvironment=yes
LicenseFile={#DossierSource}\LICENSE
ArchitecturesInstallIn64BitMode=x64compatible

[Languages]
Name: "french"; MessagesFile: "compiler:Languages\French.isl"

[Tasks]
Name: "icone_bureau"; Description: "Créer un raccourci « Fadéra IDE » sur le &Bureau"; GroupDescription: "Raccourcis :"
Name: "associer"; Description: "Ouvrir les fichiers &.fa avec l'éditeur Fadéra (double-clic) et leur donner l'icône Fadéra"; GroupDescription: "Fichiers :"
Name: "chemin"; Description: "Ajouter Fadéra au &PATH (pour taper « fadera » dans un terminal)"; GroupDescription: "Ligne de commande :"

[Files]
Source: "{#DossierSource}\fadera.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#DossierSource}\Fadera-IDE.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#DossierSource}\fadera.ico"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#DossierSource}\ide\*"; DestDir: "{app}\ide"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "{#DossierSource}\LICENSE"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#DossierSource}\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#DossierSource}\GUIDE_FADERA.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#DossierSource}\CHANGELOG.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#DossierSource}\examples\*"; DestDir: "{app}\examples"; Flags: ignoreversion recursesubdirs createallsubdirs skipifsourcedoesntexist

[Icons]
Name: "{autoprograms}\Fadéra\Fadéra IDE"; Filename: "{app}\Fadera-IDE.exe"; IconFilename: "{app}\fadera.ico"
Name: "{autoprograms}\Fadéra\Fadéra (terminal)"; Filename: "{cmd}"; Parameters: "/K ""{app}\fadera.exe"" --version"; WorkingDir: "{userdocs}"; IconFilename: "{app}\fadera.ico"
Name: "{autoprograms}\Fadéra\Désinstaller Fadéra"; Filename: "{uninstallexe}"
Name: "{autodesktop}\Fadéra IDE"; Filename: "{app}\Fadera-IDE.exe"; IconFilename: "{app}\fadera.ico"; Tasks: icone_bureau

[Registry]
Root: HKA; Subkey: "Software\Classes\.fa"; ValueType: string; ValueName: ""; ValueData: "Fadera.Programme"; Flags: uninsdeletevalue; Tasks: associer
Root: HKA; Subkey: "Software\Classes\Fadera.Programme"; ValueType: string; ValueName: ""; ValueData: "Programme Fadera"; Flags: uninsdeletekey; Tasks: associer
Root: HKA; Subkey: "Software\Classes\Fadera.Programme\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\fadera.ico,0"; Tasks: associer
; double-clic : ouvre le fichier dans l'éditeur
Root: HKA; Subkey: "Software\Classes\Fadera.Programme\shell\open"; ValueType: string; ValueName: ""; ValueData: "Ouvrir dans l'éditeur Fadéra"; Tasks: associer
Root: HKA; Subkey: "Software\Classes\Fadera.Programme\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\Fadera-IDE.exe"" ""%1"""; Tasks: associer
; clic droit : exécuter dans un terminal
Root: HKA; Subkey: "Software\Classes\Fadera.Programme\shell\executer"; ValueType: string; ValueName: ""; ValueData: "Exécuter avec Fadéra"; Tasks: associer
Root: HKA; Subkey: "Software\Classes\Fadera.Programme\shell\executer\command"; ValueType: string; ValueName: ""; ValueData: "cmd.exe /K """"{app}\fadera.exe"" run ""%1"""""; Tasks: associer
; PATH (utilisateur ou machine selon le mode d'installation)
Root: HKA; Subkey: "{code:CleEnvironnement}"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Check: DoitAjouterAuPath(ExpandConstant('{app}')); Tasks: chemin

[Run]
Filename: "{app}\Fadera-IDE.exe"; Description: "Lancer Fadéra IDE maintenant"; Flags: postinstall nowait skipifsilent

[Code]

const
  // couleurs de Fadéra au format Delphi $00BBGGRR
  CFond = $331B0F;    // bleu nuit  #0F1B33
  CBande = $26140B;   // bandeau    #0B1426
  CTexte = $F5EDE8;   // texte      #E8EDF5
  CDiscret = $D0B7A9; // texte discret #A9B7D0
  COrange = $3BA9F2;  // orange     #F2A93B

procedure Clair(L: TNewStaticText; Couleur: TColor);
begin
  L.Font.Color := Couleur;
end;

procedure InitializeWizard;
begin
  WizardForm.Color := CFond;
  WizardForm.MainPanel.Color := CBande;
  WizardForm.InnerPage.Color := CFond;
  WizardForm.WelcomePage.Color := CFond;
  WizardForm.FinishedPage.Color := CFond;
  WizardForm.LicensePage.Color := CFond;
  WizardForm.SelectDirPage.Color := CFond;
  WizardForm.SelectTasksPage.Color := CFond;
  WizardForm.ReadyPage.Color := CFond;
  WizardForm.InstallingPage.Color := CFond;

  // titres en orange, textes en clair
  Clair(WizardForm.PageNameLabel, COrange);
  Clair(WizardForm.PageDescriptionLabel, CTexte);
  Clair(WizardForm.WelcomeLabel1, COrange);
  Clair(WizardForm.WelcomeLabel2, CTexte);
  Clair(WizardForm.FinishedHeadingLabel, COrange);
  Clair(WizardForm.FinishedLabel, CTexte);
  Clair(WizardForm.LicenseLabel1, CTexte);
  Clair(WizardForm.SelectDirLabel, CTexte);
  Clair(WizardForm.SelectDirBrowseLabel, CTexte);
  Clair(WizardForm.DiskSpaceLabel, CDiscret);
  Clair(WizardForm.SelectTasksLabel, CTexte);
  Clair(WizardForm.ReadyLabel, CTexte);
  Clair(WizardForm.StatusLabel, CTexte);
  Clair(WizardForm.FilenameLabel, CDiscret);
  Clair(WizardForm.BeveledLabel, CDiscret);
  WizardForm.LicenseAcceptedRadio.Font.Color := CTexte;
  WizardForm.LicenseNotAcceptedRadio.Font.Color := CTexte;

  // cases à cocher : fond bleu nuit, texte clair, lignes un peu plus hautes (cases entières)
  WizardForm.TasksList.Color := CFond;
  WizardForm.TasksList.Font.Color := CTexte;
  WizardForm.TasksList.MinItemHeight := ScaleY(26);
  WizardForm.RunList.Color := CFond;
  WizardForm.RunList.Font.Color := CTexte;
  WizardForm.RunList.MinItemHeight := ScaleY(26);
  WizardForm.LicenseMemo.Color := CBande;
  WizardForm.LicenseMemo.Font.Color := CTexte;
  WizardForm.ReadyMemo.Color := CBande;
  WizardForm.ReadyMemo.Font.Color := CTexte;
end;

function CleEnvironnement(Param: string): string;
begin
  if IsAdminInstallMode then
    Result := 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment'
  else
    Result := 'Environment';
end;

function DoitAjouterAuPath(Dossier: string): Boolean;
var
  Existant: string;
begin
  if not RegQueryStringValue(HKA, CleEnvironnement(''), 'Path', Existant) then
  begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Uppercase(Dossier) + ';', ';' + Uppercase(Existant) + ';') = 0;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Existant, Dossier: string;
  P: Integer;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    Dossier := ExpandConstant('{app}');
    if RegQueryStringValue(HKA, CleEnvironnement(''), 'Path', Existant) then
    begin
      P := Pos(';' + Uppercase(Dossier), Uppercase(Existant));
      if P > 0 then
      begin
        Delete(Existant, P, Length(Dossier) + 1);
        RegWriteExpandStringValue(HKA, CleEnvironnement(''), 'Path', Existant);
      end;
    end;
  end;
end;

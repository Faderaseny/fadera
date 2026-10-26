//! Synthèse vocale de la ligne de commande : la fonction `parler("texte")`.
//!
//! Elle utilise la voix déjà présente sur l'ordinateur, en français quand elle existe :
//!   Windows : la voix du système (System.Speech, via PowerShell) ;
//!   macOS   : la commande `say` ;
//!   Linux   : `espeak-ng`, `espeak` ou `spd-say`, s'ils sont installés.
//! Sans voix disponible, `parler` ne fait rien (le programme continue), comme dans un terminal sans haut-parleur.
//! Le texte n'est jamais placé dans une commande de shell : il passe par une variable d'environnement, l'entrée standard
//! ou un argument après « -- », donc rien de ce qu'il contient ne peut être exécuté.
//! Les phrases se suivent dans l'ordre : `parler` attend la fin de la phrase précédente, puis lance la suivante
//! sans attendre qu'elle se termine (le programme continue pendant qu'elle est prononcée).
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

static EN_COURS: Mutex<Option<Child>> = Mutex::new(None);
const MAX_CARACTERES: usize = 2000;

/// Prononce `texte` (sans bloquer, sauf pour attendre la fin de la phrase précédente).
pub fn dire(texte: &str) {
    let texte: String = texte.chars().take(MAX_CARACTERES).collect();
    if texte.trim().is_empty() {
        return;
    }
    let mut verrou = match EN_COURS.lock() {
        Ok(v) => v,
        Err(e) => e.into_inner(),
    };
    if let Some(mut precedent) = verrou.take() {
        let _ = precedent.wait();
    }
    *verrou = lancer(&texte);
}

/// À appeler avant de quitter : laisse la dernière phrase se terminer.
pub fn attendre_fin() {
    let mut verrou = match EN_COURS.lock() {
        Ok(v) => v,
        Err(e) => e.into_inner(),
    };
    if let Some(mut c) = verrou.take() {
        let _ = c.wait();
    }
}

#[cfg(windows)]
fn lancer(texte: &str) -> Option<Child> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    const SCRIPT: &str = "Add-Type -AssemblyName System.Speech; \
        $v = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
        try { $v.SelectVoiceByHints([System.Speech.Synthesis.VoiceGender]::NotSet, [System.Speech.Synthesis.VoiceAge]::NotSet, 0, [System.Globalization.CultureInfo]::GetCultureInfo('fr-FR')) } catch {}; \
        $v.Speak($env:FADERA_PAROLE)";
    Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", SCRIPT])
        .env("FADERA_PAROLE", texte)
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .ok()
}

#[cfg(target_os = "macos")]
fn lancer(texte: &str) -> Option<Child> {
    use std::io::Write;
    for voix in [Some("Thomas"), None] {
        let mut c = Command::new("say");
        if let Some(v) = voix {
            c.args(["-v", v]);
        }
        c.args(["-f", "-"]).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null());
        if let Ok(mut enfant) = c.spawn() {
            if let Some(mut e) = enfant.stdin.take() {
                let _ = e.write_all(texte.as_bytes());
            }
            return Some(enfant);
        }
    }
    None
}

#[cfg(all(unix, not(target_os = "macos")))]
fn lancer(texte: &str) -> Option<Child> {
    let essais: [(&str, &[&str]); 3] = [("espeak-ng", &["-v", "fr", "--"]), ("espeak", &["-v", "fr", "--"]), ("spd-say", &["-l", "fr", "-w", "--"])];
    for (programme, args) in essais {
        if let Ok(c) = Command::new(programme).args(args).arg(texte).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
            return Some(c);
        }
    }
    None
}

#[cfg(not(any(windows, unix)))]
fn lancer(_texte: &str) -> Option<Child> {
    None
}

//! Fadéra IDE : fenêtre native (WebView2 sous Windows) qui affiche l'éditeur Fadéra.
//!
//! Fonctionnement :
//!  1. lance `fadera.exe ide --sans-navigateur --discret [fichier]` (le petit serveur local qui sert
//!     l'éditeur et transmet le fichier double-cliqué) ; s'il tourne déjà, il se contente de lui passer le fichier ;
//!  2. attend que le serveur réponde, puis ouvre une fenêtre « Fadéra » sans barre d'adresse ni onglets ;
//!  3. si WebView2 est absent ou échoue, retombe sur `fadera.exe ide` (fenêtre d'application Edge/Chrome).
//!
//! Aucune console ne s'affiche (sous-système graphique). Ailleurs que sous Windows, ce programme ne fait rien.
#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(not(windows))]
fn main() {
    eprintln!("Fadéra IDE (fenêtre native) n'existe que pour Windows. Sur ce système : fadera ide [fichier.fa]");
}

#[cfg(windows)]
mod fenetre {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::os::windows::process::CommandExt;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    const PORT: u16 = 4747;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    const ICONE: &[u8] = include_bytes!("icone.png");

    fn dossier_exe() -> PathBuf {
        std::env::current_exe()
            .ok()
            .and_then(|e| e.parent().map(|p| p.to_path_buf()))
            .unwrap_or_default()
    }

    fn serveur_repond() -> bool {
        let adresse = format!("127.0.0.1:{}", PORT).parse().unwrap();
        match TcpStream::connect_timeout(&adresse, Duration::from_millis(300)) {
            Ok(mut s) => {
                let _ = s.set_read_timeout(Some(Duration::from_secs(2)));
                let _ = s.write_all(
                    b"GET /__fadera HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n",
                );
                let mut r = String::new();
                let _ = s.read_to_string(&mut r);
                r.contains("fadera-ide")
            }
            Err(_) => false,
        }
    }

    fn lancer_moteur(
        fadera: &PathBuf,
        extra: &[&str],
        fichier: &Option<String>,
    ) -> Result<(), String> {
        let mut c = Command::new(fadera);
        c.arg("ide").args(extra);
        if let Some(f) = fichier {
            c.arg(f);
        }
        c.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(CREATE_NO_WINDOW);
        c.spawn().map(|_| ()).map_err(|e| e.to_string())
    }

    /// Note une ligne dans %LOCALAPPDATA%\Fadera\ide.log (pour comprendre un blocage : antivirus, port occupé…).
    /// Ne doit jamais empêcher le programme de continuer : toute erreur d'écriture est ignorée.
    fn journal(texte: &str) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let Some(base) = std::env::var_os("LOCALAPPDATA") else {
            return;
        };
        let dossier = PathBuf::from(base).join("Fadera");
        let _ = std::fs::create_dir_all(&dossier);
        let secondes = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dossier.join("ide.log"))
        {
            let _ = writeln!(f, "[{}] {}", secondes, texte);
        }
    }

    fn message(texte: &str) {
        // dernier recours : une boîte de dialogue simple, via PowerShell (aucune dépendance)
        let script = format!(
            "Add-Type -AssemblyName PresentationFramework; [System.Windows.MessageBox]::Show('{}', 'Fadéra IDE') | Out-Null",
            texte.replace('\'', "''")
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
            .creation_flags(CREATE_NO_WINDOW)
            .status();
    }

    fn icone() -> Option<tao::window::Icon> {
        let dec = png::Decoder::new(std::io::Cursor::new(ICONE));
        let mut lecteur = dec.read_info().ok()?;
        let mut tampon = vec![0u8; lecteur.output_buffer_size()];
        let info = lecteur.next_frame(&mut tampon).ok()?;
        if info.color_type != png::ColorType::Rgba || info.bit_depth != png::BitDepth::Eight {
            return None;
        }
        tampon.truncate(info.buffer_size());
        tao::window::Icon::from_rgba(tampon, info.width, info.height).ok()
    }

    fn ouvrir_fenetre(url: &str) -> Result<(), String> {
        use tao::dpi::LogicalSize;
        use tao::event::{Event, WindowEvent};
        use tao::event_loop::{ControlFlow, EventLoop};
        use tao::window::WindowBuilder;
        use wry::{WebContext, WebViewBuilder};

        let boucle = EventLoop::new();
        let fenetre = WindowBuilder::new()
            .with_title("Fadéra")
            .with_inner_size(LogicalSize::new(1360.0, 860.0))
            .with_min_inner_size(LogicalSize::new(720.0, 480.0))
            .with_window_icon(icone())
            .build(&boucle)
            .map_err(|e| e.to_string())?;
        // données du navigateur intégré (projets enregistrés, réglages) : dossier de l'utilisateur, jamais « Program Files »
        let donnees = std::env::var_os("LOCALAPPDATA")
            .map(|d| PathBuf::from(d).join("Fadera").join("WebView2"));
        let mut contexte = WebContext::new(donnees);
        let _vue = WebViewBuilder::with_web_context(&mut contexte)
            .with_url(url)
            .with_devtools(false)
            .build(&fenetre)
            .map_err(|e| e.to_string())?;
        boucle.run(move |evenement, _, flux| {
            *flux = ControlFlow::Wait;
            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } = evenement
            {
                *flux = ControlFlow::Exit;
            }
        });
    }

    pub fn demarrer() {
        let fichier = std::env::args()
            .skip(1)
            .find(|a| !a.starts_with("--"))
            .map(|f| {
                // chemin absolu : le moteur peut être lancé depuis un autre dossier
                std::fs::canonicalize(&f)
                    .map(|p| p.to_string_lossy().trim_start_matches(r"\\?\").to_string())
                    .unwrap_or(f)
            });
        let fadera = dossier_exe().join("fadera.exe");
        if !fadera.is_file() {
            message(&format!(
                "fadera.exe est introuvable ({}). Il a peut-être été supprimé ou mis en quarantaine par un antivirus : restaurez-le ou réinstallez Fadéra.",
                fadera.display()
            ));
            return;
        }
        if let Err(e) = lancer_moteur(&fadera, &["--sans-navigateur", "--discret"], &fichier) {
            journal(&format!(
                "moteur non démarré : {} — {}",
                fadera.display(),
                e
            ));
            message(&format!(
                "Impossible de démarrer le moteur Fadéra.\n\nRaison : {}\nFichier : {}\n\nSi la raison est « Accès refusé » ou « bloqué » : votre antivirus (Avast, Defender…) ou Windows bloque ce fichier. Ajoutez le dossier d'installation de Fadéra aux exceptions de l'antivirus, puis relancez.",
                e,
                fadera.display()
            ));
            return;
        }
        let debut = Instant::now();
        while !serveur_repond() {
            if debut.elapsed() > Duration::from_secs(10) {
                journal("l'éditeur n'a pas répondu sur le port 4747 en 10 secondes");
                message("L'éditeur n'a pas démarré (port 4747 occupé, éditeur introuvable ou moteur bloqué par l'antivirus). Essayez dans un terminal : fadera ide");
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let url = format!("http://127.0.0.1:{}/", PORT);
        if let Err(e) = ouvrir_fenetre(&url) {
            // WebView2 absent : repli sur une fenêtre d'application Edge/Chrome
            journal(&format!("fenêtre native impossible : {}", e));
            let _ = lancer_moteur(&fadera, &["--discret"], &fichier);
        }
    }
}

#[cfg(windows)]
fn main() {
    fenetre::demarrer();
}

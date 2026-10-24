//! Fadéra — moteur du langage de programmation en français (port Rust du moteur de référence JavaScript).

pub mod ast;
pub mod crypto;
pub mod debogueur;
pub mod erreur;
pub mod espaces;
pub mod fmt;
pub mod interp;
pub mod mathfn;
pub mod lexer;
pub mod modules;
pub mod modules_std;
pub mod vm;
pub mod natives;
pub mod parser;
pub mod stdlib;
pub mod util;
pub mod valeur;
#[cfg(feature = "wasm")]
pub mod wasm;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use erreur::{ErreurFadera, Flow};
pub use interp::{lancer_tests, BilanTests, analyser_code, analyser_code_module, decrire_erreur, executer_programme, executer_programme_complet, Hote, HoteErr, Interp};
pub use valeur::{formater, Valeur};

/// Exécute `f` dans un thread à grande pile (la récursion Fadéra peut aller jusqu'à 5000 appels imbriqués).
pub fn avec_grande_pile<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(1 << 30)
        .spawn(f)
        .expect("thread")
        .join()
        .expect("le moteur Fadéra a paniqué")
}

pub use avec_grande_pile as executer_avec_grande_pile;

/// Exécute un programme avec saisies prédéfinies et fichiers virtuels en mémoire ; renvoie la sortie
/// (lignes séparées par « \n », sans retour final) ou « ERR:Ligne N : message » en cas d'erreur.
pub fn executer_capture(code: &str, saisies: Vec<String>, fichiers: &[(&str, &str)]) -> String {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    struct H {
        sortie: Rc<RefCell<Vec<String>>>,
        saisies: Vec<String>,
        fichiers: Rc<RefCell<HashMap<String, String>>>,
    }
    impl Hote for H {
        fn afficher(&mut self, t: &str) {
            self.sortie.borrow_mut().push(t.to_string());
        }
        fn saisir(&mut self, _: &str) -> Result<Option<String>, HoteErr> {
            Ok(if self.saisies.is_empty() { None } else { Some(self.saisies.remove(0)) })
        }
        fn lire_fichier(&mut self, p: &str) -> Result<String, HoteErr> {
            self.fichiers.borrow().get(p).cloned().ok_or_else(|| HoteErr::Msg("absent".into()))
        }
        fn ecrire_fichier(&mut self, p: &str, c: &str) -> Result<(), HoteErr> {
            self.fichiers.borrow_mut().insert(p.to_string(), c.to_string());
            Ok(())
        }
    }
    let sortie = Rc::new(RefCell::new(Vec::new()));
    let f: HashMap<String, String> = fichiers.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
    let h = H { sortie: sortie.clone(), saisies, fichiers: Rc::new(RefCell::new(f)) };
    if let Err(e) = executer_programme(code, Box::new(h)) {
        sortie.borrow_mut().push(format!("ERR:{}", decrire_erreur(&e, code).0));
    }
    let r = sortie.borrow().join("\n");
    r
}

/// Comme `executer_capture`, mais lance les tests (`tester "nom" … fin`) ; renvoie le rapport.
pub fn tester_capture(code: &str) -> String {
    use std::cell::RefCell;
    use std::rc::Rc;
    struct H {
        sortie: Rc<RefCell<Vec<String>>>,
    }
    impl Hote for H {
        fn afficher(&mut self, t: &str) {
            self.sortie.borrow_mut().push(t.to_string());
        }
    }
    let sortie = Rc::new(RefCell::new(Vec::new()));
    if let Err(e) = lancer_tests(code, Box::new(H { sortie: sortie.clone() })) {
        sortie.borrow_mut().push(format!("ERR:{}", decrire_erreur(&e, code).0));
    }
    let r = sortie.borrow().join("\n");
    r
}

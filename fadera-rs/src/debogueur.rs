//! Débogueur pas à pas du moteur Rust : pauses, pas à pas (passer / entrer / sortir), points d'arrêt, état, arrêt.
//!
//! Même comportement que le débogueur du moteur JavaScript (classe `Debogueur`) :
//!   * une pause avant chaque instruction (hors déclarations de fonctions, code des modules et code généré) ;
//!   * modes de reprise « continuer », « passer », « entrer », « sortir » ;
//!   * à chaque pause, l'hôte reçoit un instantané JSON et répond par une commande (texte) :
//!       continuer | passer | entrer | sortir | stop          (suivi, au choix, de `;pts=3,7,12` : nouvelle liste de points d'arrêt)
//!       eval:<code>                                          évalue une expression (ou exécute des instructions) dans la portée
//!                                                            de la ligne en pause, puis redemande une commande.
//!   * « stop » arrête le programme : l'erreur `MSG_STOP` n'est pas rattrapable par `essayer`.
//!
//! Instantané : {"ligne":N,"raison":"pas"|"point","locales":[{"nom","type","valeur"}],"globales":[…],
//!               "pile":[{"nom","ligne"}], "resultat":null|{"ok":bool,"texte":"…"}}
use std::collections::HashSet;
use std::rc::Rc;

use crate::ast::*;
use crate::erreur::*;
use crate::interp::Interp;
use crate::valeur::*;

pub const MSG_STOP: &str = "■ Exécution arrêtée.";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Continuer,
    Passer,
    Entrer,
    Sortir,
}

pub struct Debog {
    pub mode: Mode,
    /// Profondeur d'appel au moment de la dernière pause (référence pour « passer » et « sortir »).
    pub prof_ref: usize,
    pub points: HashSet<usize>,
    pub en_pause: bool,
}

impl Debog {
    /// Premier arrêt : dès la première instruction, sauf s'il y a des points d'arrêt (on court jusqu'au premier).
    pub fn nouveau(points: &[usize]) -> Debog {
        Debog {
            mode: if points.is_empty() { Mode::Entrer } else { Mode::Continuer },
            prof_ref: 0,
            points: points.iter().copied().collect(),
            en_pause: false,
        }
    }
}

/// Résumé d'une valeur pour le panneau des variables (même règle que `resumeValeur` côté IDE).
fn resume(v: &Valeur) -> String {
    if let Valeur::Liste(l) = v {
        let l = l.borrow();
        if l.len() > 30 {
            let debut = formater(&Valeur::liste(l.iter().take(5).cloned().collect()));
            let mut d: Vec<char> = debut.chars().collect();
            d.pop();
            return format!("liste de {} éléments : {}, …]", l.len(), d.into_iter().collect::<String>());
        }
    }
    let t = formater(v);
    let c: Vec<char> = t.chars().collect();
    if c.len() > 120 {
        format!("{}…", c[..117].iter().collect::<String>())
    } else {
        t
    }
}

fn variable(nom: &str, v: &Valeur) -> Valeur {
    Valeur::struct_de(vec![("nom", Valeur::texte(nom)), ("type", Valeur::texte(type_de(v))), ("valeur", Valeur::texte(resume(v)))])
}

impl Interp {
    /// Active le débogage (à appeler avant l'exécution). `points` : numéros de lignes.
    pub fn activer_debogage(&mut self, points: &[usize]) {
        self.debog = Some(Box::new(Debog::nouveau(points)));
    }

    /// Appelé avant chaque instruction quand un débogueur est actif.
    pub(crate) fn sonde(&mut self, ligne: usize, synth: bool) -> R<()> {
        let (mode, prof_ref, point) = match &self.debog {
            Some(d) if !d.en_pause => (d.mode, d.prof_ref, d.points.contains(&ligne)),
            _ => return Ok(()),
        };
        if synth || ligne == 0 {
            return Ok(());
        }
        if let Some(&(module, genere)) = self.ctx.last() {
            if module || genere {
                return Ok(());
            }
        }
        let prof = self.profondeur;
        let raison = match mode {
            Mode::Entrer => Some("pas"),
            Mode::Passer if prof <= prof_ref => Some("pas"),
            Mode::Sortir if prof < prof_ref => Some("pas"),
            _ => None,
        };
        let raison = raison.or(if point { Some("point") } else { None });
        match raison {
            Some(r) => self.pause(ligne, r),
            None => Ok(()),
        }
    }

    fn pause(&mut self, ligne: usize, raison: &str) -> R<()> {
        if let Some(d) = self.debog.as_mut() {
            d.en_pause = true;
        }
        let mut resultat = Valeur::Rien;
        let issue: R<()> = loop {
            let json = crate::util::json_texte(&self.instantane(ligne, raison, resultat.clone()), "").unwrap_or_default();
            let reponse = self.hote.debogage(&json);
            let (cmd, reste) = match reponse.split_once(';') {
                Some((c, r)) => (c.to_string(), r.to_string()),
                None => (reponse.clone(), String::new()),
            };
            if let Some(p) = reste.strip_prefix("pts=") {
                if let Some(d) = self.debog.as_mut() {
                    d.points = p.split(',').filter_map(|x| x.trim().parse::<usize>().ok()).collect();
                }
            }
            if let Some(code) = cmd.strip_prefix("eval:") {
                resultat = self.evaluer_en_pause(code);
                continue;
            }
            let prof = self.profondeur;
            let d = self.debog.as_mut().unwrap();
            d.mode = match cmd.as_str() {
                "continuer" => Mode::Continuer,
                "passer" => Mode::Passer,
                "sortir" => Mode::Sortir,
                "stop" => break Err(Flow::Err(ErreurFadera::new(MSG_STOP, Some(ligne)))),
                _ => Mode::Entrer,
            };
            d.prof_ref = prof;
            break Ok(());
        };
        if let Some(d) = self.debog.as_mut() {
            d.en_pause = false;
        }
        issue
    }

    /// Instantané de l'état à l'arrêt : variables locales / globales et pile d'appels (de la plus récente à la plus ancienne).
    fn instantane(&self, ligne: usize, raison: &str, resultat: Valeur) -> Valeur {
        let mut locales: Vec<Valeur> = Vec::new();
        let mut vus: HashSet<u32> = HashSet::new();
        let mut m = self.pile.last().cloned();
        while let Some(sc) = m {
            if Rc::ptr_eq(&sc, &self.globale) {
                break;
            }
            if let Vars::Petit(v) = &*sc.vars.borrow() {
                for (id, x) in v.iter() {
                    if vus.insert(*id) {
                        if let Some(nom) = nom_depuis_id(*id) {
                            locales.push(variable(&nom, x));
                        }
                    }
                }
            }
            m = sc.parent.clone();
        }
        let mut globales: Vec<Valeur> = Vec::new();
        if let Vars::Dense(v) = &*self.globale.vars.borrow() {
            for (id, x) in v.iter().enumerate() {
                if let Some(x) = x {
                    if let Some(nom) = nom_depuis_id(id as u32) {
                        if !nom.starts_with("__") && !nom.contains(".#parent") {
                            globales.push(variable(&nom, x));
                        }
                    }
                }
            }
        }
        let mut pile: Vec<Valeur> = Vec::new();
        let mut courante = ligne;
        for (nom, l) in self.appels.iter().rev() {
            pile.push(Valeur::struct_de(vec![("nom", Valeur::texte(nom)), ("ligne", Valeur::Nombre(courante as f64))]));
            courante = *l;
        }
        pile.push(Valeur::struct_de(vec![("nom", Valeur::texte("programme")), ("ligne", Valeur::Nombre(courante as f64))]));
        Valeur::struct_de(vec![
            ("ligne", Valeur::Nombre(ligne as f64)),
            ("raison", Valeur::texte(raison)),
            ("locales", Valeur::liste(locales)),
            ("globales", Valeur::liste(globales)),
            ("pile", Valeur::liste(pile)),
            ("resultat", resultat),
        ])
    }

    /// Évalue du code saisi pendant une pause, dans la portée de la ligne en pause : une expression (son résultat
    /// est renvoyé) ou, à défaut, des instructions (exécutées ; pas de résultat).
    fn evaluer_en_pause(&mut self, code: &str) -> Valeur {
        let res = |ok: bool, t: String| Valeur::struct_de(vec![("ok", Valeur::Bool(ok)), ("texte", Valeur::texte(t))]);
        let essai = crate::interp::analyser_code(&format!("soit __valeur_repl = {}", code));
        if let Ok(ast) = &essai {
            if let [Stmt { k: StmtK::DeclVar(_, e), .. }] = ast.as_slice() {
                return match self.evaluer(e) {
                    Ok(Valeur::Rien) => res(true, String::new()),
                    Ok(v) => res(true, formater(&v)),
                    Err(f) => res(false, self.texte_erreur(f, code)),
                };
            }
        }
        match crate::interp::analyser_code(code) {
            Err(e) => res(false, crate::interp::decrire_erreur(&e, code).0.replacen("Ligne 1 : ", "", 1)),
            Ok(ast) => {
                for i in &ast {
                    if let Err(f) = self.executer(i) {
                        return match f {
                            Flow::Retour(_) => res(true, String::new()),
                            f => res(false, self.texte_erreur(f, code)),
                        };
                    }
                }
                res(true, String::new())
            }
        }
    }

    fn texte_erreur(&self, f: Flow, code: &str) -> String {
        match f {
            Flow::Err(e) => crate::interp::decrire_erreur(&e, code).0.replacen("Ligne 1 : ", "", 1),
            Flow::Nat(m) => m,
            _ => "interruption inattendue".to_string(),
        }
    }
}

/// Comme `executer_programme_complet`, avec le débogueur actif (voir le protocole en tête de fichier).
pub fn executer_programme_debogue(code: &str, hote: Box<dyn crate::interp::Hote>, points: &[usize]) -> (Option<Interp>, Option<ErreurFadera>) {
    let ast = match crate::interp::analyser_code(code) {
        Ok(a) => a,
        Err(e) => return (None, Some(e)),
    };
    let mut it = Interp::new(hote);
    it.activer_debogage(points);
    let erreur = match it.executer_programme_ast(&ast) {
        Ok(()) | Err(Flow::Retour(_)) => None,
        Err(Flow::Err(e)) => Some(e),
        Err(Flow::Nat(m)) => Some(ErreurFadera::new(m, None)),
        Err(_) => None,
    };
    it.debog = None;
    (Some(it), erreur)
}

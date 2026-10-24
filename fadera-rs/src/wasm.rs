//! Liaison WebAssembly (fonctionnalité `wasm`) pour le navigateur.
//! Compilation : `wasm-pack build --release --target web -- --features wasm`
//! ⚠ NON exécutée ici : la cible wasm32 n'était pas installable dans l'environnement de développement.
use crate::{decrire_erreur, executer_programme_complet, Hote, HoteErr, Valeur};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct HoteWasm {
    sortie: Rc<RefCell<Vec<String>>>,
    saisies: Vec<String>,
}

impl Hote for HoteWasm {
    fn afficher(&mut self, t: &str) {
        self.sortie.borrow_mut().push(t.to_string());
    }
    fn saisir(&mut self, _invite: &str) -> Result<Option<String>, HoteErr> {
        Ok(if self.saisies.is_empty() { None } else { Some(self.saisies.remove(0)) })
    }
    fn maintenant_ms(&self) -> f64 {
        js_sys::Date::now()
    }
    fn entropie(&self) -> u64 {
        (js_sys::Math::random() * 9.007199254740992e15) as u64 ^ (js_sys::Date::now() as u64)
    }
}

/// Exécute du code Fadéra. `saisies` : réponses successives aux appels `saisir(...)`.
/// Renvoie un JSON : {"sortie": [lignes...], "erreur": null | "Ligne N : message", "source": "ligne fautive", "etat": {"variables": {...}, "fonctions": {...}}}.
#[wasm_bindgen]
pub fn executer(code: &str, saisies: Vec<String>) -> String {
    let sortie = Rc::new(RefCell::new(Vec::new()));
    let (it, err) = executer_programme_complet(code, Box::new(HoteWasm { sortie: sortie.clone(), saisies }));
    let lignes: Vec<Valeur> = sortie.borrow().iter().map(Valeur::texte).collect();
    let mut o = resultat(code, it.as_ref(), err);
    o.push(("sortie", Valeur::liste(lignes)));
    crate::util::json_texte(&Valeur::struct_de(o), "").unwrap_or_default()
}

/// Champs communs du résultat : erreur, source, et état (variables / fonctions) pour le panneau de l'IDE.
fn resultat(code: &str, it: Option<&crate::Interp>, err: Option<crate::ErreurFadera>) -> Vec<(&'static str, Valeur)> {
    let mut detail: Vec<(&'static str, Valeur)> = Vec::new();
    let (erreur, source) = match err {
        None => (Valeur::Rien, Valeur::texte("")),
        Some(e) => {
            let (t, s) = decrire_erreur(&e, code);
            // détail structuré pour l'IDE : message sans « Ligne N : », ligne, pile d'appels
            let msg = if e.interne { format!("Erreur interne : {}", e.message) } else { e.message.clone() };
            detail.push(("message", Valeur::texte(msg)));
            detail.push(("ligne", e.ligne.map(|l| Valeur::Nombre(l as f64)).unwrap_or(Valeur::Rien)));
            let pile: Vec<Valeur> = e
                .pile
                .iter()
                .map(|(f, l)| Valeur::struct_de(vec![("fonction", Valeur::texte(f.as_str())), ("ligne", Valeur::Nombre(*l as f64))]))
                .collect();
            detail.push(("pile", Valeur::liste(pile)));
            detail.push(("ligne_programme", e.cadre.map(|l| Valeur::Nombre(l as f64)).unwrap_or(Valeur::Rien)));
            (Valeur::texte(t), Valeur::texte(s))
        }
    };
    let arret = matches!(&erreur, Valeur::Texte(t) if t.contains(crate::debogueur::MSG_STOP));
    detail.push(("arret", Valeur::Bool(arret)));
    let etat = it.map(|i| i.exporter_etat()).unwrap_or_else(|| Valeur::struct_de(vec![]));
    let mut r = vec![("erreur", erreur), ("source", source), ("etat", etat)];
    r.extend(detail);
    r
}

// ---------------------------------------------------------------------------------------------
// Hôte JavaScript « synchrone » : pensé pour un Web Worker. Chaque méthode de l'objet `hote` peut
// bloquer (Atomics.wait) le temps que le fil principal réponde (saisie, fichiers, réseau…).
//   hote.afficher(texte)                       hote.saisir(invite) -> texte | null
//   hote.lire(chemin) -> texte (lève Error)    hote.ecrire(chemin, contenu) (lève Error)
//   hote.pause(ms)                             hote.http(methode, url, corps|null, entetesJSON) -> '{"statut":200,"corps":"…"}'
// Une méthode absente => fonctionnalité « indisponible » côté Fadéra.
// ---------------------------------------------------------------------------------------------
struct HoteJs {
    hote: JsValue,
}

impl HoteJs {
    fn methode(&self, nom: &str) -> Option<js_sys::Function> {
        js_sys::Reflect::get(&self.hote, &JsValue::from_str(nom)).ok().and_then(|f| f.dyn_into::<js_sys::Function>().ok())
    }
    fn appeler(&self, nom: &str, args: &[JsValue]) -> Result<JsValue, HoteErr> {
        let f = self.methode(nom).ok_or(HoteErr::Indisponible)?;
        let tab = js_sys::Array::new();
        for a in args {
            tab.push(a);
        }
        f.apply(&self.hote, &tab).map_err(|e| {
            let m = js_sys::Reflect::get(&e, &JsValue::from_str("message")).ok().and_then(|m| m.as_string());
            HoteErr::Msg(m.or_else(|| e.as_string()).unwrap_or_else(|| "erreur de l'hôte".to_string()))
        })
    }
}

impl Hote for HoteJs {
    fn afficher(&mut self, t: &str) {
        let _ = self.appeler("afficher", &[JsValue::from_str(t)]);
    }
    fn saisir(&mut self, invite: &str) -> Result<Option<String>, HoteErr> {
        let v = self.appeler("saisir", &[JsValue::from_str(invite)])?;
        Ok(v.as_string())
    }
    fn lire_fichier(&mut self, chemin: &str) -> Result<String, HoteErr> {
        let v = self.appeler("lire", &[JsValue::from_str(chemin)])?;
        v.as_string().ok_or_else(|| HoteErr::Msg("lecture impossible".into()))
    }
    fn ecrire_fichier(&mut self, chemin: &str, contenu: &str) -> Result<(), HoteErr> {
        self.appeler("ecrire", &[JsValue::from_str(chemin), JsValue::from_str(contenu)]).map(|_| ())
    }
    fn pause(&mut self, ms: f64) {
        let _ = self.appeler("pause", &[JsValue::from_f64(ms)]);
    }
    fn http(&mut self, methode: &str, url: &str, corps: Option<&str>, entetes: &[(String, String)]) -> Result<(u16, String), HoteErr> {
        let h: Vec<Valeur> = entetes.iter().map(|(k, v)| Valeur::liste(vec![Valeur::texte(k), Valeur::texte(v)])).collect();
        let hjson = crate::util::json_texte(&Valeur::liste(h), "").unwrap_or_else(|_| "[]".to_string());
        let c = match corps {
            Some(c) => JsValue::from_str(c),
            None => JsValue::NULL,
        };
        let v = self.appeler("http", &[JsValue::from_str(methode), JsValue::from_str(url), c, JsValue::from_str(&hjson)])?;
        let txt = v.as_string().ok_or_else(|| HoteErr::Msg("réponse invalide".into()))?;
        let st = js_sys::JSON::parse(&txt).map_err(|_| HoteErr::Msg("réponse invalide".into()))?;
        let statut = js_sys::Reflect::get(&st, &JsValue::from_str("statut")).ok().and_then(|x| x.as_f64()).unwrap_or(0.0) as u16;
        let body = js_sys::Reflect::get(&st, &JsValue::from_str("corps")).ok().and_then(|x| x.as_string()).unwrap_or_default();
        Ok((statut, body))
    }
    fn debogage(&mut self, etat: &str) -> String {
        // hote.debogage(instantaneJSON) -> commande (bloque dans le Worker le temps que l'IDE réponde) ; absent => on continue
        match self.appeler("debogage", &[JsValue::from_str(etat)]) {
            Ok(v) => v.as_string().unwrap_or_else(|| "continuer".to_string()),
            Err(_) => "continuer".to_string(),
        }
    }
    fn maintenant_ms(&self) -> f64 {
        js_sys::Date::now()
    }
    fn entropie(&self) -> u64 {
        (js_sys::Math::random() * 9.007199254740992e15) as u64 ^ (js_sys::Date::now() as u64)
    }
}

/// Exécute du code Fadéra avec un hôte JavaScript synchrone (à appeler dans un Web Worker).
/// La sortie est envoyée au fil de l'eau via `hote.afficher`. Renvoie {"erreur": null | "Ligne N : …", "source": "…", "etat": {…}}.
#[wasm_bindgen]
pub fn executer_hote(code: &str, hote: JsValue) -> String {
    let (it, err) = executer_programme_complet(code, Box::new(HoteJs { hote }));
    let o = resultat(code, it.as_ref(), err);
    crate::util::json_texte(&Valeur::struct_de(o), "").unwrap_or_default()
}

/// Lance les tests (`tester "nom" … fin`) avec un hôte synchrone. Le rapport part au fil de l'eau via `hote.afficher`.
/// Renvoie {"total": N, "reussis": R, "erreur": null | "Ligne N : …"}.
#[wasm_bindgen]
pub fn tester_hote(code: &str, hote: JsValue) -> String {
    let (total, reussis, erreur) = match crate::lancer_tests(code, Box::new(HoteJs { hote })) {
        Ok(b) => (b.total, b.reussis, Valeur::Rien),
        Err(e) => (0, 0, Valeur::texte(decrire_erreur(&e, code).0)),
    };
    let o = Valeur::struct_de(vec![("total", Valeur::Nombre(total as f64)), ("reussis", Valeur::Nombre(reussis as f64)), ("erreur", erreur)]);
    crate::util::json_texte(&o, "").unwrap_or_default()
}

/// Exécute du code Fadéra pas à pas (à appeler dans un Web Worker). `points` : numéros de lignes des points d'arrêt.
/// À chaque pause, `hote.debogage(instantaneJSON)` doit renvoyer une commande (voir `debogueur.rs` : continuer, passer,
/// entrer, sortir, stop, eval:<code>, éventuellement suivie de `;pts=3,7`). Résultat : comme `executer_hote`, plus `arret`.
#[wasm_bindgen]
pub fn deboguer_hote(code: &str, hote: JsValue, points: Vec<u32>) -> String {
    let pts: Vec<usize> = points.iter().map(|p| *p as usize).collect();
    let (it, err) = crate::debogueur::executer_programme_debogue(code, Box::new(HoteJs { hote }), &pts);
    let o = resultat(code, it.as_ref(), err);
    crate::util::json_texte(&Valeur::struct_de(o), "").unwrap_or_default()
}

/// Vérifie la syntaxe sans exécuter. Renvoie {"ok": bool, "erreur": null | "Ligne N : message", "message": "…", "ligne": N | null, "source": "…"}.
#[wasm_bindgen]
pub fn verifier_syntaxe(code: &str) -> String {
    let o = match crate::analyser_code(code) {
        Ok(_) => Valeur::struct_de(vec![("ok", Valeur::Bool(true)), ("erreur", Valeur::Rien)]),
        Err(e) => {
            let (t, s) = decrire_erreur(&e, code);
            Valeur::struct_de(vec![
                ("ok", Valeur::Bool(false)),
                ("erreur", Valeur::texte(t)),
                ("message", Valeur::texte(e.message.as_str())),
                ("ligne", e.ligne.map(|l| Valeur::Nombre(l as f64)).unwrap_or(Valeur::Rien)),
                ("source", Valeur::texte(s)),
            ])
        }
    };
    crate::util::json_texte(&o, "").unwrap_or_default()
}

/// Session persistante de la ligne de commande interactive : les variables et fonctions survivent d'une ligne à l'autre.
/// `hote` : comme pour `executer_hote` (afficher, saisir, lire, ecrire…), appelé de façon synchrone dans le fil principal.
#[wasm_bindgen]
pub struct Session {
    it: crate::Interp,
}

#[wasm_bindgen]
impl Session {
    #[wasm_bindgen(constructor)]
    pub fn new(hote: JsValue) -> Session {
        Session { it: crate::Interp::new(Box::new(HoteJs { hote })) }
    }

    /// Évalue une ligne. `instruction` : vrai pour une instruction (soit, si, fonction…), faux pour une expression dont on
    /// renvoie la valeur. Résultat : {"valeur": null | "texte", "erreur": null | "message (sans « Ligne 1 : »)"}.
    pub fn evaluer(&mut self, code: &str, instruction: bool) -> String {
        let (v, e) = self.it.evaluer_ligne(code, instruction);
        let o = Valeur::struct_de(vec![
            ("valeur", v.map(|t| Valeur::texte(t)).unwrap_or(Valeur::Rien)),
            ("erreur", e.map(|t| Valeur::texte(t)).unwrap_or(Valeur::Rien)),
        ]);
        crate::util::json_texte(&o, "").unwrap_or_default()
    }

    /// État courant : {"variables": {…}, "fonctions": {…}} (comme le champ `etat` de `executer`).
    pub fn etat(&self) -> String {
        crate::util::json_texte(&self.it.exporter_etat(), "").unwrap_or_default()
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    crate::VERSION.to_string()
}

use std::collections::HashSet;
use std::rc::Rc;

use crate::ast::*;
use crate::erreur::*;
use crate::lexer::lexer;
use crate::natives;
use crate::parser::Parser;
use crate::valeur::*;

pub enum HoteErr {
    Indisponible,
    Msg(String),
}

/// Interface avec l'environnement (terminal, fichiers, horloge…).
pub trait Hote {
    fn afficher(&mut self, texte: &str);
    fn saisir(&mut self, _invite: &str) -> Result<Option<String>, HoteErr> {
        Err(HoteErr::Indisponible)
    }
    fn lire_fichier(&mut self, _chemin: &str) -> Result<String, HoteErr> {
        Err(HoteErr::Indisponible)
    }
    fn ecrire_fichier(&mut self, _chemin: &str, _contenu: &str) -> Result<(), HoteErr> {
        Err(HoteErr::Indisponible)
    }
    /// Millisecondes depuis 1970 (UTC).
    fn maintenant_ms(&self) -> f64 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as f64).unwrap_or(0.0)
        }
        #[cfg(target_arch = "wasm32")]
        {
            0.0
        }
    }
    /// Pause (millisecondes) ; sans effet dans le navigateur (WASM).
    fn pause(&mut self, _ms: f64) {
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::sleep(std::time::Duration::from_millis(_ms as u64));
    }
    /// Requête HTTP ; par défaut indisponible (l'hôte doit l'implémenter). Renvoie (statut, corps).
    fn http(&mut self, _methode: &str, _url: &str, _corps: Option<&str>, _entetes: &[(String, String)]) -> Result<(u16, String), HoteErr> {
        Err(HoteErr::Indisponible)
    }
    /// Débogueur : appelé à chaque pause avec l'instantané JSON ; renvoie la commande (voir `debogueur.rs`).
    fn debogage(&mut self, _etat: &str) -> String {
        "continuer".to_string()
    }
    /// Graine pour le générateur pseudo-aléatoire (hasard, réseaux de neurones).
    fn entropie(&self) -> u64 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(42) ^ 0x9E3779B97F4A7C15
        }
        #[cfg(target_arch = "wasm32")]
        {
            0x1234_5678_9ABC_DEF0
        }
    }
}

pub struct Interp {
    pub hote: Box<dyn Hote>,
    pub fonctions: Vec<Option<Rc<FnDef>>>,
    pub(crate) portees_libres: Vec<Scope>,
    pub globale: Scope,
    pub pile: Vec<Scope>,
    pub etapes: u64,
    pub(crate) vs: Vec<Valeur>,
    /// true : exécute avec l'interpréteur à arbre au lieu du bytecode (variable d'environnement FADERA_ARBRE=1, tests de comparaison).
    pub arbre: bool,
    pub limite: u64,
    pub profondeur: usize,
    pub importes: HashSet<String>,
    /// Débogueur pas à pas (None = exécution normale, aucun surcoût).
    pub debog: Option<Box<crate::debogueur::Debog>>,
    /// Pile d'appels vivante (nom, ligne de l'appel) : tenue seulement pendant un débogage.
    pub(crate) appels: Vec<(String, usize)>,
    /// Contexte du code en cours (code d'un module, code généré) : tenu seulement pendant un débogage.
    pub(crate) ctx: Vec<(bool, bool)>,
    /// Dossier du module en cours d'import (pour que `importer "autre"` le cherche d'abord à côté de lui).
    dossiers_modules: Vec<String>,
    rng: u64,
}

pub fn analyser_code(code: &str) -> Result<Vec<Stmt>, ErreurFadera> {
    Parser::new(lexer(code)?).analyser()
}

/// Comme `analyser_code`, pour le source d'un module : les fonctions savent de quel module elles viennent.
pub fn analyser_code_module(code: &str, module: &str, alias: Option<&str>) -> Result<Vec<Stmt>, ErreurFadera> {
    let mut toks = lexer(code)?;
    if let Some(a) = alias {
        crate::espaces::renommer(&mut toks, a);
    }
    Parser::pour_module(toks, module).analyser()
}

enum Sortie {
    Normal,
    Rompre,
    Continuer,
}

impl Interp {
    pub fn new(hote: Box<dyn Hote>) -> Interp {
        let g = ScopeData::globale();
        let graine = hote.entropie() | 1;
        Interp {
            hote,
            fonctions: Vec::new(),
            portees_libres: Vec::new(),
            globale: g.clone(),
            pile: vec![g],
            etapes: 0,
            vs: Vec::with_capacity(256),
            arbre: cfg!(not(target_arch = "wasm32")) && std::env::var("FADERA_ARBRE").map(|v| v == "1").unwrap_or(false),
            limite: if cfg!(not(target_arch = "wasm32")) { std::env::var("FADERA_LIMITE").ok().and_then(|v| v.parse().ok()).unwrap_or(20_000_000) } else { 20_000_000 },
            profondeur: 0,
            importes: HashSet::new(),
            debog: None,
            appels: Vec::new(),
            ctx: Vec::new(),
            dossiers_modules: Vec::new(),
            rng: graine,
        }
    }

    /// Nombre pseudo-aléatoire dans [0, 1).
    pub fn alea(&mut self) -> f64 {
        // xorshift64*
        let mut x = self.rng;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.rng = x;
        let r = x.wrapping_mul(0x2545F4914F6CDD1D);
        (r >> 11) as f64 / (1u64 << 53) as f64
    }

    #[inline]
    pub(crate) fn fn_get(&self, id: u32) -> Option<Rc<FnDef>> {
        self.fonctions.get(id as usize).and_then(|x| x.clone())
    }
    pub(crate) fn fn_set(&mut self, d: &Rc<FnDef>) {
        let i = d.ident.id as usize;
        if i >= self.fonctions.len() {
            self.fonctions.resize(i + 1, None);
        }
        self.fonctions[i] = Some(d.clone());
    }

    pub(crate) fn memoire(&self) -> &Scope {
        self.pile.last().unwrap()
    }

    /// Référence à une variable (sans clone ni garde RefCell). Les emprunts de portée sont toujours de très courte
    /// durée (jamais conservés pendant une évaluation) : lire sans garde est donc sans risque.
    #[inline(always)]
    pub(crate) fn var_ref(&self, nom: &Nom) -> Option<&Valeur> {
        let mut s: &Scope = self.pile.last().unwrap();
        loop {
            // SAFETY : aucun emprunt mutable n'est actif pendant l'exécution d'une instruction de lecture
            let v = unsafe { s.vars.try_borrow_unguarded() }.ok()?;
            if let Some(x) = v.get(nom.id) {
                return Some(x);
            }
            match &s.parent {
                Some(p) => s = p,
                None => break,
            }
        }
        unsafe { self.globale.vars.try_borrow_unguarded() }.ok()?.get(nom.id)
    }

    /// Lecture sans cloner de portée : parcourt la chaîne locale → parents → globale.
    #[inline]
    pub(crate) fn chercher(&self, nom: &Nom) -> Option<Valeur> {
        let mut s: &Scope = self.memoire();
        loop {
            if let Some(v) = s.vars.borrow().get(nom.id) {
                return Some(v.clone());
            }
            match &s.parent {
                Some(p) => s = p,
                None => break,
            }
        }
        self.globale.vars.borrow().get(nom.id).cloned()
    }

    /// Affectation en place (sans réallouer le nom) ; false si la variable n'existe pas.
    #[inline]
    pub(crate) fn affecter(&self, nom: &Nom, v: Valeur) -> bool {
        let mut s: &Scope = self.memoire();
        loop {
            if let Some(slot) = s.vars.borrow_mut().get_mut(nom.id) {
                *slot = v;
                return true;
            }
            match &s.parent {
                Some(p) => s = p,
                None => break,
            }
        }
        if let Some(slot) = self.globale.vars.borrow_mut().get_mut(nom.id) {
            *slot = v;
            return true;
        }
        false
    }

    /// `nom = nom + extra` quand `nom` contient un texte : ajoute en place (O(1) amorti ; copie seulement si le
    /// texte est partagé avec une autre variable). false si la variable n'existe pas ou n'est pas un texte.
    pub(crate) fn ajouter_texte(&self, nom: &Nom, extra: &str) -> bool {
        let mut s: &Scope = self.memoire();
        loop {
            if let Some(slot) = s.vars.borrow_mut().get_mut(nom.id) {
                return match slot {
                    Valeur::Texte(t) => {
                        Rc::make_mut(t).push_str(extra);
                        true
                    }
                    _ => false,
                };
            }
            match &s.parent {
                Some(p) => s = p,
                None => break,
            }
        }
        if let Some(Valeur::Texte(t)) = self.globale.vars.borrow_mut().get_mut(nom.id) {
            Rc::make_mut(t).push_str(extra);
            return true;
        }
        false
    }

    pub(crate) fn lire(&self, nom: &Nom, ligne: usize) -> R<Valeur> {
        match self.chercher(nom) {
            Some(v) => Ok(v),
            None => {
                if let Some(d) = self.fn_get(nom.id) {
                    return Ok(Valeur::Fonction(Rc::new(FonctionFadera {
                        parametres: d.parametres.clone(),
                        corps: d.corps.clone(),
                        fermeture: None,
                        nom: Some(nom.to_string()),
                        natif: None,
                    })));
                }
                if natives::existe(nom) {
                    return Ok(Valeur::Fonction(Rc::new(FonctionFadera {
                        parametres: Rc::new(vec![]),
                        corps: Rc::new(Corps::nouveau(vec![])),
                        fermeture: None,
                        nom: Some(nom.to_string()),
                        natif: Some(nom.to_string()),
                    })));
                }
                if nom.contains('.') {
                    if let Some(v) = self.lire_chemin(nom, ligne)? {
                        return Ok(v);
                    }
                }
                Err(fad_l(
                    format!("La variable \"{}\" n'existe pas. Déclarez-la d'abord avec : soit {} = ...", nom, nom),
                    ligne,
                ))
            }
        }
    }

    // ------------------------------------------------------------ accès pointés : a.b.c, a.b.m(…), a.b = v
    // (repli à l'exécution : n'a lieu que si le nom exact n'existe pas ; les espaces de noms de modules gardent la priorité)

    /// Plus long préfixe (au plus `max` segments, au moins 1) de `nom` qui est une variable : (chemin, valeur, segments restants).
    fn resoudre_chemin(&self, nom: &str, max: usize) -> Option<(String, Valeur, Vec<String>)> {
        let segs: Vec<&str> = nom.split('.').collect();
        let mut k = max.min(segs.len());
        while k >= 1 {
            let p = segs[..k].join(".");
            if let Some(v) = self.chercher(&Nom::nouveau(&p)) {
                return Some((p, v, segs[k..].iter().map(|x| x.to_string()).collect()));
            }
            k -= 1;
        }
        None
    }

    fn champ_absent(&self, champ: &str, chemin: &str, d: &Dict, ligne: usize) -> Flow {
        let ks = d.cles_ordonnees();
        fad_l(
            format!(
                "Le champ \"{}\" n'existe pas dans \"{}\" {}",
                champ,
                chemin,
                if ks.is_empty() { "(aucun champ).".to_string() } else { format!("(champs disponibles : {}).", ks.join(", ")) }
            ),
            ligne,
        )
    }

    /// Descend dans les champs ; renvoie la valeur finale et son chemin.
    fn descendre(&self, mut v: Valeur, mut chemin: String, segs: &[String], ligne: usize) -> R<(Valeur, String)> {
        for s in segs {
            let suivant = match &v {
                Valeur::Struct(d) => {
                    let d = d.borrow();
                    match d.get(s) {
                        Some(x) => x.clone(),
                        None => return Err(self.champ_absent(s, &chemin, &d, ligne)),
                    }
                }
                autre => {
                    return Err(fad_l(
                        format!(
                            "Impossible de lire le champ \"{}\" : \"{}\" est de type {} (une structure ou une instance est attendue).",
                            s,
                            chemin,
                            type_de(autre)
                        ),
                        ligne,
                    ))
                }
            };
            v = suivant;
            chemin.push('.');
            chemin.push_str(s);
        }
        Ok((v, chemin))
    }

    /// Lecture pointée `a.b.c` ; None = aucun préfixe n'est une variable.
    fn lire_chemin(&self, nom: &str, ligne: usize) -> R<Option<Valeur>> {
        let max = nom.split('.').count() - 1;
        match self.resoudre_chemin(nom, max) {
            None => Ok(None),
            Some((p, v, reste)) => Ok(Some(self.descendre(v, p, &reste, ligne)?.0)),
        }
    }

    /// Affectation `nom = v` : variable existante, sinon écriture d'un champ `a.b.c = v` ; erreur si rien ne correspond.
    pub(crate) fn affecter_ou_champ(&mut self, nom: &Nom, v: Valeur, ligne: usize) -> R<()> {
        if self.affecter(nom, v.clone()) {
            return Ok(());
        }
        if let Some((chemin, dernier)) = nom.rsplit_once('.') {
            if !chemin.is_empty() {
                let max = chemin.split('.').count();
                // objet = tout sauf le dernier segment
                if let Some((p, o, reste)) = self.resoudre_chemin(chemin, max) {
                    let (obj, cheminobj) = self.descendre(o, p, &reste, ligne)?;
                    return match &obj {
                        Valeur::Struct(d) => {
                            d.borrow_mut().set(dernier, v);
                            Ok(())
                        }
                        autre => Err(fad_l(
                            format!(
                                "Impossible d'écrire le champ \"{}\" : \"{}\" est de type {} (une structure ou une instance est attendue).",
                                dernier,
                                cheminobj,
                                type_de(autre)
                            ),
                            ligne,
                        )),
                    };
                }
            }
        }
        Err(fad_l(
            format!("La variable \"{}\" n'existe pas. Déclarez-la d'abord avec : soit {} = ...", nom, nom),
            ligne,
        ))
    }

    /// Appel `a.b.m(args)` : méthode d'une instance ou fonction stockée dans un champ. None = objet introuvable.
    fn appeler_pointe(&mut self, nom: &Nom, vals: &[Valeur], ligne: usize) -> R<Option<Valeur>> {
        let (chemin, m) = match nom.rsplit_once('.') {
            Some((c, m)) if !c.is_empty() => (c.to_string(), m.to_string()),
            _ => return Ok(None),
        };
        let obj = match self.chercher(&Nom::nouveau(&chemin)) {
            Some(v) => v,
            None if chemin.contains('.') => match self.lire_chemin(&chemin, ligne)? {
                Some(v) => v,
                None => return Ok(None),
            },
            None => return Ok(None),
        };
        self.appeler_sur(obj, &chemin, &m, vals, ligne).map(Some)
    }

    /// Appel de la méthode (ou du champ fonction) `m` sur l'objet `obj` ; `chemin` ne sert qu'aux messages d'erreur.
    fn appeler_sur(&mut self, obj: Valeur, chemin: &str, m: &str, vals: &[Valeur], ligne: usize) -> R<Valeur> {
        let d = match &obj {
            Valeur::Struct(d) => d.clone(),
            autre => {
                return Err(fad_l(
                    format!("Impossible d'appeler \"{}\" : \"{}\" est de type {} (une instance est attendue).", m, chemin, type_de(autre)),
                    ligne,
                ))
            }
        };
        let champ = d.borrow().get(m).cloned();
        if let Some(f) = champ {
            if !matches!(f, Valeur::Fonction(_)) {
                return Err(fad_l(
                    format!("Le champ \"{}\" de \"{}\" n'est pas une fonction (reçu : {}).", m, chemin, type_de(&f)),
                    ligne,
                ));
            }
            return self.appeler_valeur(&f, vals.to_vec(), ligne);
        }
        let classe = d.borrow().classe.clone();
        let classe = match classe {
            Some(c) => c,
            None => return Err(self.champ_absent(m, chemin, &d.borrow(), ligne)),
        };
        let mut cour: Option<String> = Some(classe.to_string());
        let mut prof = 0;
        while let Some(c) = cour {
            if prof >= 100 {
                break;
            }
            prof += 1;
            let complet = format!("{}.{}", c, m);
            if let Some(def) = self.fn_get(Nom::nouveau(&complet).id) {
                if vals.len() != def.parametres.len() - 1 {
                    return Err(fad_l(
                        format!(
                            "La méthode \"{}\" de la classe \"{}\" attend {} argument(s), {} reçu(s).",
                            m,
                            classe,
                            def.parametres.len() - 1,
                            vals.len()
                        ),
                        ligne,
                    ));
                }
                let mut args = Vec::with_capacity(vals.len() + 1);
                args.push(obj.clone());
                args.extend(vals.iter().cloned());
                return self.appeler_def(&def.parametres, &def.corps, args, ligne, &complet, None);
            }
            cour = match self.chercher(&Nom::nouveau(&format!("{}.#parent", c))) {
                Some(Valeur::Texte(p)) => Some(p.to_string()),
                _ => None,
            };
        }
        Err(fad_l(format!("La méthode \"{}\" n'existe pas dans la classe \"{}\".", m, classe), ligne))
    }

    #[inline]
    pub(crate) fn definir(&self, nom: &Nom, v: Valeur) {
        self.memoire().vars.borrow_mut().definir(nom.id, v);
    }

    pub(crate) fn tic(&mut self) -> R<()> {
        self.etapes += 1;
        if self.etapes > self.limite {
            return Err(fad("Limite d'exécution dépassée : boucle infinie probable."));
        }
        Ok(())
    }

    pub(crate) fn verif_index(&self, len: usize, i: &Valeur, ligne: Option<usize>) -> R<usize> {
        let mk = |m: String| Flow::Err(ErreurFadera::new(m, ligne));
        match i {
            Valeur::Nombre(n) if est_entier(*n) => {
                if *n < 0.0 || *n >= len as f64 {
                    Err(mk(format!(
                        "Index {} hors limites ({}).",
                        js_num(*n),
                        if len > 0 { format!("indices valides : 0 à {}", len - 1) } else { "la liste est vide".to_string() }
                    )))
                } else {
                    Ok(*n as usize)
                }
            }
            _ => Err(mk(format!("L'index doit être un nombre entier (reçu : {}).", formater(i)))),
        }
    }

    // ------------------------------------------------------------------ exécution
    pub(crate) fn bloc(&mut self, instrs: &[Stmt]) -> R<()> {
        for i in instrs {
            self.executer(i)?;
        }
        Ok(())
    }

    fn corps_boucle(&mut self, corps: &[Stmt]) -> R<Sortie> {
        match self.bloc(corps) {
            Ok(()) => Ok(Sortie::Normal),
            Err(Flow::Rompre) => Ok(Sortie::Rompre),
            Err(Flow::Continuer) => Ok(Sortie::Continuer),
            Err(e) => Err(e),
        }
    }

    pub fn executer(&mut self, s: &Stmt) -> R<()> {
        self.tic()?;
        if self.debog.is_some() && !matches!(s.k, StmtK::DeclFonction(_)) {
            self.sonde(s.ligne, false).map_err(|f| match f {
                Flow::Err(mut e) => {
                    e.ligne = Some(s.ligne);
                    Flow::Err(e)
                }
                f => f,
            })?;
        }
        match self.executer_noeud(s) {
            Err(Flow::Err(mut e)) => {
                if e.ligne.is_none() {
                    e.ligne = Some(s.ligne);
                }
                if e.cadre.is_none() {
                    e.cadre = Some(s.ligne);
                }
                Err(Flow::Err(e))
            }
            Err(Flow::Nat(m)) => Err(fad_l(m, s.ligne)),
            r => r,
        }
    }

    pub fn executer_programme_ast(&mut self, corps: &[Stmt]) -> R<()> {
        for i in corps {
            if let StmtK::DeclFonction(d) = &i.k {
                self.fn_set(d);
            }
        }
        crate::vm::executer_programme(self, corps)
    }

    /// Ancien chemin (arbre) : conservé pour les imports et comme référence de comparaison.
    pub fn executer_programme_arbre(&mut self, corps: &[Stmt]) -> R<()> {
        for i in corps {
            if let StmtK::DeclFonction(d) = &i.k {
                self.fn_set(d);
            }
        }
        for i in corps {
            self.executer(i)?;
        }
        Ok(())
    }

    pub(crate) fn executer_noeud(&mut self, s: &Stmt) -> R<()> {
        match &s.k {
            StmtK::DeclFonction(d) => {
                self.fn_set(d);
            }
            StmtK::DeclVar(nom, e) => {
                let v = self.evaluer(e)?;
                self.definir(nom, v);
            }
            StmtK::Affect(nom, e) => {
                let v = self.evaluer(e)?;
                self.affecter_ou_champ(nom, v, s.ligne)?;
            }
            StmtK::Afficher(es) => {
                let mut parts = Vec::new();
                for e in es {
                    let v = self.evaluer(e)?;
                    parts.push(formater(&v));
                }
                let t = parts.join(" ");
                self.hote.afficher(&t);
            }
            StmtK::Expr(e) => {
                self.evaluer(e)?;
            }
            StmtK::Renvoyer(e) => {
                let v = match e {
                    Some(e) => self.evaluer(e)?,
                    None => Valeur::Rien,
                };
                return Err(Flow::Retour(v));
            }
            StmtK::Si(c, alors, sinon) => {
                let v = self.evaluer(c)?;
                let bloc = if v.vrai() { Some(alors) } else { sinon.as_ref() };
                if let Some(b) = bloc {
                    self.bloc(b)?;
                }
            }
            StmtK::TantQue(c, corps) => {
                while self.evaluer(c)?.vrai() {
                    if let Sortie::Rompre = self.corps_boucle(corps)? {
                        break;
                    }
                    self.tic()?;
                }
            }
            StmtK::Pour { nom, debut, fin, pas, corps } => {
                let d = self.evaluer(debut)?;
                let f = self.evaluer(fin)?;
                let p = match pas {
                    Some(p) => self.evaluer(p)?,
                    None => Valeur::Nombre(1.0),
                };
                let mut nums = [0f64; 3];
                for (k, (nm, v)) in [("de départ", &d), ("d'arrivée", &f), ("pas", &p)].iter().enumerate() {
                    match v {
                        Valeur::Nombre(n) if !n.is_nan() => nums[k] = *n,
                        _ => {
                            return Err(fad(format!(
                                "La valeur {} de la boucle pour doit être un nombre (reçu : {}).",
                                nm,
                                type_de(v)
                            )))
                        }
                    }
                }
                let (debut, fin, pas) = (nums[0], nums[1], nums[2]);
                if pas == 0.0 {
                    return Err(fad("Le pas de la boucle \"pour\" ne peut pas être 0."));
                }
                let mut i = debut;
                while if pas > 0.0 { i <= fin } else { i >= fin } {
                    self.definir(nom, Valeur::Nombre(i));
                    if let Sortie::Rompre = self.corps_boucle(corps)? {
                        break;
                    }
                    self.tic()?;
                    i += pas;
                }
            }
            StmtK::Rompre => return Err(Flow::Rompre),
            StmtK::Continuer => return Err(Flow::Continuer),
            StmtK::PourChaque { nom, iterable, corps } => {
                let it = self.evaluer(iterable)?;
                let elems: Vec<Valeur> = match &it {
                    Valeur::Liste(l) => l.borrow().clone(),
                    Valeur::Texte(t) => t.chars().map(|c| Valeur::texte(c.to_string())).collect(),
                    Valeur::Struct(d) => {
                        let d = d.borrow();
                        d.cles_ordonnees().into_iter().map(Valeur::texte).collect()
                    }
                    _ => {
                        return Err(fad(format!(
                            "pour chaque attend une liste, un texte ou une structure (reçu : {}).",
                            type_de(&it)
                        )))
                    }
                };
                for x in elems {
                    self.definir(nom, x);
                    if let Sortie::Rompre = self.corps_boucle(corps)? {
                        break;
                    }
                    self.tic()?;
                }
            }
            StmtK::Selon { expr, cas, defaut } => {
                let v = self.evaluer(expr)?;
                for c in cas {
                    for e in &c.valeurs {
                        let w = self.evaluer(e)?;
                        if egal(&v, &w) {
                            return self.bloc(&c.corps);
                        }
                    }
                }
                if let Some(d) = defaut {
                    self.bloc(d)?;
                }
            }
            StmtK::Importer(e, alias) => {
                let v = self.evaluer(e)?;
                self.importer(js_texte(&v), alias.as_deref(), s.ligne)?;
            }
            StmtK::Essayer { corps, nom, gestion } => {
                let prof = self.profondeur;
                let taille = self.pile.len();
                match self.bloc(corps) {
                    Ok(()) => {}
                    Err(Flow::Err(e)) => {
                        if e.message.starts_with("Limite d'exécution") || e.message.contains(crate::debogueur::MSG_STOP) {
                            return Err(Flow::Err(e));
                        }
                        self.profondeur = prof;
                        self.pile.truncate(taille);
                        self.definir(nom, Valeur::texte(&e.message));
                        self.bloc(gestion)?;
                    }
                    Err(Flow::Nat(m)) => {
                        self.profondeur = prof;
                        self.pile.truncate(taille);
                        self.definir(nom, Valeur::texte(&m));
                        self.bloc(gestion)?;
                    }
                    Err(e) => return Err(e),
                }
            }
            StmtK::EcrireFichier(p, c) => {
                let chemin = js_texte(&self.evaluer(p)?);
                let contenu = formater(&self.evaluer(c)?);
                match self.hote.ecrire_fichier(&chemin, &contenu) {
                    Ok(()) => {}
                    Err(HoteErr::Indisponible) => return Err(fad("ecrireFichier n'est pas disponible dans cet environnement.")),
                    Err(HoteErr::Msg(m)) => return Err(interne(m)),
                }
            }
        }
        Ok(())
    }

    fn importer(&mut self, mut chemin: String, alias: Option<&str>, ligne: usize) -> R<()> {
        let a_ext = {
            let b = chemin.as_bytes();
            let mut i = b.len();
            while i > 0 && b[i - 1].is_ascii_alphanumeric() {
                i -= 1;
            }
            i > 0 && i < b.len() && b[i - 1] == b'.'
        };
        if !a_ext {
            chemin.push_str(".fa");
        }
        // un même module peut être importé sous plusieurs alias (ou sans alias) : la clé du cache en tient compte
        let pre = alias.map(|a| format!("{}=", a)).unwrap_or_default();
        let (cle, src): (String, String) = if chemin.starts_with("std/") {
            // modules standard intégrés au moteur (mêmes sources avec le moteur JavaScript)
            let nom = chemin.strip_suffix(".fa").unwrap_or(&chemin).to_string();
            let src = match crate::modules_std::source(&nom) {
                Some(s) => s,
                None => {
                    return Err(fad(format!(
                        "importer : module standard \"{}\" inconnu (modules disponibles : {}).",
                        nom,
                        crate::modules_std::noms().join(", ")
                    )))
                }
            };
            if self.importes.contains(&format!("{}{}", pre, chemin)) {
                return Ok(());
            }
            self.importes.insert(format!("{}{}", pre, chemin));
            (chemin.clone(), src.to_string())
        } else {
            // un module qui en importe un autre le cherche d'abord à côté de lui, puis depuis le dossier courant
            let rel = self.dossiers_modules.last().cloned().unwrap_or_default();
            let candidats: Vec<String> = if !rel.is_empty() && !chemin.starts_with('/') {
                vec![format!("{}/{}", rel, chemin), chemin.clone()]
            } else {
                vec![chemin.clone()]
            };
            let mut trouve: Option<(String, String)> = None;
            let mut derniere = String::new();
            for c in &candidats {
                if self.importes.contains(&format!("{}{}", pre, c)) {
                    return Ok(());
                }
                match self.hote.lire_fichier(c) {
                    Ok(s) => {
                        trouve = Some((c.clone(), s));
                        break;
                    }
                    Err(HoteErr::Indisponible) => return Err(fad("importer n'est pas disponible dans cet environnement.")),
                    Err(HoteErr::Msg(m)) => derniere = m,
                }
            }
            match trouve {
                Some((c, s)) => {
                    self.importes.insert(format!("{}{}", pre, c));
                    (c, s)
                }
                None => return Err(fad(format!("importer : module \"{}\" introuvable ({}).", chemin, derniere))),
            }
        };
        let prefixe = |e: &ErreurFadera, chemin: &str| -> Flow {
            let l = match e.ligne {
                Some(l) => format!(", ligne {}", l),
                None => String::new(),
            };
            fad_l(format!("[module {}{}] {}", chemin, l, e.message), ligne)
        };
        let ast = match analyser_code_module(&src, &cle, alias) {
            Ok(a) => a,
            Err(e) => return Err(prefixe(&e, &cle)),
        };
        let sauv = std::mem::replace(&mut self.pile, vec![self.globale.clone()]);
        self.dossiers_modules.push(match cle.rfind('/') {
            Some(i) => cle[..i].to_string(),
            None => String::new(),
        });
        if self.debog.is_some() {
            self.ctx.push((true, false));
        }
        let r = (|| -> R<()> {
            for i in &ast {
                if let StmtK::DeclFonction(d) = &i.k {
                    self.fn_set(d);
                }
            }
            for i in &ast {
                self.executer(i)?;
            }
            Ok(())
        })();
        if self.debog.is_some() {
            self.ctx.pop();
        }
        self.pile = sauv;
        self.dossiers_modules.pop();
        match r {
            Err(Flow::Err(mut e)) => {
                crate::erreur::sortie_module(&mut e, &cle);
                if e.origine_module {
                    e.ligne = Some(ligne);
                }
                Err(Flow::Err(e))
            }
            r => r,
        }
    }

    // ------------------------------------------------------------------ expressions
    pub fn evaluer(&mut self, e: &Expr) -> R<Valeur> {
        match e {
            Expr::Lit(v) => Ok(v.clone()),
            Expr::Var(nom, l) => match self.chercher(nom) {
                Some(v) => Ok(v),
                None => self.lire(nom, *l),
            },
            Expr::Bin(op, g, d, l) => {
                let gv = self.evaluer(g)?;
                let dv = self.evaluer(d)?;
                operer(*op, &gv, &dv, *l)
            }
            _ => self.evaluer_autre(e),
        }
    }

    #[inline(never)]
    fn evaluer_autre(&mut self, e: &Expr) -> R<Valeur> {
        match e {
            Expr::Lit(v) => Ok(v.clone()),
            Expr::Var(nom, l) => self.lire(nom, *l),
            Expr::Liste(es) => {
                let mut r = Vec::with_capacity(es.len());
                for x in es {
                    r.push(self.evaluer(x)?);
                }
                Ok(Valeur::liste(r))
            }
            Expr::Struct(paires) => {
                let mut d = Dict::nouveau();
                for (k, v) in paires {
                    let x = self.evaluer(v)?;
                    d.set(k, x);
                }
                Ok(Valeur::structure(d))
            }
            Expr::Lambda { parametres, corps } => Ok(Valeur::Fonction(Rc::new(FonctionFadera {
                parametres: parametres.clone(),
                corps: corps.clone(),
                fermeture: Some(self.memoire().clone()),
                nom: None,
                natif: None,
            }))),
            Expr::Taille(c, l) => {
                let c = self.evaluer(c)?;
                match &c {
                    Valeur::Liste(x) => Ok(Valeur::Nombre(x.borrow().len() as f64)),
                    Valeur::Texte(t) => Ok(Valeur::Nombre(t.chars().count() as f64)),
                    Valeur::Struct(d) => Ok(Valeur::Nombre(d.borrow().len() as f64)),
                    _ => Err(fad_l(
                        format!("taille attend une liste, un texte ou une structure (reçu : {}).", type_de(&c)),
                        *l,
                    )),
                }
            }
            Expr::LireElement(t, i, l) => {
                let t = self.evaluer(t)?;
                let i = self.evaluer(i)?;
                match &t {
                    Valeur::Liste(x) => {
                        let n = x.borrow().len();
                        let k = self.verif_index(n, &i, Some(*l))?;
                        Ok(x.borrow()[k].clone())
                    }
                    Valeur::Texte(s) => {
                        let n = s.chars().count();
                        let k = self.verif_index(n, &i, Some(*l))?;
                        Ok(Valeur::texte(s.chars().nth(k).unwrap().to_string()))
                    }
                    _ => Err(fad_l(format!("lireElement attend une liste ou un texte (reçu : {}).", type_de(&t)), *l)),
                }
            }
            Expr::LireCle(s, c, l) => {
                let s = self.evaluer(s)?;
                let c = js_texte(&self.evaluer(c)?);
                match &s {
                    Valeur::Struct(d) => {
                        let d = d.borrow();
                        match d.get(&c) {
                            Some(v) => Ok(v.clone()),
                            None => {
                                let ks = d.cles_ordonnees().join(", ");
                                Err(fad_l(
                                    format!(
                                        "La clé \"{}\" n'existe pas dans la structure (clés disponibles : {}).",
                                        c,
                                        if ks.is_empty() { "aucune".to_string() } else { ks }
                                    ),
                                    *l,
                                ))
                            }
                        }
                    }
                    _ => Err(fad_l(format!("lireCle attend une structure (reçu : {}).", type_de(&s)), *l)),
                }
            }
            Expr::Hasard(a, b, l) => {
                let a = self.evaluer(a)?;
                let b = self.evaluer(b)?;
                match (&a, &b) {
                    (Valeur::Nombre(x), Valeur::Nombre(y)) => {
                        let mut a = x.ceil();
                        let mut b = y.floor();
                        if a > b {
                            std::mem::swap(&mut a, &mut b);
                        }
                        let r = (self.alea() * (b - a + 1.0)).floor() + a;
                        Ok(Valeur::Nombre(r))
                    }
                    _ => Err(fad_l("hasard(min, max) attend deux nombres.", *l)),
                }
            }
            Expr::Champ(e, nom, l) => {
                let v = self.evaluer(e)?;
                Ok(self.descendre(v, "(expression)".to_string(), &[nom.clone()], *l)?.0)
            }
            Expr::Methode(e, nom, args, l) => {
                let v = self.evaluer(e)?;
                let mut vals = Vec::with_capacity(args.len());
                for a in args {
                    vals.push(self.evaluer(a)?);
                }
                self.appeler_sur(v, "(expression)", nom, &vals, *l)
            }
            Expr::Saisir(invite) => {
                let inv = match invite {
                    Some(e) => formater(&self.evaluer(e)?),
                    None => String::new(),
                };
                match self.hote.saisir(&inv) {
                    Err(HoteErr::Indisponible) => Err(fad("saisir n'est pas disponible dans cet environnement.")),
                    Err(HoteErr::Msg(m)) => Err(interne(m)),
                    Ok(None) => Ok(Valeur::texte("")),
                    Ok(Some(r)) => {
                        if est_nombre_saisi(&r) {
                            Ok(Valeur::Nombre(js_trim(&r).parse().unwrap_or(f64::NAN)))
                        } else {
                            Ok(Valeur::texte(r))
                        }
                    }
                }
            }
            Expr::LireFichier(p) => {
                let chemin = js_texte(&self.evaluer(p)?);
                match self.hote.lire_fichier(&chemin) {
                    Ok(s) => Ok(Valeur::texte(s)),
                    Err(HoteErr::Indisponible) => Err(fad("lireFichier n'est pas disponible dans cet environnement.")),
                    Err(HoteErr::Msg(m)) => Err(interne(m)),
                }
            }
            Expr::Logique(et, g, d) => {
                let gv = self.evaluer(g)?;
                if *et {
                    if gv.vrai() {
                        self.evaluer(d)
                    } else {
                        Ok(gv)
                    }
                } else if gv.vrai() {
                    Ok(gv)
                } else {
                    self.evaluer(d)
                }
            }
            Expr::Non(x) => Ok(Valeur::Bool(!self.evaluer(x)?.vrai())),
            Expr::Neg(x, l) => match self.evaluer(x)? {
                Valeur::Nombre(n) => Ok(Valeur::Nombre(-n)),
                v => Err(fad_l(format!("Le signe \"-\" attend un nombre (reçu : {}).", type_de(&v)), *l)),
            },
            Expr::Bin(op, g, d, l) => {
                let gv = self.evaluer(g)?;
                let dv = self.evaluer(d)?;
                operer(*op, &gv, &dv, *l)
            }
            Expr::Appel(nom, args, l) => self.appeler(nom, args, *l),
            Expr::Ajouter(t, x, l) => {
                let t = self.evaluer(t)?;
                match &t {
                    Valeur::Liste(li) => {
                        let v = self.evaluer(x)?;
                        li.borrow_mut().push(v);
                        Ok(t.clone())
                    }
                    _ => Err(fad_l(format!("ajouter attend une liste en premier argument (reçu : {}).", type_de(&t)), *l)),
                }
            }
            Expr::ModifElem(t, i, v, l) => {
                let t = self.evaluer(t)?;
                match &t {
                    Valeur::Liste(li) => {
                        let iv = self.evaluer(i)?;
                        let n = li.borrow().len();
                        let k = self.verif_index(n, &iv, Some(*l))?;
                        let nv = self.evaluer(v)?;
                        // la liste a pu changer pendant l'évaluation
                        let mut b = li.borrow_mut();
                        if k < b.len() {
                            b[k] = nv;
                        } else {
                            while b.len() < k {
                                b.push(Valeur::Rien);
                            }
                            b.push(nv);
                        }
                        drop(b);
                        Ok(t.clone())
                    }
                    _ => Err(fad_l(
                        format!("modifierElement attend une liste en premier argument (reçu : {}).", type_de(&t)),
                        *l,
                    )),
                }
            }
            Expr::ModifCle(s, c, v, l) => {
                let s = self.evaluer(s)?;
                match &s {
                    Valeur::Struct(d) => {
                        let cle = js_texte(&self.evaluer(c)?);
                        let nv = self.evaluer(v)?;
                        d.borrow_mut().set(&cle, nv);
                        Ok(s.clone())
                    }
                    _ => Err(fad_l(
                        format!("modifierCle attend une structure en premier argument (reçu : {}).", type_de(&s)),
                        *l,
                    )),
                }
            }
        }
    }

    // ------------------------------------------------------------------ appels
    fn appeler(&mut self, nom: &Nom, args: &[Expr], ligne: usize) -> R<Valeur> {
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.evaluer(a)?);
        }
        self.appeler_vals(nom, vals, ligne)
    }

    pub(crate) fn appeler_vals(&mut self, nom: &Nom, vals: Vec<Valeur>, ligne: usize) -> R<Valeur> {
        if let Some(f @ Valeur::Fonction(_)) = self.chercher(nom) {
            return self.appeler_valeur(&f, vals, ligne);
        }
        if let Some(def) = self.fn_get(nom.id) {
            return self.appeler_def(&def.parametres, &def.corps, vals, ligne, nom, None);
        }
        if let Some(f) = natives::trouver(nom) {
            return match f(self, &vals, ligne) {
                Err(Flow::Nat(m)) => Err(fad_l(format!("{} : {}", nom, m), ligne)),
                r => r,
            };
        }
        if nom.contains('.') {
            if let Some(r) = self.appeler_pointe(nom, &vals, ligne)? {
                return Ok(r);
            }
        }
        Err(fad_l(
            format!(
                "Fonction \"{}\" inconnue : elle n'est ni déclarée dans le script (mot-clé \"fonction\"), ni une fonction native.",
                nom
            ),
            ligne,
        ))
    }

    pub fn appeler_valeur<A: AsRef<[Valeur]> + IntoIterator<Item = Valeur>>(&mut self, f: &Valeur, args: A, ligne: usize) -> R<Valeur> {
        let f = match f {
            Valeur::Fonction(f) => f.clone(),
            _ => {
                return Err(fad_l(
                    format!("Cette valeur n'est pas une fonction (reçu : {}).", type_de(f)),
                    ligne,
                ))
            }
        };
        if let Some(nom) = &f.natif {
            let nf = natives::trouver(nom).unwrap();
            return match nf(self, args.as_ref(), ligne) {
                Err(Flow::Nat(m)) => Err(fad_l(format!("{} : {}", nom, m), ligne)),
                r => r,
            };
        }
        let nom: &str = f.nom.as_deref().unwrap_or("fonction anonyme");
        self.appeler_def(&f.parametres, &f.corps, args, ligne, nom, f.fermeture.clone())
    }

    fn appeler_def<A: AsRef<[Valeur]> + IntoIterator<Item = Valeur>>(
        &mut self,
        params: &Rc<Vec<Nom>>,
        corps: &Rc<Corps>,
        args: A,
        ligne: usize,
        nom: &str,
        fermeture: Option<Scope>,
    ) -> R<Valeur> {
        if args.as_ref().len() != params.len() {
            return Err(fad_l(
                format!("La fonction \"{}\" attend {} argument(s), {} reçu(s).", nom, params.len(), args.as_ref().len()),
                ligne,
            ));
        }
        if self.profondeur >= 5000 {
            return Err(fad_l("Récursion trop profonde (plus de 5000 appels imbriqués).", ligne));
        }
        let locale = self.nouvelle_portee(fermeture, params.len());
        {
            let mut v = locale.vars.borrow_mut();
            for (p, a) in params.iter().zip(args.into_iter()) {
                v.definir(p.id, a);
            }
        }
        if self.debog.is_some() {
            self.appels.push((nom.to_string(), ligne));
        }
        let r = self.executer_dans(corps, locale);
        if self.debog.is_some() {
            self.appels.pop();
        }
        r.map_err(|f| crate::erreur::passer_fonction(f, nom, corps.module.as_deref()))
    }

    /// Portée de fonction recyclée (évite deux allocations par appel).
    #[inline]
    fn nouvelle_portee(&mut self, parent: Option<Scope>, n: usize) -> Scope {
        match self.portees_libres.pop() {
            Some(mut p) => {
                // les portées recyclées sont uniques (voir `rendre_portee`)
                if let Some(sd) = Rc::get_mut(&mut p) {
                    sd.parent = parent;
                    return p;
                }
                ScopeData::avec_capacite(parent, n + 2)
            }
            None => ScopeData::avec_capacite(parent, n + 2),
        }
    }

    #[inline]
    fn rendre_portee(&mut self, mut p: Scope) {
        if self.portees_libres.len() < 64 {
            if let Some(sd) = Rc::get_mut(&mut p) {
                // pas capturée par une fermeture : on peut la recycler
                if let Vars::Petit(v) = sd.vars.get_mut() {
                    v.clear();
                    sd.parent = None;
                    self.portees_libres.push(p);
                }
            }
        }
    }

    #[inline]
    fn executer_dans(&mut self, corps: &Rc<Corps>, locale: Scope) -> R<Valeur> {
        self.pile.push(locale);
        self.profondeur += 1;
        if self.debog.is_some() {
            self.ctx.push((corps.module.is_some(), corps.synth));
        }
        let r = crate::vm::executer_corps(self, corps);
        if self.debog.is_some() {
            self.ctx.pop();
        }
        let l = self.pile.pop().unwrap();
        self.profondeur -= 1;
        self.rendre_portee(l);
        r
    }

    /// Appel d'une fonction du script dont les arguments sont les `argc` valeurs au sommet de la pile de la VM.
    pub(crate) fn appeler_pile(&mut self, nom: &Nom, argc: usize, ligne: usize) -> R<Valeur> {
        if let Some(f @ Valeur::Fonction(_)) = self.chercher(nom) {
            let debut = self.vs.len() - argc;
            let vals: Vec<Valeur> = self.vs.drain(debut..).collect();
            return self.appeler_valeur(&f, vals, ligne);
        }
        if let Some(def) = self.fn_get(nom.id) {
            let debut = self.vs.len() - argc;
            if argc != def.parametres.len() {
                self.vs.truncate(debut);
                return Err(fad_l(
                    format!("La fonction \"{}\" attend {} argument(s), {} reçu(s).", nom, def.parametres.len(), argc),
                    ligne,
                ));
            }
            if self.profondeur >= 5000 {
                self.vs.truncate(debut);
                return Err(fad_l("Récursion trop profonde (plus de 5000 appels imbriqués).", ligne));
            }
            let locale = self.nouvelle_portee(None, argc);
            {
                let mut v = locale.vars.borrow_mut();
                for (p, a) in def.parametres.iter().zip(self.vs.drain(debut..)) {
                    v.definir(p.id, a);
                }
            }
            if self.debog.is_some() {
                self.appels.push((nom.to_string(), ligne));
            }
            let r = self.executer_dans(&def.corps, locale);
            if self.debog.is_some() {
                self.appels.pop();
            }
            return r.map_err(|f| crate::erreur::passer_fonction(f, &nom.to_string(), def.corps.module.as_deref()));
        }
        let debut = self.vs.len() - argc;
        let vals: Vec<Valeur> = self.vs.drain(debut..).collect();
        self.appeler_vals(nom, vals, ligne)
    }
}

fn est_nombre_saisi(r: &str) -> bool {
    // /^\s*-?\d+(\.\d+)?\s*$/
    let t = js_trim(r);
    let t = t.strip_prefix('-').unwrap_or(t);
    let mut parts = t.splitn(2, '.');
    let a = parts.next().unwrap_or("");
    if a.is_empty() || !a.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    match parts.next() {
        None => true,
        Some(b) => !b.is_empty() && b.bytes().all(|c| c.is_ascii_digit()),
    }
}

/// a % b identique à fmod (et à JavaScript), avec un chemin rapide pour les entiers (fmod logiciel = lent).
#[inline(always)]
pub(crate) fn modulo(a: f64, b: f64) -> f64 {
    // entiers : reste entier exact (les conversions saturent ; NaN → 0 mais 0 as f64 != NaN)
    let (ai, bi) = (a as i64, b as i64);
    if ai as f64 == a && bi as f64 == b && bi != 0 {
        let r = ai.wrapping_rem(bi);
        if r == 0 && a.is_sign_negative() {
            return -0.0;
        }
        return r as f64;
    }
    a % b
}

/// Ajoute la représentation textuelle de `v` (celle de `afficher`) à `s`, sans copie intermédiaire pour un texte.
#[inline]
pub(crate) fn ajouter_formate(s: &mut String, v: &Valeur) {
    match v {
        Valeur::Texte(t) => s.push_str(t),
        autre => s.push_str(&formater(autre)),
    }
}

pub fn puissance(a: f64, b: f64, ligne: usize) -> R<Valeur> {
    if a == 0.0 && b < 0.0 {
        return Err(fad_l("Division par zéro.", ligne));
    }
    let r = crate::natives::js_pow(a, b);
    if r.is_nan() {
        return Err(fad_l("Puissance impossible : une base négative demande un exposant entier.", ligne));
    }
    Ok(Valeur::Nombre(r))
}

pub fn operer(op: Op, g: &Valeur, d: &Valeur, ligne: usize) -> R<Valeur> {
    // chemin rapide : deux nombres
    if let (Valeur::Nombre(a), Valeur::Nombre(b)) = (g, d) {
        let (a, b) = (*a, *b);
        return match op {
            Op::Plus => Ok(Valeur::Nombre(a + b)),
            Op::Moins => Ok(Valeur::Nombre(a - b)),
            Op::Fois => Ok(Valeur::Nombre(a * b)),
            Op::Div | Op::Mod if b == 0.0 => Err(fad_l("Division par zéro.", ligne)),
            Op::Div => Ok(Valeur::Nombre(a / b)),
            Op::Mod => Ok(Valeur::Nombre(modulo(a, b))),
            Op::Puiss => puissance(a, b, ligne),
            Op::Egal => Ok(Valeur::Bool(a == b)),
            Op::Diff => Ok(Valeur::Bool(a != b)),
            Op::Inf => Ok(Valeur::Bool(a < b)),
            Op::Sup => Ok(Valeur::Bool(a > b)),
            Op::InfEg => Ok(Valeur::Bool(a <= b)),
            Op::SupEg => Ok(Valeur::Bool(a >= b)),
        };
    }
    match op {
        Op::Plus => {
            if matches!(g, Valeur::Texte(_)) || matches!(d, Valeur::Texte(_)) {
                let mut r = String::new();
                ajouter_formate(&mut r, g);
                ajouter_formate(&mut r, d);
                return Ok(Valeur::texte_string(r));
            }
            if let (Valeur::Liste(a), Valeur::Liste(b)) = (g, d) {
                let mut r = a.borrow().clone();
                r.extend(b.borrow().iter().cloned());
                return Ok(Valeur::liste(r));
            }
            Err(fad_l(format!("Impossible d'additionner {} et {}.", type_de(g), type_de(d)), ligne))
        }
        Op::Moins | Op::Fois | Op::Div | Op::Mod | Op::Puiss => Err(fad_l(
            format!("L'opérateur \"{}\" attend deux nombres (reçu : {} et {}).", op.symbole(), type_de(g), type_de(d)),
            ligne,
        )),
        Op::Egal => Ok(Valeur::Bool(egal(g, d))),
        Op::Diff => Ok(Valeur::Bool(!egal(g, d))),
        _ => {
            let ordre = match (g, d) {
                (Valeur::Texte(a), Valeur::Texte(b)) => Some(cmp_utf16(a, b)),
                _ => {
                    return Err(fad_l(
                        format!("Impossible de comparer {} et {} avec \"{}\".", type_de(g), type_de(d), op.symbole()),
                        ligne,
                    ))
                }
            };
            use std::cmp::Ordering::*;
            let r = match (op, ordre) {
                (_, None) => false,
                (Op::Inf, Some(o)) => o == Less,
                (Op::Sup, Some(o)) => o == Greater,
                (Op::InfEg, Some(o)) => o != Greater,
                (_, Some(o)) => o != Less,
            };
            Ok(Valeur::Bool(r))
        }
    }
}

// ------------------------------------------------------------------ API publique
pub fn decrire_erreur(e: &ErreurFadera, code: &str) -> (String, String) {
    let e = &ErreurFadera {
        message: if e.interne { format!("Erreur interne : {}", e.message) } else { e.message.clone() },
        ligne: e.ligne,
        interne: false,
        pile: Vec::new(),
        cadre: None,
        origine_fixee: false,
        origine_module: false,
    };
    match e.ligne {
        Some(l) if l > 0 => (
            format!("Ligne {} : {}", l, e.message),
            code.split('\n').nth(l - 1).unwrap_or("").trim().to_string(),
        ),
        _ => (e.message.clone(), String::new()),
    }
}

/// Analyse et exécute un programme. Renvoie l'interpréteur (pour inspection) ou l'erreur.
pub fn executer_programme(code: &str, hote: Box<dyn Hote>) -> Result<Interp, ErreurFadera> {
    let ast = analyser_code(code)?;
    let mut it = Interp::new(hote);
    match it.executer_programme_ast(&ast) {
        Ok(()) | Err(Flow::Retour(_)) => Ok(it),
        Err(Flow::Err(e)) => Err(e),
        Err(Flow::Nat(m)) => Err(ErreurFadera::new(m, None)),
        Err(_) => Ok(it),
    }
}

/// Bilan d'un lancement de tests (`tester "nom" … fin`).
pub struct BilanTests {
    pub total: usize,
    pub reussis: usize,
}

/// Lanceur de tests : exécute le programme, puis chaque `tester "nom" … fin` du niveau supérieur (dans l'ordre du source).
/// Rapport via `hote.afficher` (✓ / ✗ puis ligne finale). Une erreur hors d'un test interrompt tout (comme une exécution normale).
pub fn lancer_tests(code: &str, hote: Box<dyn Hote>) -> Result<BilanTests, ErreurFadera> {
    let ast = analyser_code(code)?;
    let mut it = Interp::new(hote);
    match it.executer_programme_ast(&ast) {
        Ok(()) | Err(Flow::Retour(_)) => {}
        Err(Flow::Err(e)) => return Err(e),
        Err(Flow::Nat(m)) => return Err(ErreurFadera::new(m, None)),
        Err(_) => {}
    }
    let tests: Vec<(usize, Rc<FnDef>)> = ast
        .iter()
        .filter_map(|s| match &s.k {
            StmtK::DeclFonction(d) if d.nom.starts_with("#test:") => Some((s.ligne, d.clone())),
            _ => None,
        })
        .collect();
    let mut reussis = 0;
    for (ligne, d) in &tests {
        let nom = &d.nom["#test:".len()..];
        match it.appeler_def(&d.parametres, &d.corps, Vec::<Valeur>::new(), *ligne, &d.nom, None) {
            Ok(_) => {
                reussis += 1;
                it.hote.afficher(&format!("✓ {}", nom));
            }
            Err(Flow::Err(e)) => {
                it.hote.afficher(&format!("✗ {}", nom));
                let t = decrire_erreur(&e, code).0;
                it.hote.afficher(&format!("    {}", t));
            }
            Err(Flow::Nat(m)) => {
                it.hote.afficher(&format!("✗ {}", nom));
                it.hote.afficher(&format!("    {}", m));
            }
            Err(_) => {
                reussis += 1;
                it.hote.afficher(&format!("✓ {}", nom));
            }
        }
    }
    let n = tests.len();
    it.hote.afficher(&if n == 0 {
        "Aucun test (écrivez : tester \"nom\" ... fin).".to_string()
    } else {
        format!("{} test(s) : {} réussi(s), {} échoué(s).", n, reussis, n - reussis)
    });
    Ok(BilanTests { total: n, reussis })
}

/// Comme `executer_programme`, mais conserve l'interpréteur même quand le programme échoue à l'exécution
/// (pour afficher les variables telles qu'elles étaient au moment de l'erreur). Une erreur d'analyse
/// (syntaxe) n'a pas d'interpréteur : `None`.
pub fn executer_programme_complet(code: &str, hote: Box<dyn Hote>) -> (Option<Interp>, Option<ErreurFadera>) {
    let ast = match analyser_code(code) {
        Ok(a) => a,
        Err(e) => return (None, Some(e)),
    };
    let mut it = Interp::new(hote);
    let erreur = match it.executer_programme_ast(&ast) {
        Ok(()) | Err(Flow::Retour(_)) => None,
        Err(Flow::Err(e)) => Some(e),
        Err(Flow::Nat(m)) => Some(ErreurFadera::new(m, None)),
        Err(_) => None,
    };
    (Some(it), erreur)
}

const PROFONDEUR_EXPORT: usize = 8;

fn valeur_exportable(v: &Valeur, prof: usize) -> Valeur {
    match v {
        Valeur::Rien | Valeur::Bool(_) | Valeur::Texte(_) => v.clone(),
        Valeur::Nombre(n) if n.is_finite() => v.clone(),
        Valeur::Liste(l) if prof < PROFONDEUR_EXPORT => {
            Valeur::liste(l.borrow().iter().map(|x| valeur_exportable(x, prof + 1)).collect())
        }
        Valeur::Struct(d) if prof < PROFONDEUR_EXPORT => {
            let d = d.borrow();
            let mut n = Dict::nouveau();
            for k in d.cles() {
                if let Some(x) = d.get(k) {
                    n.set(k, valeur_exportable(x, prof + 1));
                }
            }
            Valeur::structure(n)
        }
        // structures trop profondes (ou qui se contiennent elles-mêmes) : on s'arrête, sans appeler `formater`
        Valeur::Liste(_) => Valeur::texte("[…]"),
        Valeur::Struct(_) => Valeur::texte("{…}"),
        // nombres non finis, fonctions, réseaux : texte affichable
        autre => Valeur::texte(formater(autre)),
    }
}

impl Interp {
    /// État observable après l'exécution, pour le panneau des variables de l'IDE :
    /// { "variables": {nom: valeur…}, "fonctions": {nom: {"parametres": […], "lignes": N}} }.
    /// Prépare une nouvelle ligne de la ligne de commande interactive (session persistante) : remet à zéro les compteurs.
    pub fn preparer_ligne(&mut self) {
        self.etapes = 0;
        self.pile.truncate(1);
        self.profondeur = 0;
    }

    /// Évalue une ligne de la ligne de commande interactive dans la session persistante.
    /// `instruction` : instruction (soit, si, fonction…) ; sinon expression dont on renvoie la valeur affichable (None pour `rien`).
    /// Renvoie (valeur, erreur) ; l'erreur est le message formaté (« Ligne 1 : … »).
    pub fn evaluer_ligne(&mut self, code: &str, instruction: bool) -> (Option<String>, Option<String>) {
        self.preparer_ligne();
        let src = if instruction { code.to_string() } else { format!("soit __valeur_repl = {}", code) };
        let mut valeur = None;
        let mut erreur = None;
        match analyser_code(&src) {
            Err(e) => erreur = Some(decrire_erreur(&e, &src).0),
            Ok(ast) => match self.executer_programme_ast(&ast) {
                Ok(()) | Err(Flow::Retour(_)) => {
                    if !instruction {
                        if let Some(v) = self.globale_valeur("__valeur_repl") {
                            if !matches!(v, Valeur::Rien) {
                                valeur = Some(formater(&v));
                            }
                        }
                    }
                }
                Err(Flow::Err(e)) => erreur = Some(decrire_erreur(&e, &src).0),
                Err(Flow::Nat(m)) => erreur = Some(m),
                Err(_) => {}
            },
        }
        self.supprimer_globale("__valeur_repl");
        (valeur, erreur)
    }

    /// Supprime une variable globale (valeur temporaire de la ligne de commande).
    pub fn supprimer_globale(&mut self, nom: &str) {
        let id = crate::ast::Nom::nouveau(nom).id as usize;
        if let Vars::Dense(v) = &mut *self.globale.vars.borrow_mut() {
            if id < v.len() {
                v[id] = None;
            }
        }
    }

    /// Valeur d'une variable globale.
    pub fn globale_valeur(&self, nom: &str) -> Option<Valeur> {
        let id = crate::ast::Nom::nouveau(nom).id;
        self.globale.vars.borrow().get(id).cloned()
    }

    pub fn exporter_etat(&self) -> Valeur {
        let mut vars = Dict::nouveau();
        if let Vars::Dense(v) = &*self.globale.vars.borrow() {
            for (id, x) in v.iter().enumerate() {
                if let Some(x) = x {
                    if let Some(nom) = crate::ast::nom_depuis_id(id as u32) {
                        // noms internes (instances de classe, valeur de la ligne de commande) : invisibles
                        if nom.starts_with("__") || nom.contains(".#parent") {
                            continue;
                        }
                        vars.set(&nom, valeur_exportable(x, 0));
                    }
                }
            }
        }
        let mut fns = Dict::nouveau();
        for d in self.fonctions.iter().flatten() {
            let params: Vec<Valeur> = d.parametres.iter().map(|p| Valeur::texte(&**p)).collect();
            fns.set(
                &d.nom,
                Valeur::struct_de(vec![("parametres", Valeur::liste(params)), ("lignes", Valeur::Nombre(d.corps.stmts.len() as f64))]),
            );
        }
        Valeur::struct_de(vec![("variables", Valeur::structure(vars)), ("fonctions", Valeur::structure(fns))])
    }
}

use std::rc::Rc;

use crate::ast::*;
use crate::erreur::ErreurFadera;
use crate::lexer::*;
use crate::valeur::Valeur;

type PR<T> = Result<T, ErreurFadera>;

fn err<T>(m: impl Into<String>, l: usize) -> PR<T> {
    Err(ErreurFadera::new(m, Some(l)))
}

fn nom_type(t: Type) -> &'static str {
    match t {
        Type::Identifiant => "un nom",
        Type::Numero => "un nombre",
        Type::Chaine => "un texte",
        Type::Operateur => "un opérateur",
        Type::Symbole => "un symbole",
        Type::MotCle => "un mot-clé",
        Type::Logique => "et / ou",
    }
}

pub struct Parser {
    toks: Vec<Jeton>,
    pos: usize,
    prof_boucle: usize,
    /// Nom du module analysé (None pour le script de l'utilisateur) : copié dans chaque corps de fonction.
    module: Option<Rc<str>>,
    /// Instructions supplémentaires produites par une instruction (une `classe` en génère plusieurs) :
    /// les appelants les ajoutent juste après elle, au même niveau.
    suite: Vec<Stmt>,
}

impl Parser {
    pub fn new(toks: Vec<Jeton>) -> Parser {
        Parser { toks, pos: 0, prof_boucle: 0, module: None, suite: Vec::new() }
    }
    pub fn pour_module(toks: Vec<Jeton>, module: &str) -> Parser {
        Parser { toks, pos: 0, prof_boucle: 0, module: Some(Rc::from(module)), suite: Vec::new() }
    }
    fn corps(&self, stmts: Vec<Stmt>) -> Corps {
        Corps::dans_module(stmts, self.module.clone())
    }
    fn actuel(&self) -> Option<&Jeton> {
        self.toks.get(self.pos)
    }
    fn suivant(&self, k: usize) -> Option<&Jeton> {
        self.toks.get(self.pos + k)
    }
    fn avancer(&mut self) {
        self.pos += 1;
    }
    fn ligne_courante(&self) -> usize {
        self.actuel().or_else(|| self.toks.last()).map(|j| j.ligne).unwrap_or(1)
    }
    fn est_mot(&self, v: &str) -> bool {
        matches!(self.actuel(), Some(a) if a.t == Type::MotCle && a.valeur == v)
    }
    fn est_symbole(&self, v: &str) -> bool {
        matches!(self.actuel(), Some(a) if a.t == Type::Symbole && a.valeur == v)
    }
    fn est_op(&self, ops: &[&str]) -> bool {
        matches!(self.actuel(), Some(a) if a.t == Type::Operateur && ops.contains(&a.valeur.as_str()))
    }
    fn est_logique(&self, v: &str) -> bool {
        matches!(self.actuel(), Some(a) if a.t == Type::Logique && a.valeur == v)
    }

    fn consommer(&mut self, t: Type, valeur: Option<&str>) -> PR<Jeton> {
        let attendu = match valeur {
            Some(v) => format!("\"{}\"", v),
            None => nom_type(t).to_string(),
        };
        let j = match self.actuel() {
            None => {
                return err(
                    format!("Fin de code inattendue : {} attendu. Il manque probablement un \"fin\".", attendu),
                    self.ligne_courante(),
                )
            }
            Some(j) => j.clone(),
        };
        if j.t != t || valeur.map_or(false, |v| j.valeur != v) {
            let mut msg = format!("{} attendu, mais {} trouvé.", attendu, decrire(Some(&j)));
            if t == Type::Identifiant && j.t == Type::MotCle {
                msg = format!("\"{}\" est un mot réservé du langage : choisissez un autre nom.", j.valeur);
            } else if j.t == Type::Operateur && j.valeur == "=" && matches!(valeur, Some("alors") | Some("faire")) {
                msg.push_str(" Pour comparer deux valeurs, utilisez \"==\".");
            }
            return err(msg, j.ligne);
        }
        self.avancer();
        Ok(j)
    }

    pub fn analyser(&mut self) -> PR<Vec<Stmt>> {
        let mut corps = Vec::new();
        while self.pos < self.toks.len() {
            corps.push(self.analyser_instruction()?);
            corps.append(&mut self.suite);
        }
        Ok(corps)
    }

    fn analyser_bloc(&mut self, fins: &[&str], nom_bloc: &str, ligne_debut: usize) -> PR<Vec<Stmt>> {
        let mut corps = Vec::new();
        loop {
            match self.actuel() {
                None => {
                    return err(
                        format!("Le bloc \"{}\" ouvert ligne {} n'est jamais fermé : il manque \"fin\".", nom_bloc, ligne_debut),
                        ligne_debut,
                    )
                }
                Some(a) if a.t == Type::MotCle && fins.contains(&a.valeur.as_str()) => return Ok(corps),
                _ => {
                    corps.push(self.analyser_instruction()?);
                    corps.append(&mut self.suite);
                }
            }
        }
    }

    fn analyser_instruction(&mut self) -> PR<Stmt> {
        let ligne = self.actuel().unwrap().ligne;
        let j = self.actuel().unwrap().clone();
        // mots contextuels (pas réservés) : « classe Nom … fin » et « tester "nom" … fin »
        if j.t == Type::Identifiant {
            match (j.valeur.as_str(), self.suivant(1).map(|s| s.t)) {
                ("classe", Some(Type::Identifiant)) => return self.analyser_classe(),
                ("tester", Some(Type::Chaine)) => return self.analyser_test(),
                _ => {}
            }
        }
        let k = self.instruction(&j)?;
        Ok(Stmt { ligne, k })
    }

    fn instruction(&mut self, j: &Jeton) -> PR<StmtK> {
        if j.t == Type::MotCle {
            match j.valeur.as_str() {
                "soit" => return self.analyser_variable(),
                "structure" => return self.analyser_structure(),
                "si" => return self.analyser_condition(),
                "afficher" => return self.analyser_affichage(),
                "tantque" => return self.analyser_tantque(),
                "pour" => return self.analyser_pour(),
                "fonction" => return self.analyser_fonction(),
                "renvoyer" => return self.analyser_renvoyer(),
                "ajouter" | "modifierElement" | "modifierCle" => {
                    return Ok(StmtK::Expr(self.analyser_primaire()?));
                }
                "ecrireFichier" => {
                    let mut a = self.appel_mot("ecrireFichier", 2, 2)?;
                    let c = a.pop().unwrap();
                    let p = a.pop().unwrap();
                    return Ok(StmtK::EcrireFichier(p, c));
                }
                "fnf" => {
                    self.avancer();
                    if let Some(a) = self.actuel() {
                        if a.t == Type::MotCle && a.valeur != "fnf" {
                            let a = a.clone();
                            return self.instruction(&a);
                        }
                    }
                    let id = self.consommer(Type::Identifiant, None)?;
                    self.consommer(Type::Symbole, Some("("))?;
                    let args = self.arguments_entre(")")?;
                    return Ok(StmtK::Expr(Expr::Appel(Nom::nouveau(&id.valeur), args, id.ligne)));
                }
                "essayer" => return self.analyser_essayer(),
                "rompre" | "continuer" => {
                    if self.prof_boucle == 0 {
                        return err(
                            format!("\"{}\" ne peut être utilisé qu'à l'intérieur d'une boucle (pour, tantque).", j.valeur),
                            j.ligne,
                        );
                    }
                    self.avancer();
                    return Ok(if j.valeur == "rompre" { StmtK::Rompre } else { StmtK::Continuer });
                }
                "selon" => return self.analyser_selon(),
                "importer" => {
                    self.avancer();
                    let e = self.analyser_expression()?;
                    // importer "outils" comme o   (« comme » n'est reconnu que collé à l'importer, sur la même ligne)
                    let mut alias = None;
                    if let Some(c) = self.actuel() {
                        if c.t == Type::Identifiant && c.valeur == "comme" && c.ligne == j.ligne {
                            let c = c.clone();
                            match self.suivant(1) {
                                Some(a) if a.t == Type::Identifiant => alias = Some(a.valeur.clone()),
                                autre => return err(format!("importer ... comme : un nom est attendu après \"comme\", mais {} trouvé.", decrire(autre)), c.ligne),
                            }
                            self.avancer();
                            self.avancer();
                        }
                    }
                    return Ok(StmtK::Importer(e, alias));
                }
                "cas" | "defaut" => {
                    return err(format!("\"{}\" inattendu : il doit se trouver dans un bloc \"selon ... faire\".", j.valeur), j.ligne)
                }
                "attraper" => return err("\"attraper\" inattendu : il doit suivre un bloc \"essayer\".", j.ligne),
                "saisir" | "hasard" | "taille" | "lireElement" | "lireCle" | "lireFichier" | "liste" => {
                    return Ok(StmtK::Expr(self.analyser_expression()?));
                }
                "fin" | "sinon" => {
                    return err(
                        format!("\"{}\" inattendu : aucun bloc (si, pour, tantque, fonction) n'est ouvert à cet endroit.", j.valeur),
                        j.ligne,
                    )
                }
                _ => {}
            }
        }
        if j.t == Type::Identifiant {
            if let Some(s) = self.suivant(1) {
                if s.t == Type::Symbole && s.valeur == "(" {
                    return Ok(StmtK::Expr(self.analyser_expression()?));
                }
                if s.t == Type::Operateur && s.valeur == "=" {
                    self.avancer();
                    self.avancer();
                    return Ok(StmtK::Affect(Nom::nouveau(&j.valeur), self.analyser_expression()?));
                }
            }
        }
        let extra = if j.t == Type::Identifiant {
            " Pour créer une variable écrivez \"soit nom = valeur\", pour la modifier \"nom = valeur\", pour appeler une fonction \"nom(...)\"."
        } else {
            ""
        };
        err(format!("Instruction inconnue : {}.{}", decrire(Some(j)), extra), j.ligne)
    }

    /// `classe C [herite P] … fin`  →  fonctions `C.m(soi, …)`, variable `C.#parent`, constructeur `C(…)`
    /// (même réduction que le moteur JavaScript : voir CONCEPTION_CLASSES_TESTS.md).
    fn analyser_classe(&mut self) -> PR<Stmt> {
        let l = self.actuel().unwrap().ligne;
        self.avancer(); // « classe »
        let nom = self.consommer(Type::Identifiant, None)?.valeur;
        let mut parent: Option<String> = None;
        if let Some(a) = self.actuel() {
            if a.t == Type::Identifiant && a.valeur == "herite" && a.ligne == l {
                self.avancer();
                match self.actuel() {
                    Some(p) if p.t == Type::Identifiant => {
                        parent = Some(p.valeur.clone());
                        self.avancer();
                    }
                    autre => {
                        return err(
                            format!("classe ... herite : un nom de classe est attendu après \"herite\", mais {} trouvé.", decrire(autre)),
                            l,
                        )
                    }
                }
            }
        }
        let mut noeuds: Vec<Stmt> = Vec::new();
        let mut vues: Vec<String> = Vec::new();
        let mut creer: Option<Vec<Nom>> = None;
        let soi = Nom::nouveau("soi");
        loop {
            let j = match self.actuel() {
                None => {
                    return err(
                        format!("Le bloc \"classe {}\" ouvert ligne {} n'est jamais fermé : il manque \"fin\".", nom, l),
                        l,
                    )
                }
                Some(j) => j.clone(),
            };
            if j.t == Type::MotCle && j.valeur == "fin" {
                break;
            }
            if !(j.t == Type::MotCle && j.valeur == "fonction") {
                return err(
                    format!("Dans une classe, seules les méthodes (fonction ... fin) sont permises (reçu : {}).", decrire(Some(&j))),
                    j.ligne,
                );
            }
            let lm = j.ligne;
            let d = match self.analyser_fonction()? {
                StmtK::DeclFonction(d) => d,
                _ => unreachable!(),
            };
            if vues.contains(&d.nom) {
                return err(format!("La méthode \"{}\" est déjà définie dans la classe \"{}\".", d.nom, nom), lm);
            }
            vues.push(d.nom.clone());
            if d.parametres.iter().any(|p| &*p.s == "soi") {
                return err("\"soi\" est réservé dans une méthode : il désigne déjà l'objet courant.", lm);
            }
            if d.nom == "creer" {
                creer = Some(d.parametres.to_vec());
            }
            let mut params = vec![soi.clone()];
            params.extend(d.parametres.iter().cloned());
            let complet = format!("{}.{}", nom, d.nom);
            noeuds.push(Stmt {
                ligne: lm,
                k: StmtK::DeclFonction(Rc::new(FnDef {
                    ident: Nom::nouveau(&complet),
                    nom: complet,
                    parametres: Rc::new(params),
                    corps: d.corps.clone(),
                })),
            });
        }
        self.consommer(Type::MotCle, Some("fin"))?;
        if let Some(p) = parent {
            noeuds.push(Stmt {
                ligne: l,
                k: StmtK::DeclVar(Nom::nouveau(&format!("{}.#parent", nom)), Expr::Lit(Valeur::texte(&p))),
            });
        }
        let params: Vec<Nom> = creer.clone().unwrap_or_default();
        let mut corps = vec![Stmt {
            ligne: l,
            k: StmtK::DeclVar(soi.clone(), Expr::Appel(Nom::nouveau("__instance"), vec![Expr::Lit(Valeur::texte(&nom))], l)),
        }];
        if creer.is_some() {
            let mut args = vec![Expr::Var(soi.clone(), l)];
            args.extend(params.iter().map(|p| Expr::Var(p.clone(), l)));
            corps.push(Stmt { ligne: l, k: StmtK::Expr(Expr::Appel(Nom::nouveau(&format!("{}.creer", nom)), args, l)) });
        }
        corps.push(Stmt { ligne: l, k: StmtK::Renvoyer(Some(Expr::Var(soi.clone(), l))) });
        noeuds.push(Stmt {
            ligne: l,
            k: StmtK::DeclFonction(Rc::new(FnDef {
                ident: Nom::nouveau(&nom),
                nom: nom.clone(),
                parametres: Rc::new(params),
                corps: Rc::new({
                    let mut c = self.corps(corps);
                    c.synth = true;
                    c
                }),
            })),
        });
        let mut it = noeuds.into_iter();
        let premier = it.next().unwrap();
        self.suite.extend(it);
        Ok(premier)
    }

    /// `tester "nom" … fin`  →  fonction sans paramètre `#test:nom` (jamais appelée par une exécution normale).
    fn analyser_test(&mut self) -> PR<Stmt> {
        let l = self.actuel().unwrap().ligne;
        self.avancer(); // « tester »
        let nom = self.consommer(Type::Chaine, None)?.valeur;
        let sauv = self.prof_boucle;
        self.prof_boucle = 0;
        let corps = self.analyser_bloc(&["fin"], "tester", l);
        self.prof_boucle = sauv;
        let corps = corps?;
        self.consommer(Type::MotCle, Some("fin"))?;
        let complet = format!("#test:{}", nom);
        Ok(Stmt {
            ligne: l,
            k: StmtK::DeclFonction(Rc::new(FnDef {
                ident: Nom::nouveau(&complet),
                nom: complet,
                parametres: Rc::new(vec![]),
                corps: Rc::new(self.corps(corps)),
            })),
        })
    }

    fn analyser_variable(&mut self) -> PR<StmtK> {
        self.consommer(Type::MotCle, Some("soit"))?;
        let nom = self.consommer(Type::Identifiant, None)?.valeur;
        self.consommer(Type::Operateur, Some("="))?;
        Ok(StmtK::DeclVar(Nom::nouveau(&nom), self.analyser_expression()?))
    }
    fn analyser_structure(&mut self) -> PR<StmtK> {
        self.consommer(Type::MotCle, Some("structure"))?;
        let nom = self.consommer(Type::Identifiant, None)?.valeur;
        self.consommer(Type::Operateur, Some("="))?;
        Ok(StmtK::DeclVar(Nom::nouveau(&nom), self.analyser_litteral_structure()?))
    }
    fn analyser_litteral_structure(&mut self) -> PR<Expr> {
        self.consommer(Type::Symbole, Some("{"))?;
        let mut paires = Vec::new();
        while self.actuel().is_some() && !self.est_symbole("}") {
            let k = self.actuel().unwrap().clone();
            if !matches!(k.t, Type::Identifiant | Type::MotCle | Type::Chaine) {
                return err(format!("Nom de champ attendu, mais {} trouvé.", decrire(Some(&k))), k.ligne);
            }
            self.avancer();
            self.consommer(Type::Symbole, Some(":"))?;
            paires.push((k.valeur, self.analyser_expression()?));
            if self.est_symbole(",") {
                self.avancer();
            } else {
                break;
            }
        }
        self.consommer(Type::Symbole, Some("}"))?;
        Ok(Expr::Struct(paires))
    }
    fn analyser_selon(&mut self) -> PR<StmtK> {
        let l = self.consommer(Type::MotCle, Some("selon"))?.ligne;
        let expr = self.analyser_expression()?;
        self.consommer(Type::MotCle, Some("faire"))?;
        let mut cas = Vec::new();
        let mut defaut: Option<Vec<Stmt>> = None;
        loop {
            if self.actuel().is_none() {
                return err(format!("Le bloc \"selon\" ouvert ligne {} n'est jamais fermé : il manque \"fin\".", l), l);
            }
            if self.est_mot("cas") {
                let tc = self.consommer(Type::MotCle, Some("cas"))?;
                let mut valeurs = vec![self.analyser_expression()?];
                while self.est_symbole(",") {
                    self.avancer();
                    valeurs.push(self.analyser_expression()?);
                }
                self.consommer(Type::MotCle, Some("alors"))?;
                let corps = self.analyser_bloc(&["cas", "defaut", "fin"], "cas", tc.ligne)?;
                cas.push(Cas { valeurs, corps });
            } else if self.est_mot("defaut") {
                let td = self.consommer(Type::MotCle, Some("defaut"))?;
                if defaut.is_some() {
                    return err("Un bloc \"selon\" ne peut avoir qu'un seul \"defaut\".", td.ligne);
                }
                if self.est_mot("alors") {
                    self.avancer();
                }
                defaut = Some(self.analyser_bloc(&["cas", "defaut", "fin"], "defaut", td.ligne)?);
            } else if self.est_mot("fin") {
                self.avancer();
                break;
            } else {
                let a = self.actuel().unwrap().clone();
                return err(
                    format!("\"cas\", \"defaut\" ou \"fin\" attendu dans un bloc selon, mais {} trouvé.", decrire(Some(&a))),
                    a.ligne,
                );
            }
        }
        Ok(StmtK::Selon { expr, cas, defaut })
    }
    fn liste_parametres(&mut self) -> PR<Vec<Nom>> {
        let mut p = Vec::new();
        if !self.est_symbole(")") {
            p.push(Nom::nouveau(&self.consommer(Type::Identifiant, None)?.valeur));
            while self.est_symbole(",") {
                self.avancer();
                p.push(Nom::nouveau(&self.consommer(Type::Identifiant, None)?.valeur));
            }
        }
        Ok(p)
    }
    fn est_lambda_parenthese(&self) -> bool {
        let mut prof = 0i32;
        let mut i = self.pos;
        while i < self.toks.len() {
            let t = &self.toks[i];
            if t.t == Type::Symbole {
                if t.valeur == "(" {
                    prof += 1;
                } else if t.valeur == ")" {
                    prof -= 1;
                    if prof == 0 {
                        break;
                    }
                }
            }
            i += 1;
        }
        i < self.toks.len() && matches!(self.toks.get(i + 1), Some(s) if s.t == Type::Symbole && s.valeur == "=>")
    }
    fn analyser_lambda(&mut self) -> PR<Expr> {
        let ligne = self.actuel().unwrap().ligne;
        let sauv = self.prof_boucle;
        self.prof_boucle = 0;
        let r = self.analyser_lambda_interne(ligne);
        self.prof_boucle = sauv;
        r
    }
    fn analyser_lambda_interne(&mut self, ligne: usize) -> PR<Expr> {
        if self.est_mot("fonction") {
            self.avancer();
            self.consommer(Type::Symbole, Some("("))?;
            let parametres = self.liste_parametres()?;
            self.consommer(Type::Symbole, Some(")"))?;
            let corps = self.analyser_bloc(&["fin"], "fonction anonyme", ligne)?;
            self.consommer(Type::MotCle, Some("fin"))?;
            return Ok(Expr::Lambda { parametres: Rc::new(parametres), corps: Rc::new(self.corps(corps)) });
        }
        let parametres;
        if self.est_symbole("(") {
            self.avancer();
            parametres = self.liste_parametres()?;
            self.consommer(Type::Symbole, Some(")"))?;
        } else {
            parametres = vec![Nom::nouveau(&self.consommer(Type::Identifiant, None)?.valeur)];
        }
        self.consommer(Type::Symbole, Some("=>"))?;
        let e = self.analyser_expression()?;
        Ok(Expr::Lambda {
            parametres: Rc::new(parametres),
            corps: Rc::new(self.corps(vec![Stmt { ligne, k: StmtK::Renvoyer(Some(e)) }])),
        })
    }
    fn analyser_essayer(&mut self) -> PR<StmtK> {
        let l = self.consommer(Type::MotCle, Some("essayer"))?.ligne;
        let corps = self.analyser_bloc(&["attraper"], "essayer", l)?;
        let ta = self.consommer(Type::MotCle, Some("attraper"))?;
        let mut nom = "erreur".to_string();
        if let Some(a) = self.actuel() {
            if a.t == Type::Identifiant && a.ligne == ta.ligne {
                nom = self.consommer(Type::Identifiant, None)?.valeur;
            }
        }
        let gestion = self.analyser_bloc(&["fin"], "attraper", ta.ligne)?;
        self.consommer(Type::MotCle, Some("fin"))?;
        Ok(StmtK::Essayer { corps, nom: Nom::nouveau(&nom), gestion })
    }
    fn analyser_condition(&mut self) -> PR<StmtK> {
        let l = self.consommer(Type::MotCle, Some("si"))?.ligne;
        let condition = self.analyser_expression()?;
        self.consommer(Type::MotCle, Some("alors"))?;
        let alors = self.analyser_bloc(&["sinon", "fin"], "si", l)?;
        let mut sinon = None;
        if self.est_mot("sinon") {
            let ts = self.consommer(Type::MotCle, Some("sinon"))?;
            if self.est_mot("si") && self.actuel().unwrap().ligne == ts.ligne {
                let interne = self.analyser_condition()?;
                return Ok(StmtK::Si(condition, alors, Some(vec![Stmt { ligne: ts.ligne, k: interne }])));
            }
            sinon = Some(self.analyser_bloc(&["fin"], "sinon", ts.ligne)?);
        }
        self.consommer(Type::MotCle, Some("fin"))?;
        Ok(StmtK::Si(condition, alors, sinon))
    }
    fn analyser_tantque(&mut self) -> PR<StmtK> {
        let l = self.consommer(Type::MotCle, Some("tantque"))?.ligne;
        let condition = self.analyser_expression()?;
        self.consommer(Type::MotCle, Some("faire"))?;
        self.prof_boucle += 1;
        let corps = self.analyser_bloc(&["fin"], "tantque", l);
        self.prof_boucle -= 1;
        let corps = corps?;
        self.consommer(Type::MotCle, Some("fin"))?;
        Ok(StmtK::TantQue(condition, corps))
    }
    fn analyser_pour(&mut self) -> PR<StmtK> {
        let l = self.consommer(Type::MotCle, Some("pour"))?.ligne;
        {
            let t1 = self.suivant(0);
            let t2 = self.suivant(1);
            let t3 = self.suivant(2);
            if let (Some(t1), Some(t2), Some(t3)) = (t1, t2, t3) {
                if t1.t == Type::Identifiant
                    && t1.valeur == "chaque"
                    && t2.t == Type::Identifiant
                    && t3.t == Type::Identifiant
                    && t3.valeur == "dans"
                {
                    self.avancer();
                    let nom = self.consommer(Type::Identifiant, None)?.valeur;
                    self.avancer();
                    let iterable = self.analyser_expression()?;
                    self.consommer(Type::MotCle, Some("faire"))?;
                    self.prof_boucle += 1;
                    let corps = self.analyser_bloc(&["fin"], "pour chaque", l);
                    self.prof_boucle -= 1;
                    let corps = corps?;
                    self.consommer(Type::MotCle, Some("fin"))?;
                    return Ok(StmtK::PourChaque { nom: Nom::nouveau(&nom), iterable, corps });
                }
            }
        }
        let nom = self.consommer(Type::Identifiant, None)?.valeur;
        self.consommer(Type::MotCle, Some("allantde"))?;
        let debut = self.analyser_expression()?;
        let sep = self.actuel().cloned();
        match &sep {
            Some(s) if s.t == Type::Identifiant && (s.valeur == "a" || s.valeur == "à") => {}
            _ => {
                return err(
                    format!(
                        "\"a\" attendu entre les deux bornes (exemple : pour i allantde 1 a 10 faire), mais {} trouvé.",
                        decrire(sep.as_ref())
                    ),
                    sep.as_ref().map(|s| s.ligne).unwrap_or(l),
                )
            }
        }
        self.avancer();
        let fin = self.analyser_expression()?;
        let mut pas = None;
        if matches!(self.actuel(), Some(a) if a.t == Type::Identifiant && a.valeur == "pas") {
            self.avancer();
            pas = Some(self.analyser_expression()?);
        }
        self.consommer(Type::MotCle, Some("faire"))?;
        self.prof_boucle += 1;
        let corps = self.analyser_bloc(&["fin"], "pour", l);
        self.prof_boucle -= 1;
        let corps = corps?;
        self.consommer(Type::MotCle, Some("fin"))?;
        Ok(StmtK::Pour { nom: Nom::nouveau(&nom), debut, fin, pas, corps })
    }
    fn analyser_fonction(&mut self) -> PR<StmtK> {
        let l = self.consommer(Type::MotCle, Some("fonction"))?.ligne;
        let nom = self.consommer(Type::Identifiant, None)?.valeur;
        self.consommer(Type::Symbole, Some("("))?;
        let parametres = self.liste_parametres()?;
        self.consommer(Type::Symbole, Some(")"))?;
        let sauv = self.prof_boucle;
        self.prof_boucle = 0;
        let corps = self.analyser_bloc(&["fin"], &format!("fonction {}", nom), l);
        self.prof_boucle = sauv;
        let corps = corps?;
        self.consommer(Type::MotCle, Some("fin"))?;
        Ok(StmtK::DeclFonction(Rc::new(FnDef { ident: Nom::nouveau(&nom), nom, parametres: Rc::new(parametres), corps: Rc::new(self.corps(corps)) })))
    }
    fn analyser_renvoyer(&mut self) -> PR<StmtK> {
        self.consommer(Type::MotCle, Some("renvoyer"))?;
        match self.actuel() {
            None => return Ok(StmtK::Renvoyer(None)),
            Some(a) if a.t == Type::MotCle && ["fin", "sinon", "cas", "defaut", "attraper"].contains(&a.valeur.as_str()) => {
                return Ok(StmtK::Renvoyer(None))
            }
            _ => {}
        }
        Ok(StmtK::Renvoyer(Some(self.analyser_expression()?)))
    }
    fn analyser_affichage(&mut self) -> PR<StmtK> {
        self.consommer(Type::MotCle, Some("afficher"))?;
        let mut e = vec![self.analyser_expression()?];
        while self.est_symbole(",") {
            self.avancer();
            e.push(self.analyser_expression()?);
        }
        Ok(StmtK::Afficher(e))
    }
    fn arguments_entre(&mut self, fermant: &str) -> PR<Vec<Expr>> {
        let mut args = Vec::new();
        if !self.est_symbole(fermant) {
            args.push(self.analyser_expression()?);
            while self.est_symbole(",") {
                self.avancer();
                args.push(self.analyser_expression()?);
            }
        }
        self.consommer(Type::Symbole, Some(fermant))?;
        Ok(args)
    }
    fn appel_mot(&mut self, nom: &str, min: usize, max: usize) -> PR<Vec<Expr>> {
        let l = self.consommer(Type::MotCle, Some(nom))?.ligne;
        self.consommer(Type::Symbole, Some("("))?;
        let a = self.arguments_entre(")")?;
        if a.len() < min || a.len() > max {
            let att = if min == max { min.to_string() } else if max == usize::MAX { format!("{} à Infinity", min) } else { format!("{} à {}", min, max) };
            return err(format!("{}(...) attend {} argument(s), {} reçu(s).", nom, att, a.len()), l);
        }
        Ok(a)
    }

    // --- Expressions ---
    pub fn analyser_expression(&mut self) -> PR<Expr> {
        self.analyser_ou()
    }
    fn analyser_ou(&mut self) -> PR<Expr> {
        let mut g = self.analyser_et()?;
        while self.est_logique("ou") {
            self.avancer();
            let d = self.analyser_et()?;
            g = Expr::Logique(false, Box::new(g), Box::new(d));
        }
        Ok(g)
    }
    fn analyser_et(&mut self) -> PR<Expr> {
        let mut g = self.analyser_non()?;
        while self.est_logique("et") {
            self.avancer();
            let d = self.analyser_non()?;
            g = Expr::Logique(true, Box::new(g), Box::new(d));
        }
        Ok(g)
    }
    fn analyser_non(&mut self) -> PR<Expr> {
        if self.est_mot("non") {
            self.avancer();
            return Ok(Expr::Non(Box::new(self.analyser_non()?)));
        }
        self.binaire(0)
    }
    fn binaire(&mut self, niveau: u8) -> PR<Expr> {
        let ops: &[&str] = match niveau {
            0 => &["==", "!=", "<", ">", "<=", ">="],
            1 => &["+", "-"],
            _ => &["*", "/", "%"],
        };
        let mut g = self.suivant_niveau(niveau)?;
        while self.est_op(ops) {
            let j = self.actuel().unwrap().clone();
            self.avancer();
            let d = self.suivant_niveau(niveau)?;
            g = Expr::Bin(Op::depuis(&j.valeur), Box::new(g), Box::new(d), j.ligne);
        }
        Ok(g)
    }
    fn suivant_niveau(&mut self, niveau: u8) -> PR<Expr> {
        if niveau >= 2 {
            self.analyser_unaire()
        } else {
            self.binaire(niveau + 1)
        }
    }
    fn analyser_unaire(&mut self) -> PR<Expr> {
        if self.est_op(&["-"]) {
            let l = self.actuel().unwrap().ligne;
            self.avancer();
            return Ok(Expr::Neg(Box::new(self.analyser_unaire()?), l));
        }
        self.analyser_puissance()
    }
    /// a ** b : associatif à droite, plus prioritaire que * et que le « - » devant ; l'exposant peut être négatif.
    fn analyser_puissance(&mut self) -> PR<Expr> {
        let base = self.analyser_primaire()?;
        if self.est_op(&["**"]) {
            let j = self.actuel().unwrap().clone();
            self.avancer();
            let e = self.analyser_unaire()?;
            return Ok(Expr::Bin(Op::Puiss, Box::new(base), Box::new(e), j.ligne));
        }
        Ok(base)
    }
    /// Valeur primaire suivie d'éventuels enchaînements : `Compte(1).deposer(2).solde`.
    fn analyser_primaire(&mut self) -> PR<Expr> {
        let mut e = self.primaire_base()?;
        while self.est_symbole(".") {
            let pt = self.actuel().unwrap().clone();
            self.avancer();
            let nom = match self.actuel() {
                Some(j) if j.t == Type::Identifiant => j.clone(),
                autre => return err(format!("un nom de champ ou de méthode est attendu après \".\", mais {} trouvé.", decrire(autre)), pt.ligne),
            };
            self.avancer();
            let segs: Vec<&str> = nom.valeur.split('.').collect();
            for (i, seg) in segs.iter().enumerate() {
                if i + 1 == segs.len() && self.est_symbole("(") {
                    self.avancer();
                    let args = self.arguments_entre(")")?;
                    e = Expr::Methode(Box::new(e), seg.to_string(), args, nom.ligne);
                } else {
                    e = Expr::Champ(Box::new(e), seg.to_string(), nom.ligne);
                }
            }
        }
        Ok(e)
    }

    fn primaire_base(&mut self) -> PR<Expr> {
        let j = match self.actuel() {
            None => {
                return err("Fin de code inattendue : une valeur ou une expression est attendue.", self.ligne_courante())
            }
            Some(j) => j.clone(),
        };
        let ligne = j.ligne;
        if j.t == Type::Numero {
            self.avancer();
            return Ok(Expr::Lit(Valeur::Nombre(j.num)));
        }
        if j.t == Type::Chaine {
            self.avancer();
            return Ok(Expr::Lit(Valeur::texte(&j.valeur)));
        }
        if j.t == Type::Symbole {
            if j.valeur == "(" {
                if self.est_lambda_parenthese() {
                    return self.analyser_lambda();
                }
                self.avancer();
                let e = self.analyser_expression()?;
                self.consommer(Type::Symbole, Some(")"))?;
                return Ok(e);
            }
            if j.valeur == "[" {
                self.avancer();
                return Ok(Expr::Liste(self.arguments_entre("]")?));
            }
            if j.valeur == "{" {
                return self.analyser_litteral_structure();
            }
        }
        if j.t == Type::Identifiant {
            if let Some(s) = self.suivant(1) {
                if s.t == Type::Symbole && s.valeur == "=>" {
                    return self.analyser_lambda();
                }
                if s.t == Type::Symbole && s.valeur == "(" {
                    self.avancer();
                    self.avancer();
                    return Ok(Expr::Appel(Nom::nouveau(&j.valeur), self.arguments_entre(")")?, ligne));
                }
            }
            self.avancer();
            return Ok(Expr::Var(Nom::nouveau(&j.valeur), ligne));
        }
        if j.t == Type::MotCle {
            match j.valeur.as_str() {
                "vrai" => {
                    self.avancer();
                    return Ok(Expr::Lit(Valeur::Bool(true)));
                }
                "faux" => {
                    self.avancer();
                    return Ok(Expr::Lit(Valeur::Bool(false)));
                }
                "liste" => return Ok(Expr::Liste(self.appel_mot("liste", 0, usize::MAX)?)),
                "taille" => {
                    let mut a = self.appel_mot("taille", 1, 1)?;
                    return Ok(Expr::Taille(Box::new(a.remove(0)), ligne));
                }
                "lireElement" => {
                    let mut a = self.appel_mot("lireElement", 2, 2)?;
                    let i = a.pop().unwrap();
                    let l = a.pop().unwrap();
                    return Ok(Expr::LireElement(Box::new(l), Box::new(i), ligne));
                }
                "lireCle" => {
                    let mut a = self.appel_mot("lireCle", 2, 2)?;
                    let c = a.pop().unwrap();
                    let s = a.pop().unwrap();
                    return Ok(Expr::LireCle(Box::new(s), Box::new(c), ligne));
                }
                "hasard" => {
                    let mut a = self.appel_mot("hasard", 2, 2)?;
                    let b = a.pop().unwrap();
                    let c = a.pop().unwrap();
                    return Ok(Expr::Hasard(Box::new(c), Box::new(b), ligne));
                }
                "fonction" => return self.analyser_lambda(),
                "fnf" => {
                    self.avancer();
                    if let Some(a) = self.actuel() {
                        if a.t == Type::MotCle && a.valeur != "fnf" {
                            return self.analyser_primaire();
                        }
                    }
                    let id = self.consommer(Type::Identifiant, None)?;
                    self.consommer(Type::Symbole, Some("("))?;
                    return Ok(Expr::Appel(Nom::nouveau(&id.valeur), self.arguments_entre(")")?, id.ligne));
                }
                "ajouter" => {
                    let mut a = self.appel_mot("ajouter", 2, 2)?;
                    let e = a.pop().unwrap();
                    let l = a.pop().unwrap();
                    return Ok(Expr::Ajouter(Box::new(l), Box::new(e), ligne));
                }
                "modifierElement" => {
                    let mut a = self.appel_mot("modifierElement", 3, 3)?;
                    let v = a.pop().unwrap();
                    let i = a.pop().unwrap();
                    let l = a.pop().unwrap();
                    return Ok(Expr::ModifElem(Box::new(l), Box::new(i), Box::new(v), ligne));
                }
                "modifierCle" => {
                    let mut a = self.appel_mot("modifierCle", 3, 3)?;
                    let v = a.pop().unwrap();
                    let c = a.pop().unwrap();
                    let s = a.pop().unwrap();
                    return Ok(Expr::ModifCle(Box::new(s), Box::new(c), Box::new(v), ligne));
                }
                "saisir" => {
                    let mut a = self.appel_mot("saisir", 0, 1)?;
                    return Ok(Expr::Saisir(if a.is_empty() { None } else { Some(Box::new(a.remove(0))) }));
                }
                "lireFichier" => {
                    let mut a = self.appel_mot("lireFichier", 1, 1)?;
                    return Ok(Expr::LireFichier(Box::new(a.remove(0))));
                }
                _ => {}
            }
        }
        err(format!("Valeur ou expression attendue, mais {} trouvé.", decrire(Some(&j))), ligne)
    }
}

use crate::valeur::Valeur;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Nom interné : identifiant entier pour les accès rapides + texte pour les messages.
#[derive(Clone)]
pub struct Nom {
    pub id: u32,
    pub s: Rc<str>,
}

thread_local! {
    static SYMBOLES: RefCell<HashMap<String, Nom>> = RefCell::new(HashMap::new());
}

impl Nom {
    pub fn nouveau(texte: &str) -> Nom {
        SYMBOLES.with(|t| {
            let mut t = t.borrow_mut();
            if let Some(n) = t.get(texte) {
                return n.clone();
            }
            let n = Nom { id: t.len() as u32, s: Rc::from(texte) };
            t.insert(texte.to_string(), n.clone());
            n
        })
    }
}

/// Nom interné correspondant à un identifiant (pour exporter les variables avec leur nom).
pub fn nom_depuis_id(id: u32) -> Option<Rc<str>> {
    SYMBOLES.with(|t| t.borrow().values().find(|n| n.id == id).map(|n| n.s.clone()))
}

impl std::fmt::Display for Nom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.s)
    }
}

impl std::ops::Deref for Nom {
    type Target = str;
    fn deref(&self) -> &str {
        &self.s
    }
}

/// Opérateur binaire (résolu une fois pour toutes à l'analyse : pas de comparaison de texte à l'exécution).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Egal,
    Diff,
    Inf,
    Sup,
    InfEg,
    SupEg,
    Plus,
    Moins,
    Fois,
    Div,
    Mod,
    Puiss,
}

impl Op {
    pub fn depuis(s: &str) -> Op {
        match s {
            "==" => Op::Egal,
            "!=" => Op::Diff,
            "<" => Op::Inf,
            ">" => Op::Sup,
            "<=" => Op::InfEg,
            ">=" => Op::SupEg,
            "+" => Op::Plus,
            "-" => Op::Moins,
            "*" => Op::Fois,
            "/" => Op::Div,
            "**" => Op::Puiss,
            _ => Op::Mod,
        }
    }
    pub fn symbole(self) -> &'static str {
        match self {
            Op::Egal => "==",
            Op::Diff => "!=",
            Op::Inf => "<",
            Op::Sup => ">",
            Op::InfEg => "<=",
            Op::SupEg => ">=",
            Op::Plus => "+",
            Op::Moins => "-",
            Op::Fois => "*",
            Op::Div => "/",
            Op::Mod => "%",
            Op::Puiss => "**",
        }
    }
}

pub struct Stmt {
    pub ligne: usize,
    pub k: StmtK,
}

/// Corps d'une fonction : instructions + code compilé (créé à la première exécution, puis réutilisé).
pub struct Corps {
    pub stmts: Vec<Stmt>,
    pub code: std::cell::OnceCell<crate::vm::Code>,
    /// Module d'où vient ce code (None = script de l'utilisateur) : sert à situer les erreurs.
    pub module: Option<Rc<str>>,
    /// Code généré par l'analyseur (corps d'un constructeur de classe) : le débogueur ne s'y arrête pas.
    pub synth: bool,
}

impl Corps {
    pub fn nouveau(stmts: Vec<Stmt>) -> Corps {
        Corps { stmts, code: std::cell::OnceCell::new(), module: None, synth: false }
    }
    pub fn dans_module(stmts: Vec<Stmt>, module: Option<Rc<str>>) -> Corps {
        Corps { stmts, code: std::cell::OnceCell::new(), module, synth: false }
    }
}

pub struct FnDef {
    pub nom: String,
    pub ident: Nom,
    pub parametres: Rc<Vec<Nom>>,
    pub corps: Rc<Corps>,
}

pub struct Cas {
    pub valeurs: Vec<Expr>,
    pub corps: Vec<Stmt>,
}

pub enum StmtK {
    DeclVar(Nom, Expr),
    Affect(Nom, Expr),
    Afficher(Vec<Expr>),
    Expr(Expr),
    Renvoyer(Option<Expr>),
    Si(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    TantQue(Expr, Vec<Stmt>),
    Pour { nom: Nom, debut: Expr, fin: Expr, pas: Option<Expr>, corps: Vec<Stmt> },
    PourChaque { nom: Nom, iterable: Expr, corps: Vec<Stmt> },
    Rompre,
    Continuer,
    Selon { expr: Expr, cas: Vec<Cas>, defaut: Option<Vec<Stmt>> },
    Importer(Expr, Option<String>),
    Essayer { corps: Vec<Stmt>, nom: Nom, gestion: Vec<Stmt> },
    EcrireFichier(Expr, Expr),
    DeclFonction(Rc<FnDef>),
}

pub enum Expr {
    Lit(Valeur),
    Var(Nom, usize),
    Liste(Vec<Expr>),
    Struct(Vec<(String, Expr)>),
    Lambda { parametres: Rc<Vec<Nom>>, corps: Rc<Corps> },
    Taille(Box<Expr>, usize),
    LireElement(Box<Expr>, Box<Expr>, usize),
    LireCle(Box<Expr>, Box<Expr>, usize),
    Hasard(Box<Expr>, Box<Expr>, usize),
    Saisir(Option<Box<Expr>>),
    LireFichier(Box<Expr>),
    Logique(bool /* true = et */, Box<Expr>, Box<Expr>),
    Non(Box<Expr>),
    Neg(Box<Expr>, usize),
    Bin(Op, Box<Expr>, Box<Expr>, usize),
    Appel(Nom, Vec<Expr>, usize),
    Ajouter(Box<Expr>, Box<Expr>, usize),
    ModifElem(Box<Expr>, Box<Expr>, Box<Expr>, usize),
    ModifCle(Box<Expr>, Box<Expr>, Box<Expr>, usize),
    /// `expr.champ` (enchaînement après un appel).
    Champ(Box<Expr>, String, usize),
    /// `expr.methode(args)` (enchaînement après un appel).
    Methode(Box<Expr>, String, Vec<Expr>, usize),
}

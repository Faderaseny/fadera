//! Compilateur vers bytecode + machine virtuelle à pile.
//!
//! Principe : l'AST est « aplati » une fois en une suite d'instructions (sauts, pile de valeurs) au lieu
//! d'être parcouru récursivement. La sémantique est celle de l'interpréteur à arbre (`interp.rs`), qui sert
//! de référence : même ordre d'évaluation, mêmes messages d'erreur, mêmes numéros de ligne, même comptage
//! des « étapes » (limite de boucle infinie).
//!
//! Les constructions rares (selon, essayer/attraper, importer, ecrireFichier, lambda, structures littérales,
//! lireCle, hasard, saisir, lireFichier, modifierElement/Cle) ne sont pas compilées : elles s'exécutent via
//! l'interpréteur à arbre (instructions `ExecArbre` / `EvalArbre`) — même portées, mêmes variables.
//!
//! Les pointeurs bruts désignent des nœuds de l'AST. L'AST n'est jamais modifié ni déplacé après l'analyse, et
//! le `Code` est rangé dans le même `Corps` que l'AST qu'il référence (ou vit moins longtemps que lui).

use std::rc::Rc;

use crate::ast::*;
use crate::erreur::*;
use crate::interp::{ajouter_formate, operer, Interp};
use crate::valeur::*;

const AUCUN: usize = usize::MAX;

pub enum Instr {
    // ----- expressions (empilent une valeur)
    Const(Valeur),
    Var(Nom, usize),
    Bin(Op, usize),
    /// variable ⊕ constante / variable ⊕ variable / (pile) ⊕ constante : sans passer par la pile
    BinVC { op: Op, n: Nom, lv: usize, c: Valeur, l: usize },
    BinVV { op: Op, a: Nom, la: usize, b: Nom, lb: usize, l: usize },
    BinC { op: Op, c: Valeur, l: usize },
    /// comparaison fusionnée avec le saut conditionnel (si faux, saute à `saut`)
    CmpVC { op: Op, n: Nom, lv: usize, c: Valeur, l: usize, saut: usize },
    CmpVV { op: Op, a: Nom, la: usize, b: Nom, lb: usize, l: usize, saut: usize },
    Non,
    Neg(usize),
    /// et : si le sommet est faux, on le garde et on saute ; sinon on le retire
    Et(usize),
    /// ou : si le sommet est vrai, on le garde et on saute ; sinon on le retire
    Ou(usize),
    Appel(Nom, usize, usize),
    Taille(usize),
    LireElem(usize),
    Liste(usize),
    /// vérifie que le sommet est une liste (avant d'évaluer la valeur à ajouter)
    AjVerif(usize),
    Aj,
    EvalArbre(*const Expr),
    /// chaîne « a + x1 + x2 + … » où chaque xi est une constante ou une variable : la valeur de `a` est sur la pile.
    /// Même résultat et mêmes erreurs que la suite de `Bin(Plus)`, mais le texte est construit en une seule fois.
    Concat(Vec<OperandeConcat>),
    /// instruction « nom = nom + x1 + x2 … » (xi constante ou variable) : si `nom` contient un texte, on ajoute en place
    /// (sans recopier le texte) et on saute à `fin` ; sinon on continue vers le code générique qui suit.
    AjoutTexte { nom: Nom, ops: Vec<OperandeConcat>, fin: usize },
    // ----- instructions
    Stmt { ligne: usize, encl: Option<usize>, synth: bool },
    Pop,
    Def(Nom),
    Set(Nom, usize),
    Print(usize),
    Jump(usize),
    SiNon(usize),
    Ret,
    Lever(bool),
    LoopTic(usize),
    /// LoopTic + saut en arrière (tantque)
    TicSaut(usize, usize),
    /// fin d'itération de « pour » : tic, pas, test ; si on continue : variable = i et saut au début du corps
    PourSuite { l: usize, nom: Nom, corps: usize },
    PourInit,
    PourTest(Nom, usize),
    PourStep,
    ChaqueInit,
    ChaqueNext(Nom, usize),
    LoopPop,
    ExecArbre { stmt: *const Stmt, encl: Option<usize>, brk: usize, cont: usize },
}

pub enum OperandeConcat {
    Const(Valeur, usize),
    Var(Nom, usize, usize),
}

pub struct Code {
    ops: Vec<Instr>,
}

// ------------------------------------------------------------------ compilation
struct Boucle {
    sorties: Vec<usize>,  // sauts à pointer vers la sortie
    suites: Vec<usize>,   // sauts à pointer vers LoopTic
    arbres: Vec<usize>,   // ExecArbre à compléter (brk, cont)
}

struct Compilateur {
    ops: Vec<Instr>,
    boucles: Vec<Boucle>,
}

pub fn compiler(stmts: &[Stmt]) -> Code {
    let mut c = Compilateur { ops: Vec::new(), boucles: Vec::new() };
    c.bloc(stmts, None);
    Code { ops: c.ops }
}

impl Compilateur {
    fn emettre(&mut self, i: Instr) -> usize {
        self.ops.push(i);
        self.ops.len() - 1
    }
    fn ici(&self) -> usize {
        self.ops.len()
    }
    fn cibler(&mut self, idx: usize, cible: usize) {
        match &mut self.ops[idx] {
            Instr::Jump(t) | Instr::SiNon(t) | Instr::Et(t) | Instr::Ou(t) | Instr::PourTest(_, t) | Instr::ChaqueNext(_, t) => *t = cible,
            Instr::CmpVC { saut, .. } | Instr::CmpVV { saut, .. } => *saut = cible,
            Instr::AjoutTexte { fin, .. } => *fin = cible,
            _ => unreachable!(),
        }
    }

    /// Reconnaît « base + x1 + x2 + … » (au moins deux `+` ; chaque xi est une constante ou une variable).
    /// Renvoie la base (expression la plus à gauche) et les opérandes dans l'ordre d'évaluation.
    fn chaine_plus<'a>(&self, e: &'a Expr) -> Option<(&'a Expr, Vec<OperandeConcat>)> {
        let mut ops = Vec::new();
        let mut cur = e;
        while let Expr::Bin(Op::Plus, g, d, l) = cur {
            match &**d {
                Expr::Lit(v) => ops.push(OperandeConcat::Const(v.clone(), *l)),
                Expr::Var(n, ln) => ops.push(OperandeConcat::Var(n.clone(), *ln, *l)),
                _ => return None,
            }
            cur = g;
        }
        if ops.len() < 2 {
            return None;
        }
        ops.reverse();
        Some((cur, ops))
    }

    /// Compile une condition suivie d'un saut « si faux » ; renvoie l'indice de l'instruction de saut à cibler.
    fn condition(&mut self, c: &Expr) -> usize {
        if let Expr::Bin(op, g, d, l) = c {
            if matches!(op, Op::Egal | Op::Diff | Op::Inf | Op::Sup | Op::InfEg | Op::SupEg) {
                match (&**g, &**d) {
                    (Expr::Var(a, la), Expr::Lit(k)) => {
                        return self.emettre(Instr::CmpVC { op: *op, n: a.clone(), lv: *la, c: k.clone(), l: *l, saut: 0 });
                    }
                    (Expr::Var(a, la), Expr::Var(b, lb)) => {
                        return self.emettre(Instr::CmpVV { op: *op, a: a.clone(), la: *la, b: b.clone(), lb: *lb, l: *l, saut: 0 });
                    }
                    _ => {}
                }
            }
        }
        self.expr(c);
        self.emettre(Instr::SiNon(0))
    }

    fn bloc(&mut self, stmts: &[Stmt], encl: Option<usize>) {
        for s in stmts {
            self.instruction(s, encl);
        }
    }

    fn arbre(&mut self, s: &Stmt, encl: Option<usize>) {
        let i = self.emettre(Instr::ExecArbre { stmt: s as *const Stmt, encl, brk: AUCUN, cont: AUCUN });
        if let Some(b) = self.boucles.last_mut() {
            b.arbres.push(i);
        }
    }

    fn instruction(&mut self, s: &Stmt, encl: Option<usize>) {
        let l = s.ligne;
        match &s.k {
            StmtK::DeclVar(nom, e) => {
                // `C.#parent = "P"` est produit par l'analyseur pour « classe C herite P » : le débogueur ne s'y arrête pas
                self.emettre(Instr::Stmt { ligne: l, encl, synth: nom.s.ends_with(".#parent") });
                self.expr(e);
                self.emettre(Instr::Def(nom.clone()));
            }
            StmtK::Affect(nom, e) => {
                self.emettre(Instr::Stmt { ligne: l, encl, synth: false });
                // « s = s + a + b … » : ajout en place si s est un texte (sinon le code générique qui suit s'exécute)
                let ajout = match self.chaine_plus(e) {
                    Some((Expr::Var(b, _), ops)) if b.id == nom.id => {
                        Some(self.emettre(Instr::AjoutTexte { nom: nom.clone(), ops, fin: 0 }))
                    }
                    _ => None,
                };
                self.expr(e);
                self.emettre(Instr::Set(nom.clone(), l));
                if let Some(a) = ajout {
                    let ici = self.ici();
                    self.cibler(a, ici);
                }
            }
            StmtK::Afficher(es) => {
                self.emettre(Instr::Stmt { ligne: l, encl, synth: false });
                for e in es {
                    self.expr(e);
                }
                self.emettre(Instr::Print(es.len()));
            }
            StmtK::Expr(e) => {
                self.emettre(Instr::Stmt { ligne: l, encl, synth: false });
                self.expr(e);
                self.emettre(Instr::Pop);
            }
            StmtK::Renvoyer(e) => {
                self.emettre(Instr::Stmt { ligne: l, encl, synth: false });
                match e {
                    Some(e) => self.expr(e),
                    None => {
                        self.emettre(Instr::Const(Valeur::Rien));
                    }
                }
                self.emettre(Instr::Ret);
            }
            StmtK::Si(c, alors, sinon) => {
                self.emettre(Instr::Stmt { ligne: l, encl, synth: false });
                let saut = self.condition(c);
                self.bloc(alors, Some(l));
                match sinon {
                    Some(b) => {
                        let fin = self.emettre(Instr::Jump(0));
                        let ici = self.ici();
                        self.cibler(saut, ici);
                        self.bloc(b, Some(l));
                        let ici = self.ici();
                        self.cibler(fin, ici);
                    }
                    None => {
                        let ici = self.ici();
                        self.cibler(saut, ici);
                    }
                }
            }
            StmtK::TantQue(c, corps) => {
                self.emettre(Instr::Stmt { ligne: l, encl, synth: false });
                let tete = self.ici();
                let sortie = self.condition(c);
                self.boucles.push(Boucle { sorties: vec![], suites: vec![], arbres: vec![] });
                self.bloc(corps, Some(l));
                let suite = self.ici();
                self.emettre(Instr::TicSaut(l, tete));
                let fin = self.ici();
                self.cibler(sortie, fin);
                self.fermer_boucle(suite, fin, false);
            }
            StmtK::Pour { nom, debut, fin, pas, corps } => {
                self.emettre(Instr::Stmt { ligne: l, encl, synth: false });
                self.expr(debut);
                self.expr(fin);
                match pas {
                    Some(p) => self.expr(p),
                    None => {
                        self.emettre(Instr::Const(Valeur::Nombre(1.0)));
                    }
                }
                self.emettre(Instr::PourInit);
                let test = self.emettre(Instr::PourTest(nom.clone(), 0));
                let debut_corps = self.ici();
                self.boucles.push(Boucle { sorties: vec![], suites: vec![], arbres: vec![] });
                self.bloc(corps, Some(l));
                let suite = self.ici();
                self.emettre(Instr::PourSuite { l, nom: nom.clone(), corps: debut_corps });
                let fin_ = self.ici();
                self.emettre(Instr::LoopPop);
                self.cibler(test, fin_);
                self.fermer_boucle(suite, fin_, true);
            }
            StmtK::PourChaque { nom, iterable, corps } => {
                self.emettre(Instr::Stmt { ligne: l, encl, synth: false });
                self.expr(iterable);
                self.emettre(Instr::ChaqueInit);
                let test = self.emettre(Instr::ChaqueNext(nom.clone(), 0));
                self.boucles.push(Boucle { sorties: vec![], suites: vec![], arbres: vec![] });
                self.bloc(corps, Some(l));
                let suite = self.ici();
                self.emettre(Instr::LoopTic(l));
                self.emettre(Instr::Jump(test));
                let fin_ = self.ici();
                self.emettre(Instr::LoopPop);
                self.cibler(test, fin_);
                self.fermer_boucle(suite, fin_, true);
            }
            StmtK::Rompre | StmtK::Continuer => {
                let rompre = matches!(s.k, StmtK::Rompre);
                self.emettre(Instr::Stmt { ligne: l, encl, synth: false });
                if self.boucles.is_empty() {
                    self.emettre(Instr::Lever(rompre));
                } else {
                    let j = self.emettre(Instr::Jump(0));
                    let b = self.boucles.last_mut().unwrap();
                    if rompre {
                        b.sorties.push(j);
                    } else {
                        b.suites.push(j);
                    }
                }
            }
            // rares : exécutées par l'interpréteur à arbre (le « tic » et le numéro de ligne y sont gérés)
            StmtK::DeclFonction(_) | StmtK::Selon { .. } | StmtK::Importer(_, _) | StmtK::Essayer { .. } | StmtK::EcrireFichier(_, _) => {
                self.arbre(s, encl);
            }
        }
    }

    /// `sortie` : adresse du saut de sortie (LoopPop pour pour/pour chaque, première instruction suivante pour tantque).
    fn fermer_boucle(&mut self, suite: usize, sortie: usize, _avec_pop: bool) {
        let b = self.boucles.pop().unwrap();
        for j in b.sorties {
            self.cibler(j, sortie);
        }
        for j in b.suites {
            self.cibler(j, suite);
        }
        for i in b.arbres {
            if let Instr::ExecArbre { brk, cont, .. } = &mut self.ops[i] {
                *brk = sortie;
                *cont = suite;
            }
        }
    }

    fn expr(&mut self, e: &Expr) {
        match e {
            Expr::Lit(v) => {
                self.emettre(Instr::Const(v.clone()));
            }
            Expr::Var(n, l) => {
                self.emettre(Instr::Var(n.clone(), *l));
            }
            Expr::Bin(Op::Plus, _, _, _) if self.chaine_plus(e).is_some() => {
                let (base, ops) = self.chaine_plus(e).unwrap();
                self.expr(base);
                self.emettre(Instr::Concat(ops));
            }
            Expr::Bin(op, g, d, l) => match (&**g, &**d) {
                (Expr::Var(a, la), Expr::Lit(k)) => {
                    self.emettre(Instr::BinVC { op: *op, n: a.clone(), lv: *la, c: k.clone(), l: *l });
                }
                (Expr::Var(a, la), Expr::Var(b, lb)) => {
                    self.emettre(Instr::BinVV { op: *op, a: a.clone(), la: *la, b: b.clone(), lb: *lb, l: *l });
                }
                (_, Expr::Lit(k)) => {
                    self.expr(g);
                    self.emettre(Instr::BinC { op: *op, c: k.clone(), l: *l });
                }
                _ => {
                    self.expr(g);
                    self.expr(d);
                    self.emettre(Instr::Bin(*op, *l));
                }
            },
            Expr::Logique(et, g, d) => {
                self.expr(g);
                let saut = self.emettre(if *et { Instr::Et(0) } else { Instr::Ou(0) });
                self.expr(d);
                let ici = self.ici();
                self.cibler(saut, ici);
            }
            Expr::Non(x) => {
                self.expr(x);
                self.emettre(Instr::Non);
            }
            Expr::Neg(x, l) => {
                self.expr(x);
                self.emettre(Instr::Neg(*l));
            }
            Expr::Appel(n, args, l) => {
                for a in args {
                    self.expr(a);
                }
                self.emettre(Instr::Appel(n.clone(), args.len(), *l));
            }
            Expr::Taille(c, l) => {
                self.expr(c);
                self.emettre(Instr::Taille(*l));
            }
            Expr::LireElement(t, i, l) => {
                self.expr(t);
                self.expr(i);
                self.emettre(Instr::LireElem(*l));
            }
            Expr::Liste(es) => {
                for x in es {
                    self.expr(x);
                }
                self.emettre(Instr::Liste(es.len()));
            }
            Expr::Ajouter(t, x, l) => {
                self.expr(t);
                self.emettre(Instr::AjVerif(*l));
                self.expr(x);
                self.emettre(Instr::Aj);
            }
            _ => {
                self.emettre(Instr::EvalArbre(e as *const Expr));
            }
        }
    }
}

// ------------------------------------------------------------------ exécution
enum EtatBoucle {
    Pour { i: f64, fin: f64, pas: f64 },
    Chaque { elems: Vec<Valeur>, k: usize },
}

/// Ajoute le numéro de ligne de l'instruction en cours à une erreur qui n'en a pas (comme `Interp::executer`).
fn avec_ligne(e: Flow, ligne: Option<usize>) -> Flow {
    match e {
        Flow::Err(mut x) => {
            if x.ligne.is_none() {
                x.ligne = ligne;
            }
            if x.cadre.is_none() {
                x.cadre = ligne;
            }
            Flow::Err(x)
        }
        Flow::Nat(m) => match ligne {
            Some(l) => fad_l(m, l),
            None => Flow::Nat(m),
        },
        autre => autre,
    }
}

/// Exécute le corps d'une fonction (compilé à la première utilisation). Renvoie la valeur renvoyée (Rien sinon).
pub fn executer_corps(it: &mut Interp, corps: &Rc<Corps>) -> R<Valeur> {
    if it.arbre {
        return match it.bloc(&corps.stmts) {
            Ok(()) => Ok(Valeur::Rien),
            Err(Flow::Retour(v)) => Ok(v),
            Err(e) => Err(e),
        };
    }
    let code = corps.code.get_or_init(|| compiler(&corps.stmts));
    Ok(executer_code(it, code)?.unwrap_or(Valeur::Rien))
}

/// Exécute le programme principal. Un `renvoyer` au niveau principal arrête le programme (Err(Retour)).
pub fn executer_programme(it: &mut Interp, stmts: &[Stmt]) -> R<()> {
    if it.arbre {
        for i in stmts {
            it.executer(i)?;
        }
        return Ok(());
    }
    let code = compiler(stmts);
    match executer_code(it, &code)? {
        Some(v) => Err(Flow::Retour(v)),
        None => Ok(()),
    }
}

fn executer_code(it: &mut Interp, code: &Code) -> R<Option<Valeur>> {
    let base = it.vs.len();
    let r = boucle(it, code);
    it.vs.truncate(base);
    r
}

/// Opération binaire avec chemin rapide inline pour deux nombres.
#[inline(always)]
fn op_rapide(op: Op, g: &Valeur, d: &Valeur, l: usize) -> R<Valeur> {
    if let (Valeur::Nombre(a), Valeur::Nombre(b)) = (g, d) {
        let (a, b) = (*a, *b);
        match op {
            Op::Plus => return Ok(Valeur::Nombre(a + b)),
            Op::Moins => return Ok(Valeur::Nombre(a - b)),
            Op::Fois => return Ok(Valeur::Nombre(a * b)),
            Op::Mod if b != 0.0 => return Ok(Valeur::Nombre(crate::interp::modulo(a, b))),
            Op::Div if b != 0.0 => return Ok(Valeur::Nombre(a / b)),
            Op::Inf => return Ok(Valeur::Bool(a < b)),
            Op::Sup => return Ok(Valeur::Bool(a > b)),
            Op::InfEg => return Ok(Valeur::Bool(a <= b)),
            Op::SupEg => return Ok(Valeur::Bool(a >= b)),
            Op::Egal => return Ok(Valeur::Bool(a == b)),
            Op::Diff => return Ok(Valeur::Bool(a != b)),
            _ => {}
        }
    }
    operer(op, g, d, l)
}

#[inline(always)]
fn cmp_rapide(op: Op, g: &Valeur, d: &Valeur, l: usize) -> R<bool> {
    if let (Valeur::Nombre(a), Valeur::Nombre(b)) = (g, d) {
        let (a, b) = (*a, *b);
        return Ok(match op {
            Op::Inf => a < b,
            Op::Sup => a > b,
            Op::InfEg => a <= b,
            Op::SupEg => a >= b,
            Op::Egal => a == b,
            _ => a != b,
        });
    }
    Ok(operer(op, g, d, l)?.vrai())
}

macro_rules! tri {
    ($it:expr, $ligne:expr, $e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(avec_ligne(e, $ligne)),
        }
    };
}

/// État de la machine entre deux « tranches » d'exécution.
struct EtatVm {
    pc: usize,
    ligne: Option<usize>,
    boucles: Vec<EtatBoucle>,
}

enum Pas {
    Fin(Option<Valeur>),
    Suite,
}

/// Nombre de tours de boucle exécutés avant de rendre la main à `boucle`.
/// Pourquoi : en WebAssembly (V8), une fonction qui ne rend jamais la main reste compilée par le compilateur
/// de base (Liftoff, 2 à 3 fois plus lent) : il n'y a pas de remplacement « en cours d'exécution ». En
/// découpant l'exécution en tranches, chaque appel suivant utilise le code optimisé (TurboFan).
/// Aucun effet sur la sémantique ; coût négligeable en natif.
const TRANCHE: u32 = 20_000;

fn boucle(it: &mut Interp, code: &Code) -> R<Option<Valeur>> {
    let mut st = EtatVm { pc: 0, ligne: None, boucles: Vec::new() };
    loop {
        match tranche(it, code, &mut st)? {
            Pas::Fin(v) => return Ok(v),
            Pas::Suite => {}
        }
    }
}

#[inline(never)]
fn tranche(it: &mut Interp, code: &Code, st: &mut EtatVm) -> R<Pas> {
    let ops = &code.ops[..];
    let mut pc = st.pc;
    let mut ligne: Option<usize> = st.ligne;
    let mut boucles: Vec<EtatBoucle> = std::mem::take(&mut st.boucles);
    let mut budget = TRANCHE;
    macro_rules! ceder {
        () => {
            budget -= 1;
            if budget == 0 {
                st.pc = pc;
                st.ligne = ligne;
                st.boucles = std::mem::take(&mut boucles);
                return Ok(Pas::Suite);
            }
        };
    }
    loop {
        let instr = match ops.get(pc) {
            Some(i) => i,
            None => return Ok(Pas::Fin(None)),
        };
        pc += 1;
        match instr {
            Instr::Const(v) => it.vs.push(v.clone()),
            Instr::Var(n, l) => {
                let v = match it.chercher(n) {
                    Some(v) => v,
                    None => tri!(it, ligne, it.lire(n, *l)),
                };
                it.vs.push(v);
            }
            Instr::Concat(ops) => {
                let g = it.vs.pop().unwrap();
                if let Valeur::Texte(base) = &g {
                    // base texte : chaque « + » ajoute la représentation textuelle (aucune erreur possible) ; on construit en une fois
                    let mut r = String::with_capacity(base.len() + 8 * ops.len());
                    r.push_str(base);
                    for op in ops.iter() {
                        match op {
                            OperandeConcat::Const(v, _) => ajouter_formate(&mut r, v),
                            OperandeConcat::Var(n, ln, _) => match it.var_ref(n) {
                                Some(v) => ajouter_formate(&mut r, v),
                                None => {
                                    let v = tri!(it, ligne, it.lire(n, *ln));
                                    ajouter_formate(&mut r, &v);
                                }
                            },
                        }
                    }
                    it.vs.push(Valeur::texte_string(r));
                } else {
                    // base non texte (nombre, liste…) : même suite d'opérations et mêmes erreurs que les `Bin(Plus)`
                    let mut acc = g;
                    for op in ops.iter() {
                        let (d, l) = match op {
                            OperandeConcat::Const(v, l) => (v.clone(), *l),
                            OperandeConcat::Var(n, ln, l) => (tri!(it, ligne, it.lire(n, *ln)), *l),
                        };
                        acc = tri!(it, ligne, operer(Op::Plus, &acc, &d, l));
                    }
                    it.vs.push(acc);
                }
            }
            Instr::AjoutTexte { nom, ops, fin } => {
                if matches!(it.var_ref(nom), Some(Valeur::Texte(_))) {
                    let mut extra = String::new();
                    for op in ops.iter() {
                        match op {
                            OperandeConcat::Const(v, _) => ajouter_formate(&mut extra, v),
                            OperandeConcat::Var(n, ln, _) => match it.var_ref(n) {
                                Some(v) => ajouter_formate(&mut extra, v),
                                None => {
                                    let v = tri!(it, ligne, it.lire(n, *ln));
                                    ajouter_formate(&mut extra, &v);
                                }
                            },
                        }
                    }
                    if it.ajouter_texte(nom, &extra) {
                        pc = *fin;
                    }
                }
            }
            Instr::Bin(op, l) => {
                let d = it.vs.pop().unwrap();
                let g = it.vs.pop().unwrap();
                let r = tri!(it, ligne, operer(*op, &g, &d, *l));
                it.vs.push(r);
            }
            Instr::BinVC { op, n, lv, c, l } => {
                let r = match it.var_ref(n) {
                    Some(g) => op_rapide(*op, g, c, *l),
                    None => {
                        let g = tri!(it, ligne, it.lire(n, *lv));
                        op_rapide(*op, &g, c, *l)
                    }
                };
                let r = tri!(it, ligne, r);
                it.vs.push(r);
            }
            Instr::BinVV { op, a, la, b, lb, l } => {
                let r = match (it.var_ref(a), it.var_ref(b)) {
                    (Some(g), Some(d)) => op_rapide(*op, g, d, *l),
                    _ => {
                        let g = tri!(it, ligne, it.lire(a, *la));
                        let d = tri!(it, ligne, it.lire(b, *lb));
                        op_rapide(*op, &g, &d, *l)
                    }
                };
                let r = tri!(it, ligne, r);
                it.vs.push(r);
            }
            Instr::BinC { op, c, l } => {
                let g = it.vs.pop().unwrap();
                let r = tri!(it, ligne, op_rapide(*op, &g, c, *l));
                it.vs.push(r);
            }
            Instr::CmpVC { op, n, lv, c, l, saut } => {
                let t = match it.var_ref(n) {
                    Some(g) => cmp_rapide(*op, g, c, *l),
                    None => {
                        let g = tri!(it, ligne, it.lire(n, *lv));
                        cmp_rapide(*op, &g, c, *l)
                    }
                };
                if !tri!(it, ligne, t) {
                    pc = *saut;
                }
            }
            Instr::CmpVV { op, a, la, b, lb, l, saut } => {
                let t = match (it.var_ref(a), it.var_ref(b)) {
                    (Some(g), Some(d)) => cmp_rapide(*op, g, d, *l),
                    _ => {
                        let g = tri!(it, ligne, it.lire(a, *la));
                        let d = tri!(it, ligne, it.lire(b, *lb));
                        cmp_rapide(*op, &g, &d, *l)
                    }
                };
                if !tri!(it, ligne, t) {
                    pc = *saut;
                }
            }
            Instr::Non => {
                let v = it.vs.pop().unwrap();
                it.vs.push(Valeur::Bool(!v.vrai()));
            }
            Instr::Neg(l) => {
                let v = it.vs.pop().unwrap();
                match v {
                    Valeur::Nombre(n) => it.vs.push(Valeur::Nombre(-n)),
                    v => {
                        return Err(avec_ligne(
                            fad_l(format!("Le signe \"-\" attend un nombre (reçu : {}).", type_de(&v)), *l),
                            ligne,
                        ))
                    }
                }
            }
            Instr::Et(t) => {
                if it.vs.last().unwrap().vrai() {
                    it.vs.pop();
                } else {
                    pc = *t;
                }
            }
            Instr::Ou(t) => {
                if it.vs.last().unwrap().vrai() {
                    pc = *t;
                } else {
                    it.vs.pop();
                }
            }
            Instr::Appel(nom, n, l) => {
                let r = tri!(it, ligne, it.appeler_pile(nom, *n, *l));
                it.vs.push(r);
            }
            Instr::Taille(l) => {
                let c = it.vs.pop().unwrap();
                let n = match &c {
                    Valeur::Liste(x) => x.borrow().len(),
                    Valeur::Texte(t) => t.chars().count(),
                    Valeur::Struct(d) => d.borrow().len(),
                    _ => {
                        return Err(avec_ligne(
                            fad_l(format!("taille attend une liste, un texte ou une structure (reçu : {}).", type_de(&c)), *l),
                            ligne,
                        ))
                    }
                };
                it.vs.push(Valeur::Nombre(n as f64));
            }
            Instr::LireElem(l) => {
                let i = it.vs.pop().unwrap();
                let t = it.vs.pop().unwrap();
                let v = match &t {
                    Valeur::Liste(x) => {
                        let n = x.borrow().len();
                        let k = tri!(it, ligne, it.verif_index(n, &i, Some(*l)));
                        x.borrow()[k].clone()
                    }
                    Valeur::Texte(s) => {
                        let n = s.chars().count();
                        let k = tri!(it, ligne, it.verif_index(n, &i, Some(*l)));
                        Valeur::texte(s.chars().nth(k).unwrap().to_string())
                    }
                    _ => {
                        return Err(avec_ligne(
                            fad_l(format!("lireElement attend une liste ou un texte (reçu : {}).", type_de(&t)), *l),
                            ligne,
                        ))
                    }
                };
                it.vs.push(v);
            }
            Instr::Liste(n) => {
                let debut = it.vs.len() - *n;
                let v: Vec<Valeur> = it.vs.drain(debut..).collect();
                it.vs.push(Valeur::liste(v));
            }
            Instr::AjVerif(l) => {
                let ok = matches!(it.vs.last().unwrap(), Valeur::Liste(_));
                if !ok {
                    let t = it.vs.last().unwrap();
                    return Err(avec_ligne(
                        fad_l(format!("ajouter attend une liste en premier argument (reçu : {}).", type_de(t)), *l),
                        ligne,
                    ));
                }
            }
            Instr::Aj => {
                let x = it.vs.pop().unwrap();
                if let Some(Valeur::Liste(li)) = it.vs.last() {
                    li.borrow_mut().push(x);
                }
                // la liste (sommet de pile) est la valeur de l'expression
            }
            Instr::EvalArbre(e) => {
                let e: &Expr = unsafe { &**e };
                let v = tri!(it, ligne, it.evaluer(e));
                it.vs.push(v);
            }
            Instr::Stmt { ligne: l, encl, synth } => {
                // une limite dépassée est attribuée à l'instruction englobante (comme dans l'interpréteur à arbre)
                if let Err(e) = it.tic() {
                    return Err(avec_ligne(e, *encl));
                }
                if it.debog.is_some() {
                    if let Err(e) = it.sonde(*l, *synth) {
                        return Err(avec_ligne(e, Some(*l)));
                    }
                }
                ligne = Some(*l);
            }
            Instr::Pop => {
                it.vs.pop();
            }
            Instr::Def(n) => {
                let v = it.vs.pop().unwrap();
                it.definir(n, v);
            }
            Instr::Set(n, l) => {
                let v = it.vs.pop().unwrap();
                tri!(it, ligne, it.affecter_ou_champ(n, v, *l));
            }
            Instr::Print(n) => {
                let debut = it.vs.len() - *n;
                let mut parts: Vec<String> = Vec::with_capacity(*n);
                for v in it.vs.drain(debut..) {
                    parts.push(formater(&v));
                }
                let t = parts.join(" ");
                it.hote.afficher(&t);
            }
            Instr::Jump(t) => pc = *t,
            Instr::SiNon(t) => {
                let v = it.vs.pop().unwrap();
                if !v.vrai() {
                    pc = *t;
                }
            }
            Instr::Ret => {
                let v = it.vs.pop().unwrap();
                return Ok(Pas::Fin(Some(v)));
            }
            Instr::Lever(rompre) => {
                return Err(if *rompre { Flow::Rompre } else { Flow::Continuer });
            }
            Instr::LoopTic(l) => {
                ligne = Some(*l);
                tri!(it, ligne, it.tic());
                ceder!();
            }
            Instr::TicSaut(l, cible) => {
                ligne = Some(*l);
                tri!(it, ligne, it.tic());
                pc = *cible;
                ceder!();
            }
            Instr::PourSuite { l, nom, corps } => {
                ligne = Some(*l);
                tri!(it, ligne, it.tic());
                if let Some(EtatBoucle::Pour { i, fin, pas }) = boucles.last_mut() {
                    *i += *pas;
                    let continuer = if *pas > 0.0 { *i <= *fin } else { *i >= *fin };
                    if continuer {
                        it.definir(nom, Valeur::Nombre(*i));
                        pc = *corps;
                    }
                }
                ceder!();
            }
            Instr::PourInit => {
                let p = it.vs.pop().unwrap();
                let f = it.vs.pop().unwrap();
                let d = it.vs.pop().unwrap();
                let mut nums = [0f64; 3];
                for (k, (nm, v)) in [("de départ", &d), ("d'arrivée", &f), ("pas", &p)].iter().enumerate() {
                    match v {
                        Valeur::Nombre(n) if !n.is_nan() => nums[k] = *n,
                        _ => {
                            return Err(avec_ligne(
                                fad(format!(
                                    "La valeur {} de la boucle pour doit être un nombre (reçu : {}).",
                                    nm,
                                    type_de(v)
                                )),
                                ligne,
                            ))
                        }
                    }
                }
                if nums[2] == 0.0 {
                    return Err(avec_ligne(fad("Le pas de la boucle \"pour\" ne peut pas être 0."), ligne));
                }
                boucles.push(EtatBoucle::Pour { i: nums[0], fin: nums[1], pas: nums[2] });
            }
            Instr::PourTest(nom, sortie) => {
                if let Some(EtatBoucle::Pour { i, fin, pas }) = boucles.last() {
                    let continuer = if *pas > 0.0 { *i <= *fin } else { *i >= *fin };
                    if continuer {
                        it.definir(nom, Valeur::Nombre(*i));
                    } else {
                        pc = *sortie;
                    }
                }
            }
            Instr::PourStep => {
                if let Some(EtatBoucle::Pour { i, pas, .. }) = boucles.last_mut() {
                    *i += *pas;
                }
            }
            Instr::ChaqueInit => {
                let iter = it.vs.pop().unwrap();
                let elems: Vec<Valeur> = match &iter {
                    Valeur::Liste(l) => l.borrow().clone(),
                    Valeur::Texte(t) => t.chars().map(|c| Valeur::texte(c.to_string())).collect(),
                    Valeur::Struct(d) => {
                        let d = d.borrow();
                        d.cles_ordonnees().into_iter().map(Valeur::texte).collect()
                    }
                    _ => {
                        return Err(avec_ligne(
                            fad(format!(
                                "pour chaque attend une liste, un texte ou une structure (reçu : {}).",
                                type_de(&iter)
                            )),
                            ligne,
                        ))
                    }
                };
                boucles.push(EtatBoucle::Chaque { elems, k: 0 });
            }
            Instr::ChaqueNext(nom, sortie) => {
                if let Some(EtatBoucle::Chaque { elems, k }) = boucles.last_mut() {
                    if *k >= elems.len() {
                        pc = *sortie;
                    } else {
                        let x = elems[*k].clone();
                        *k += 1;
                        it.definir(nom, x);
                    }
                }
            }
            Instr::LoopPop => {
                boucles.pop();
            }
            Instr::ExecArbre { stmt, encl, brk, cont } => {
                let s: &Stmt = unsafe { &**stmt };
                ligne = *encl;
                match it.executer(s) {
                    Ok(()) => {}
                    Err(Flow::Rompre) if *brk != AUCUN => pc = *brk,
                    Err(Flow::Continuer) if *cont != AUCUN => pc = *cont,
                    Err(Flow::Retour(v)) => return Ok(Pas::Fin(Some(v))),
                    Err(e) => return Err(avec_ligne(e, ligne)),
                }
                ligne = Some(s.ligne);
            }
        }
    }
}

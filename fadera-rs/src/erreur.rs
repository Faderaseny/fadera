use crate::valeur::Valeur;

#[derive(Clone, Debug)]
pub struct ErreurFadera {
    pub message: String,
    pub ligne: Option<usize>,
    /// Erreur venue de l'hôte (pas une erreur Fadéra) : affichée avec le préfixe « Erreur interne : ».
    pub interne: bool,
    /// Pile d'appels : (fonction, ligne en cours dans cette fonction), de la plus récente à la plus ancienne.
    pub pile: Vec<(String, usize)>,
    /// Ligne de l'instruction en cours dans la fonction que l'erreur est en train de quitter.
    pub cadre: Option<usize>,
    /// L'origine (script ou module) est décidée par la première fonction ou le premier module que l'erreur quitte.
    pub origine_fixee: bool,
    pub origine_module: bool,
}

impl ErreurFadera {
    pub fn new(m: impl Into<String>, ligne: Option<usize>) -> Self {
        ErreurFadera { message: m.into(), ligne, interne: false, pile: Vec::new(), cadre: ligne, origine_fixee: false, origine_module: false }
    }
}

/// Flux de contrôle de l'interpréteur.
pub enum Flow {
    Err(ErreurFadera),
    /// Erreur simple d'une fonction native : sera préfixée par « nom : ».
    Nat(String),
    Retour(Valeur),
    Rompre,
    Continuer,
}

pub type R<T> = Result<T, Flow>;

/// Erreur Fadéra sans numéro de ligne (la ligne est ajoutée par l'instruction englobante).
pub fn fad(m: impl Into<String>) -> Flow {
    Flow::Err(ErreurFadera::new(m, None))
}
pub fn fad_l(m: impl Into<String>, ligne: usize) -> Flow {
    Flow::Err(ErreurFadera::new(m, Some(ligne)))
}
pub fn nat(m: impl Into<String>) -> Flow {
    Flow::Nat(m.into())
}

pub fn interne(m: impl Into<String>) -> Flow {
    Flow::Err(ErreurFadera { message: m.into(), ligne: None, interne: true, pile: Vec::new(), cadre: None, origine_fixee: false, origine_module: false })
}

/// Marque le passage d'une erreur à travers une fonction du script (alimente la pile d'appels).
pub fn passer_fonction(f: Flow, nom: &str, module: Option<&str>) -> Flow {
    match f {
        Flow::Err(mut e) => {
            if e.pile.len() < 64 {
                let l = e.cadre.or(e.ligne).unwrap_or(0);
                let n = match module {
                    Some(m) => format!("{} [{}]", nom, m),
                    None => nom.to_string(),
                };
                e.pile.push((n, l));
            }
            e.cadre = None;
            match module {
                Some(m) => sortie_module(&mut e, m),
                None => e.origine_fixee = true,
            }
            Flow::Err(e)
        }
        autre => autre,
    }
}

/// Une erreur qui quitte du code d'un module : si elle y est née, son message dit dans quel module et à quelle ligne,
/// et sa ligne sera celle de l'appel dans le script. Si elle est née ailleurs (fonction de l'utilisateur appelée
/// par le module), on n'y touche pas.
pub fn sortie_module(e: &mut ErreurFadera, cle: &str) {
    if !e.origine_fixee {
        e.origine_fixee = true;
        e.origine_module = true;
        let l = match e.ligne {
            Some(l) => format!(", ligne {}", l),
            None => String::new(),
        };
        e.message = format!("[module {}{}] {}", cle, l, e.message);
    }
    if e.origine_module {
        e.ligne = None;
    }
}

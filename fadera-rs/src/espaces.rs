//! Espaces de noms de modules : `importer "outils" comme o`  puis  `o.double(3)`.
//!
//! Principe : avant l'analyse, tous les noms déclarés au niveau supérieur du module (fonctions, variables, structures,
//! alias d'imports) sont préfixés par `alias.` (« double » devient « o.double »). Les paramètres et variables locales
//! qui portent le même nom restent intacts. Le lexer accepte `o.double` comme un seul identifiant : le reste du moteur
//! n'a donc rien à savoir. (Même algorithme que `renommerModule` dans index.html.)

use crate::lexer::{Jeton, Type};
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq)]
enum Bloc {
    Rien,
    Fonction(usize),
    Classe,
}

struct Fonc {
    locaux: HashSet<String>,
    parent: Option<usize>,
}

fn mot(j: &Jeton, m: &str) -> bool {
    j.t == Type::MotCle && j.valeur == m
}
fn sym(j: &Jeton, m: &str) -> bool {
    j.t == Type::Symbole && j.valeur == m
}

pub fn renommer(toks: &mut [Jeton], alias: &str) {
    let n = toks.len();
    let mut fns: Vec<Fonc> = Vec::new();
    // pile des blocs ouverts : Rien (si, boucle…), Fonction(indice) ou Classe
    let mut pile: Vec<Bloc> = Vec::new();
    let mut methodes: HashSet<usize> = HashSet::new();
    let mut ouvertes: Vec<usize> = Vec::new();
    let mut haut: HashSet<String> = HashSet::new();
    let mut portee: Vec<Option<usize>> = vec![None; n];
    let mut lambdas: Vec<(usize, usize, HashSet<String>)> = Vec::new();

    let mut i = 0;
    while i < n {
        let cour = ouvertes.last().copied();
        portee[i] = cour;
        let t = &toks[i];
        let declarer = |nom: &str, fns: &mut Vec<Fonc>, haut: &mut HashSet<String>| match cour {
            None => {
                haut.insert(nom.to_string());
            }
            Some(f) => {
                fns[f].locaux.insert(nom.to_string());
            }
        };
        if t.t == Type::MotCle {
            match t.valeur.as_str() {
                "fonction" => {
                    let mut k = i + 1;
                    let methode = pile.last() == Some(&Bloc::Classe);
                    if k < n && toks[k].t == Type::Identifiant {
                        if methode {
                            // nom de méthode : ni déclaré ni renommé
                            methodes.insert(k);
                        } else {
                            let nom = toks[k].valeur.clone();
                            declarer(&nom, &mut fns, &mut haut);
                        }
                        k += 1;
                    }
                    let f = fns.len();
                    let mut locaux = HashSet::new();
                    if methode {
                        locaux.insert("soi".to_string());
                    }
                    fns.push(Fonc { locaux, parent: cour });
                    if k < n && sym(&toks[k], "(") {
                        k += 1;
                        while k < n && !sym(&toks[k], ")") {
                            if toks[k].t == Type::Identifiant {
                                fns[f].locaux.insert(toks[k].valeur.clone());
                            }
                            k += 1;
                        }
                    }
                    pile.push(Bloc::Fonction(f));
                    ouvertes.push(f);
                }
                "si" => {
                    let sinon_si = i > 0 && mot(&toks[i - 1], "sinon") && toks[i - 1].ligne == t.ligne;
                    if !sinon_si {
                        pile.push(Bloc::Rien);
                    }
                }
                "tantque" | "selon" | "essayer" => pile.push(Bloc::Rien),
                "pour" => {
                    pile.push(Bloc::Rien);
                    let mut k = i + 1;
                    if k < n && toks[k].valeur == "chaque" {
                        k += 1;
                    }
                    if k < n && toks[k].t == Type::Identifiant && cour.is_some() {
                        let nom = toks[k].valeur.clone();
                        declarer(&nom, &mut fns, &mut haut);
                    }
                }
                "fin" => {
                    if let Some(Bloc::Fonction(_)) = pile.pop() {
                        ouvertes.pop();
                    }
                }
                "soit" | "structure" => {
                    if i + 1 < n && toks[i + 1].t == Type::Identifiant {
                        let nom = toks[i + 1].valeur.clone();
                        declarer(&nom, &mut fns, &mut haut);
                    }
                }
                "attraper" => {
                    if i + 1 < n && toks[i + 1].t == Type::Identifiant && cour.is_some() {
                        let nom = toks[i + 1].valeur.clone();
                        declarer(&nom, &mut fns, &mut haut);
                    }
                }
                "importer" => {
                    let mut k = i + 1;
                    while k + 1 < n && toks[k].ligne == t.ligne {
                        if toks[k].t == Type::Identifiant && toks[k].valeur == "comme" && toks[k + 1].t == Type::Identifiant {
                            let nom = toks[k + 1].valeur.clone();
                            declarer(&nom, &mut fns, &mut haut);
                            break;
                        }
                        k += 1;
                    }
                }
                _ => {}
            }
        } else if t.t == Type::Identifiant && t.valeur == "classe" && i + 1 < n && toks[i + 1].t == Type::Identifiant {
            let nom = toks[i + 1].valeur.clone();
            declarer(&nom, &mut fns, &mut haut);
            pile.push(Bloc::Classe);
        } else if t.t == Type::Identifiant && t.valeur == "tester" && i + 1 < n && toks[i + 1].t == Type::Chaine {
            // un test est une fonction sans paramètre : ses variables sont locales
            let f = fns.len();
            fns.push(Fonc { locaux: HashSet::new(), parent: cour });
            pile.push(Bloc::Fonction(f));
            ouvertes.push(f);
        } else if sym(t, "=>") && i > 0 {
            // lambda : paramètres locaux jusqu'à la fin de la ligne
            let mut params = HashSet::new();
            let debut;
            if sym(&toks[i - 1], ")") {
                let mut k = i - 1;
                while k > 0 && !sym(&toks[k], "(") {
                    if toks[k].t == Type::Identifiant {
                        params.insert(toks[k].valeur.clone());
                    }
                    k -= 1;
                }
                debut = k;
            } else {
                if toks[i - 1].t == Type::Identifiant {
                    params.insert(toks[i - 1].valeur.clone());
                }
                debut = i - 1;
            }
            let mut fin = i;
            while fin + 1 < n && toks[fin + 1].ligne == t.ligne {
                fin += 1;
            }
            lambdas.push((debut, fin, params));
        }
        i += 1;
    }

    let local = |idx: usize, seg: &str| -> bool {
        let mut f = portee[idx];
        while let Some(x) = f {
            if fns[x].locaux.contains(seg) {
                return true;
            }
            f = fns[x].parent;
        }
        lambdas.iter().any(|(d, e, p)| idx >= *d && idx <= *e && p.contains(seg))
    };

    for idx in 0..n {
        if toks[idx].t != Type::Identifiant || methodes.contains(&idx) {
            continue;
        }
        let seg = toks[idx].valeur.split('.').next().unwrap_or("").to_string();
        if !haut.contains(&seg) {
            continue;
        }
        // clé de structure littérale { nom: ... } : on n'y touche pas
        let cle = idx > 0
            && (sym(&toks[idx - 1], "{") || sym(&toks[idx - 1], ","))
            && idx + 1 < n
            && sym(&toks[idx + 1], ":");
        if cle || local(idx, &seg) {
            continue;
        }
        toks[idx].valeur = format!("{}.{}", alias, toks[idx].valeur);
    }
}

//! Modules standard écrits en Fadéra : `importer "std/maths"`.
//! Les sources sont dans `src/std/*.fa` ; le moteur JavaScript embarque exactement les mêmes textes.

const MODULES: &[(&str, &str)] = &[
    ("std/dates", include_str!("std/dates.fa")),
    ("std/dictionnaire", include_str!("std/dictionnaire.fa")),
    ("std/ensemble", include_str!("std/ensemble.fa")),
    ("std/file", include_str!("std/file.fa")),
    ("std/json", include_str!("std/json.fa")),
    ("std/listes", include_str!("std/listes.fa")),
    ("std/maths", include_str!("std/maths.fa")),
    ("std/pile", include_str!("std/pile.fa")),
    ("std/regex", include_str!("std/regex.fa")),
    ("std/tas", include_str!("std/tas.fa")),
    ("std/textes", include_str!("std/textes.fa")),
    ("std/tri", include_str!("std/tri.fa")),
];

/// Suite de tests de la bibliothèque standard (bloc `tester` pour chaque module).
pub fn tests_std() -> &'static str {
    include_str!("std/tests_std.fa")
}

/// Source d'un module standard (nom sans extension : « std/maths »).
pub fn source(nom: &str) -> Option<&'static str> {
    MODULES.iter().find(|(n, _)| *n == nom).map(|(_, s)| *s)
}

/// Noms des modules standard, triés.
pub fn noms() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = MODULES.iter().map(|(n, _)| *n).collect();
    v.sort();
    v
}

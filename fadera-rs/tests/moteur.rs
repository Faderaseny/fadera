//! Tests du moteur Rust : cas qui ne peuvent pas être comparés au moteur JS (aléatoire, temps, E/S, crypto).
use fadera::executer_capture;

fn run(code: &str) -> String {
    executer_capture(code, vec![], &[("outils.fa", "fonction double(x)\n renvoyer x * 2\nfin")])
}

#[test]
fn hasard_dans_les_bornes() {
    assert_eq!(run("pour i allantde 1 a 500 faire\n soit h = hasard(3, 5)\n si h < 3 ou h > 5 alors\n  afficher \"hors\"\n fin\nfin\nafficher \"ok\""), "ok");
    assert_eq!(run("afficher hasard(7, 7)"), "7");
    assert_eq!(run("soit h = hasard(5, 1)\nafficher h >= 1 et h <= 5"), "vrai");
}

#[test]
fn saisie() {
    let r = executer_capture("soit n = saisir(\"x ?\")\nsoit t = saisir()\nsoit v = saisir()\nafficher n + 1\nafficher t\nafficher \"[\" + v + \"]\"", vec!["41".into(), "texte".into()], &[]);
    assert_eq!(r, "42\ntexte\n[]");
}

#[test]
fn fichiers() {
    assert_eq!(run("creer_fichier(\"a.txt\", \"salut\")\nafficher lireFichier(\"a.txt\")"), "salut");
    assert_eq!(run("ecrire_log(\"l.log\", \"un\")\necrire_log(\"l.log\", \"deux\")\nafficher lireFichier(\"l.log\")"), "un\ndeux\n");
    assert_eq!(run("ecrireFichier(\"b.txt\", [1, 2])\nafficher lireFichier(\"b.txt\")"), "[1, 2]");
    assert_eq!(run("importer \"outils\"\nafficher double(4)"), "8");
}

#[test]
fn temps() {
    assert_eq!(run("afficher maintenant() > 1700000000000\nafficher taille(date_iso()) == 24"), "vrai\nvrai");
}

#[test]
fn chiffrement_symetrique() {
    let r = run("soit c = chiffrer(\"secret é\", \"mdp\")\nafficher dechiffrer(c, \"mdp\")\nafficher commence_par(c, \"AES256GCM_\")\nafficher chiffrer(\"a\", \"k\") != chiffrer(\"a\", \"k\")\nessayer\n dechiffrer(c, \"autre\")\nattraper e\n afficher e\nfin");
    assert_eq!(r, "secret é\nvrai\nvrai\ndechiffrer : déchiffrement impossible : mauvaise clé ou données altérées.");
}

#[test]
fn chiffrement_asymetrique() {
    let r = run("soit k = crypto_generer_cles_rsa()\nsoit m = \"long \" + repeter(\"é\", 700)\nsoit c = crypto_chiffrer_asymetrique(m, lireCle(k, \"cle_publique\"))\nafficher crypto_dechiffrer_asymetrique(c, lireCle(k, \"cle_privee\")) == m\nsoit k2 = crypto_generer_cles_rsa()\nessayer\n crypto_dechiffrer_asymetrique(c, lireCle(k2, \"cle_privee\"))\nattraper e\n afficher contient(e, \"déchiffrement impossible\")\nfin");
    assert_eq!(r, "vrai\nvrai");
}

#[test]
fn hachages_connus() {
    assert_eq!(run("afficher sha256(\"abc\")"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    assert_eq!(
        run("afficher crypto_hacher_sha512(\"abc\")"),
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
    );
    // RFC 4231, cas 2
    assert_eq!(
        run("afficher crypto_hmac_sha256(\"what do ya want for nothing?\", \"Jefe\")"),
        "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
    );
    assert_eq!(run("afficher taille(crypto_aleatoire_hex(8))\nafficher taille(crypto_aleatoire_hex())"), "16\n32");
}

#[test]
fn reseau_de_neurones() {
    avec_essais(|| run("soit r = creerReseau(2, 4, 1)\npour i allantde 1 a 4000 faire\n iaEntrainer(r, [0,1], [1])\n iaEntrainer(r, [1,0], [1])\n iaEntrainer(r, [0,0], [0])\n iaEntrainer(r, [1,1], [0])\nfin\nafficher arrondir(lireElement(iaPredire(r, [0,1]), 0))\nafficher arrondir(lireElement(iaPredire(r, [1,1]), 0))\nafficher r") == "1\n0\n[Réseau 2-4-1]");
}

/// L'entraînement part de poids aléatoires : on tolère quelques minima locaux (comme en JS).
fn avec_essais(f: impl Fn() -> bool) {
    for _ in 0..6 {
        if f() {
            return;
        }
    }
    panic!("l'entraînement n'a jamais convergé en 6 essais");
}

#[test]
fn ia_xor_et_multicouche() {
    avec_essais(|| run("soit x = ia_entrainer_reseau_xor(8000)\nafficher lireCle(x, \"epoques\")\nsoit res = lireCle(x, \"resultats\")\nafficher lireCle(res, \"0,1\") > 0.5\nafficher lireCle(res, \"1,1\") < 0.5") == "8000\nvrai\nvrai");
    avec_essais(|| run("soit d = [{entrees: [0,0], attendu: [0]}, {entrees: [0,1], attendu: [1]}, {entrees: [1,0], attendu: [1]}, {entrees: [1,1], attendu: [0]}]\nsoit m = ia_reseau_multicouche([2, 4, 1], {dataset: d, epoques: 6000, tauxApprentissage: 0.5})\nsoit p = lireElement(lireCle(m, \"predictions\"), 1)\nafficher arrondir(lireElement(lireCle(p, \"sortie\"), 0))") == "1");
}

#[test]
fn neurone_simple() {
    let r = run("soit n = ia_creer_neurone(2, \"sigmoid\")\nafficher taille(lireCle(n, \"poids\"))\nsoit s = ia_activation_perceptron(n, [1, 0])\nafficher s > 0 et s < 1\nia_descente_gradient(n, {entrees: [1, 0], erreur: 1})\nafficher lireCle(n, \"gradient\") != 0");
    assert_eq!(r, "2\nvrai\nvrai");
}

// ---------------------------------------------------------------- modules standard (aléatoire / temps / HTTP)
#[test]
fn modules_aleatoire() {
    assert_eq!(run("afficher taille(alea_uuid())\nafficher alea_uuid() != alea_uuid()"), "36\nvrai");
    assert_eq!(
        run("soit ok = vrai\npour i allantde 1 a 300 faire\n soit x = alea_entier(3, 5)\n si x < 3 ou x > 5 alors\n  ok = faux\n fin\nfin\nafficher ok"),
        "vrai"
    );
    assert_eq!(run("soit l = alea_melanger([1,2,3,4,5])\nafficher somme(l) + \" \" + taille(l)"), "15 5");
    assert_eq!(run("afficher alea_choisir([7])"), "7");
    assert!(run("afficher alea_choisir([])").contains("la liste est vide"));
    assert!(run("afficher alea_entier(5, 1)").contains("le minimum doit être"));
}

#[test]
fn modules_temps_et_http() {
    assert_eq!(run("soit t0 = chrono()\npause(30)\nafficher chrono() - t0 >= 25"), "vrai");
    // hôte de test sans réseau : message clair, pas de plantage
    assert!(run("afficher http_obtenir(\"http://127.0.0.1:1/\")").contains("réseau indisponible"));
    assert!(run("afficher http_json(\"x\")").contains("réseau indisponible"));
}

// ---- état exporté pour le panneau des variables de l'IDE
struct Muet;
impl fadera::Hote for Muet {
    fn afficher(&mut self, _: &str) {}
}

fn etat(code: &str) -> (String, bool) {
    let (it, err) = fadera::executer_programme_complet(code, Box::new(Muet));
    let json = it.map(|i| fadera::util::json_texte(&i.exporter_etat(), "").unwrap_or_default()).unwrap_or_default();
    (json, err.is_some())
}

#[test]
fn etat_variables_et_fonctions() {
    let (j, err) = etat("soit n = 42\nsoit t = \"salut\"\nsoit l = [1, \"a\", vrai]\nstructure s = { nom: \"Seny\", age: 33 }\nfonction add(a, b)\n renvoyer a + b\nfin\nsoit z = add(1, 2)");
    assert!(!err);
    assert_eq!(
        j,
        r#"{"variables":{"n":42,"t":"salut","l":[1,"a",true],"s":{"nom":"Seny","age":33},"z":3},"fonctions":{"add":{"parametres":["a","b"],"lignes":1}}}"#
    );
}

#[test]
fn etat_conserve_apres_erreur() {
    let (j, err) = etat("soit a = 1\nsoit b = 2\nafficher inconnue\nsoit c = 3");
    assert!(err);
    assert!(j.contains(r#""a":1"#) && j.contains(r#""b":2"#) && !j.contains(r#""c""#), "{}", j);
    // erreur de syntaxe : pas d'interpréteur, donc pas d'état
    let (j2, err2) = etat("soit = =");
    assert!(err2 && j2.is_empty());
}

#[test]
fn etat_cas_limites() {
    // nombre non fini, fonction anonyme, liste qui se contient elle-même : texte, jamais de plantage
    let (j, err) = etat("soit moi = [1]\najouter(moi, moi)\nsoit f = fonction(x) renvoyer x fin");
    assert!(!err, "{}", j);
    assert!(j.contains(r#""moi":[1,[1,["#) && j.contains("[…]") && j.contains(r#""f":"#), "{}", j);
}

// ---- concaténation de texte : ajout en place (s = s + …) et chaînes « a + b + c »
#[test]
fn concat_semantique() {
    // le texte n'est jamais modifié dans une autre variable ou une liste qui le partage
    assert_eq!(run("soit s = \"a\"\nsoit t = s\ns = s + \"b\" + \"c\"\nafficher s\nafficher t"), "abc\na");
    assert_eq!(run("soit s = \"a\"\nsoit l = [s]\ns = s + \"b\" + \"c\"\nafficher lireElement(l, 0)\nafficher s"), "a\nabc");
    // auto-référence
    assert_eq!(run("soit s = \"x\"\ns = s + s + s\nafficher s"), "xxx");
    // nombres : l'addition reste numérique tant qu'il n'y a pas de texte
    assert_eq!(run("soit n = 1\nn = n + 2 + 3\nafficher n"), "6");
    assert_eq!(run("soit n = 1\nn = n + 2 + \"a\"\nafficher n"), "3a");
    // autres types ajoutés à un texte
    assert_eq!(run("soit s = \"v:\"\nsoit l = [1, 2]\nsoit b = vrai\ns = s + l + b\nafficher s"), "v:[1, 2]vrai");
    // portée locale et globale
    assert_eq!(run("fonction f()\n soit r = \"\"\n pour i allantde 1 a 3 faire\n  r = r + i + \",\"\n fin\n renvoyer r\nfin\nafficher f()"), "1,2,3,");
    assert_eq!(run("soit g = \"z\"\nfonction f()\n g = g + \"1\" + \"2\"\nfin\nf()\nf()\nafficher g"), "z1212");
    // variable inexistante dans la chaîne : erreur à la bonne ligne, texte inchangé après un essayer/attraper
    assert!(run("soit s = \"a\"\ns = s + \"b\" + zzz").starts_with("ERR:Ligne 2"));
    assert_eq!(run("soit s = \"a\"\nessayer\n s = s + \"b\" + zzz\nattraper e\n afficher \"err\"\nfin\nafficher s"), "err\na");
    // accents et très long texte
    assert_eq!(run("soit s = \"\"\npour i allantde 1 a 1000 faire\n s = s + \"é\" + \"ü\"\nfin\nafficher taille(s)"), "2000");
    // texte + liste + liste : erreur d'addition d'un nombre et d'une liste, comme avant
    assert!(run("soit n = 1\nn = n + [1] + \"a\"").starts_with("ERR:"));
}

// ---- structures : petites (recherche linéaire) et grandes (index), ordre des clés, écrasement, suppression
#[test]
fn structures_petites_et_grandes() {
    let mut prog = String::from("soit s = {}\n");
    for i in 0..12 {
        prog.push_str(&format!("modifierCle(s, \"k{}\", {})\n", i, i * 10));
    }
    prog.push_str("modifierCle(s, \"k3\", 999)\nafficher lireCle(s, \"k3\") + \" \" + lireCle(s, \"k11\") + \" \" + taille(cles(s))\n");
    prog.push_str("soit t = supprimer_cle(s, \"k0\")\nafficher taille(cles(t)) + \" \" + a_cle(t, \"k0\") + \" \" + a_cle(t, \"k5\")\nafficher lireElement(cles(t), 0)\n");
    let r = run(&prog);
    assert!(r.starts_with("999 110 12\n"), "{}", r);
    // une petite structure reste ordonnée et comparable
    assert_eq!(run("soit a = { x: 1, y: 2 }\nsoit b = { y: 2, x: 1 }\nafficher a == b\nafficher cles(a)"), "vrai\n[\"x\", \"y\"]");
}

// ---- pile d'appels dans les erreurs ----
fn erreur_de(code: &str) -> fadera::ErreurFadera {
    let (_it, err) = fadera::executer_programme_complet(code, Box::new(Muet));
    err.expect("une erreur était attendue")
}

#[test]
fn pile_appels_fonctions_imbriquees() {
    let e = erreur_de("fonction a(x)\n renvoyer b(x)\nfin\nfonction b(x)\n renvoyer 1/0\nfin\nafficher a(1)");
    assert_eq!(e.ligne, Some(5));
    let p: Vec<(String, usize)> = e.pile.clone();
    assert_eq!(p, vec![("b".to_string(), 5), ("a".to_string(), 2)]);
    assert_eq!(e.cadre, Some(7));
}

#[test]
fn pile_appels_recursion_et_lambda() {
    let e = erreur_de("fonction f(n)\n si n == 0 alors\n  renvoyer 1/0\n fin\n renvoyer f(n-1)\nfin\nafficher f(3)");
    assert_eq!(e.pile.len(), 4);
    assert!(e.pile.iter().all(|(f, _)| f == "f"));
    assert_eq!(e.pile[0].1, 3);
    assert_eq!(e.pile[1].1, 5);
    let e = erreur_de("soit r = filtrer([1,2], x => 1 / 0)");
    assert!(!e.pile.is_empty());
}

#[test]
fn pile_vide_au_niveau_programme() {
    let e = erreur_de("soit a = 1\nafficher 1/0");
    assert!(e.pile.is_empty());
    assert_eq!(e.ligne, Some(2));
}

// ---- erreurs dans les modules : la ligne est celle du script, le message dit où est le vrai problème ----
#[test]
fn erreur_dans_un_module_standard() {
    let e = erreur_de("importer \"std/listes\"\nafficher premier([])");
    assert_eq!(e.ligne, Some(2));
    assert!(e.message.starts_with("[module std/listes.fa, ligne 9] Index 0 hors limites"), "{}", e.message);
    assert_eq!(e.pile[0], ("premier [std/listes.fa]".to_string(), 9));
    assert_eq!(e.cadre, Some(2));
    assert!(e.origine_module);
}

#[test]
fn erreur_de_l_utilisateur_via_un_module_n_est_pas_attribuee_au_module() {
    let e = erreur_de("importer \"std/listes\"\nafficher compter_si([1, 2], (x) => 1 / 0 > x)");
    assert_eq!(e.ligne, Some(2));
    assert!(e.message.starts_with("Division par zéro"), "{}", e.message);
    assert!(!e.origine_module);
}

#[test]
fn erreur_au_niveau_superieur_d_un_module() {
    let (_it, err) = fadera::executer_programme_complet("afficher 0\nimporter \"casse2\"", Box::new(Fichiers));
    let e = err.expect("erreur attendue");
    assert_eq!(e.ligne, Some(2));
    assert!(e.message.starts_with("[module casse2.fa, ligne 2] Index 5 hors limites"), "{}", e.message);
}

struct Fichiers;
impl fadera::Hote for Fichiers {
    fn afficher(&mut self, _: &str) {}
    fn saisir(&mut self, _: &str) -> Result<Option<String>, fadera::HoteErr> {
        Ok(None)
    }
    fn lire_fichier(&mut self, p: &str) -> Result<String, fadera::HoteErr> {
        if p == "casse2.fa" {
            Ok("afficher 1\nafficher lireElement([1], 5)".to_string())
        } else {
            Err(fadera::HoteErr::Msg("absent".into()))
        }
    }
    fn ecrire_fichier(&mut self, _: &str, _: &str) -> Result<(), fadera::HoteErr> {
        Ok(())
    }
}

// ---- maths étendues, copie, texte à trous, bases ----
#[test]
fn maths_etendues() {
    assert_eq!(run("afficher pi()"), "3.14159265359");
    assert_eq!(run("afficher arrondir(sinus(pi() / 6), 10)"), "0.5");
    assert_eq!(run("afficher logarithme(1024, 2) + \" \" + logarithme(1000, 10) + \" \" + logarithme(exponentielle(3))"), "10 3 3");
    assert_eq!(run("afficher modulo(-7, 3) + \" \" + modulo(7, -3) + \" \" + signe(-9)"), "2 -2 -1");
    assert!(run("afficher logarithme(0)").contains("logarithme : l'argument doit être strictement positif."));
    assert!(run("afficher modulo(1, 0)").contains("modulo : division par zéro."));
    assert!(run("afficher arc_sinus(3)").contains("entre -1 et 1"));
}

#[test]
fn copie_profonde() {
    assert_eq!(run("soit a = [1, [2]]\nsoit b = copier(a)\najouter(lireElement(b, 1), 3)\nafficher a\nafficher b"), "[1, [2]]\n[1, [2, 3]]");
    // une liste qui se contient elle-même : pas de boucle infinie, et la copie est indépendante de l'original
    assert_eq!(run("soit l = [1]\najouter(l, l)\nsoit c = copier(l)\najouter(c, 5)\nafficher taille(l) + \" \" + taille(c)"), "2 3");
    assert_eq!(run("structure s = {a: 1}\nmodifierCle(s, \"moi\", s)\nsoit c = copier(s)\nmodifierCle(c, \"a\", 2)\nafficher lireCle(s, \"a\") + \" \" + lireCle(c, \"a\")"), "1 2");
}

#[test]
fn texte_a_trous() {
    assert_eq!(run("afficher texte_format(\"{} a {} ans\", [\"Awa\", 30])"), "Awa a 30 ans");
    assert_eq!(run("afficher texte_format(\"{nom}/{{x}}\", {nom: \"é\"})"), "é/{x}");
    assert!(run("afficher texte_format(\"{} {}\", [1])").contains("aucune valeur pour {1} (1 valeur(s) fournie(s))."));
    assert!(run("afficher texte_format(\"a } b\", [])").contains("accolade « } » isolée"));
}

#[test]
fn bases_numeriques() {
    assert_eq!(run("afficher en_base(255, 16) + \" \" + en_base(-10, 2) + \" \" + en_base(0, 7)"), "ff -1010 0");
    assert_eq!(run("afficher depuis_base(\"FF\", 16) + \" \" + depuis_base(\"-1010\", 2)"), "255 -10");
    assert_eq!(run("afficher en_base(9007199254740991, 16)"), "1fffffffffffff");
    assert!(run("afficher depuis_base(\"12\", 2)").contains("n'est pas un nombre valide en base 2."));
    assert!(run("afficher depuis_base(\"zzzzzzzzzzzzzzzz\", 36)").contains("valeur trop grande"));
}

#[test]
fn notation_scientifique_et_puissance() {
    assert_eq!(run("afficher 1e7 + \" \" + 2.5e-3 + \" \" + 4E+2 + \" \" + 1.5e3"), "10000000 0.0025 400 1500");
    assert_eq!(run("soit e2 = 5\nafficher e2 * 2"), "10");
    assert_eq!(run("afficher 2 ** 10 + \" \" + 2 ** -1 + \" \" + 10 ** 0"), "1024 0.5 1");
    assert_eq!(run("afficher 2 ** 3 ** 2"), "512");
    assert_eq!(run("afficher 3 * 2 ** 2 + \" \" + -2 ** 2 + \" \" + (-2) ** 2"), "12 -4 4");
    assert!(run("afficher 0 ** -1").contains("Division par zéro."));
    assert!(run("afficher (-8) ** 0.5").contains("Puissance impossible"));
    assert!(run("afficher \"a\" ** 2").contains("attend deux nombres (reçu : texte et nombre)"));
}

fn run_ns(code: &str) -> String {
    executer_capture(
        code,
        vec![],
        &[
            ("ns_a.fa", "soit PI2 = 6.28\nsoit compteur = 0\nfonction double(x)\n compteur = compteur + 1\n renvoyer x * 2\nfin\nfonction quadruple(x)\n renvoyer double(double(x))\nfin\nfonction forme(PI2)\n renvoyer PI2 + 1\nfin\nfonction cfg()\n renvoyer {double: 1, PI2: PI2}\nfin"),
            ("ns_b.fa", "fonction double(x)\n renvoyer x * 100\nfin"),
            ("ns_c.fa", "importer \"ns_b\" comme b\nfonction triple(x)\n renvoyer b.double(x) + 1\nfin"),
        ],
    )
}

#[test]
fn espaces_de_noms() {
    assert_eq!(run_ns("importer \"ns_a\" comme o\nafficher o.double(3)\nafficher o.PI2"), "6\n6.28");
    assert_eq!(run_ns("importer \"ns_a\" comme o\nimporter \"ns_b\" comme b\nafficher o.double(3)\nafficher b.double(3)"), "6\n300");
    assert_eq!(run_ns("importer \"ns_a\" comme o\nafficher o.quadruple(2)\nafficher o.compteur"), "8\n2");
    assert_eq!(run_ns("importer \"ns_a\" comme o\nsoit double = 5\nafficher double\nafficher o.double(1)"), "5\n2");
    assert_eq!(run_ns("importer \"ns_a\" comme o\nafficher o.forme(10)"), "11");
    assert_eq!(run_ns("importer \"ns_a\" comme o\nafficher o.cfg()"), "{ double: 1, PI2: 6.28 }");
    assert_eq!(run_ns("importer \"ns_c\" comme c\nafficher c.triple(2)"), "201");
    assert_eq!(run_ns("importer \"ns_b\" comme x\nimporter \"ns_b\" comme y\nafficher x.double(1)\nafficher y.double(2)"), "100\n200");
    assert_eq!(run_ns("importer \"ns_b\"\nafficher double(1)"), "100");
    assert_eq!(run_ns("importer \"std/maths\" comme m\nafficher m.factorielle(4)"), "24");
    assert_eq!(run_ns("soit comme = 3\nafficher comme"), "3");
}

#[test]
fn espaces_de_noms_erreurs() {
    assert!(run_ns("importer \"ns_b\" comme b\nafficher b.nope(1)").contains("Fonction \"b.nope\" inconnue"));
    assert!(run_ns("importer \"ns_b\" comme").contains("un nom est attendu après \"comme\""));
}

#[test]
fn classes_et_methodes() {
    let c = "classe P\n fonction creer(x)\n  soi.x = x\n fin\n fonction double()\n  renvoyer soi.x * 2\n fin\nfin\nclasse Q herite P\n fonction creer(x)\n  P.creer(soi, x + 1)\n fin\nfin\n";
    assert_eq!(run(&format!("{c}soit p = P(4)\nafficher p.double(), p\nsoit q = Q(4)\nafficher q.double(), q, est_instance(q, \"P\"), est_instance(p, \"Q\")")), "8 P { x: 4 }\n10 Q { x: 5 } vrai faux");
    assert_eq!(run(&format!("{c}soit p = P(1)\nafficher p == P(1), p == Q(0), classe_de(p), taille(p)")), "vrai faux P 1");
    assert!(run(&format!("{c}soit p = P(1)\np.nope()")).contains("La méthode \"nope\" n'existe pas dans la classe \"P\"."));
    assert!(run("classe A\n soit x = 1\nfin").contains("seules les méthodes"));
    assert!(run("classe A\n fonction f()\n fin\n fonction f()\n fin\nfin").contains("déjà définie"));
    assert!(run("classe A\n fonction f(soi)\n fin\nfin").contains("\"soi\" est réservé"));
    assert!(run("classe A\n fonction f()\n fin").contains("n'est jamais fermé"));
    // « classe » reste utilisable comme nom de variable
    assert_eq!(run("soit classe = 3\nafficher classe + 1"), "4");
}

#[test]
fn tests_integres() {
    let r = fadera::tester_capture("fonction f(x)\n renvoyer x + 1\nfin\ntester \"ok\"\n verifier_egal(f(1), 2)\nfin\ntester \"ko\"\n verifier_egal(f(1), 3)\nfin\ntester \"err\"\n verifier_erreur(() => 1 / 0, \"zéro\")\nfin");
    assert_eq!(r, "✓ ok\n✗ ko\n    Ligne 8 : Échec du test : obtenu 2, attendu 3.\n✓ err\n3 test(s) : 2 réussi(s), 1 échoué(s).");
    assert_eq!(fadera::tester_capture("afficher 1"), "1\nAucun test (écrivez : tester \"nom\" ... fin).");
    // une exécution normale n'exécute pas les tests
    assert_eq!(run("tester \"x\"\n afficher \"non\"\nfin\nafficher \"oui\""), "oui");
}

#[test]
fn enchainements() {
    let c = "classe C\n fonction creer(s)\n  soi.s = s\n fin\n fonction plus(m)\n  soi.s = soi.s + m\n  renvoyer soi\n fin\nfin\nfonction f()\n renvoyer {a: {b: 7}}\nfin\n";
    assert_eq!(run(&format!("{c}afficher C(1).plus(2).plus(3).s\nafficher f().a.b\nC(1).plus(1)")), "6\n7");
    assert!(run(&format!("{c}afficher f().z")).contains("Le champ \"z\" n'existe pas dans \"(expression)\""));
}

#[test]
fn session_ligne_de_commande() {
    use fadera::{Hote, Interp};
    struct H;
    impl Hote for H {
        fn afficher(&mut self, _: &str) {}
    }
    let mut it = Interp::new(Box::new(H));
    assert_eq!(it.evaluer_ligne("soit x = 5", true), (None, None));
    assert_eq!(it.evaluer_ligne("x * 2", false), (Some("10".to_string()), None));
    assert_eq!(it.evaluer_ligne("fonction f(a)\n renvoyer a + x\nfin", true), (None, None));
    assert_eq!(it.evaluer_ligne("f(1)", false), (Some("6".to_string()), None));
    let (_, e) = it.evaluer_ligne("y + 1", false);
    assert!(e.unwrap().contains("La variable \"y\" n'existe pas"));
    // la valeur temporaire ne fuit pas dans l'état exporté
    let etat = fadera::formater(&it.exporter_etat());
    assert!(!etat.contains("__valeur_repl") && etat.contains("x: 5"));
    assert_eq!(it.evaluer_ligne("classe P\n fonction creer()\n  soi.v = 1\n fin\nfin", true), (None, None));
    assert_eq!(it.evaluer_ligne("P().v", false), (Some("1".to_string()), None));
}

#[test]
fn suite_bibliotheque_standard() {
    let r = fadera::tester_capture(fadera::modules_std::tests_std());
    assert!(r.ends_with("12 test(s) : 12 réussi(s), 0 échoué(s)."), "{}", r);
}

#[test]
fn formateur() {
    use fadera::fmt::formater_source;
    assert_eq!(formater_source("soit   x=1+2*3", 4).unwrap(), "soit x = 1 + 2 * 3\n");
    assert_eq!(formater_source("si x>1 alors\nafficher(-x)  # c\nsinon\nafficher 0\nfin", 4).unwrap(), "si x > 1 alors\n    afficher(-x)  # c\nsinon\n    afficher 0\nfin\n");
    assert_eq!(formater_source("classe A\nfonction creer(x)\nsoi.x=x\nfin\nfin", 2).unwrap(), "classe A\n  fonction creer(x)\n    soi.x = x\n  fin\nfin\n");
    // idempotent, et un code invalide n'est jamais reformaté
    let une = formater_source("pour i allantde 1 a 3 faire\nafficher   i\nfin", 4).unwrap();
    assert_eq!(formater_source(&une, 4).unwrap(), une);
    assert!(formater_source("soit = 1", 4).is_err());
    assert_eq!(formater_source("pour chaque x dans[1,2] faire\nafficher x\nfin", 4).unwrap(), "pour chaque x dans [1, 2] faire\n    afficher x\nfin\n");
    assert_eq!(formater_source("pour i allantde 5 a 1 pas -1 faire\nafficher i\nfin", 4).unwrap(), "pour i allantde 5 a 1 pas -1 faire\n    afficher i\nfin\n");
    assert_eq!(formater_source("soit pas = 2\nafficher pas -1", 4).unwrap(), "soit pas = 2\nafficher pas - 1\n");
}

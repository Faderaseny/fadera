#!/usr/bin/env node
// =============================================================================
//  Suite de tests Fadéra  —  usage :  node tests_fadera.js [chemin/vers/index.html]
//  Teste le VRAI moteur (lexer, parser, interpréteur, fonctions natives) et le
//  shell à la Linux contenus dans index.html. Code de sortie 1 si un test échoue.
// =============================================================================
const fs = require("fs");
const path = require("path");
const fichier = path.resolve(process.argv[2] || path.join(__dirname, "index.html"));
const html = fs.readFileSync(fichier, "utf8");
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const c = html.indexOf("      const FadFS = (() => {"), d = html.indexOf("      // 🔗 LIAISON");
if (a < 0 || b < 0 || c < 0 || d < 0) { console.error("Moteur introuvable dans " + fichier); process.exit(2); }
const M = new Function("module", "require", "window", "globalThis",
  html.slice(a, b) + "\nreturn {analyserCode,executerProgramme,Interpreteur,decrireErreur,detaillerErreur,formater,FONCTIONS_NATIVES,ErreurFadera,Debogueur,SignalStop,lancerTests,TESTS_STD,formaterSource,OUTILS_EDITEUR,OUTILS_ZIP,OUTILS_PARTAGE};")({ exports: {} }, require, undefined, globalThis);
const S = new Function(html.slice(c, d) + ";return {FadFS,Shell}")();

let ok = 0, ko = 0, groupe = "";
const FICH = { "cl_mod.fa": 'classe Pt\n fonction creer(x)\n  soi.x = x\n fin\n fonction double()\n  renvoyer soi.x * 2\n fin\nfin\nsoit origine = Pt(0)\nfonction fabrique(n)\n renvoyer Pt(n)\nfin', "outils.fa": 'fonction double(x)\n renvoyer x * 2\nfin\nsoit PI2 = 6.28', "ns_a.fa": 'soit PI2 = 6.28\nsoit compteur = 0\nfonction double(x)\n compteur = compteur + 1\n renvoyer x * 2\nfin\nfonction quadruple(x)\n renvoyer double(double(x))\nfin\nfonction forme(PI2)\n renvoyer PI2 + 1\nfin\nfonction cfg()\n renvoyer {double: 1, PI2: PI2}\nfin', "ns_b.fa": 'fonction double(x)\n renvoyer x * 100\nfin', "ns_c.fa": 'importer "ns_b" comme b\nfonction triple(x)\n renvoyer b.double(x) + 1\nfin', "boucle.fa": 'importer "boucle"\nafficher 1', "casse.fa": 'fonction casse(x)\n renvoyer lireElement([1], x)\nfin\nsoit z = casse(0)', "casse2.fa": 'afficher 1\nafficher lireElement([1], 5)', "relais.fa": 'importer "casse"\nfonction relais(x)\n renvoyer casse(x)\nfin' };
async function run(code, saisies = []) {
  const out = [];
  const hote = { afficher: (t) => out.push(t), saisir: () => saisies.shift() || "", lireFichier: (p) => { if (p in FICH) return FICH[p]; throw new Error("absent"); }, ecrireFichier: () => {} };
  try { await M.executerProgramme(code, hote); } catch (e) { out.push("ERR:" + M.decrireErreur(e, code).texte); }
  return out.join("\n");
}
async function runT(code) {
  const out = [];
  const hote = { afficher: (t) => out.push(t), saisir: () => "", lireFichier: (p) => { if (p in FICH) return FICH[p]; throw new Error("absent"); }, ecrireFichier: () => {} };
  try { await M.lancerTests(code, hote); } catch (e) { out.push("ERR:" + M.decrireErreur(e, code).texte); }
  return out.join("\n");
}
async function tt(nom, code, att) {
  const r = await runT(code);
  const bon = att instanceof RegExp ? att.test(r) : r === att;
  if (bon) ok++; else { ko++; console.log("✗ [" + groupe + "] " + nom + "\n    obtenu : " + JSON.stringify(r) + "\n    attendu: " + String(att)); }
}
async function t(nom, code, att) {
  const r = await run(code);
  const bon = att instanceof RegExp ? att.test(r) : r === att;
  if (bon) ok++; else { ko++; console.log("✗ [" + groupe + "] " + nom + "\n    obtenu : " + JSON.stringify(r) + "\n    attendu: " + String(att)); }
}

async function moteur() {
  groupe = "moteur";
await t("arith",'afficher 2+3*4',"14");
await t("fnf stmt",'fnf afficher "a"','a');
await t("fnf hasard",'fnf hasard(5,5)',"");
await t("fnf expr",'soit x = fnf hasard(5,5)\nafficher x',"5");
await t("masque",'afficher masqueReseau(26)',"255.255.255.192");
await t("masque fnf",'soit m = fnf masqueReseau(24)\nafficher m',"255.255.255.0");
await t("plage",'afficher plageReseau("192.168.1.77", 26)',"192.168.1.64 - 192.168.1.127");
await t("plage stmt",'fnf plageReseau("10.0.0.5",8)',"");
await t("reseau",'soit r = creerReseau(2,3,1)\nsoit p = iaPredire(r,[0,1])\nafficher taille(p)\nafficher r',/^1\n\[Réseau 2-3-1\]$/);
await t("entrainer",'soit r = creerReseau(2,4,1)\nsoit e0 = 0\npour i allantde 1 a 3000 faire\n iaEntrainer(r,[0,1],[1])\n iaEntrainer(r,[1,0],[1])\n iaEntrainer(r,[0,0],[0])\n iaEntrainer(r,[1,1],[0])\nfin\nafficher lireElement(iaPredire(r,[0,1]),0) > 0.8',"vrai");
await t("ajouter expr",'soit l=[1]\nl = ajouter(l, 15)\nafficher l',"[1, 15]");
await t("ajouter stmt",'soit l=[1]\najouter(l, 2)\nafficher l',"[1, 2]");
await t("modifierElement expr",'soit l=[1,2]\nl = modifierElement(l,0,9)\nafficher l',"[9, 2]");
await t("modifierCle",'structure s = {a: 1}\nmodifierCle(s,"a",5)\nafficher lireCle(s,"a")',"5");
await t("essayer",'essayer\n afficher lireElement([1],5)\nattraper e\n afficher "pris: " + e\nfin',/^pris: Index 5/);
await t("essayer défaut",'essayer\n soit x = 1/0\n afficher x\nattraper\n afficher "oups"\nfin','oups'.length? /./:'');
await t("essayer ok",'essayer\n afficher 1\nattraper\n afficher 2\nfin\nafficher 3',"1\n3");
await t("essayer renvoyer",'fonction f()\n essayer\n  renvoyer 7\n attraper\n  renvoyer 0\n fin\nfin\nafficher f()',"7");
await t("essayer erreur dans fonction",'fonction g(n)\n renvoyer g(n+1)\nfin\nessayer\n g(1)\nattraper e\n afficher "ok"\nfin\nafficher f2(1)\nfonction f2(a)\n renvoyer a\nfin',"ok\n1");
await t("trier",'afficher trier([3,1,2])',"[1, 2, 3]");
await t("sha256",'afficher hacher_sha256("abc")',"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
await t("sha256 crypto_",'afficher crypto_hacher_sha256("abc")',"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
await t("b64",'soit c = chiffrer_base64("héllo")\nafficher c\nafficher dechiffrer_base64(c)',"aMOpbGxv\nhéllo");
await t("aes",'soit c = chiffrer("secret","mdp")\nafficher dechiffrer(c,"mdp")',"secret");
await t("stats",'afficher stats_moyenne([1,2,3])',"2");
await t("somme",'afficher somme([1,2,3])',"6");
await t("date_creer/composants",'soit d = date_creer(2026,10,14,10,30,0)\nsoit c = date_composants(d)\nafficher lireCle(c,"annee") + "-" + lireCle(c,"mois") + "-" + lireCle(c,"jour") + " " + lireCle(c,"jour_semaine")',"2026-10-14 mercredi");
await t("date_ajouter",'soit d = date_creer(2026,1,31,0,0,0)\nafficher date_formater(date_ajouter(d, 1, "jours"), "JJ/MM/AAAA")',"01/02/2026");
await t("date_difference",'afficher date_difference(date_creer(2026,1,1), date_creer(2026,3,1), "jours")',"59");
await t("date_analyser",'afficher date_formater(date_analyser("2026-12-25T08:05:09"), "AAAA-MM-JJ hh:mm:ss")',"2026-12-25 08:05:09");
await t("bissextile",'afficher est_bissextile(2024) + " " + est_bissextile(1900) + " " + est_bissextile(2000)',"vrai faux vrai");
await t("date inconnue unité",'afficher date_ajouter(0, 1, "lustres")',/ERR:.*unité inconnue/);
await t("remplir",'afficher texte_remplir_gauche("7", 3, "0") + "|" + texte_remplir_droite("ab", 4, ".")',"007|ab..");
await t("capitaliser/titre",'afficher texte_capitaliser("fADÉRA") + " " + texte_titre("le petit prince")',"Fadéra Le Petit Prince");
await t("slug",'afficher texte_slug("Été à Dakar : 2026 !")',"ete-a-dakar-2026");
await t("palindrome",'afficher texte_est_palindrome("Esope reste ici et se repose") + " " + texte_est_palindrome("Fadera")',"vrai faux");
await t("distance",'afficher texte_distance("chat","chats") + "-" + texte_distance("kitten","sitting")',"1-3");
await t("mots/lignes",'afficher taille(texte_mots("Bonjour, le monde !")) + " " + taille(texte_lignes("a\nb\nc"))',"3 3");
await t("url",'afficher url_encoder("a b&é") + " " + url_decoder("a%20b")',"a%20b%26%C3%A9 a b");
await t("alea_entier",'soit ok = vrai\npour i allantde 1 a 200 faire\n soit x = alea_entier(3, 5)\n si x < 3 ou x > 5 alors\n  ok = faux\n fin\nfin\nafficher ok',"vrai");
await t("alea_melanger",'soit l = alea_melanger([1,2,3,4,5])\nafficher somme(l) + " " + taille(l)',"15 5");
await t("alea_uuid",'afficher taille(alea_uuid())',"36");
await t("alea_choisir vide",'afficher alea_choisir([])',/ERR:.*liste est vide/);
await t("liste_decouper",'afficher liste_decouper([1,2,3,4,5], 2)',"[[1, 2], [3, 4], [5]]");
await t("liste_zip/cumul",'afficher liste_zip([1,2],["a","b"]) + " " + liste_cumul([1,2,3])','[[1, "a"], [2, "b"]] [1, 3, 6]');
await t("frequences",'soit f = liste_frequences(["a","b","a"])\nafficher lireCle(f,"a")',"2");
await t("csv_lire",'soit m = csv_lire("nom;age\nAna;30\nLi;25", ";")\nafficher lireElement(lireElement(m,2),0) + " " + (lireElement(lireElement(m,1),1) + 1)',"Li 31");
await t("csv_texte",'afficher csv_texte([["a","b,c"],[1,2]])','a,"b,c"\n1,2');
await t("pause",'soit t0 = chrono()\npause(30)\nafficher chrono() - t0 >= 25',"vrai");
await t("parler renvoie le texte",'afficher parler("")',"");
await t("parler refuse un nombre",'essayer\n  parler(5)\nattraper e\n  afficher e\nfin',"parler : l\'argument 1 doit être un texte (reçu : nombre).");
await t("http hors-ligne",'afficher http_obtenir("http://127.0.0.1:1/")',/ERR:.*requête impossible/);
await t("err fn inconnue",'afficher foo(1)',/ERR:Ligne 1 : Fonction "foo" inconnue/);
await t("err réseau arg",'afficher iaPredire(5,[1])',/doit être un réseau/);
await t("cidr err",'afficher masqueReseau(40)',/entre 0 et 32/);
await t("pas contextuels",'soit a = 3\nsoit pas = 2\npour i allantde 1 a 7 pas 3 faire\n afficher i\nfin',"1\n4\n7");
await t("sinon si",'soit n=5\nsi n<3 alors\n afficher "p"\nsinon si n<10 alors\n afficher "m"\nsinon\n afficher "g"\nfin',"m");
await t("récursion",'fonction fib(n)\n si n<=1 alors\n renvoyer n\n fin\n renvoyer fib(n-1)+fib(n-2)\nfin\nafficher fib(15)',"610");
await t("point-virgule",'soit x = 3;\nafficher x;',"3");
await t("saisir",'soit n = saisir("nom ?")\nafficher "x"+n',"x");
await t("rompre pour",'pour i allantde 1 a 10 faire\n si i == 4 alors\n  rompre\n fin\n afficher i\nfin',"1\n2\n3");
await t("continuer pour",'pour i allantde 1 a 5 faire\n si i % 2 == 0 alors\n  continuer\n fin\n afficher i\nfin',"1\n3\n5");
await t("rompre tantque",'soit n = 0\ntantque vrai faire\n n = n + 1\n si n > 3 alors\n  rompre\n fin\nfin\nafficher n',"4");
await t("continuer tantque",'soit n = 0\nsoit s = 0\ntantque n < 6 faire\n n = n + 1\n si n == 3 alors\n  continuer\n fin\n s = s + n\nfin\nafficher s',"18");
await t("rompre hors boucle",'rompre',/ERR:Ligne 1 : "rompre" ne peut être utilisé qu'à l'intérieur d'une boucle/);
await t("rompre dans fonction hors boucle",'fonction f()\n rompre\nfin\nf()',/ERR/);
await t("rompre imbriqué",'pour i allantde 1 a 3 faire\n pour j allantde 1 a 3 faire\n  si j == 2 alors\n   rompre\n  fin\n  afficher i + "-" + j\n fin\nfin',"1-1\n2-1\n3-1");
await t("rompre dans essayer",'pour i allantde 1 a 5 faire\n essayer\n  si i == 2 alors\n   rompre\n  fin\n  afficher i\n attraper e\n  afficher "x"\n fin\nfin\nafficher "fin"',"1\nfin");
// pour chaque
await t("pour chaque liste",'pour chaque x dans [10,20,30] faire\n afficher x\nfin',"10\n20\n30");
await t("pour chaque texte",'pour chaque c dans "abc" faire\n afficher c\nfin',"a\nb\nc");
await t("pour chaque struct",'structure s = {a: 1, b: 2}\npour chaque k dans s faire\n afficher k\nfin',"a\nb");
await t("pour chaque rompre",'pour chaque x dans [1,2,3,4] faire\n si x == 3 alors\n  rompre\n fin\n afficher x\nfin',"1\n2");
await t("pour chaque modifié",'soit l=[1,2,3]\npour chaque x dans l faire\n ajouter(l, x)\nfin\nafficher taille(l)',"6");
await t("chaque variable ok",'soit chaque = 5\nafficher chaque + 1',"6");
// selon
await t("selon",'soit n = 2\nselon n faire\n cas 1 alors\n  afficher "un"\n cas 2, 3 alors\n  afficher "deux ou trois"\n defaut\n  afficher "autre"\nfin',"deux ou trois");
await t("selon defaut",'selon "z" faire\n cas "a" alors\n  afficher 1\n defaut alors\n  afficher "dflt"\nfin',"dflt");
await t("selon sans match",'selon 9 faire\n cas 1 alors\n  afficher 1\nfin\nafficher "ok"',"ok");
await t("selon texte",'selon "b" faire\n cas "a" alors\n  afficher 1\n cas "b" alors\n  afficher 2\nfin',"2");
// lambdas, fonctions valeurs
await t("lambda",'soit d = x => x * 2\nafficher d(4)',"8");
await t("lambda 2 params",'soit add = (a, b) => a + b\nafficher add(2, 3)',"5");
await t("lambda fonction",'soit f = fonction(x)\n renvoyer x + 1\nfin\nafficher f(1)',"2");
await t("fermeture",'fonction compteur()\n soit c = 0\n renvoyer () => c + 1\nfin\nsoit f = compteur()\nafficher f()',"1");
await t("fermeture param",'fonction ajouteur(n)\n renvoyer x => x + n\nfin\nsoit a5 = ajouteur(5)\nafficher a5(10)',"15");
await t("fonction en argument",'fonction appliquer(f, v)\n renvoyer f(v)\nfin\nfonction carre(x)\n renvoyer x * x\nfin\nafficher appliquer(carre, 6)',"36");
await t("native en argument",'afficher transformer([4, 9], racine)',"[2, 3]");
await t("transformer",'afficher transformer([1,2,3], x => x * x)',"[1, 4, 9]");
await t("filtrer",'afficher filtrer([1,2,3,4,5,6], x => x % 2 == 0)',"[2, 4, 6]");
await t("reduire",'afficher reduire([1,2,3,4], (a, b) => a + b, 0)',"10");
await t("trier comparateur",'afficher trier([3,1,2], (a, b) => b - a)',"[3, 2, 1]");
await t("trier_par",'afficher trier_par(["ccc","a","bb"], t => taille(t))',/\["a", "bb", "ccc"\]/);
await t("tous/existe",'afficher tous([2,4], x => x % 2 == 0)\nafficher existe([1,3], x => x > 2)',"vrai\nvrai");
await t("type fonction",'afficher type_de(x => x)',/fonction/);
await t("afficher fonction",'fonction f()\n renvoyer 1\nfin\nafficher f',/fonction/);
// texte
await t("majuscule",'afficher majuscule("été")',"ÉTÉ");
await t("minuscule",'afficher minuscule("ABC")',"abc");
await t("decouper",'afficher decouper("a,b,c", ",")',/\["a", "b", "c"\]/);
await t("joindre",'afficher joindre(["a","b"], "-")',"a-b");
await t("remplacer",'afficher remplacer("a-b-c", "-", "+")',"a+b+c");
await t("contient",'afficher contient("bonjour", "jou")\nafficher contient([1,2],3)',"vrai\nfaux");
await t("commence/termine",'afficher commence_par("fadera","fa")\nafficher termine_par("fadera","ra")',"vrai\nvrai");
await t("sous_texte",'afficher sous_texte("fadera", 1, 4)',"ade");
await t("position",'afficher position("fadera","d")\nafficher position("fadera","z")',"2\n-1");
await t("nettoyer",'afficher "[" + nettoyer("  x  ") + "]"',"[x]");
await t("repeter",'afficher repeter("ab", 3)',"ababab");
await t("inverser",'afficher inverser("abc")\nafficher inverser([1,2,3])',"cba\n[3, 2, 1]");
await t("est_nombre",'afficher est_nombre("3.5")\nafficher est_nombre("x")',"vrai\nfaux");
await t("code",'afficher code_caractere("A")\nafficher depuis_code(66)',"65\nB");
// listes
await t("plage (fin incluse)",'afficher plage(1, 5)\nafficher plage(0, 10, 5)',"[1, 2, 3, 4, 5]\n[0, 5, 10]");
await t("tranche",'afficher tranche([1,2,3,4], 1, 3)',"[2, 3]");
await t("unique",'afficher unique([1,1,2,2,3])',"[1, 2, 3]");
await t("inserer/retirer",'soit l=[1,3]\ninserer(l,1,2)\nafficher l\nretirer(l,0)\nafficher l',"[1, 2, 3]\n[2, 3]");
await t("aplatir",'afficher aplatir([[1,2],[3]])',"[1, 2, 3]");
await t("compter",'afficher compter([1,2,1], 1)',"2");
// dicts / json
await t("cles/valeurs",'structure s = {a: 1, b: 2}\nafficher cles(s)\nafficher valeurs(s)',"[\"a\", \"b\"]\n[1, 2]");
await t("a_cle",'structure s = {a: 1}\nafficher a_cle(s,"a")\nafficher a_cle(s,"z")',"vrai\nfaux");
await t("supprimer_cle",'structure s = {a: 1, b: 2}\nsupprimer_cle(s,"a")\nafficher cles(s)',"[\"b\"]");
await t("fusionner",'afficher fusionner({a:1},{b:2})',/a: 1.*b: 2/);
await t("json",'soit j = json_texte({a: [1,2], b: "x"})\nafficher j\nsoit o = json_lire(j)\nafficher lireCle(o, "b")',/^\{"a":\[1,2\],"b":"x"\}\nx$/);
await t("json invalide",'afficher json_lire("{x")',/ERR/);
// regex / dates / maths
await t("regex",'afficher regex_test("abc123", "[0-9]+")\nafficher regex_trouver("a1b22", "[0-9]+")\nafficher regex_remplacer("a1b2", "[0-9]", "#")',/^vrai\n\["1", "22"\]\na#b#$/);
await t("regex invalide",'afficher regex_test("a","(")',/ERR/);
await t("dates",'afficher est_nombre(texte(maintenant()))\nafficher taille(date_iso())>=10',/vrai/);
await t("plancher",'afficher plancher(3.7)\nafficher plafond(3.2)\nafficher minimum(2,5)\nafficher maximum(2,5)',"3\n4\n2\n5");
// importer
await t("importer",'importer "outils"\nafficher double(4)\nafficher PI2',"8\n6.28");
await t("importer absent",'importer "inconnu"',/ERR:.*introuvable/);
await t("importer cycle",'importer "boucle"\nafficher 2',"1\n2");
await t("ns: alias",'importer "ns_a" comme o\nafficher o.double(3)\nafficher o.PI2',"6\n6.28");
await t("ns: pas de collision",'importer "ns_a" comme o\nimporter "ns_b" comme b\nafficher o.double(3)\nafficher b.double(3)',"6\n300");
await t("ns: interne au module",'importer "ns_a" comme o\nafficher o.quadruple(2)\nafficher o.compteur',"8\n2");
await t("ns: le global reste libre",'importer "ns_a" comme o\nsoit double = 5\nafficher double\nafficher o.double(1)',"5\n2");
await t("ns: parametre local prime",'importer "ns_a" comme o\nafficher o.forme(10)',"11");
await t("ns: cle de structure intacte",'importer "ns_a" comme o\nafficher o.cfg()',"{ double: 1, PI2: 6.28 }");
await t("ns: imports imbriques",'importer "ns_c" comme c\nafficher c.triple(2)',"201");
await t("ns: deux alias du meme module",'importer "ns_b" comme x\nimporter "ns_b" comme y\nafficher x.double(1)\nafficher y.double(2)',"100\n200");
await t("ns: sans alias inchange",'importer "ns_b"\nafficher double(1)',"100");
await t("ns: std/maths",'importer "std/maths" comme m\nafficher m.factorielle(4)',"24");
await t("ns: inconnu",'importer "ns_b" comme b\nafficher b.nope(1)',/Fonction "b\.nope" inconnue/);
await t("ns: comme sans nom",'importer "ns_b" comme',/un nom est attendu après "comme"/);
await t("ns: variable nommee comme",'soit comme = 3\nafficher comme',"3");
await t("texte: points de code",'afficher taille("😀a")\nafficher lireElement("😀a", 1)\nafficher sous_texte("😀abc", 1, 3)\nafficher position("😀abc", "b")',"2\na\nab\n2");
await t("cycle: affichage",'soit a = []\najouter(a, a)\nafficher a',"[[...]]");
await t("cycle: egalite",'soit a = []\najouter(a, a)\nsoit b = []\najouter(b, b)\nafficher a == a\nafficher a == b',"vrai\nvrai");
await t("joindre composes",'afficher joindre([[1],{a:[2]}], "|")',"[1]|{ a: [2] }");
await t("maths: arrondi correct",'afficher sinus(1) == 0.8414709848078965\nafficher cosinus(1) == 0.5403023058681398\nafficher exponentielle(1) == 2.718281828459045\nafficher logarithme(10) == 2.302585092994046\nafficher 2 ** 0.5 == 1.4142135623730951\nafficher arc_tangente(1) == 0.7853981633974483\nafficher 10 ** -2 == 0.01',"vrai\nvrai\nvrai\nvrai\nvrai\nvrai\nvrai");
await t("maths: puissance cas limites",'afficher 2 ** 10\nafficher (-8) ** 3\nafficher 0 ** 0\nafficher (-8) ** 0.5',/^1024\n-512\n1\nERR:.*Puissance impossible/);
await t("importer deux fois",'importer "outils"\nimporter "outils.fa"\nafficher double(1)',"2");
// modules standard (std/...) : mêmes sources dans les deux moteurs
await t("std/maths factorielle",'importer "std/maths"\nafficher factorielle(5)\nafficher factorielle(0)',"120\n1");
await t("std/maths fibonacci",'importer "std/maths"\nafficher fibonacci(10)\nafficher fibonacci(0)\nafficher fibonacci(1)',"55\n0\n1");
await t("std/maths est_premier",'importer "std/maths"\nafficher est_premier(97)\nafficher est_premier(1)\nafficher est_premier(91)',"vrai\nfaux\nfaux");
await t("std/maths premiers_jusqua",'importer "std/maths"\nafficher premiers_jusqua(20)',"[2, 3, 5, 7, 11, 13, 17, 19]");
await t("std/maths somme_chiffres",'importer "std/maths"\nafficher somme_chiffres(-1234)\nafficher somme_chiffres(0)',"10\n0");
await t("std/maths limiter",'importer "std/maths"\nafficher limiter(15, 0, 10)\nafficher limiter(-5, 0, 10)\nafficher limiter(5, 0, 10)',"10\n0\n5");

// bibliothèque standard étendue (classes : pile, file, dictionnaire, ensemble, tas)
await t("std/pile","importer \"std/pile\"\nsoit p = Pile()\np.empiler(1).empiler(2).empiler(3)\nafficher p.depiler() + \" \" + p.sommet() + \" \" + p.longueur() + \" \" + p.est_vide()\nafficher p.en_liste()\nafficher p","3 2 2 faux\n[1, 2]\nPile { elements: [1, 2] }");
await t("std/pile vide","importer \"std/pile\"\nessayer\n  Pile().depiler()\nattraper e\n  afficher e\nfin","[module std/pile.fa, ligne 22] Pile vide : rien à dépiler.");
await t("std/file","importer \"std/file\"\nsoit f = File()\npour i allantde 1 a 100 faire\n  f.enfiler(i)\nfin\nsoit s = 0\ntantque non f.est_vide() faire\n  s = s + f.defiler()\nfin\nafficher s\nf.enfiler(\"a\").enfiler(\"b\")\nafficher f.en_liste()\nafficher f.tete()","5050\n[\"a\", \"b\"]\na");
await t("std/dictionnaire ordre","importer \"std/dictionnaire\"\nsoit d = Dictionnaire()\nd.definir(\"z\", 1).definir(10, \"dix\").definir(2, \"deux\").definir(\"z\", 9)\nafficher d.liste_cles()\nafficher d.liste_paires()\nafficher d.obtenir_ou(\"q\", \"?\") + d.obtenir(2)\nafficher d.supprimer(10)\nafficher d.liste_cles()","[\"z\", \"10\", \"2\"]\n[[\"z\", 9], [\"10\", \"dix\"], [\"2\", \"deux\"]]\n?deux\ndix\n[\"z\", \"2\"]");
await t("std/ensemble","importer \"std/ensemble\"\nsoit e = ensemble_depuis([3, 1, 3, \"1\", 2])\nafficher e.en_liste()\nafficher e.union(ensemble_depuis([9, 1])).en_liste()\nafficher e.intersection(ensemble_depuis([1, 2, 7])).en_liste()\nafficher e.difference(ensemble_depuis([1])).en_liste()\ne.exclure(3)\nafficher e.longueur()","[3, 1, \"1\", 2]\n[3, 1, \"1\", 2, 9]\n[1, 2]\n[3, \"1\", 2]\n3");
await t("std/tas","importer \"std/tas\"\nsoit t = tas_min()\npour chaque x dans [5, 3, 8, 1, 9, 2, 7] faire\n  t.pousser(x)\nfin\nsoit out = []\ntantque non t.est_vide() faire\n  ajouter(out, t.extraire())\nfin\nafficher out\nsoit t2 = Tas((a, b) => b.age - a.age)\nt2.pousser({nom: \"a\", age: 3}).pousser({nom: \"b\", age: 30})\nafficher t2.extraire().nom\nafficher tas_max().pousser(\"m\").pousser(\"z\").extraire()","[1, 2, 3, 5, 7, 8, 9]\nb\nz");
await t("std/tri","importer \"std/tri\"\nsoit l = [5, 2, 9, 1, 5, 6]\nafficher tri_fusion(l, ordre_naturel)\nafficher tri_rapide(l, ordre_naturel)\nafficher tri_insertion(l, (a, b) => b - a)\nafficher l\nsoit s = tri_fusion(l, ordre_naturel)\nafficher recherche_binaire(s, 6) + \" \" + recherche_binaire(s, 7) + \" \" + borne_inf(s, 5) + \" \" + borne_sup(s, 5)\nafficher plus_petits(l, 2)\nafficher plus_grands(l, 3)","[1, 2, 5, 5, 6, 9]\n[1, 2, 5, 5, 6, 9]\n[9, 6, 5, 5, 2, 1]\n[5, 2, 9, 1, 5, 6]\n4 -1 2 4\n[1, 2]\n[9, 6, 5]");
await t("std/tri multi","importer \"std/tri\"\nsoit g = [{n: \"a\", age: 30, v: \"x\"}, {n: \"b\", age: 20, v: \"y\"}, {n: \"c\", age: 30, v: \"a\"}]\nafficher transformer(trier_multi(g, [x => x.age, x => x.v]), x => x.n)\nafficher grouper_par(g, x => x.age)","[\"b\", \"c\", \"a\"]\n{ 20: [{ n: \"b\", age: 20, v: \"y\" }], 30: [{ n: \"a\", age: 30, v: \"x\" }, { n: \"c\", age: 30, v: \"a\" }] }");
await t("std/json","importer \"std/json\" comme j\nafficher j.json_aplatir({a: {b: [1, {c: 2}]}, d: \"x\", e: []})\nafficher j.json_profondeur({a: {b: [1]}})\nafficher j.json_selectionner([{a:1,b:2},{a:3,b:4}], [\"b\"])\nafficher j.json_lire_ou(\"{oops\", \"bad\")\nafficher j.json_egal({a:1}, {a:1})","{ a.b[0]: 1, a.b[1].c: 2, d: \"x\", e: [] }\n3\n[{ b: 2 }, { b: 4 }]\nbad\nvrai");
await t("std/regex","importer \"std/regex\"\nafficher est_courriel(\"a.b@ex.fr\") + \" \" + est_courriel(\"a@b\") + \" \" + est_url(\"https://x.io/p?q=1\") + \" \" + est_entier(\"-12\") + \" \" + est_telephone(\"+33 6 12 34 56 78\") + \" \" + est_telephone(\"12\")\nafficher extraire_nombres(\"a 3, -4.5 et 10\")\nafficher extraire_mots(\"Été chaud, déjà là !\")\nafficher reduire_espaces(\"  a   b \\n c \")\nafficher remplacer_chaque(\"a1b22\", \"\\\\d+\", n => \"<\" + (nombre(n) * 2) + \">\")","vrai faux vrai vrai vrai faux\n[3, -4.5, 10]\n[\"Été\", \"chaud\", \"déjà\", \"là\"]\na b c\na<2>b<44>");
await t("std/dates +","importer \"std/dates\"\nsoit ts = date_creer(2026, 10, 22)\nafficher date_longue(ajouter_jours(ts, 1))\nafficher date_formater(fin_du_mois(ts), \"AAAA-MM-JJ\") + \" \" + date_formater(debut_du_mois(ts), \"AAAA-MM-JJ\")\nafficher jours_ouvres_entre(date_creer(2026,10,19), date_creer(2026,10,26)) + \" \" + date_formater(ajouter_jours_ouvres(ts, 3), \"AAAA-MM-JJ\")\nafficher date_formater(prochain_jour(ts, \"Lundi\"), \"AAAA-MM-JJ\")\nafficher calendrier(2026, 10)\nafficher duree_texte(93784000) + \" | \" + duree_texte(0)","vendredi 23 octobre 2026\n2026-10-31 2026-10-01\n5 2026-10-27\n2026-10-26\nLu Ma Me Je Ve Sa Di\n          1  2  3  4\n 5  6  7  8  9 10 11\n12 13 14 15 16 17 18\n19 20 21 22 23 24 25\n26 27 28 29 30 31\n1 j 2 h 3 min 4 s | 0 s");
await t("std/maths +","importer \"std/maths\"\nafficher pgcd(12, 18) + \" \" + ppcm(4, 6) + \" \" + somme([1,2,3]) + \" \" + produit([2,3,4]) + \" \" + moyenne([1,2,3,4]) + \" \" + mediane([5,1,3]) + \" \" + mediane([1,2,3,4])\nafficher variance([2,4,4,4,5,5,7,9]) + \" \" + ecart_type([2,4,4,4,5,5,7,9])\nafficher combinaisons(5, 2) + \" \" + combinaisons(50, 25) + \" \" + arrangements(5, 2) + \" \" + puissance_mod(2, 100, 1000007) + \" \" + racine_entiere(99) + \" \" + est_carre_parfait(144)\nafficher diviseurs(36)\nafficher transposer([[1,2,3],[4,5,6]])\nafficher produit_matrices([[1,2],[3,4]], [[5,6],[7,8]])","6 12 6 24 2.5 3 2.5\n4 2\n10 126410606437752 20 698635 9 vrai\n[1, 2, 3, 4, 6, 9, 12, 18, 36]\n[[1, 4], [2, 5], [3, 6]]\n[[19, 22], [43, 50]]");
if (typeof M !== "undefined" && M.TESTS_STD) await tt("suite de tests de la bibliothèque standard (bouton du menu Applications)", M.TESTS_STD, /12 test\(s\) : 12 réussi\(s\), 0 échoué\(s\)\./);
await t("lever","essayer\n  lever(\"boum\")\nattraper e\n  afficher \"attrapé: \" + e\nfin","attrapé: boum");
await t("std/textes est_vide",'importer "std/textes"\nafficher est_vide("   ")\nafficher est_vide("a")',"vrai\nfaux");
await t("std/textes centrer",'importer "std/textes"\nafficher "[" + centrer("ab", 6) + "]"\nafficher "[" + centrer("abc", 2) + "]"',"[  ab  ]\n[abc]");
await t("std/textes tronquer",'importer "std/textes"\nafficher tronquer("abcdef", 4)\nafficher tronquer("abc", 5)',"abc…\nabc");
await t("std/textes inverser_texte",'importer "std/textes"\nafficher inverser_texte("Fadéra")',"arédaF");
await t("std/textes compter_occurrences",'importer "std/textes"\nafficher compter_occurrences("banana", "an")\nafficher compter_occurrences("aaaa", "aa")\nafficher compter_occurrences("abc", "")',"2\n2\n0");
await t("std/listes premier dernier",'importer "std/listes"\nafficher premier([4, 5, 6])\nafficher dernier([4, 5, 6])',"4\n6");
await t("std/listes premier vide",'importer "std/listes"\nafficher premier([])',/^ERR:Ligne 2 : \[module std\/listes\.fa, ligne 9\] Index 0 hors limites \(la liste est vide\)\.$/);
await t("std/listes prendre sauter",'importer "std/listes"\nafficher prendre([1, 2, 3, 4], 2)\nafficher sauter([1, 2, 3, 4], 1)\nafficher prendre([1, 2], -3)',"[1, 2]\n[2, 3, 4]\n[]");
await t("std/listes compter_si",'importer "std/listes"\nafficher compter_si([1, 2, 3, 4, 5], (x) => x % 2 == 0)',"2");
await t("std/listes partitionner",'importer "std/listes"\nafficher partitionner([1, 2, 3, 4], (x) => x > 2)',"[[3, 4], [1, 2]]");
await t("std/dates date_longue",'importer "std/dates"\nafficher date_longue(date_creer(2026, 10, 17, 13, 29, 0))',"samedi 17 octobre 2026");
await t("std/dates est_week_end",'importer "std/dates"\nafficher est_week_end(date_creer(2026, 10, 17, 9, 0, 0))\nafficher est_week_end(date_creer(2026, 10, 18, 9, 0, 0))\nafficher est_week_end(date_creer(2026, 10, 19, 9, 0, 0))',"vrai\nvrai\nfaux");
await t("std/dates jours_entre",'importer "std/dates"\nafficher jours_entre(date_creer(2026, 10, 1, 0, 0, 0), date_creer(2026, 10, 17, 0, 0, 0))\nafficher jours_entre(date_creer(2026, 10, 17, 0, 0, 0), date_creer(2026, 10, 1, 0, 0, 0))',"16\n-16");
await t("std inconnu",'importer "std/zzz"',/ERR:.*module standard "std\/zzz" inconnu \(modules disponibles : std\/dates, std\/dictionnaire, std\/ensemble, std\/file, std\/json, std\/listes, std\/maths, std\/pile, std\/regex, std\/tas, std\/textes, std\/tri\)/);
// erreurs dans les modules : la ligne rapportée est celle du script, le message dit où est le vrai problème
await t("erreur dans une fonction de module",'importer "casse"\nafficher casse(3)',/^ERR:Ligne 2 : \[module casse\.fa, ligne 2\] Index 3 hors limites \(indices valides : 0 à 0\)\.$/);
await t("erreur au niveau supérieur d'un module",'afficher 0\nimporter "casse2"',/^0\n1\nERR:Ligne 2 : \[module casse2\.fa, ligne 2\] Index 5 hors limites/);
await t("erreur : module qui appelle un module",'importer "relais"\nafficher relais(7)',/^ERR:Ligne 2 : \[module casse\.fa, ligne 2\] Index 7 hors limites/);
await t("erreur d'une fonction de l'utilisateur appelée par un module : pas attribuée au module",'importer "std/listes"\nafficher compter_si([1, 2], (x) => 1 / 0 > x)',/^ERR:Ligne 2 : Division par zéro\.$/);
await t("erreur de l'utilisateur dans sa fonction, appelée depuis un module",'importer "std/listes"\nfonction mauvais(x)\n renvoyer lireElement([], x)\nfin\nafficher compter_si([1], mauvais)',/^ERR:Ligne 3 : Index 1 hors limites \(la liste est vide\)\.$/);
await t("std deux fois",'importer "std/maths"\nimporter "std/maths.fa"\nafficher factorielle(3)',"6");
await t("std + module utilisateur",'importer "std/maths"\nimporter "outils"\nafficher double(factorielle(3))',"12");
// régressions
await t("fonction récursive",'fonction f(n)\n si n == 0 alors\n  renvoyer 0\n fin\n renvoyer n + f(n - 1)\nfin\nafficher f(100)',"5050");
await t("fnf",'fnf afficher "ok"',"ok");
await t("variable cas réservé msg",'soit cas = 1',/ERR:.*cas/);

// ---- tests intégrés au langage
await t("verifier ok",'verifier(1 + 1 == 2, "maths")\nverifier_egal([1,2], [1,2])\nafficher "ok"',"ok");
await t("verifier échec",'verifier(1 == 2, "faux")',/ERR:Ligne 1 : Échec du test : faux\./);
await t("verifier_egal échec",'verifier_egal(3, 4, "somme")',/Échec du test \(somme\) : obtenu 3, attendu 4/);

// ---- bibliothèque standard étendue : regex, JSON, dates (mêmes tests rejoués contre le moteur Rust par tests/parite.js)
await t("regex_groupes",'soit g = regex_groupes("2026-10-17", "(\\d+)-(\\d+)-(\\d+)")\nafficher taille(g)\nafficher lireElement(g,0) + "|" + lireElement(g,1) + "|" + lireElement(g,3)',"4\n2026-10-17|2026|17");
await t("regex_groupes aucun",'afficher taille(regex_groupes("abc", "(\\d+)"))',"0");
await t("regex_groupes optionnel",'afficher regex_groupes("ab", "(a)(x)?(b)")','["ab", "a", "", "b"]');
await t("regex_groupes drapeau i",'afficher lireElement(regex_groupes("ABC", "(b)", "i"), 1)',"B");
await t("regex_decouper",'afficher regex_decouper("a1b22c", "\\d+")','["a", "b", "c"]');
await t("regex_decouper séparateurs",'afficher regex_decouper("a, b;c", "[,;] *")','["a", "b", "c"]');
await t("regex_decouper vide ignoré",'afficher regex_decouper("abc", "x*")','["abc"]');
await t("regex_decouper aux bords",'afficher regex_decouper(",a,", ",")','["", "a", ""]');
await t("regex_decouper accents",'afficher regex_decouper("été-hiver", "-")','["été", "hiver"]');
await t("regex_echapper",'afficher regex_echapper("a.b*c(d)[e]")','a\\.b\\*c\\(d\\)\\[e\\]');
await t("regex_echapper aller-retour",'soit m = "^" + regex_echapper("a.b") + "$"\nafficher regex_test("a.b", m)\nafficher regex_test("axb", m)',"vrai\nfaux");
await t("regex_groupes erreur type",'afficher regex_groupes(5, "a")',/ERR:Ligne 1 : regex_groupes : l'argument 1 doit être un texte \(reçu : nombre\)\./);
await t("regex_decouper motif invalide",'afficher regex_decouper("a", "(")',/ERR:Ligne 1 : regex_decouper : expression régulière invalide/);
await t("json_valide",'afficher json_valide("{\\"a\\":1}")\nafficher json_valide("{a")\nafficher json_valide(3)\nafficher json_valide("[1,2]")',"vrai\nfaux\nfaux\nvrai");
const DJ = 'soit d = json_lire("{\\"client\\":{\\"adresses\\":[{\\"ville\\":\\"Dakar\\"},{\\"ville\\":\\"Paris\\"}],\\"age\\":30}}")\n';
await t("json_chemin",DJ+'afficher json_chemin(d, "client.adresses[1].ville")\nafficher json_chemin(d, "client.adresses.0.ville")\nafficher json_chemin(d, "client.age")',"Paris\nDakar\n30");
await t("json_chemin défaut",DJ+'afficher json_chemin(d, "client.tel", "inconnu")\nafficher json_chemin(d, "client.adresses[9]", 0)',"inconnu\n0");
await t("json_chemin introuvable",'soit d = {a: 1}\nafficher json_chemin(d, "a.b")',"ERR:Ligne 2 : json_chemin : chemin « a.b » introuvable (rien à « b » après « a »).");
await t("json_chemin index hors liste",'soit d = {a: [1]}\nafficher json_chemin(d, "a[3]")',"ERR:Ligne 2 : json_chemin : chemin « a[3] » introuvable (rien à « [3] » après « a »).");
await t("json_chemin racine liste",'afficher json_chemin([[1,2],[3,4]], "[1][0]")',"3");
await t("json_fusionner",'afficher json_texte(json_fusionner({a: 1, b: {c: 2, d: 3}}, {b: {c: 9}, e: [1]}))','{"a":1,"b":{"c":9,"d":3},"e":[1]}');
await t("json_fusionner n'altère pas",'soit x = {a: {b: 1}}\nsoit y = json_fusionner(x, {a: {c: 2}})\nafficher json_texte(x)\nafficher json_texte(y)','{"a":{"b":1}}\n{"a":{"b":1,"c":2}}');
await t("json_fusionner erreur",'afficher json_fusionner(1, {})',"ERR:Ligne 1 : json_fusionner : deux structures sont attendues.");
const DC = 'soit c = date_composants(RES)\nafficher lireCle(c,"annee") + "-" + lireCle(c,"mois") + "-" + lireCle(c,"jour") + " " + lireCle(c,"heure") + ":" + lireCle(c,"minute") + ":" + lireCle(c,"seconde")';
const dc = (expr) => DC.replace("RES", expr);
await t("date_debut jour",dc('date_debut(date_creer(2026,10,17,15,30,45), "jour")'),"2026-10-17 0:0:0");
await t("date_debut défaut = jour",dc('date_debut(date_creer(2026,10,17,15,30,45))'),"2026-10-17 0:0:0");
await t("date_debut semaine (lundi)",dc('date_debut(date_creer(2026,10,17,15,30,45), "semaine")'),"2026-10-12 0:0:0");
await t("date_debut semaine un lundi",dc('date_debut(date_creer(2026,10,12,8), "semaine")'),"2026-10-12 0:0:0");
await t("date_debut mois",dc('date_debut(date_creer(2026,10,17,15), "mois")'),"2026-10-1 0:0:0");
await t("date_debut annee",dc('date_debut(date_creer(2026,10,17,15), "annee")'),"2026-1-1 0:0:0");
await t("date_debut unité inconnue",'afficher date_debut(date_creer(2026,1,1), "siecle")',"ERR:Ligne 1 : date_debut : unité inconnue « siecle » (jour, semaine, mois, annee).");
await t("date_jours_dans_mois",'afficher date_jours_dans_mois(2024,2)\nafficher date_jours_dans_mois(2026,2)\nafficher date_jours_dans_mois(1900,2)\nafficher date_jours_dans_mois(2000,2)\nafficher date_jours_dans_mois(2026,4)\nafficher date_jours_dans_mois(2026,12)',"29\n28\n28\n29\n30\n31");
await t("date_jours_dans_mois erreur",'afficher date_jours_dans_mois(2026,13)',"ERR:Ligne 1 : date_jours_dans_mois : le mois doit être un entier de 1 à 12.");
await t("date_ajouter_mois fin de mois",dc('date_ajouter_mois(date_creer(2026,1,31), 1)'),"2026-2-28 0:0:0");
await t("date_ajouter_mois bissextile",dc('date_ajouter_mois(date_creer(2024,1,31), 1)'),"2024-2-29 0:0:0");
await t("date_ajouter_mois année suivante",dc('date_ajouter_mois(date_creer(2026,10,17,9,5,7), 3)'),"2027-1-17 9:5:7");
await t("date_ajouter_mois négatif",dc('date_ajouter_mois(date_creer(2026,3,15), -10)'),"2025-5-15 0:0:0");
await t("date_ajouter_mois zéro",dc('date_ajouter_mois(date_creer(2026,3,15,1,2,3), 0)'),"2026-3-15 1:2:3");
await t("date_ajouter_mois entier",'afficher date_ajouter_mois(date_creer(2026,1,1), 1.5)',"ERR:Ligne 1 : date_ajouter_mois : le nombre de mois doit être un entier.");
await t("date_semaine",'afficher date_semaine(date_creer(2026,10,14))\nafficher date_semaine(date_creer(2026,1,1))\nafficher date_semaine(date_creer(2021,1,1))\nafficher date_semaine(date_creer(2021,1,4))\nafficher date_semaine(date_creer(2024,12,30))\nafficher date_semaine(date_creer(2026,12,31))',"42\n1\n53\n1\n1\n53");
await t("date_age",'soit n = date_creer(2000,6,15)\nafficher date_age(n, date_creer(2026,6,14))\nafficher date_age(n, date_creer(2026,6,15))\nafficher date_age(n, date_creer(2026,10,17))\nafficher date_age(n, date_creer(2000,6,15))',"25\n26\n26\n0");
await t("date_age défaut",'afficher date_age(date_creer(1990,1,1)) >= 36',"vrai");
await t("date_age type",'afficher date_age("x")',/ERR:Ligne 1 : date_age : l'argument 1 doit être un nombre/);

// ---- maths étendues, copier, texte_format, bases (parité JS/Rust : ces cas sont aussi rejoués par le moteur Rust) ----
await t("pi",'afficher pi()',"3.14159265359");
await t("sinus",'afficher sinus(pi() / 2)',"1");
await t("sinus(pi) = presque 0",'afficher arrondir(sinus(pi()), 10)',"0");
await t("cosinus",'afficher cosinus(0)',"1");
await t("cosinus(pi)",'afficher cosinus(pi())',"-1");
await t("tangente",'afficher arrondir(tangente(pi() / 4), 10)',"1");
await t("arc_sinus",'afficher arrondir(arc_sinus(1) * 2, 10)',"3.1415926536");
await t("arc_cosinus",'afficher arc_cosinus(1)',"0");
await t("arc_tangente",'afficher arrondir(arc_tangente(1) * 4, 10)',"3.1415926536");
await t("arc_tangente2",'afficher arrondir(arc_tangente2(1, -1), 6)',"2.356194");
await t("exponentielle",'afficher exponentielle(0) + " " + arrondir(exponentielle(1), 8)',"1 2.71828183");
await t("logarithme népérien",'afficher logarithme(exponentielle(3))',"3");
await t("logarithme base 2",'afficher logarithme(1024, 2)',"10");
await t("logarithme base 10",'afficher logarithme(1000, 10)',"3");
await t("logarithme base 3",'afficher arrondir(logarithme(81, 3), 9)',"4");
await t("logarithme10",'afficher logarithme10(100000)',"5");
await t("radians/degres",'afficher radians(180) == pi()\nafficher arrondir(degres(pi() / 3), 9)',"vrai\n60");
await t("signe",'afficher signe(-4) + " " + signe(0) + " " + signe(2.5)',"-1 0 1");
await t("modulo positif",'afficher modulo(-7, 3) + " " + modulo(7, 3) + " " + modulo(-1, 5)',"2 1 4");
await t("modulo diviseur négatif",'afficher modulo(7, -3)',"-2");
await t("modulo décimal",'afficher modulo(5.5, 2)',"1.5");
await t("notation 1e7",'afficher 1e7 + " " + 2.5e-3 + " " + 4E+2 + " " + 1.5e3',"10000000 0.0025 400 1500");
await t("e sans chiffre reste un identifiant",'soit e2 = 5\nafficher e2 * 2',"10");
await t("puissance **",'afficher 2 ** 10 + " " + 2 ** 0.5 + " " + 2 ** -1 + " " + 10 ** 0',"1024 1.41421356237 0.5 1");
await t("** associatif à droite",'afficher 2 ** 3 ** 2',"512");
await t("** plus prioritaire que * et que le moins",'afficher 3 * 2 ** 2 + " " + -2 ** 2 + " " + (-2) ** 2',"12 -4 4");
await t("** division par zéro",'afficher 0 ** -1',/ERR:.*Division par zéro/);
await t("** base négative exposant décimal",'afficher (-8) ** 0.5',/ERR:.*Puissance impossible : une base négative/);
await t("** sur du texte",'afficher "a" ** 2',/ERR:.*attend deux nombres \(reçu : texte et nombre\)/);
await t("logarithme(0)",'afficher logarithme(0)',/ERR:Ligne 1 : logarithme : l'argument doit être strictement positif\./);
await t("logarithme(-1)",'afficher logarithme(-1)',/strictement positif/);
await t("logarithme base 1",'afficher logarithme(5, 1)',/la base doit être positive et différente de 1/);
await t("logarithme10(0)",'afficher logarithme10(0)',/logarithme10 : l'argument doit être strictement positif/);
await t("arc_sinus hors domaine",'afficher arc_sinus(2)',/arc_sinus : l'argument doit être entre -1 et 1/);
await t("arc_cosinus hors domaine",'afficher arc_cosinus(-1.5)',/arc_cosinus : l'argument doit être entre -1 et 1/);
await t("modulo par zéro",'afficher modulo(5, 0)',/modulo : division par zéro/);
await t("sinus texte",'afficher sinus("a")',/sinus : l'argument 1 doit être un nombre \(reçu : texte\)/);
await t("logarithme base texte",'afficher logarithme(8, "a")',/logarithme : l'argument 2 doit être un nombre/);
await t("copier liste imbriquée",'soit a = [1, [2, 3]]\nsoit b = copier(a)\najouter(lireElement(b, 1), 99)\nafficher a\nafficher b',"[1, [2, 3]]\n[1, [2, 3, 99]]");
await t("copier structure",'structure s = {nom: "Awa", notes: [1, 2]}\nsoit c = copier(s)\najouter(lireCle(c, "notes"), 3)\nmodifierCle(c, "nom", "Bob")\nafficher s\nafficher c',/^\{ nom: "Awa", notes: \[1, 2\] \}\n\{ nom: "Bob", notes: \[1, 2, 3\] \}$/);
await t("sans copier : partagé",'soit a = [1]\nsoit b = a\najouter(b, 2)\nafficher a',"[1, 2]");
await t("copier valeurs simples",'afficher copier(5) + " " + copier("x") + " " + copier(vrai)',"5 x vrai");
await t("copier liste vide",'afficher copier([])',"[]");
await t("copier structure cyclique",'structure s = {a: 1}\nmodifierCle(s, "moi", s)\nsoit c = copier(s)\nmodifierCle(c, "a", 2)\nafficher lireCle(s, "a") + " " + lireCle(c, "a")',"1 2");
await t("copier liste cyclique",'soit l = [1]\najouter(l, l)\nsoit c = copier(l)\najouter(c, 5)\nafficher taille(l) + " " + taille(c)',"2 3");
await t("copier fonction",'soit f = x => x + 1\nsoit g = copier(f)\nafficher g(1)',"2");
await t("texte_format séquentiel",'afficher texte_format("{} a {} ans", ["Awa", 30])',"Awa a 30 ans");
await t("texte_format indices",'afficher texte_format("{1}-{0}-{1}", ["a", "b"])',"b-a-b");
await t("texte_format structure",'afficher texte_format("{nom} ({age})", {nom: "Awa", age: 30})',"Awa (30)");
await t("texte_format valeur seule",'afficher texte_format("x={}", 5)',"x=5");
await t("texte_format sans valeurs",'afficher texte_format("sans trou")',"sans trou");
await t("texte_format accolades",'afficher texte_format("{{}} et {{{}}}", [7])',"{} et {7}");
await t("texte_format formate comme afficher",'afficher texte_format("{} {} {} {}", [vrai, [1, "a"], 2.50, {k: 1}])',/^vrai \[1, "a"\] 2\.5 \{ k: 1 \}$/);
await t("texte_format accents",'afficher texte_format("é{}ü", ["à"])',"éàü");
await t("texte_format valeur manquante",'afficher texte_format("{} {}", [1])',/texte_format : aucune valeur pour \{1\} \(1 valeur\(s\) fournie\(s\)\)\./);
await t("texte_format indice manquant",'afficher texte_format("{3}", [1, 2])',/aucune valeur pour \{3\} \(2 valeur\(s\)/);
await t("texte_format nom sur liste",'afficher texte_format("{nom}", [1])',/« \{nom\} » demande une structure comme 2e argument/);
await t("texte_format {} sur structure",'afficher texte_format("{}", {a: 1})',/« \{\} » demande une liste de valeurs/);
await t("texte_format clé absente",'afficher texte_format("{zz}", {a: 1})',/la clé « zz » n'existe pas dans la structure/);
await t("texte_format } isolée",'afficher texte_format("a } b", [])',/accolade « \} » isolée/);
await t("texte_format { non fermée",'afficher texte_format("a { b", [])',/accolade « \{ » non fermée/);
await t("texte_format modèle non texte",'afficher texte_format(5, [])',/texte_format : l'argument 1 doit être un texte/);
await t("en_base hexa",'afficher en_base(255, 16)',"ff");
await t("en_base binaire",'afficher en_base(10, 2)',"1010");
await t("en_base négatif",'afficher en_base(-10, 2)',"-1010");
await t("en_base zéro",'afficher en_base(0, 36)',"0");
await t("en_base 36",'afficher en_base(35, 36) + en_base(36, 36)',"z10");
await t("en_base grand",'afficher en_base(9007199254740991, 16)',"1fffffffffffff");
await t("depuis_base hexa",'afficher depuis_base("ff", 16) + " " + depuis_base("FF", 16)',"255 255");
await t("depuis_base binaire",'afficher depuis_base("1010", 2)',"10");
await t("depuis_base négatif",'afficher depuis_base("-1010", 2)',"-10");
await t("depuis_base -0",'afficher depuis_base("-0", 10)',"0");
await t("aller-retour bases",'soit ok = vrai\npour b allantde 2 a 36 faire\n  si depuis_base(en_base(123456789, b), b) != 123456789 alors\n    ok = faux\n  fin\nfin\nafficher ok',"vrai");
await t("depuis_base chiffre trop grand",'afficher depuis_base("12", 2)',/depuis_base : « 12 » n'est pas un nombre valide en base 2\./);
await t("depuis_base vide",'afficher depuis_base("", 10)',/n'est pas un nombre valide en base 10/);
await t("depuis_base tiret seul",'afficher depuis_base("-", 10)',/n'est pas un nombre valide/);
await t("depuis_base caractère accentué",'afficher depuis_base("é", 36)',/n'est pas un nombre valide/);
await t("depuis_base trop grande",'afficher depuis_base("zzzzzzzzzzzzzzzz", 36)',/valeur trop grande/);
await t("en_base non entier",'afficher en_base(1.5, 2)',/en_base : le nombre doit être un entier/);
await t("en_base base invalide",'afficher en_base(5, 40)',/en_base : la base doit être un entier de 2 à 36/);
await t("en_base base 1",'afficher en_base(5, 1)',/la base doit être un entier de 2 à 36/);
await t("depuis_base base invalide",'afficher depuis_base("1", 37)',/depuis_base : la base doit être un entier de 2 à 36/);
await t("suggestion pour nom proche",'afficher sinnus(1)',/Fonction "sinnus" inconnue/);

await t("classe: constructeur et méthode",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nsoit p = Point(3, 4)\nafficher p.norme()',"5");
await t("classe: affichage d'une instance",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nafficher Point(3, 4)',"Point { x: 3, y: 4 }");
await t("classe: écriture et lecture de champ",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nsoit p = Point(3, 4)\np.x = 10\nafficher p.x\nafficher p',"10\nPoint { x: 10, y: 4 }");
await t("classe: instance = structure (taille, cles)",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nsoit p = Point(3, 4)\nafficher taille(p)\nafficher cles(p)',"2\n[\"x\", \"y\"]");
await t("classe: sans constructeur",'classe Vide\n fonction dire()\n  renvoyer 42\n fin\nfin\nsoit v = Vide()\nafficher v.dire()\nafficher v',"42\nVide {  }");
await t("classe: classe_de",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nafficher classe_de(Point(1, 2))\nafficher classe_de({a: 1})\nafficher classe_de(5)',"Point\nrien\nrien");
await t("classe: égalité (même classe, mêmes champs)",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nafficher Point(1, 2) == Point(1, 2)\nafficher Point(1, 2) == Point(1, 3)\nafficher Point(1, 2) == {x: 1, y: 2}',"vrai\nfaux\nfaux");
await t("classe: copier conserve la classe",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nsoit p = Point(1, 2)\nsoit q = copier(p)\nq.x = 9\nafficher p\nafficher q',"Point { x: 1, y: 2 }\nPoint { x: 9, y: 2 }");
await t("classe: les instances sont des références",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nsoit p = Point(1, 2)\nsoit q = p\nq.x = 7\nafficher p.x',"7");
await t("classe: méthode appelle une autre méthode",'classe A\n fonction creer(n)\n  soi.n = n\n fin\n fonction nom()\n  renvoyer "A" + soi.n\n fin\n fonction salut()\n  renvoyer "salut " + soi.nom()\n fin\nfin\nafficher A(1).salut()',"salut A1");
await t("classe: héritage et redéfinition",'classe A\n fonction creer(n)\n  soi.n = n\n fin\n fonction nom()\n  renvoyer "A" + soi.n\n fin\n fonction salut()\n  renvoyer "salut " + soi.nom()\n fin\nfin\nclasse B herite A\n fonction nom()\n  renvoyer "B" + soi.n\n fin\nfin\nsoit b = B()\nb.n = 5\nsoit a = A(1)\nafficher b.salut()\nafficher a.salut()',"salut B5\nsalut A1");
await t("classe: appel explicite du parent",'classe A\n fonction creer(n)\n  soi.n = n\n fin\nfin\nclasse B herite A\n fonction creer(n, m)\n  A.creer(soi, n)\n  soi.m = m\n fin\nfin\nafficher B(1, 2)',"B { n: 1, m: 2 }");
await t("classe: est_instance",'classe A\nfin\nclasse B herite A\nfin\nsoit b = B()\nafficher est_instance(b, "A")\nafficher est_instance(b, "B")\nafficher est_instance(A(), "B")\nafficher est_instance(5, "A")',"vrai\nvrai\nfaux\nfaux");
await t("classe: champ fonction (lambda) appelé",'soit s = {g: x => x + 1}\nafficher s.g(4)',"5");
await t("classe: champ pointé sur structure simple",'soit s = {a: 1}\nafficher s.a\ns.b = 2\nafficher s',"1\n{ a: 1, b: 2 }");
await t("classe: champs imbriqués",'soit s = {a: {b: 1}}\nafficher s.a.b\ns.a.b = 5\nafficher s.a.b',"1\n5");
await t("classe: champ inexistant",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nsoit p = Point(1, 2)\nafficher p.z',/^ERR:Ligne 11 : Le champ "z" n'existe pas dans "p" \(champs disponibles : x, y\)\.$/);
await t("classe: structure vide, champ absent",'soit s = {}\nafficher s.x',/Le champ "x" n'existe pas dans "s" \(aucun champ\)\./);
await t("classe: lecture sur un nombre",'soit n = 5\nafficher n.x',/Impossible de lire le champ "x" : "n" est de type nombre \(une structure ou une instance est attendue\)\./);
await t("classe: écriture sur un nombre",'soit n = 5\nn.x = 1',/Impossible d'écrire le champ "x" : "n" est de type nombre/);
await t("classe: méthode inconnue",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nPoint(1, 2).m',/./);
await t("classe: méthode inconnue (appel)",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nsoit p = Point(1, 2)\np.m()',/La méthode "m" n'existe pas dans la classe "Point"\./);
await t("classe: mauvais nombre d'arguments",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nsoit p = Point(1, 2)\np.norme(1)',/La méthode "norme" de la classe "Point" attend 0 argument\(s\), 1 reçu\(s\)\./);
await t("classe: constructeur, mauvais nombre d'arguments",'classe Point\n fonction creer(x, y)\n  soi.x = x\n  soi.y = y\n fin\n fonction norme()\n  renvoyer racine(soi.x ** 2 + soi.y ** 2)\n fin\nfin\nPoint(1)',/La fonction "Point" attend 2 argument\(s\), 1 reçu\(s\)\./);
await t("classe: appel sur un non-objet",'soit n = 3\nn.m()',/Impossible d'appeler "m" : "n" est de type nombre \(une instance est attendue\)\./);
await t("classe: champ non fonction",'soit s = {a: 1}\ns.a()',/Le champ "a" de "s" n'est pas une fonction \(reçu : nombre\)\./);
await t("classe: variable inexistante pointée",'afficher zz.x',/La variable "zz.x" n'existe pas/);
await t("classe: seules des méthodes",'classe C\n soit x = 1\nfin',/^ERR:Ligne 2 : Dans une classe, seules les méthodes \(fonction \.\.\. fin\) sont permises \(reçu : "soit"\)\.$/);
await t("classe: soi réservé",'classe C\n fonction f(soi)\n fin\nfin',/"soi" est réservé dans une méthode/);
await t("classe: méthode en double",'classe C\n fonction f()\n fin\n fonction f()\n fin\nfin',/^ERR:Ligne 4 : La méthode "f" est déjà définie dans la classe "C"\.$/);
await t("classe: bloc non fermé",'classe C\n fonction f()\n fin',/^ERR:Ligne 1 : Le bloc "classe C" ouvert ligne 1 n'est jamais fermé/);
await t("classe: contextuel (classe et tester restent des noms libres)",'soit classe = 4\nsoit tester = 5\nafficher classe + tester',"9");
await t("classe: déclarée dans une fonction",'fonction f()\n classe L\n  fonction creer()\n   soi.v = 1\n  fin\n fin\n renvoyer L()\nfin\nafficher f()',"L { v: 1 }");
await t("classe: dans un module avec alias",'importer "cl_mod" comme m\nsoit z = m.Pt(4)\nafficher z\nafficher z.double()\nafficher m.origine.x\nsoit f = m.fabrique(7)\nafficher f.double()\nm.origine.x = 9\nafficher m.origine.double()',"m.Pt { x: 4 }\n8\n0\n14\n18");
await t("classe: module sans alias",'importer "cl_mod"\nafficher origine\nafficher Pt(3).double()',"Pt { x: 0 }\n6");
await t("enchaînement: méthode sur un appel",'classe C\n fonction creer(s)\n  soi.s = s\n fin\n fonction plus(m)\n  soi.s = soi.s + m\n  renvoyer soi\n fin\n fonction val()\n  renvoyer soi.s\n fin\nfin\nafficher C(1).plus(2).plus(3).val()\nafficher C(1).plus(1).s\nC(5).plus(1)',"6\n2");
await t("enchaînement: champs après un appel",'fonction f()\n renvoyer {a: {b: 7}}\nfin\nafficher f().a.b\nafficher lireElement([f()], 0).a.b',"7\n7");
await t("enchaînement: erreurs",'fonction f()\n renvoyer {a: 1}\nfin\nessayer afficher f().z attraper e afficher e fin\nessayer afficher f().a.b attraper e afficher e fin\nessayer f().a() attraper e afficher e fin\nafficher (1 + 2).x',/^Le champ "z" n'existe pas dans "\(expression\)" \(champs disponibles : a\)\.\nImpossible de lire le champ "b" : "\(expression\)" est de type nombre/);
await t("classe: erreur dans une méthode (pile)",'classe C\n fonction f()\n  renvoyer 1 / 0\n fin\nfin\nsoit c = C()\nc.f()',/^ERR:Ligne 3 : Division par zéro\.$/);
await tt("tester: rapport ✓ ✗ et total",'fonction f(x)\n renvoyer x + 1\nfin\ntester "un"\n verifier_egal(f(1), 2)\nfin\ntester "deux"\n verifier(f(1) == 3, "f(1) devrait valoir 3")\nfin\ntester "trois"\n verifier_egal(f(1), 3)\nfin\ntester "quatre"\n verifier_erreur(() => 1 / 0, "zéro")\n verifier_erreur(() => 1, "x")\nfin',"✓ un\n✗ deux\n    Ligne 8 : Échec du test : f(1) devrait valoir 3.\n✗ trois\n    Ligne 11 : Échec du test : obtenu 2, attendu 3.\n✗ quatre\n    Ligne 15 : Échec du test : aucune erreur levée.\n4 test(s) : 1 réussi(s), 3 échoué(s).");
await tt("tester: aucun test",'afficher 1',"1\nAucun test (écrivez : tester \"nom\" ... fin).");
await t("tester: ignoré par une exécution normale",'tester "x"\n afficher 1\nfin\nafficher 2',"2");
await tt("tester: classes + verifier_erreur sur méthode",'classe Compte\n fonction creer(s)\n  soi.solde = s\n fin\n fonction retirer(m)\n  si m > soi.solde alors\n   soit x = 1 / 0\n  fin\n  soi.solde = soi.solde - m\n fin\nfin\ntester "retrait"\n soit c = Compte(100)\n c.retirer(30)\n verifier_egal(c.solde, 70)\nfin\ntester "trop"\n soit c = Compte(10)\n verifier_erreur(() => c.retirer(50), "zéro")\nfin',"✓ retrait\n✓ trop\n2 test(s) : 2 réussi(s), 0 échoué(s).");
await tt("tester: les variables d'un test sont locales",'tester "a"\n soit x = 1\nfin\ntester "b"\n verifier_erreur(() => x, "n\'existe pas")\nfin',"✓ a\n✓ b\n2 test(s) : 2 réussi(s), 0 échoué(s).");
await tt("tester: verifier avec texte et valeurs formatées",'tester "t"\n verifier_egal("a", "b")\nfin\ntester "l"\n verifier_egal([1, "x"], [1, "y"], "listes")\nfin',"✗ t\n    Ligne 2 : Échec du test : obtenu \"a\", attendu \"b\".\n✗ l\n    Ligne 5 : Échec du test (listes) : obtenu [1, \"x\"], attendu [1, \"y\"].\n2 test(s) : 0 réussi(s), 2 échoué(s).");
await tt("tester: verifier_erreur, texte absent du message",'tester "e"\n verifier_erreur(() => 1 / 0, "autre")\nfin',"✗ e\n    Ligne 2 : Échec du test : l'erreur « Division par zéro. » ne contient pas « autre ».\n1 test(s) : 0 réussi(s), 1 échoué(s).");
await t("tester: bloc non fermé",'tester "a"\n afficher 1',/^ERR:Ligne 1 : Le bloc "tester" ouvert ligne 1 n'est jamais fermé/);
}

async function shell() {
  groupe = "shell";
  const { FadFS, Shell } = S;
  FadFS.init();
  const ctx = { cwd: FadFS.HOME, env: { HOME: FadFS.HOME, USER: "fadera" }, historique: [], formater: (src, n) => { try { return { ok: true, texte: M.formaterSource(src, n) }; } catch (e) { return { ok: false, texte: M.decrireErreur(e, src).texte }; } }, vars: (n) => (n === "x" ? 42 : undefined),
    ouvrirEditeur: async (p) => ({ out: "ouvert " + p + "\n" }), lancer: async () => 0, aRenomme: () => {}, executerSilencieux: async () => ({ ok: true }),
    testerBlocs: async (p) => { const code = FadFS.lire(p); if (!/^[ \t]*tester[ \t]+["']/m.test(code)) return null; const l = []; const b = await M.lancerTests(code, { afficher: (x) => l.push(x) }); return { ok: b.echoues === 0, total: b.total, reussis: b.reussis, lignes: l }; } };
  async function sh(l) {
    const o = [];
    const r = await Shell.executer(l, ctx, (x) => o.push(x.err ? "E:" + x.err : x.html !== undefined ? "H:" + x.html : x.out));
    return r ? o.join("\n") : "<<ancien>>";
  }
  async function t(l, att) {
    const r = await sh(l);
    const bon = att instanceof RegExp ? att.test(r) : r === att;
    if (bon) ok++; else { ko++; console.log("✗ [shell] " + l + "\n    obtenu : " + JSON.stringify(r) + "\n    attendu: " + String(att)); }
  }
await t('pwd','/home/fadera');
await t("echo 'soit  x=1+2' > fm.fa",'');
await t('fmt fm.fa','soit x = 1 + 2');
await t('fmt --verifier fm.fa','fm.fa');
await t('fmt -w fm.fa','E:formaté : fm.fa');
await t('fmt --verifier fm.fa','');
await t('cat fm.fa','soit x = 1 + 2');
await t('fadera fmt fm.fa','soit x = 1 + 2');
await t("echo 'soit  = 1' > fmbad.fa",'');
await t('fmt fmbad.fa',/^E:fmbad\.fa : Ligne 1 : /);
await t('echo "si  vrai alors\nafficher 1\nfin" | fmt','si vrai alors\n    afficher 1\nfin');
await t('rm fm.fa fmbad.fa','');
await t('variables',/aucune variable/);
await t('variables zzz','E:variables : \'zzz\' : variable inconnue\n');
await t('mkdir projets','');
await t('mkdir projets','E:mkdir : impossible de créer le répertoire \'projets\' : Le fichier existe');
await t('echo "Bonjour Fadéra" > projets/a.txt','');
await t('cat projets/a.txt','Bonjour Fadéra');
await t('echo ligne2 >> projets/a.txt && cat projets/a.txt','Bonjour Fadéra\nligne2');
await t('cd projets','');
await t('pwd','/home/fadera/projets');
await t('ls',/^H:a\.txt$/);
await t('ls -l',/^H:total \d+\n-rw-r--r-- 1 fadera fadera +\d+ .* a\.txt$/);
await t('cp a.txt b.txt','');
await t('ls | wc -l',/^ *2$/);
await t('grep -n ligne a.txt','2:ligne2');
await t('grep Bonjour *.txt','a.txt:Bonjour Fadéra\nb.txt:Bonjour Fadéra');
await t('cat a.txt | grep -i bonjour | wc -w',/^ *2$/);
await t('mv b.txt c.txt','');
await t('ls | sort -r',/^c\.txt\na\.txt$/);
await t('head -n 1 a.txt','Bonjour Fadéra');
await t('tail -1 a.txt','ligne2');
await t('cat nope','E:cat : nope : Aucun fichier ou dossier de ce type');
await t('rm projets','E:rm : impossible de supprimer \'projets\' : Aucun fichier ou dossier de ce type');
await t('cd ..','');
await t('rm projets','E:rm : impossible de supprimer \'projets\' : est un dossier (utilisez -r)');
await t('tree',/projets\n.*a\.txt/s);
await t('find . -name "*.txt"','./projets/a.txt\n./projets/c.txt');
await t('echo $x $HOME','42 /home/fadera');
await t("echo '$x'",'$x');
await t('chmod +x projets/a.txt && ls -l projets',/-rwxr-xr-x/);
await t('chmod 600 projets/a.txt && stat projets/a.txt',/0600\/-rw-------/);
await t('mkdir -p a/b/c && cd a/b/c && pwd','/home/fadera/a/b/c');
await t('cd ~ ; pwd','/home/fadera');
await t('sed \'s/Bonjour/Salut/\' projets/a.txt','Salut Fadéra\nligne2');
await t('creer_dossier(docs)','');
await t('deplacer_fichier(projets/a.txt, docs)','');
await t('lister_dossier(docs)',/^H:a\.txt$/);
await t('rm -r docs projets a','');
await t('ls','');
await t('nano script.fa','ouvert /home/fadera/script.fa');
await t('fadera script.fa','');
await t('./script.fa','');
await t('mkdir -p maths && printf "fonction carre(x)\\n renvoyer x * x\\nfin\\n" > maths/outils.fa && echo \'{"description":"mini maths","version":"2.1.0"}\' > maths/paquet.json && pkg empaqueter maths',/1 fichier\(s\) empaqueté\(s\) → ~\/maths\.fapkg/);
await t('rm -r maths && pkg install maths.fapkg',/paquet maths@2\.1\.0 → lib\/maths\/  \(utilisation : importer "lib\/maths\/outils"\)/);
await t('cat lib/maths/outils.fa | head -n 1','fonction carre(x)');
await t('pkg list',/PAQUETS FADÉRA \(~\/lib\) : maths@2\.1\.0/);
await t('echo \'{"format":"fapkg/1","nom":"x","fichiers":{"../evil":"1"}}\' > mal.fapkg && pkg install mal.fapkg',/^E:pkg : chemin refusé/);
await t('echo \'pas du json\' > b.fapkg && pkg install b.fapkg',/^E:pkg : paquet illisible/);
await t('pkg remove maths && ls lib','✓ paquet « maths » supprimé');
await t('echo "a" > /nope/x','E:fadera : /nope/x : Aucun fichier ou dossier de ce type');
await t('ping google.com',/^E:ping : non disponible ici/);
await t('foo | cat',/^E:foo : commande introuvable/);
// ===== extension : commandes Linux supplémentaires =====
await t('printf "b\\na\\nc\\n" > l.txt && tac l.txt','c\na\nb');
await t('rev l.txt','b\na\nc');
await t('nl l.txt',/^ +1\tb\n +2\ta\n +3\tc$/);
await t('echo "a:b:c" | cut -d: -f2,3','b:c');
await t('echo "abcdef" | cut -c2-4','bcd');
await t('echo hello | tr a-z A-Z','HELLO');
await t('echo "aabbcc" | tr -d b','aacc');
await t('echo "un deux trois" | awk \'{print $2, $NF}\'','deux trois');
await t('printf "1 a\\n2 b\\n3 c\\n" | awk \'{s += $1} END {print s}\'','6');
await t('printf "x:1\\ny:2\\n" | awk -F: \'/y/ {print $2}\'','2');
await t('echo abc | md5sum','900150983cd24fb0d6963f7d28e17f72  -'.replace('900150983cd24fb0d6963f7d28e17f72','0bee89b07a248e27c83fc3d5951213c1'));
await t('printf abc > h.txt && md5sum h.txt','900150983cd24fb0d6963f7d28e17f72  h.txt');
await t('sha256sum h.txt','ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  h.txt');
await t('sha1sum h.txt','a9993e364706816aba3e25717850c26c9cd0d89d  h.txt');
await t('sha256sum h.txt > sums && sha256sum -c sums','h.txt: OK');
await t('echo salut | base64','c2FsdXQK');
await t('echo c2FsdXQK | base64 -d','salut');
await t('printf hello > x.bin && xxd x.bin',/^00000000: 6865 6c6c 6f +hello$/);
await t('printf "a\\nb\\n" > f1 && printf "a\\nc\\n" > f2 && diff f1 f2','2c2\n< b\n---\n> c');
await t('diff -u f1 f2',/^--- f1\n\+\+\+ f2\n@@ .* @@\n a\n-b\n\+c$/);
await t('cmp f1 f2',/différents : octet 3/);
await t('seq 3','1\n2\n3');
await t('seq 2 2 8','2\n4\n6\n8');
await t('echo $((2+3*4))','14');
await t('x=5 ; echo $x','5');
await t('export NOM=Fadera && echo "Salut $NOM"','Salut Fadera');
await t('echo $(echo imbriqué)','imbriqué');
await t('echo "n=$(seq 3 | wc -l)"',/^n= *3$/);
await t('false || echo secours','secours');
await t('true || echo non','');
await t('false ; echo $?','1');
await t('test -f l.txt && echo oui','oui');
await t('[ 3 -gt 2 ] && echo plus','plus');
await t('[ -d nope ] || echo absent','absent');
await t('basename /a/b/c.txt .txt','c');
await t('dirname /a/b/c.txt','/a/b');
await t('echo "3 4" | xargs echo somme','somme 3 4');
await t('echo hi | tee t1.txt | wc -c',/^ *3$/);
await t('cat t1.txt','hi');
await t('bc "2^10"'.replace('bc "2^10"','echo "2^10" | bc'),'1024');
await t('expr 7 + 3','10');
await t('factor 84','84: 2 2 3 7');
await t('printf "%05.1f|%-4s|%d\\n" 3.14159 ab 42','003.1|ab  |42');
await t('alias ll=\'ls -l\' && alias',/alias ll='ls -l'/);
await t('ll',/^H:total/);
await t('unalias ll && ll','E:ll : commande introuvable (tapez « aide » ou « pkg list »)');
await t('type ls cd',/ls est une commande intégrée.*\ncd est une commande intégrée/);
await t('file l.txt','l.txt: ASCII text');
await t('echo "afficher 1" > s.fa && file s.fa','s.fa: Fadéra source, UTF-8 Unicode text');
await t('du -s .',/^\d+\t\.$/);
await t('whoami','fadera');
await t('uname','FaderaOS');
await t('date +%Y',/^20\d\d$/);
await t('sudo echo ok','ok');
await t('echo "a b" > /dev/null','');
await t('nmap 10.0.0.1',/^E:nmap : non disponible ici/);
// scripts shell
await t('printf \'for i in 1 2 3; do\\necho "n$i"\\ndone\\n\' > b.sh && sh b.sh','n1\nn2\nn3');
await t('for f in f1 f2; do echo fichier-$f; done','fichier-f1\nfichier-f2');
await t('if test -f f1; then echo trouvé; else echo absent; fi','trouvé');
await t('if test -f zzz; then echo A; elif test -f f2; then echo B; else echo C; fi','B');
await t('printf \'echo "arg1=$1 n=$#"\\n\' > a.sh && sh a.sh x y','arg1=x n=2');
await t('i=0 ; while [ $i -lt 3 ]; do echo i$i; i=$((i+1)); done','i0\ni1\ni2');
await t('for n in $(seq 3); do echo "n$n"; done','n1\nn2\nn3');
await t('sh -c "echo dans-sh"','dans-sh');
await t('echo \'echo salut-$1\' > mac.sh && alias dis=\'sh mac.sh\' && dis Seny','salut-Seny');
// packs
await t('pkg list',/\[ \] securite[\s\S]*\[ \] reseau[\s\S]*\[ \] dev/);
await t('hashid abc','<<ancien>>');
await t('pkg install securite',/pack « securite » installé/);
await t('hashid 5d41402abc4b2a76b9719d911017c592',/MD5/);
await t('hashid \'$2b$12$abcdefghijklmnopqrstuuABCDEFGHIJKLMNOPQRSTUVWXYZ01234\'',/bcrypt/);
await t('pwcheck password',/TRÈS FAIBLE/);
await t('pwcheck "Xk9$mQ2!vL7#pR4zT"',/EXCELLENT|BON/);
await t('pwgen -n 16 | wc -c',/^ *17$/);
await t('echo aaaaaaaa | entropie',/Entropie de Shannon : 0\.000/);
await t('echo "password=hunter2abc" > conf.txt && scansecrets conf.txt',/mot de passe en dur[\s\S]*1 secret/);
await t('chmod 777 conf.txt && audit .',/modifiable par tous/);
await t('mkdir wd && echo A > wd/a && integrite init wd',/Référence enregistrée : 1/);
await t('integrite check wd',/Intégrité vérifiée/);
await t('echo B > wd/b && echo Z > wd/a && integrite check wd',/MODIFIÉ .*wd\/a\nNOUVEAU .*wd\/b|NOUVEAU .*wd\/b\nMODIFIÉ/);
await t('printf "Failed password for root from 1.2.3.4 port 1\\nFailed password for invalid user bob from 1.2.3.4 port 2\\nAccepted password for ana\\n" > auth.log && logscan auth.log',/Connexions réussies : 1\nÉchecs d'authentification : 2[\s\S]*1\.2\.3\.4/);
await t('echo Bonjour | rot13','Obawbhe');
await t('urlencode "a b&c"','a%20b%26c');
await t('echo secret | hmac -k clé',/^[0-9a-f]{64}$/);
await t('portinfo 22',/SSH/);
await t('pkg remove securite && hashid x',/retiré\nE:hashid : commande introuvable/);
await t('pkg install reseau && ipcalc 192.168.1.77/26',/Réseau     : 192\.168\.1\.64\/26\nBroadcast  : 192\.168\.1\.127[\s\S]*Hôtes      : 62/);
await t('curl http://127.0.0.1:1/',/^E:curl : \(6\)/);
await t('pkg install dev && echo \'{"a":{"b":[10,20]}}\' | jq -c .a.b[1]',/\n20$/);
await t('echo \'{"commandes":{"salut":"echo Salut $1"}}\' > mon.json && pkg install mon.json && salut Fadera',/importé depuis mon\.json : salut\nSalut Fadera$/);
await t('echo "oups','E:fadera : guillemet non fermé');
  FadFS.ecrire(FadFS.HOME + "/blocs_ok.fa", 'fonction f(x)\n renvoyer x + 1\nfin\ntester "a"\n verifier_egal(f(1), 2)\nfin\ntester "b"\n verifier(f(2) == 3)\nfin', false);
  FadFS.ecrire(FadFS.HOME + "/blocs_ko.fa", 'tester "a"\n verifier_egal(1, 1)\nfin\ntester "faux"\n verifier_egal(1, 2)\nfin', false);
  await t('tester blocs_ok.fa',/^✓ blocs_ok\.fa \(2 test\(s\)\)\n\n1 réussi\(s\), 0 échoué\(s\) sur 1 fichier\(s\)\.$/);
  await t('tester blocs_ko.fa',/^✗ blocs_ko\.fa\n  ✗ faux\n      Ligne 5 : Échec du test : obtenu 1, attendu 2\.\n\n0 réussi\(s\), 1 échoué\(s\) sur 1 fichier\(s\)\.$/);
}

// ---- erreurs détaillées : colonne, extrait, suggestion, pile d'appels -----------------------------------
async function det(code) {
  const hote = { afficher: () => {}, saisir: () => "", lireFichier: (p) => { if (p in FICH) return FICH[p]; throw new Error("absent"); }, ecrireFichier: () => {} };
  try { await M.executerProgramme(code, hote); } catch (e) { return M.detaillerErreur(M.decrireErreur(e, code).detail, code); }
  return null;
}
async function te(nom, code, verif) {
  const d = await det(code);
  let bon = false;
  try { bon = !!d && verif(d); } catch (_) { bon = false; }
  if (bon) ok++; else { ko++; console.log("✗ [erreurs] " + nom + "\n    obtenu : " + JSON.stringify(d)); }
}
async function erreurs() {
  await te("variable mal écrite : colonne + suggestion", 'soit total = 5\nafficher totl', (d) => d.ligne === 2 && d.colonne === 10 && d.longueur === 4 && d.aide[0] === "Tu voulais écrire « total » ?");
  await te("variable déclarée plus bas", 'afficher x\nsoit x = 1', (d) => d.ligne === 1 && /ligne 2/.test(d.aide[0]));
  await te("fonction mal écrite", 'fonction double(x)\n renvoyer x*2\nfin\nafficher doubel(2)', (d) => d.ligne === 4 && d.colonne === 10 && d.aide[0] === "Tu voulais écrire « double » ?");
  await te("fonction native mal écrite", 'afficher arondir(2.6)', (d) => d.aide[0] === "Tu voulais écrire « arrondir » ?");
  await te("instruction mal écrite", 'afichier 3', (d) => d.colonne === 1 && d.longueur === 8 && d.aide[0] === "Tu voulais écrire « afficher » ?");
  await te("accents ignorés pour la suggestion", 'soit élève = 1\nafficher eleve2', (d) => d.aide[0] === "Tu voulais écrire « élève » ?");
  await te("= au lieu de ==", 'soit a = 1\nsi a = 1 alors\n afficher "oui"\nfin', (d) => d.ligne === 2 && /==/.test(d.aide[0]));
  await te("caractère inattendu : colonne", 'afficher 3 @ 4', (d) => d.colonne === 12 && d.longueur === 1);
  await te("mot présent dans un texte ignoré", 'afficher "totl"\nafficher totl', (d) => d.ligne === 2 && d.colonne === 10);
  await te("aucune suggestion si rien de proche", 'soit a = 1\nafficher zzzzzz', (d) => d.aide.length === 0 && d.colonne === 10);
  await te("pile d'appels", 'fonction a(n)\n renvoyer b(n)\nfin\nfonction b(n)\n renvoyer lireElement([1],n)\nfin\nafficher a(5)', (d) => d.pile.join("|") === "dans b (ligne 5)|appelée depuis a (ligne 2)|appelée depuis le programme (ligne 7)");
  await te("pile regroupée pour la récursion", 'fonction g(n)\n renvoyer g(n+1) + 1\nfin\nafficher g(1)', (d) => /^dans g \(ligne 2\) ×\d+$/.test(d.pile[0]) && d.pile.some((l) => /64 appels/.test(l)));
  await te("erreur dans un module : ligne du script, pile avec le module", 'importer "std/listes"\nafficher premier([])', (d) => d.ligne === 2 && d.pile.join("|") === "dans premier [std/listes.fa] (ligne 9)|appelée depuis le programme (ligne 2)" && d.aide.length === 0);
  await te("pas de pile au niveau du programme", 'afficher 1/0', (d) => d.pile.length === 0);
}


// ---- débogueur pas à pas : pas, points d'arrêt, état, arrêt --------------------------------------------
// plan : modes de reprise successifs (« continuer » quand il est épuisé) ; options.points : lignes ; options.mode : mode initial ;
// options.sur(d, l, instantane) : action personnalisée à chaque pause (à la place de la reprise automatique)
async function deb(code, plan, options) {
  options = options || {};
  const log = [], out = [];
  const hote = { afficher: (t) => out.push(t), saisir: () => "", lireFichier: (p) => { if (p in FICH) return FICH[p]; throw new Error("absent"); }, ecrireFichier: () => {} };
  const it = new M.Interpreteur(hote);
  const d = new M.Debogueur({
    pause: (it2, l, r) => {
      const snap = M.Debogueur.instantane(it2, l);
      log.push({ l, r, prof: it2.profondeur, snap });
      if (options.sur) options.sur(d, l, snap);
      else d.reprendre(plan.length ? plan.shift() : "continuer");
    },
  });
  (options.points || []).forEach((p) => d.points.add(p));
  if (options.mode) d.mode = options.mode;
  it.dbg = d;
  let err = null;
  try { await it.executer(M.analyserCode(code)); } catch (e) { err = e; }
  return { log, out, err, lignes: log.map((x) => x.l), it, d };
}
async function td(nom, code, plan, options, verif) {
  const r = await deb(code, plan, options);
  let bon = false;
  try { bon = !!verif(r); } catch (_) { bon = false; }
  if (bon) ok++; else { ko++; console.log("✗ [débogueur] " + nom + "\n    pauses : " + JSON.stringify(r.lignes) + "  erreur : " + (r.err && r.err.message) + "  sortie : " + JSON.stringify(r.out)); }
}
const FN = 'fonction f(x)\n  soit y = x * 2\n  renvoyer y\nfin\nsoit a = f(3)\nafficher a';
async function debogueur() {
  await td("pas à pas : une pause par instruction", 'soit a = 1\nsoit b = 2\nafficher a + b', ["entrer", "entrer"], null, (r) => r.lignes.join() === "1,2,3" && r.out.join() === "3" && !r.err);
  await td("passer : le corps de la fonction est sauté", FN, ["passer", "passer"], null, (r) => r.lignes.join() === "5,6" && r.out.join() === "6");
  await td("entrer : on descend dans la fonction", FN, ["entrer", "entrer", "entrer"], null, (r) => r.lignes.join() === "5,2,3,6");
  await td("sortir : on termine la fonction", FN, ["entrer", "sortir"], null, (r) => r.lignes.join() === "5,2,6");
  await td("passer dans la fonction : retour à l'appelant", FN, ["entrer", "passer", "passer"], null, (r) => r.lignes.join() === "5,2,3,6");
  await td("continuer : plus aucune pause", FN, ["continuer"], null, (r) => r.lignes.join() === "5" && r.out.join() === "6");
  await td("point d'arrêt dans une boucle : une pause par tour", 'soit s = 0\npour i allantde 1 a 3 faire\n  s = s + i\nfin\nafficher s', [], { mode: "continuer", points: [3] }, (r) => r.lignes.join() === "3,3,3" && r.log.map((x) => x.snap.globales.find((v) => v.nom === "s").valeur).join() === "0,1,3" && r.out.join() === "6");
  await td("raison de la pause", FN, ["continuer"], { mode: "continuer", points: [6] }, (r) => r.log.length === 1 && r.log[0].r === "point");
  await td("point d'arrêt dans une fonction", FN, [], { mode: "continuer", points: [3] }, (r) => r.lignes.join() === "3");
  await td("état : variables locales et pile d'appels", FN, ["entrer", "entrer"], null, (r) => {
    const s = r.log[1].snap; // pause à la ligne 2, dans f
    return s.locales.some((v) => v.nom === "x" && v.valeur === 3) && s.pile.map((p) => p.nom + ":" + p.ligne).join() === "f:2,programme:5" && s.globales.length === 0;
  });
  await td("état : locales visibles après une affectation", FN, ["entrer", "entrer", "entrer"], null, (r) => {
    const s = r.log[2].snap; // pause à la ligne 3
    return s.locales.some((v) => v.nom === "y" && v.valeur === 6);
  });
  await td("récursion : la pile grandit", 'fonction g(n)\n  si n > 0 alors\n    g(n - 1)\n  fin\nfin\ng(2)', [], { mode: "continuer", points: [2] }, (r) => r.log.length === 3 && r.log[2].snap.pile.length === 4 && r.log[2].snap.pile[0].nom === "g");
  await td("arrêt pendant une pause", 'soit a = 1\nsoit b = 2\nafficher "fin"', [], { sur: (d) => d.arreter() }, (r) => r.err instanceof M.SignalStop && r.out.length === 0);
  await td("arrêt non rattrapé par essayer", 'essayer\n  soit a = 1\n  soit b = 2\nattraper e\n  afficher "pris"\nfin\nafficher "fin"', [], { sur: (d, l) => { if (l === 3) d.arreter(); else d.reprendre("entrer"); } }, (r) => r.err instanceof M.SignalStop && r.out.length === 0);
  await td("arrêt à travers une fonction native", 'fonction d(x)\n  renvoyer x * 2\nfin\nsoit r = transformer([1, 2, 3], d)\nafficher r', [], { sur: (d, l) => { if (l === 2) d.arreter(); else d.reprendre("entrer"); } }, (r) => r.err instanceof M.SignalStop);
  await td("arrêt demandé hors pause (programme qui tourne)", 'soit s = 0\npour i allantde 1 a 100000 faire\n  s = s + 1\nfin', [], { mode: "continuer" , points: [1], sur: (d) => { d.arreter(); d.reprendre("continuer"); } }, (r) => r.err instanceof M.SignalStop);
  await td("module importé : aucune pause dans son code", 'importer "outils"\nafficher double(2)', ["entrer", "entrer"], null, (r) => r.lignes.join() === "1,2" && r.out.join() === "4");
  await td("pas de pause sur la déclaration d'une fonction", 'fonction f()\n  renvoyer 1\nfin\nafficher f()', ["entrer", "entrer"], null, (r) => r.lignes.join() === "4,2");
  await td("sans débogueur rien ne change", 'afficher 1 + 1', [], { mode: "continuer" }, (r) => r.lignes.length === 0 && r.out.join() === "2");
  { // résumé des valeurs et évaluation en pause (partagés avec le moteur Rust : voir tests/parite_debogueur.js)
    const R = (v, att, nom) => { if (M.Debogueur.resumer(v) === att) ok++; else { ko++; console.log("✗ [débogueur] résumé " + nom + "\n    obtenu : " + JSON.stringify(M.Debogueur.resumer(v))); } };
    R([1, 2, 3], "[1, 2, 3]", "liste courte");
    R(Array.from({ length: 31 }, (_, i) => i), "liste de 31 éléments : [0, 1, 2, 3, 4, …]", "liste longue");
    R("x".repeat(130), "x".repeat(117) + "…", "texte long");
    const it = new M.Interpreteur({ afficher: () => {} });
    await it.executer(M.analyserCode("soit a = 5"));
    const E = async (code, att, nom) => { const r = await M.Debogueur.evaluer(it, code); const bon = r.ok === att.ok && r.texte === att.texte; if (bon) ok++; else { ko++; console.log("✗ [débogueur] évaluer " + nom + "\n    obtenu : " + JSON.stringify(r)); } };
    await E("a * 2", { ok: true, texte: "10" }, "expression");
    await E("soit b = a + 1", { ok: true, texte: "" }, "instruction");
    await E("b", { ok: true, texte: "6" }, "variable créée par l'instruction");
    await E("zz", { ok: false, texte: 'La variable "zz" n\'existe pas. Déclarez-la d\'abord avec : soit zz = ...' }, "variable inconnue");
    await E("soit = 3", { ok: false, texte: "un nom attendu, mais \"=\" trouvé." }, "erreur de syntaxe");
  }
  await td("évaluation pendant une pause : pas de pause imbriquée", 'soit a = 5\nafficher a', [], { sur: async (d, l) => { if (l === 1) { d.reprendre("entrer"); } else if (l === 2) { await M.executerProgramme("afficher 1", { afficher: () => {} }); d.reprendre("continuer"); } } }, (r) => r.lignes.join() === "1,2" && r.out.join() === "5");
}

async function formateur() {
  groupe = "formateur";
  const F = (nom, code, att, ind) => {
    let r;
    try { r = M.formaterSource(code, ind); } catch (e) { r = "ERR:" + M.decrireErreur(e, code).texte; }
    const bon = att instanceof RegExp ? att.test(r) : r === att;
    if (bon) ok++; else { ko++; console.log("✗ [formateur] " + nom + "\n    obtenu : " + JSON.stringify(r) + "\n    attendu: " + String(att)); }
  };
  F("espaces et opérateurs", "soit   x=1+2*3-4", "soit x = 1 + 2 * 3 - 4\n");
  F("unaire", "soit x = 5 - -3\nsoit y=-x\nafficher(-y)", "soit x = 5 - -3\nsoit y = -x\nafficher(-y)\n");
  F("indentation si/sinon", "si x>1 alors\nafficher 1\nsinon si x<0 alors\nafficher 2\nsinon\nafficher 3\nfin", "si x > 1 alors\n    afficher 1\nsinon si x < 0 alors\n    afficher 2\nsinon\n    afficher 3\nfin\n");
  F("selon / cas", "selon x faire\ncas 1 alors\nafficher 1\ndefaut\nafficher 0\nfin", "selon x faire\n    cas 1 alors\n        afficher 1\n    defaut\n        afficher 0\nfin\n");
  F("classe et méthodes", "classe A\nfonction creer(x)\nsoi.x=x\nfin\nfin", "classe A\n    fonction creer(x)\n        soi.x = x\n    fin\nfin\n");
  F("tester", 'tester "t"\nverifier(vrai)\nfin', 'tester "t"\n    verifier(vrai)\nfin\n');
  F("commentaires conservés", "afficher 1   # fin de ligne\n# seul\nafficher 2 // autre", "afficher 1   # fin de ligne\n# seul\nafficher 2  // autre\n");
  F("# dans un texte", 'afficher "a # b"', 'afficher "a # b"\n');
  F("lignes vides réduites à une", "afficher 1\n\n\n\nafficher 2", "afficher 1\n\nafficher 2\n");
  F("fin de fichier", "afficher 1\n\n\n", "afficher 1\n");
  F("appels et crochets", "afficher f( 1 ,2 )\nsoit l=[ 1,2 ]\nsoit s={a:1}\nafficher taille ( l )", "afficher f(1, 2)\nsoit l = [1, 2]\nsoit s = { a: 1 }\nafficher taille(l)\n");
  F("liste sur plusieurs lignes", "soit l = [\n1,\n2\n]", "soit l = [\n    1,\n    2\n]\n");
  F("continuation", "soit t = 1 +\n2", "soit t = 1 +\n    2\n");
  F("lambda", "soit f = x=>x*2", "soit f = x => x * 2\n");
  F("indentation 2", "si vrai alors\nafficher 1\nfin", "si vrai alors\n  afficher 1\nfin\n", 2);
  F("CRLF conservé", "si vrai alors\r\nafficher 1\r\nfin\r\n", "si vrai alors\r\n    afficher 1\r\nfin\r\n");
  F("tabulations", "si vrai alors\n\t\tafficher 1\nfin", "si vrai alors\n    afficher 1\nfin\n");
  F("code invalide non formaté", "soit = 1", /^ERR:Ligne 1 : /);
  F("texte non fermé", 'afficher "abc', /^ERR:Ligne 1 : Texte non fermé/);
  F("pour chaque … dans [liste]", "pour chaque x dans[1,2] faire\nafficher x\nfin", "pour chaque x dans [1, 2] faire\n    afficher x\nfin\n");
  F("pas négatif", "pour i allantde 5 a 1 pas -1 faire\nafficher i\nfin", "pour i allantde 5 a 1 pas -1 faire\n    afficher i\nfin\n");
  F("variables nommées pas / dans", "soit pas = 2\nafficher pas -1\nsoit dans = [1]\nafficher lireElement(dans, 0)", "soit pas = 2\nafficher pas - 1\nsoit dans = [1]\nafficher lireElement(dans, 0)\n");
  F("idempotent", M.formaterSource("si x>1 alors\nafficher( 1 )  # c\nfin"), "si x > 1 alors\n    afficher(1)  # c\nfin\n");
}

async function editeur() {
  groupe = "éditeur";
  const O = M.OUTILS_EDITEUR, Z = M.OUTILS_ZIP;
  const eq = (nom, obtenu, att) => {
    const bon = att instanceof RegExp ? att.test(String(obtenu)) : JSON.stringify(obtenu) === JSON.stringify(att);
    if (bon) ok++; else { ko++; console.log("✗ [éditeur] " + nom + "\n    obtenu : " + JSON.stringify(obtenu) + "\n    attendu: " + String(att instanceof RegExp ? att : JSON.stringify(att))); }
  };
  const SRC = '# démo\nimporter "std/pile" comme p\nclasse Compte herite Base\n    fonction creer(titulaire, solde)\n        soi.titulaire = titulaire\n        soi.solde = solde\n    fin\n    fonction deposer(montant)\n        soi.solde = soi.solde + montant\n        renvoyer soi\n    fin\nfin\nsoit c = Compte("Awa", 100)\nsoit pile = p.Pile()\npour i allantde 1 a 3 pas 1 faire\n    afficher c.deposer(i).solde\nfin\nsoit f = x => x * 2\nstructure cfg = { nom: "x", port: 80 }\n';
  const depuisHtml = (h) => h.replace(/<[^>]+>/g, "").replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&amp;/g, "&");
  // --- analyse par le vrai analyseur
  const a = O.analyser(SRC);
  eq("analyse : valide", a.ok && a.erreur === null, true);
  eq("analyse : classe", a.rec.find((r) => r.g === "classe"), { g: "classe", nom: "Compte", parent: "Base", ligne: 3, fin: 12 });
  eq("analyse : méthode", a.rec.find((r) => r.g === "fonction" && r.nom === "deposer"), { g: "fonction", nom: "deposer", params: ["montant"], ligne: 8, fin: 11, classe: "Compte" });
  eq("analyse : champs", a.rec.filter((r) => r.g === "champ").map((r) => r.nom), ["titulaire", "solde", "solde"]);
  eq("analyse : import", a.rec.find((r) => r.g === "import"), { g: "import", module: "std/pile", alias: "p", ligne: 2 });
  eq("analyse : variable initialisée par un constructeur", a.rec.find((r) => r.g === "var" && r.nom === "c").init, { k: "appel", nom: "Compte" });
  eq("analyse : structure", a.rec.find((r) => r.g === "var" && r.nom === "cfg").init, { k: "structure", cles: ["nom", "port"] });
  eq("analyse : compteur de boucle", a.rec.find((r) => r.g === "var" && r.nom === "i").genre, "boucle");
  eq("analyse : lambda", a.rec.find((r) => r.g === "lambda").params, ["x"]);
  // l'analyse ne change pas ce que le programme fait
  eq("analyse : n'influence pas l'exécution", await run('fonction f(x)\n  renvoyer x + 1\nfin\nafficher f(1)'), "2");
  // --- coloration
  const h = O.html(a);
  eq("coloration : même texte que le source", depuisHtml(h), SRC + "\n");
  eq("coloration : mot-clé", /<span class='t-mc'>importer<\/span>/.test(h), true);
  eq("coloration : commentaire", /<span class='t-co'># démo<\/span>/.test(h), true);
  eq("coloration : nom de classe", /<span class='t-cl'>Compte<\/span>/.test(h), true);
  eq("coloration : champ après soi.", /<span class='t-mc'>soi<\/span>\.<span class='t-pr'>titulaire<\/span>/.test(h), true);
  eq("coloration : appel de méthode", /<span class='t-fn'>deposer<\/span>/.test(h), true);
  eq("coloration : namespace d'import", /<span class='t-ns'>p<\/span>\.<span class='t-fn'>Pile<\/span>/.test(h), true);
  eq("coloration : paramètre", /<span class='t-va'>montant<\/span>/.test(h), true);
  eq("coloration : « pas » et « allantde »", /<span class='t-mc'>allantde<\/span>/.test(h) && /<span class='t-mc'>pas<\/span>/.test(h), true);
  eq("coloration : fonction native", /<span class='t-nat'>/.test(O.html(O.analyser("soit n = racine(4)\nafficher n"))), true);
  eq("coloration : '<' échappé", depuisHtml(O.html(O.analyser("afficher 1 < 2 et 3 > 2"))), "afficher 1 < 2 et 3 > 2\n");
  // texte non fermé : le reste des lignes reste coloré, l'erreur est signalée
  const b = O.analyser('afficher "abc\nsoit x = 1\nafficher x');
  eq("tolérance : erreur signalée", b.erreur && b.erreur.ligne, 1);
  eq("tolérance : texte conservé", depuisHtml(O.html(b)), 'afficher "abc\nsoit x = 1\nafficher x\n');
  eq("tolérance : lignes suivantes colorées", /<span class='t-mc'>soit<\/span> <span class='t-va'>x<\/span>/.test(O.html(b)), true);
  eq("tolérance : guillemet non fermé = texte", /<span class='t-tx'>"abc<\/span>/.test(O.html(b)), true);
  // erreur d'analyse : code partiel, déclarations déjà vues conservées
  const c = O.analyser("soit a = 1\nfonction f(x)\n  soit y = ");
  eq("erreur d'analyse", !c.ok && c.erreur.ligne >= 3, true);
  eq("déclarations conservées malgré l'erreur", c.rec.map((r) => r.nom || ""), ["a", "f", "y"]);
  eq("fonction non fermée : portée ouverte", O.fonctionDe(c.rec, 99).nom, "f");
  eq("soulignement de l'erreur", /<span class='t-dg'>/.test(O.html(b, { ligne: 1, colonne: 9, longueur: 4 })) || /t-dg/.test(O.html(b, { ligne: 1, colonne: 9, longueur: 4 })), true);
  // --- complétion
  const sug = (t) => { const pos = t.indexOf("§"); const r = O.suggestions(t.replace("§", ""), pos); return { debut: r.debut - 0, textes: r.liste.map((x) => x.texte), genres: r.liste.map((x) => x.genre), pos }; };
  let r = sug(SRC + "c.§");
  eq("complétion : membres d'un objet", r.textes, ["deposer", "solde", "titulaire"]);
  eq("complétion : méthode vs champ", r.genres, ["méthode", "champ", "champ"]);
  r = sug(SRC + "c.dep§");
  eq("complétion : préfixe", r.textes, ["deposer"]);
  eq("complétion : position de remplacement", r.pos - r.debut, 3);
  eq("complétion : enchaînement Nom(…).", sug(SRC + "Compte(1).§").textes, ["deposer", "solde", "titulaire"]);
  eq("complétion : membres via alias", sug(SRC + "p.§").textes.includes("Pile"), true);
  eq("complétion : classe d'un module importé", sug(SRC + "pile.§").textes.includes("empiler"), true);
  eq("complétion : natives et mots", sug("tai§").textes.slice(0, 2), ["taille", "tableau"].slice(0, 1).concat(sug("tai§").textes.slice(1, 2)));
  eq("complétion : natif de bibliothèque", sug("rac§").textes.includes("racine"), true);
  eq("complétion : variable visible", sug("soit total = 1\nafficher tot§").textes[0], "total");
  eq("complétion : variable pas encore déclarée", sug("afficher tot§\nsoit total = 1").textes.includes("total"), false);
  eq("complétion : paramètre dans la fonction", sug("fonction g(zzz, y)\n  afficher zz§\nfin").textes[0], "zzz");
  eq("complétion : paramètre hors de la fonction", sug("fonction g(zzz, y)\n  afficher 1\nfin\nafficher zz§").textes.includes("zzz"), false);
  eq("complétion : soi dans une méthode", sug("classe A\n  fonction creer(x)\n    soi.val = x\n  fin\n  fonction m()\n    afficher soi.§\n  fin\nfin").textes, ["m", "val"]);
  eq("complétion : modules après importer", sug('importer "std/tr§').textes, ["std/tri"]);
  eq("complétion : liste des modules", sug('importer "§').textes.length >= 12, true);
  eq("complétion : rien dans un texte", sug('afficher "rac§').textes, []);
  eq("complétion : rien dans un commentaire", sug("# rac§").textes, []);
  eq("complétion : rien après soit (nouveau nom)", sug("soit tai§").textes, []);
  eq("complétion : champs d'une structure", sug(SRC + "cfg.§").textes, ["nom", "port"]);
  eq("complétion : héritage", sug("classe A\n  fonction creer()\n    soi.x = 1\n  fin\n  fonction un()\n    renvoyer 1\n  fin\nfin\nclasse B herite A\n  fonction deux()\n    renvoyer 2\n  fin\nfin\nsoit b = B()\nb.§").textes.sort(), ["un", "x", "deux"].sort());
  eq("complétion : insensible aux accents", sug("soit énergie = 1\nafficher energ§").textes[0], "énergie");
  eq("complétion : extrait de code", sug("pou§").genres.includes("extrait"), true);
  // --- recherche / remplacement
  eq("recherche : simple", O.chercherTout("abc ABC abc", "abc"), [{ deb: 0, fin: 3 }, { deb: 4, fin: 7 }, { deb: 8, fin: 11 }]);
  eq("recherche : casse", O.chercherTout("abc ABC abc", "abc", { casse: true }).length, 2);
  eq("recherche : mot entier", O.chercherTout("a ab aba b", "ab", { mot: true }).length, 1);
  eq("recherche : mot entier accentué", O.chercherTout("été étés", "été", { mot: true }).length, 1);
  eq("recherche : regex", O.chercherTout("a1 b22 c333", "\\d+", { regex: true }).map((m) => m.fin - m.deb), [1, 2, 3]);
  eq("recherche : caractères spéciaux littéraux", O.chercherTout("a.b a*b", "a.b").length, 1);
  eq("recherche : vide", O.chercherTout("abc", ""), []);
  eq("recherche : regex invalide", (() => { try { O.chercherTout("abc", "(", { regex: true }); return "pas d'erreur"; } catch (e) { return "erreur"; } })(), "erreur");
  eq("recherche : correspondances vides ignorées", O.chercherTout("abc", "x*", { regex: true }), []);
  eq("remplacement : tout", O.remplacerTout("a b a", "a", "zz"), { texte: "zz b zz", n: 2 });
  eq("remplacement : $ littéral hors regex", O.remplacerTout("a", "a", "$1"), { texte: "$1", n: 1 });
  eq("remplacement : groupes regex", O.remplacerTout("soit x = 1", "(\\w+) = (\\d)", "$2 = $1", { regex: true }), { texte: "soit 1 = x", n: 1 });
  eq("remplacement : $& et $$", O.remplacerTout("ab", "b", "[$&]$$", { regex: true }), { texte: "a[b]$", n: 1 });
  eq("remplacement : un seul", O.remplacerUn("a-a-a", 2, 3, "a", "Z", {}), { texte: "a-Z-a", fin: 3 });
  // --- zip
  const fich = [
    { chemin: "projet/principal.fa", texte: 'afficher "bonjour é€"\n'.repeat(50) },
    { chemin: "projet/vide.fa", texte: "" },
    { chemin: "projet/sous/dossier/accentué 日本.fa", texte: "soit x = 1" },
    { chemin: "projet/binaire.bin", octets: Uint8Array.from({ length: 300 }, (_, i) => i % 256) },
  ];
  const zip = await Z.creer(fich);
  const lu = await Z.lire(zip);
  eq("zip : chemins", lu.map((f) => f.chemin), fich.map((f) => f.chemin));
  eq("zip : contenu texte", new TextDecoder().decode(lu[0].octets), fich[0].texte);
  eq("zip : fichier vide", lu[1].octets.length, 0);
  eq("zip : nom unicode", new TextDecoder().decode(lu[2].octets), "soit x = 1");
  eq("zip : octets", Array.from(lu[3].octets), Array.from(fich[3].octets));
  eq("zip : compressé", zip.length < fich[0].texte.length, true);
  eq("zip : CRC32 connu", Z.crc32(new TextEncoder().encode("123456789")), 0xcbf43926);
  eq("zip : fichier invalide", await Z.lire(new Uint8Array(100)).then(() => "ok", (e) => e.message), "Ce fichier n'est pas une archive .zip valide.");
  const mauvais = zip.slice(); mauvais[55] ^= 0xff;
  eq("zip : corruption détectée", await Z.lire(mauvais).then(() => "ok", (e) => e.message), /corrompue|invalide/);
  const evil = await Z.creer([{ chemin: "../evil.fa", texte: "x" }]);
  eq("zip : chemin dangereux refusé", await Z.lire(evil).then(() => "ok", (e) => e.message), /Chemin refusé/);
  const slash = await Z.creer([{ chemin: "/abs/x.fa", texte: "x" }]);
  eq("zip : chemin absolu ramené", (await Z.lire(slash))[0].chemin, "abs/x.fa");
  // interopérabilité avec un autre outil (Python) si présent
  try {
    const cp = require("child_process"), fs2 = require("fs"), os2 = require("os"), path2 = require("path");
    const d = fs2.mkdtempSync(path2.join(os2.tmpdir(), "fad_zip_"));
    fs2.writeFileSync(path2.join(d, "a.zip"), zip);
    const rr = cp.spawnSync("python3", ["-I", "-c", "import zipfile,sys;z=zipfile.ZipFile(sys.argv[1]);print(z.testzip());print(sorted(z.namelist())[0])", path2.join(d, "a.zip")], { encoding: "utf8" });
    if (rr.status === 0) eq("zip : lisible par Python (CRC valides)", rr.stdout.split("\n")[0], "None");
    const mk = cp.spawnSync("python3", ["-I", "-c", "import zipfile,sys;z=zipfile.ZipFile(sys.argv[1],'w',zipfile.ZIP_DEFLATED);z.writestr('d/x.fa','soit a = 1\\n'*100);z.writestr('y.txt','é');z.close()", path2.join(d, "b.zip")]);
    if (mk.status === 0) {
      const l2 = await Z.lire(new Uint8Array(fs2.readFileSync(path2.join(d, "b.zip"))));
      eq("zip : lit une archive Python", l2.map((f) => f.chemin + ":" + f.octets.length), ["d/x.fa:1100", "y.txt:2"]);
    }
  } catch (e) {}
}

async function partage() {
  groupe = "partage";
  const P = M.OUTILS_PARTAGE;
  const eq = (nom, obtenu, att) => {
    const bon = JSON.stringify(obtenu) === JSON.stringify(att);
    if (bon) ok++; else { ko++; console.log("✗ [partage] " + nom + "\n    obtenu : " + JSON.stringify(obtenu) + "\n    attendu: " + JSON.stringify(att)); }
  };
  const echoue = async (nom, f, re) => {
    try { await f(); ko++; console.log("✗ [partage] " + nom + " : aucune erreur"); }
    catch (e) { if (re.test(e.message)) ok++; else { ko++; console.log("✗ [partage] " + nom + " : " + e.message); } }
  };
  const fich = [{ chemin: "main.fa", texte: "importer \"outils\"\nafficher double(21) // é 😀" }, { chemin: "lib/outils.fa", texte: "fonction double(x)\n renvoyer x * 2\nfin" }];
  const lien = await P.encoder(fich, "lib/outils.fa");
  eq("préfixe v1", lien.slice(0, 3), "v1.");
  eq("lien sûr pour une URL", /^[A-Za-z0-9._-]+$/.test(lien), true);
  const r = await P.decoder(lien);
  eq("aller-retour : fichiers", r.fichiers, fich);
  eq("aller-retour : fichier actif", r.actif, "lib/outils.fa");
  eq("actif inconnu → premier", (await P.decoder(await P.encoder(fich, "zzz"))).actif, "main.fa");
  const gros = "afficher 1\n".repeat(2000);
  eq("compression efficace", (await P.encoder([{ chemin: "a.fa", texte: gros }], "a.fa")).length < gros.length / 20, true);
  await echoue("trop de fichiers", () => P.encoder(Array.from({ length: 21 }, (_, i) => ({ chemin: "f" + i + ".fa", texte: "" })), "f0.fa"), /trop de fichiers/);
  await echoue("projet trop gros", () => P.encoder([{ chemin: "a.fa", texte: Array.from({ length: 20000 }, (_, i) => (i * 2654435761 % 4294967296).toString(36)).join(" ") }], "a.fa"), /trop gros/);
  await echoue("aucun fichier", () => P.encoder([], ""), /aucun fichier/);
  await echoue("préfixe inconnu", () => P.decoder("v9.abc"), /invalide/);
  await echoue("caractères interdits", () => P.decoder("v1.@@@"), /invalide/);
  await echoue("données corrompues", () => P.decoder("v1.AAAAAAAA"), /corrompu/);
  await echoue("chemin « .. » refusé", async () => P.decoder("v0." + Buffer.from(JSON.stringify({ v: 1, f: [["../x.fa", "a"]], a: 0 })).toString("base64url")), /chemin refusé/);
  await echoue("chemin absolu refusé", async () => P.decoder("v0." + Buffer.from(JSON.stringify({ v: 1, f: [["/etc/x", "a"]], a: 0 })).toString("base64url")), /chemin refusé/);
  await echoue("lecteur C: refusé", async () => P.decoder("v0." + Buffer.from(JSON.stringify({ v: 1, f: [["C:/x.fa", "a"]], a: 0 })).toString("base64url")), /chemin refusé/);
  await echoue("liste vide", async () => P.decoder("v0." + Buffer.from(JSON.stringify({ v: 1, f: [], a: 0 })).toString("base64url")), /invalide/);
  // bombe de décompression : 5 Mo de zéros compressés en quelques Ko
  const zlib = require("zlib");
  const bombe = zlib.deflateRawSync(Buffer.from('{"v":1,"f":[["a.fa","' + "0".repeat(5000000) + '"]],"a":0}'));
  await echoue("bombe de décompression", () => P.decoder("v1." + bombe.toString("base64url")), /trop gros/);
  eq("nettoyerChemin", [P.nettoyerChemin("a\\b.fa"), P.nettoyerChemin("./x.fa"), P.nettoyerChemin("a//b"), P.nettoyerChemin("")], ["a/b.fa", "x.fa", null, null]);
}

(async () => {
  await moteur();
  await editeur();
  await partage();
  await formateur();
  await erreurs();
  await debogueur();
  await shell();
  console.log("\n" + ok + " test(s) réussi(s), " + ko + " échoué(s).");
  process.exit(ko ? 1 : 0);
})();

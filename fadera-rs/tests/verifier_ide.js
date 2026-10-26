// E2E de l'IDE dans un vrai navigateur (Chromium) : onglets, coloration, complétion, recherche, formateur, IndexedDB, zip
//   usage : node tests/verifier_ide.js chemin/index.html
//   Prérequis (facultatif) : le paquet « playwright » et un navigateur Chromium (npm i playwright && npx playwright install chromium).
//   Sans Playwright, le script se termine avec le code 3 (« ignoré »).
let chromium;
for (const m of ["playwright", "playwright-core", "/opt/npm-tools/node_modules/playwright"]) {
  try { ({ chromium } = require(m)); break; } catch (e) {}
}
if (!chromium) { console.log("Playwright absent : vérification de l'IDE dans le navigateur ignorée."); process.exit(3); }
const EXE = ["/opt/pw-browsers/chromium"].find((p) => require("fs").existsSync(p));
const lancer = (profil, opts) => chromium.launchPersistentContext(profil, { ...(EXE ? { executablePath: EXE } : {}), ...opts });
const fs = require("fs"), path = require("path"), os = require("os"), cp = require("child_process");
const HTML = path.resolve(process.argv[2] || path.join(__dirname, "..", "..", "index.html"));
let ok = 0, ko = 0;
const eq = (nom, obtenu, att) => {
  const bon = att instanceof RegExp ? att.test(String(obtenu)) : JSON.stringify(obtenu) === JSON.stringify(att);
  if (bon) { ok++; console.log("✓ " + nom); } else { ko++; console.log("✗ " + nom + "\n    obtenu : " + JSON.stringify(obtenu) + "\n    attendu: " + String(att)); }
};
(async () => {
  const dossierDl = fs.mkdtempSync(path.join(os.tmpdir(), "fad_dl_"));
  const profil = fs.mkdtempSync(path.join(os.tmpdir(), "fad_prof_"));
  const ctx = await lancer(profil, { acceptDownloads: true, viewport: { width: 1400, height: 900 } });
  const errs = [];
  async function ouvrir(avant, hash) {
    const p = await ctx.newPage();
    p.on("pageerror", (e) => errs.push(String(e)));
    p.on("console", (m) => { if (m.type() === "error" && !/Failed to load resource|CORS policy/.test(m.text())) errs.push("console: " + m.text()); });
    if (avant) await p.addInitScript(avant);
    await p.goto("file://" + HTML + (hash || ""));
    await p.waitForFunction(() => !document.getElementById("editeur").readOnly && /enregistré|stockage|⚠/.test(document.getElementById("statSauve").textContent), null, { timeout: 8000 });
    return p;
  }
  const ed = (p) => p.locator("#editeur");
  const val = (p) => p.evaluate(() => document.getElementById("editeur").value);
  const rempl = async (p, txt) => { await p.evaluate((t) => { const e = document.getElementById("editeur"); e.value = t; e.dispatchEvent(new Event("input")); }, txt); await p.waitForTimeout(80); };
  const flush = (p) => p.evaluate(async () => { FS_syncAvant(); FadFS.sauverMaintenant(); await new Promise((r) => setTimeout(r, 300)); });

  // ---------- migration depuis localStorage (premier lancement avec des données anciennes) ----------
  let p = await ouvrir(() => {
    if (!localStorage.getItem("seme")) {
      localStorage.setItem("seme", "1");
      localStorage.setItem("fadera_ancienprojet", 'afficher "ancien"');
      const f = (c) => ({ t: "f", c, m: 420, mt: 1 });
      const d = (c) => ({ t: "d", c, m: 493, mt: 1 });
      localStorage.setItem("fadera_vfs_v1", JSON.stringify(d({ home: d({ fadera: d({ "script.fa": f("# ancien script\nafficher 42") }) }) })));
    }
  });
  eq("migration : l'ancien script est repris", (await val(p)).includes("ancien script"), true);
  eq("migration : ancien projet copié", await p.evaluate(() => FadFS.noeud(FadFS.HOME + "/ancienprojet.fa") && FadFS.noeud(FadFS.HOME + "/ancienprojet.fa").c), 'afficher "ancien"');
  eq("démarrage sans erreur JavaScript", errs, []);
  eq("sauvegarde : état affiché", await p.locator("#statSauve").textContent(), /enregistré/);
  eq("IndexedDB contient le système de fichiers", await p.evaluate(async () => { const r = await IDE.lireBD("vfs"); return !!(r && r.json && r.json.includes("ancien script")); }), true);

  // ---------- coloration ----------
  eq("coloration active", await p.evaluate(() => document.querySelector(".editeur-corps").classList.contains("colore")), true);
  await rempl(p, 'soit x = 1\nafficher "a < b" # fin\nsi x > 0 alors\n  afficher x\nfin');
  const calque = await p.evaluate(() => document.getElementById("calqueSyntaxe").innerHTML);
  eq("coloration : mot-clé soit", /<span class="t-mc">soit<\/span>/.test(calque), true);
  eq("coloration : texte", /<span class="t-tx">"a &lt; b"<\/span>/.test(calque), true);
  eq("coloration : commentaire", /<span class="t-co"># fin<\/span>/.test(calque), true);
  eq("coloration : même texte que l'éditeur", await p.evaluate(() => document.getElementById("calqueSyntaxe").textContent === document.getElementById("editeur").value + "\n"), true);
  // alignement pixel : la police et la hauteur de ligne sont identiques, le défilement est synchronisé
  eq("coloration : mêmes métriques que le textarea", await p.evaluate(() => { const a = getComputedStyle(document.getElementById("editeur")), b = getComputedStyle(document.getElementById("calqueSyntaxe")); return [a.fontFamily === b.fontFamily, a.fontSize === b.fontSize, a.lineHeight === b.lineHeight, a.paddingLeft === b.paddingLeft, a.paddingTop === b.paddingTop, a.tabSize === b.tabSize]; }), [true, true, true, true, true, true]);
  await rempl(p, Array.from({ length: 120 }, (_, i) => "afficher " + i).join("\n"));
  await p.evaluate(() => { const e = document.getElementById("editeur"); e.scrollTop = 777; e.dispatchEvent(new Event("scroll")); });
  eq("coloration : défilement synchronisé", await p.evaluate(() => document.getElementById("calqueSyntaxe").scrollTop), 777);
  // diagnostic de l'analyseur
  await rempl(p, "soit x = 1\nafficher (x +\n");
  await p.waitForTimeout(1000);
  eq("diagnostic affiché dans la barre d'état", await p.locator("#statDiag").textContent(), /Ligne \d/);
  eq("diagnostic souligné", await p.evaluate(() => !!document.querySelector("#calqueSyntaxe .t-dg")), true);
  await rempl(p, "soit x = 1\nafficher x");
  await p.waitForTimeout(1000);
  eq("diagnostic disparu quand le code est valide", await p.locator("#statDiag").textContent(), "");

  // ---------- complétion ----------
  await rempl(p, "classe Compte\n  fonction creer(s)\n    soi.solde = s\n  fin\n  fonction deposer(m)\n    soi.solde = soi.solde + m\n  fin\nfin\nsoit c = Compte(1)\n");
  await ed(p).click();
  await p.keyboard.press("Control+End");
  await p.keyboard.type("c.");
  await p.waitForSelector("#menuCompletion.ouvert");
  eq("complétion : membres proposés", await p.evaluate(() => [...document.querySelectorAll("#menuCompletion .cmp-t")].map((x) => x.textContent)), ["deposer", "solde"]);
  await p.keyboard.type("dep");
  eq("complétion : filtrée", await p.evaluate(() => [...document.querySelectorAll("#menuCompletion .cmp-t")].map((x) => x.textContent)), ["deposer"]);
  await p.keyboard.press("Enter");
  eq("complétion : acceptée par Entrée (sans saut de ligne)", (await val(p)).endsWith("\nc.deposer"), true);
  eq("complétion : fermée après acceptation", await p.evaluate(() => document.getElementById("menuCompletion").classList.contains("ouvert")), false);
  await p.keyboard.type("(1)");
  await p.keyboard.press("Enter");
  eq("Entrée sans menu : saut de ligne normal", (await val(p)).endsWith("c.deposer(1)\n"), true);
  await p.keyboard.type("taill");
  await p.waitForSelector("#menuCompletion.ouvert");
  await p.keyboard.press("Escape");
  eq("complétion : Échap ferme", await p.evaluate(() => document.getElementById("menuCompletion").classList.contains("ouvert")), false);
  await p.keyboard.press("Control+a");
  await p.keyboard.press("Delete");
  await p.keyboard.type("importer \"std/pi");
  await p.waitForSelector("#menuCompletion.ouvert");
  await p.keyboard.press("Tab");
  eq("complétion : module std", await val(p), 'importer "std/pile');
  await p.keyboard.press("Control+a"); await p.keyboard.press("Delete");
  await p.keyboard.type("si");
  await p.keyboard.press("Enter");
  eq("raccourci « si » + Entrée : expansion conservée", (await val(p)).startsWith("si condition alors"), true);

  // ---------- formateur ----------
  await rempl(p, "si   vrai alors\nafficher(  1 )\nfin");
  await ed(p).click();
  await p.keyboard.press("Shift+Alt+F");
  eq("formateur : raccourci", await val(p), "si vrai alors\n    afficher(1)\nfin\n");
  await p.keyboard.press("Control+z");
  eq("formateur : annulable", await val(p), "si   vrai alors\nafficher(  1 )\nfin");
  await rempl(p, "soit = 1");
  await p.evaluate(() => formaterEditeur());
  eq("formateur : code invalide non touché", await val(p), "soit = 1");

  // ---------- onglets ----------
  await rempl(p, 'afficher "un"');
  await p.evaluate(() => { demanderTexte; });
  await p.evaluate(() => nouveauProjet());
  await p.fill("#modalInput", "deux");
  await p.keyboard.press("Enter");
  await p.waitForTimeout(150);
  eq("onglets : deux onglets", await p.evaluate(() => [...document.querySelectorAll("#barreOnglets .onglet .o-nom")].map((x) => x.textContent)), ["script.fa", "deux.fa"]);
  await p.waitForTimeout(100);
  eq("onglets : le nouveau est actif et vide (Entrée du dialogue sans saut de ligne)", [await val(p), await p.evaluate(() => document.querySelector("#barreOnglets .onglet.actif .o-nom").textContent)], ["", "deux.fa"]);
  await rempl(p, 'afficher "deux"');
  await p.click("#barreOnglets .onglet:nth-child(1)");
  eq("onglets : retour au premier, contenu intact", await val(p), 'afficher "un"');
  await p.click("#barreOnglets .onglet:nth-child(2)");
  eq("onglets : deuxième, contenu intact", await val(p), 'afficher "deux"');
  eq("onglets : titre de la fenêtre", await p.evaluate(() => document.querySelector(".titre-fenetre").textContent), "deux.fa - Éditeur");
  // exécution : c'est le fichier de l'onglet actif
  await p.evaluate(() => lancerCode());
  await p.waitForTimeout(500);
  eq("onglets : exécute le fichier actif", await p.evaluate(() => document.getElementById("terminalContent").innerText.includes("deux")), true);
  // ordre par glisser-déposer
  await p.dragAndDrop("#barreOnglets .onglet:nth-child(2)", "#barreOnglets .onglet:nth-child(1)");
  eq("onglets : réordonnés par glisser-déposer", await p.evaluate(() => [...document.querySelectorAll("#barreOnglets .onglet .o-nom")].map((x) => x.textContent)), ["deux.fa", "script.fa"]);
  // fermer
  await p.click("#barreOnglets .onglet.actif .o-x");
  eq("onglets : fermeture", await p.evaluate(() => [...document.querySelectorAll("#barreOnglets .onglet .o-nom")].map((x) => x.textContent)), ["script.fa"]);
  eq("onglets : le dernier ne se ferme pas", (await p.evaluate(() => { fermerOnglet(editeurChemin); return document.querySelectorAll("#barreOnglets .onglet").length; })), 1);
  // rm dans le terminal : l'onglet disparaît
  await p.evaluate(() => nouveauProjet()); await p.fill("#modalInput", "temp"); await p.keyboard.press("Enter"); await p.waitForTimeout(100);
  await p.click("#barreOnglets .onglet:nth-child(1)");
  await p.evaluate(() => { FadFS.supprimer(FadFS.HOME + "/temp.fa"); });
  await p.evaluate(() => IDE.rendreOnglets());
  eq("onglets : fichier supprimé retiré de la barre", await p.evaluate(() => [...document.querySelectorAll("#barreOnglets .onglet .o-nom")].map((x) => x.textContent)), ["script.fa"]);

  // ---------- recherche / remplacement ----------
  await rempl(p, "chat chien Chat\nchat-chaud");
  await ed(p).click();
  await p.keyboard.press("Control+f");
  eq("recherche : panneau ouvert", await p.evaluate(() => document.getElementById("panneauRecherche").classList.contains("ouvert")), true);
  await p.keyboard.type("chat");
  await p.waitForTimeout(100);
  eq("recherche : compteur", await p.locator("#rechCompte").textContent(), "1/3");
  eq("recherche : surbrillance", await p.evaluate(() => document.querySelectorAll("#calqueRecherche mark").length), 3);
  await p.keyboard.press("Enter");
  eq("recherche : suivant", await p.locator("#rechCompte").textContent(), "2/3");
  await p.keyboard.press("Shift+Enter");
  await p.keyboard.press("Shift+Enter");
  eq("recherche : précédent (boucle)", await p.locator("#rechCompte").textContent(), "3/3");
  await p.click("#rechCasse");
  eq("recherche : casse", await p.locator("#rechCompte").textContent(), /\/2$/);
  await p.click("#rechCasse");
  await p.click("#rechMot");
  eq("recherche : mot entier", await p.locator("#rechCompte").textContent(), /\/3$/);
  await p.click("#rechMot");
  await p.click("#rechRegex");
  await p.fill("#rechInput", "(ch)(at|ien)");
  await p.waitForTimeout(100);
  eq("recherche : regex", await p.locator("#rechCompte").textContent(), /\/4$/);
  await p.fill("#rechInput", "(");
  await p.waitForTimeout(100);
  eq("recherche : regex invalide signalée", await p.evaluate(() => document.getElementById("rechInput").classList.contains("invalide")), true);
  await p.fill("#rechInput", "(ch)(at|ien)");
  await p.keyboard.press("Control+h");
  await p.fill("#rechRemp", "$2-$1");
  await p.click("#rechRempTout");
  eq("remplacement : tout (groupes)", await val(p), "at-ch ien-ch at-Ch\nat-ch-chaud");
  await p.keyboard.press("Escape");
  await p.keyboard.press("Control+z");
  eq("remplacement : annulable en une étape", await val(p), "chat chien Chat\nchat-chaud");
  await p.click("#rechRegex");
  await p.evaluate(() => IDE.ouvrirRecherche(true));
  await p.fill("#rechInput", "chat");
  await p.fill("#rechRemp", "chien");
  await p.click("#rechRempUn");
  eq("remplacement : un seul", (await val(p)).startsWith("chien chien Chat") || (await val(p)).startsWith("chat chien chien"), true);
  await p.keyboard.press("Escape");

  // ---------- durabilité : rechargement ----------
  await rempl(p, 'afficher "durable ✓"');
  await p.evaluate(() => nouveauProjet()); await p.fill("#modalInput", "dossier/autre"); await p.keyboard.press("Enter"); await p.waitForTimeout(100);
  await rempl(p, "# autre fichier");
  await flush(p);
  await p.close();
  p = await ouvrir();
  eq("durabilité : onglets restaurés", await p.evaluate(() => [...document.querySelectorAll("#barreOnglets .onglet .o-nom")].map((x) => x.textContent)), ["script.fa", "autre.fa"]);
  eq("durabilité : onglet actif restauré avec son contenu", await val(p), "# autre fichier");
  await p.click("#barreOnglets .onglet:nth-child(1)");
  eq("durabilité : contenu du premier onglet", await val(p), 'afficher "durable ✓"');
  eq("durabilité : localStorage n'est plus écrit", await p.evaluate(() => !(localStorage.getItem("fadera_vfs_v1") || "").includes("durable")), true);

  // ---------- export / import .zip ----------
  const [dl] = await Promise.all([p.waitForEvent("download"), p.evaluate(() => exporterZip(false))]);
  const fz = path.join(dossierDl, dl.suggestedFilename());
  await dl.saveAs(fz);
  eq("export : nom de fichier", dl.suggestedFilename(), /^fadera-projets-\d{8}\.zip$/);
  const py = cp.spawnSync("python3", ["-I", "-c", "import zipfile,sys;z=zipfile.ZipFile(sys.argv[1]);print(z.testzip());print(sorted(z.namelist()));print(z.read('script.fa').decode())", fz], { encoding: "utf8" });
  eq("export : archive valide (Python)", py.stdout.split("\n").slice(0, 2).join("|"), /^None\|\['ancienprojet\.fa', 'deux\.fa', 'dossier\/autre\.fa', 'script\.fa'(, 'sortie\.log')?\]$/);
  eq("export : contenu", py.stdout.split("\n").slice(2).join("\n").trim(), 'afficher "durable ✓"');
  const [dl2] = await Promise.all([p.waitForEvent("download"), p.evaluate(() => exporterZip(true))]);
  const fz2 = path.join(dossierDl, "onglets.zip"); await dl2.saveAs(fz2);
  eq("export : onglets seulement", cp.spawnSync("python3", ["-I", "-c", "import zipfile,sys;print(sorted(zipfile.ZipFile(sys.argv[1]).namelist()))", fz2], { encoding: "utf8" }).stdout.trim(), "['dossier/autre.fa', 'script.fa']");
  // import : un zip fabriqué par Python avec conflit
  const zi = path.join(dossierDl, "entree.zip");
  cp.spawnSync("python3", ["-I", "-c", "import zipfile,sys;z=zipfile.ZipFile(sys.argv[1],'w',zipfile.ZIP_DEFLATED);z.writestr('script.fa','# remplacé');z.writestr('lib/outils.fa','fonction f()\\n  renvoyer 1\\nfin');z.writestr('img.bin',bytes([255,254,0,200]));z.close()", zi]);
  await p.setInputFiles("#champImport, input[type=file]", []).catch(() => {});
  await p.evaluate(() => importerFichiers.call && 0);
  const [choix] = await Promise.all([p.waitForSelector("#impRen"), p.evaluate(async (b64) => { const bin = Uint8Array.from(atob(b64), (c) => c.charCodeAt(0)); await IDE.importerListe([new File([bin], "entree.zip")]); }, fs.readFileSync(zi).toString("base64"))]);
  eq("import : conflit proposé", await p.locator("#modalCorps h3").textContent(), "1 fichier(s) existent déjà");
  await p.click("#impRen");
  await p.waitForTimeout(200);
  eq("import : garder les deux", await p.evaluate(() => [FadFS.noeud(FadFS.HOME + "/script.fa").c, FadFS.noeud(FadFS.HOME + "/script (importé).fa").c, !!FadFS.noeud(FadFS.HOME + "/lib/outils.fa"), FadFS.noeud(FadFS.HOME + "/img.bin")]), ['afficher "durable ✓"', "# remplacé", true, null]);
  eq("import : binaire ignoré (signalé)", await p.locator("#toast").textContent(), /1 ignoré/);
  eq("import : fichiers .fa ouverts en onglets", await p.evaluate(() => [...document.querySelectorAll("#barreOnglets .onglet .o-nom")].some((x) => x.textContent.includes("outils"))), true);
  // archive piégée
  const mauvais = await p.evaluate(async () => { try { const z = await OUTILS_ZIP.creer([{ chemin: "../evil.fa", texte: "x" }]); await OUTILS_ZIP.lire(z); return "accepté"; } catch (e) { return e.message; } });
  eq("import : chemin dangereux refusé", mauvais, /Chemin refusé/);
  // suppression et confirmation
  await p.evaluate(() => supprimerProjet());
  await p.click(".modal-item[data-nom='img.bin'], .modal-item[data-nom='lib/outils.fa']");
  eq("suppression : demande confirmation", await p.locator("#modalCorps h3").textContent(), /définitivement/);
  await p.click("#modalAnnulerBtn");
  eq("suppression annulée : fichier présent", await p.evaluate(() => !!FadFS.noeud(FadFS.HOME + "/lib/outils.fa")), true);

  // ---------- partage par lien ----------
  await p.evaluate(() => {
    FadFS.mkdirP(FadFS.HOME + "/proj");
    FadFS.ecrire(FadFS.HOME + "/proj/outils.fa", "fonction double(x)\n  renvoyer x * 2\nfin", false);
    FadFS.ecrire(FadFS.HOME + "/proj/main.fa", 'importer "outils"\nafficher double(21)\nafficher "é😀"', false);
    activerOnglet(FadFS.HOME + "/proj/outils.fa", true);
    activerOnglet(FadFS.HOME + "/proj/main.fa", false);
    IDE.onglets.liste = IDE.onglets.liste.filter((c) => c.includes("/proj/"));
    IDE.rendreOnglets();
  });
  await p.evaluate(() => partagerLien());
  await p.waitForSelector("#champLien");
  const lienPartage = await p.evaluate(() => IDE.dernierLien);
  eq("partage : fenêtre du lien", await p.locator("#modalCorps h3").textContent(), /Lien de partage/);
  eq("partage : le lien contient #p=v1.", /#p=v1\./.test(lienPartage), true);
  eq("partage : le champ montre le lien", await p.locator("#champLien").inputValue(), lienPartage);
  await p.click("#lienFermer");
  const hashPartage = lienPartage.slice(lienPartage.indexOf("#"));
  // un destinataire : autre profil, rien d'enregistré
  const ctx2 = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium" }).catch(() => chromium.launch()).then((b) => b.newContext());
  const q = await ctx2.newPage();
  q.on("pageerror", (e) => errs.push("destinataire : " + String(e)));
  await q.goto("file://" + HTML + hashPartage);
  await q.waitForSelector("#partOuvrir", { timeout: 8000 });
  eq("partage reçu : confirmation demandée", await q.locator("#modalCorps h3").textContent(), /Projet partagé/);
  eq("partage reçu : liste les fichiers", await q.locator("#modalCorps .modal-item").allTextContents(), ["partage/proj/outils.fa", "partage/proj/main.fa"]);
  eq("partage reçu : rien n'est exécuté ni écrit avant accord", await q.evaluate(() => [document.getElementById("terminalContent").innerText.includes("42"), !!FadFS.noeud(FadFS.HOME + "/partage")]), [false, false]);
  await q.click("#partOuvrir");
  await q.waitForTimeout(400);
  eq("partage reçu : fichiers copiés dans partage/", await q.evaluate(() => [FadFS.noeud(FadFS.HOME + "/partage/proj/main.fa").c.includes("double(21)"), FadFS.noeud(FadFS.HOME + "/partage/proj/outils.fa").c.includes("renvoyer x * 2")]), [true, true]);
  eq("partage reçu : le fichier actif est affiché", await q.evaluate(() => document.getElementById("editeur").value.includes("double(21)")), true);
  eq("partage reçu : le lien est effacé de la barre d'adresse", await q.evaluate(() => location.hash), "");
  await q.evaluate(() => lancerCode());
  await q.waitForTimeout(600);
  if (process.env.DEBUG_E2E) console.log(await q.evaluate(() => document.getElementById("terminalContent").innerText));
  eq("partage reçu : le projet s'exécute (import relatif compris)", await q.evaluate(() => { const t = document.getElementById("terminalContent").innerText; return t.includes("42") && t.includes("é😀"); }), true);
  // lien piégé : refusé, rien n'est écrit
  const piege = "#p=v0." + Buffer.from(JSON.stringify({ v: 1, f: [["../../evil.fa", "afficher 1"]], a: 0 })).toString("base64url");
  const q2 = await (await ctx2.browser().newContext()).newPage();
  await q2.goto("file://" + HTML + piege);
  await q2.waitForFunction(() => /refusé/.test(document.getElementById("toast").textContent), null, { timeout: 8000 });
  eq("partage piégé : refusé", await q2.evaluate(() => [document.getElementById("modalOverlay").classList.contains("ouvert"), !!FadFS.noeud(FadFS.HOME + "/partage")]), [false, false]);
  // annuler laisse l'espace intact
  const q3 = await (await ctx2.browser().newContext()).newPage();
  await q3.goto("file://" + HTML + "#p=" + lienPartage.split("#p=")[1]);
  await q3.waitForSelector("#partAnnuler");
  await q3.click("#partAnnuler");
  eq("partage annulé : rien d'écrit, lien effacé", await q3.evaluate(() => [!!FadFS.noeud(FadFS.HOME + "/partage"), location.hash]), [false, ""]);
  await ctx2.browser().close();

  // ---------- paquets : installer / utiliser / retirer depuis l'IDE (faux serveur GitHub) ----------
  const FAUX = {
    "u/outils@v1/fadera-paquet.json": JSON.stringify({ nom: "outils", version: "1.2.0", principal: "principal.fa", fichiers: ["principal.fa", "aide.fa", "docs/LISEZMOI.md"] }),
    "u/outils@v1/principal.fa": 'importer "aide" comme a\nfonction double_plus_un(x)\n  renvoyer a.double(x) + 1\nfin\n',
    "u/outils@v1/aide.fa": "fonction double(x)\n  renvoyer x * 2\nfin\n",
    "u/outils@v1/docs/LISEZMOI.md": "# outils\n",
    "u/piege@main/fadera-paquet.json": JSON.stringify({ nom: "piege", fichiers: ["principal.fa", "../evil.fa"] }),
    "u/piege@main/principal.fa": "afficher 1\n",
    "u/mauvais@main/fadera-paquet.json": JSON.stringify({ nom: "Mauvais Nom", fichiers: ["principal.fa"] }),
    "u/exe@main/fadera-paquet.json": JSON.stringify({ nom: "exe", fichiers: ["principal.fa", "virus.exe"] }),
  };
  // l'adresse GitHub brute est « utilisateur/depot/version/fichier »
  for (const k of Object.keys(FAUX)) { const m = k.match(/^([^/]+)\/([^@/]+)@([^/]+)\/(.*)$/); FAUX[m[1] + "/" + m[2] + "/" + m[3] + "/" + m[4]] = FAUX[k]; delete FAUX[k]; }
  await p.context().route("https://raw.githubusercontent.com/**", (route) => {
    const cle = route.request().url().replace("https://raw.githubusercontent.com/", "");
    const tete = { "access-control-allow-origin": "*" };
    route.fulfill(cle in FAUX ? { status: 200, headers: tete, contentType: "text/plain", body: FAUX[cle] } : { status: 404, headers: tete, body: "absent" });
  });
  await p.evaluate(() => ouvrirPaquets());
  eq("paquets : la fenêtre s'ouvre", await p.evaluate(() => document.getElementById("modalOverlay").classList.contains("ouvert") && !!document.getElementById("pqSource")), true);
  eq("paquets : paquets connus proposés", await p.locator(".pq-connus .modal-item").count(), 2);
  await p.fill("#pqSource", "u/outils@v1");
  await p.click("#pqInstaller");
  await p.waitForFunction(() => /outils 1\.2\.0 installé/.test(document.getElementById("pqMessage").textContent), null, { timeout: 8000 });
  eq("paquets : installé, ligne d'importation proposée", await p.locator("#pqMessage").textContent(), /importer "paquets\/outils\/principal" comme outils/);
  eq("paquets : fichiers écrits (sous-dossier compris)", await p.evaluate(() => ["principal.fa", "aide.fa", "docs/LISEZMOI.md", "fadera-paquet.json"].map((f) => !!FadFS.noeud(FadFS.HOME + "/paquets/outils/" + f))), [true, true, true, true]);
  eq("paquets : fadera.json mis à jour", await p.evaluate(() => JSON.parse(FadFS.lire(FadFS.HOME + "/fadera.json"))), { dependances: { outils: "u/outils@v1" } });
  eq("paquets : le paquet est listé", await p.locator("#pqListe .pq-nom").allTextContents(), [/outils/].map(() => "📦 outils 1.2.0"));
  await p.click("[data-pq-inserer=outils]");
  eq("paquets : « Utiliser » ajoute la ligne importer au script", (await val(p)).includes('importer "paquets/outils/principal" comme outils'), true);
  await rempl(p, 'importer "paquets/outils/principal" comme outils\nafficher outils.double_plus_un(20)');
  await p.evaluate(() => lancerCode());
  await p.waitForTimeout(700);
  eq("paquets : le programme utilise le paquet (import imbriqué compris)", await p.evaluate(() => /(^|\n)41(\n|$)/.test(document.getElementById("terminalContent").innerText)), true);
  // refus : rien n'est écrit
  for (const [src, motif] of [["u/piege", /chemin de fichier refusé/], ["u/mauvais", /nom de paquet .* invalide/], ["u/exe", /seuls les fichiers/], ["u/absent", /404/], ["http://exemple.org/x", /https/], ["n'importe quoi", /non reconnue/]]) {
    await p.evaluate(() => ouvrirPaquets());
    await p.fill("#pqSource", src);
    await p.click("#pqInstaller");
    await p.waitForFunction(() => /✘/.test(document.getElementById("pqMessage").textContent), null, { timeout: 8000 });
    eq("paquets : refus de « " + src + " »", await p.locator("#pqMessage").textContent(), motif);
  }
  eq("paquets : aucun fichier écrit par les refus", await p.evaluate(() => [FadFS.noms(FadFS.HOME + "/paquets"), !!FadFS.noeud(FadFS.HOME + "/evil.fa")]), [["outils"], false]);
  // retirer
  await p.evaluate(() => ouvrirPaquets());
  await p.click("[data-pq-retirer=outils]");
  eq("paquets : retiré (dossier et fadera.json)", await p.evaluate(() => [!!FadFS.noeud(FadFS.HOME + "/paquets/outils"), JSON.parse(FadFS.lire(FadFS.HOME + "/fadera.json"))]), [false, { dependances: {} }]);
  await p.evaluate(() => fermerModal());

  eq("aucune erreur JavaScript pendant tout le parcours", errs, []);
  await ctx.close();
  console.log("\n" + ok + " vérification(s) réussie(s), " + ko + " échouée(s).");
  process.exit(ko ? 1 : 0);
})().catch((e) => { console.error("ERREUR DU TEST :", e); process.exit(2); });

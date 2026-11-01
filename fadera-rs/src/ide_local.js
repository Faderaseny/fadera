// Fadéra — module « dossier réel + terminal PowerShell / cmd ».
// Servi uniquement par « fadera ide » (jamais par la version en ligne) et chargé par la page de l'éditeur.
//   • les fichiers de l'éditeur sont recopiés dans un vrai dossier de l'ordinateur (Documents\Fadera) et inversement ;
//   • les commandes « powershell », « cmd » (et « pwsh ») du terminal ouvrent le vrai PowerShell / cmd de Windows
//     dans ce dossier : « fadera run mon_programme.fa » y fonctionne, et les fichiers créés réapparaissent dans l'éditeur.
// Tout passe par le serveur local, avec un jeton secret remis à cette page (voir ide.rs).
(function () {
  "use strict";
  if (location.protocol !== "http:" || !/^(127\.0\.0\.1|localhost)$/.test(location.hostname)) return;
  const meta = document.querySelector('meta[name="fadera-jeton"]');
  if (!meta || !meta.content) return;
  const JETON = meta.content;

  const EXT = /\.(fa|txt|md|json|csv|tsv|html|htm|css|xml|ya?ml|ini|log)$/i;
  const CLE_BASE = "fadera_espace_base_v1";
  let espace = null; // dossier réel (chemin Windows)
  let base = {}; // nom → { h: empreinte du contenu, mt: date côté disque } à la dernière synchronisation
  let occupe = false;
  let mode = null; // null (bash) | "powershell" | "cmd"
  let cwd = null;
  let avertis = {};
  try { base = JSON.parse(localStorage.getItem(CLE_BASE) || "{}") || {}; } catch (e) { base = {}; }
  const garder = () => { try { localStorage.setItem(CLE_BASE, JSON.stringify(base)); } catch (e) {} };

  async function api(methode, chemin, corps) {
    const r = await fetch(chemin, {
      method: methode,
      cache: "no-store",
      headers: Object.assign({ "X-Fadera-Jeton": JETON }, corps ? { "Content-Type": "application/json" } : {}),
      body: corps ? JSON.stringify(corps) : undefined,
    });
    if (!r.ok) throw new Error(chemin + " : " + r.status);
    const t = await r.text();
    return t ? JSON.parse(t) : {};
  }
  const empreinte = (s) => { let h = 5381; for (let i = 0; i < s.length; i++) h = ((h << 5) + h + s.charCodeAt(i)) | 0; return (h >>> 0).toString(36) + ":" + s.length; };
  const parent = (nom) => nom.split("/").slice(0, -1).join("/");
  const echap = (s) => String(s).replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

  // ---------------------------------------------------------------- fichiers : éditeur ⇄ dossier réel
  function fichiersLocaux() {
    const m = new Map();
    for (const f of FadFS.fichiers(FadFS.HOME)) {
      const nom = f.chemin.slice(FadFS.HOME.length + 1);
      const segs = nom.split("/");
      if (segs.length > 6 || segs.some((s) => !s || s[0] === ".") || !EXT.test(nom) || f.contenu.length > 1048576) continue;
      m.set(nom, f.contenu);
    }
    return m;
  }

  async function synchroniser() {
    if (occupe || typeof FadFS === "undefined") return;
    occupe = true;
    let touche = false;
    try {
      if (typeof FS_syncAvant === "function") FS_syncAvant();
      const dist = await api("GET", "/__espace");
      espace = dist.dossier;
      const distant = new Map(dist.fichiers.map((f) => [f.nom, f]));
      const local = fichiersLocaux();
      const noms = new Set([...local.keys(), ...distant.keys(), ...Object.keys(base)]);
      for (const nom of noms) {
        const l = local.get(nom), d = distant.get(nom), b = base[nom];
        const abs = FadFS.HOME + "/" + nom;
        const pousser = async (texte) => {
          const r = await api("POST", "/__espace/ecrire", { nom, texte });
          base[nom] = { h: empreinte(texte), mt: r.mt };
        };
        const tirer = async () => {
          const r = await api("GET", "/__espace/fichier?nom=" + encodeURIComponent(nom));
          if (parent(nom)) FadFS.mkdirP(FadFS.HOME + "/" + parent(nom));
          FadFS.ecrire(abs, r.texte, false);
          base[nom] = { h: empreinte(r.texte), mt: d.mt };
          touche = true;
        };
        if (l !== undefined && d === undefined) {
          if (!b || empreinte(l) !== b.h) await pousser(l); // nouveau (ou modifié) dans l'éditeur
          else { FadFS.supprimer(abs); delete base[nom]; touche = true; } // supprimé sur le disque
        } else if (l === undefined && d !== undefined) {
          if (!b) await tirer(); // nouveau sur le disque
          else { await api("POST", "/__espace/supprimer", { nom }); delete base[nom]; } // supprimé dans l'éditeur
        } else if (l !== undefined && d !== undefined) {
          const hl = empreinte(l);
          if (!b) {
            const r = await api("GET", "/__espace/fichier?nom=" + encodeURIComponent(nom));
            if (empreinte(r.texte) === hl) base[nom] = { h: hl, mt: d.mt };
            else conflit(nom);
          } else {
            const lc = hl !== b.h, rc = d.mt !== b.mt;
            if (lc && rc) conflit(nom);
            else if (lc) await pousser(l);
            else if (rc) await tirer();
          }
        } else if (b) delete base[nom]; // disparu des deux côtés
      }
      garder();
      if (touche && typeof FS_syncApres === "function") FS_syncApres();
      afficherPuce();
    } catch (e) {
      /* serveur arrêté ou fichier refusé : on réessaiera au prochain tour */
    } finally {
      occupe = false;
    }
  }
  function conflit(nom) {
    if (avertis[nom]) return;
    avertis[nom] = true;
    if (typeof ecrireTerminal === "function")
      ecrireTerminal("<span style='color:#f9e2af;'>⚠ « " + echap(nom) + " » a été modifié à la fois dans l'éditeur et sur le disque : aucune des deux versions n'est écrasée. Enregistrez-en une sous un autre nom, puis supprimez l'autre.</span>");
  }

  function afficherPuce() {
    let p = document.getElementById("statEspace");
    if (!p) {
      const ref = document.getElementById("statSauve");
      if (!ref || !ref.parentNode) return;
      p = document.createElement("span");
      p.id = "statEspace";
      p.style.marginLeft = "10px";
      ref.parentNode.insertBefore(p, ref.nextSibling);
    }
    p.textContent = "📁 Fadera";
    p.title = "Vos fichiers sont aussi dans ce dossier de l'ordinateur : " + espace;
  }

  // ---------------------------------------------------------------- terminal PowerShell / cmd
  const invite = () => (mode === "cmd" ? cwd + ">" : "PS " + cwd + ">");
  function majInvite() {
    const e = document.querySelector(".prompt-ps1");
    if (mode && e && e.textContent !== invite()) e.textContent = invite();
  }
  function sortie(texte, couleur) {
    if (!texte) return;
    const zone = document.getElementById("terminalContent");
    if (!zone) return;
    const d = document.createElement("div");
    d.className = "ligne-term";
    d.style.cssText = "white-space:pre-wrap;font-family:monospace;color:" + couleur;
    d.textContent = String(texte).replace(/\s+$/, "");
    zone.appendChild(d);
    zone.scrollTop = zone.scrollHeight;
  }
  function entrer(nouveau) {
    mode = nouveau;
    cwd = cwd || espace;
    const champ = document.getElementById("promptInput");
    if (champ) champ.placeholder = (mode === "cmd" ? "Commande cmd" : "Commande PowerShell") + " (exit pour revenir au terminal bash)";
    majInvite();
    ecrireTerminal("<span style='color:#89b4fa;'>" + (mode === "cmd" ? "Invite de commandes Windows" : "Windows PowerShell") + " — dossier de travail : " + echap(espace) + "<br>C'est le même dossier que celui de l'éditeur : vos fichiers .fa y sont, et « fadera run mon_programme.fa » fonctionne.<br>« editer fichier.fa » l'ouvre dans l'éditeur · « exit » revient au terminal bash.</span>");
  }
  function sortir() {
    mode = null;
    const champ = document.getElementById("promptInput");
    if (champ) champ.placeholder = "Commande Linux ou instruction Fadera (ls, cat script.fa, nano, fadera, aide...)";
    if (typeof majPS1 === "function") majPS1();
    ecrireTerminal("<span style='color:#a6e3a1;'>Retour au terminal bash.</span>");
  }
  // chemin d'un fichier du dossier de travail, relatif à celui-ci (ou null s'il est ailleurs)
  function relatifEspace(nom) {
    const norm = (p) => p.replace(/\\/g, "/").replace(/\/+$/, "");
    let abs = /^[A-Za-z]:|^\//.test(nom) ? norm(nom) : norm(cwd) + "/" + norm(nom);
    const base0 = norm(espace);
    if (abs.toLowerCase().indexOf(base0.toLowerCase() + "/") !== 0) return null;
    const segs = [];
    for (const s of abs.slice(base0.length + 1).split("/")) { if (s === "..") segs.pop(); else if (s && s !== ".") segs.push(s); }
    return segs.join("/");
  }
  async function executer(ligne, unCoup) {
    const shell = unCoup || mode;
    occupe = false;
    await synchroniser(); // l'éditeur → le disque, avant la commande
    ecrireTerminal("<span style='color:#89b4fa;font-weight:bold;'>" + echap(unCoup ? (shell === "cmd" ? cwd + ">" : "PS " + cwd + ">") : invite()) + "</span> <span style='color:#cdd6f4;'>" + echap(ligne) + "</span>");
    const m = /^(editer|éditer|ouvrir)\s+(.+)$/i.exec(ligne);
    if (m) {
      const rel = relatifEspace(m[2].trim().replace(/^["']|["']$/g, ""));
      if (!rel || typeof ouvrirDansEditeur !== "function") sortie("Ce fichier n'est pas dans le dossier de travail : " + espace, "#f38ba8");
      else {
        const r = await ouvrirDansEditeur(FadFS.HOME + "/" + rel, rel);
        if (r && r.err) sortie(r.err, "#f38ba8");
      }
      return;
    }
    let r;
    try {
      r = await api("POST", "/__terminal", { shell, commande: ligne, dossier: cwd || espace });
    } catch (e) {
      sortie("Impossible de joindre le terminal : " + e.message, "#f38ba8");
      return;
    }
    sortie(r.sortie, "#cdd6f4");
    sortie(r.erreur, "#f38ba8");
    if (!unCoup && r.dossier) cwd = r.dossier;
    majInvite();
    avertis = {};
    await synchroniser(); // le disque → l'éditeur, après la commande
  }

  window.addEventListener("keydown", async function (e) {
    const champ = e.target;
    if (!champ || champ.id !== "promptInput" || (e.key !== "Enter" && e.key !== "Tab")) return;
    if (mode && e.key === "Tab") { e.preventDefault(); e.stopImmediatePropagation(); return; }
    if (e.key !== "Enter") return;
    const ligne = champ.value.trim();
    if (!ligne) return;
    const demande = /^(powershell|pwsh|cmd)(?:\s+(.*))?$/i.exec(ligne);
    if (!mode && !demande) return; // une commande bash ou une ligne Fadéra : le terminal d'origine s'en occupe
    e.preventDefault();
    e.stopImmediatePropagation();
    champ.value = "";
    if (window.faderaHistory) { window.faderaHistory.push(ligne); window.faderaHistoryIndex = -1; }
    if (!espace) {
      await synchroniser();
      if (!espace) { ecrireTerminal("<span style='color:#f38ba8;'>Le terminal Windows n'est pas disponible (serveur local injoignable).</span>"); return; }
    }
    if (mode) {
      if (/^(exit|quitter)$/i.test(ligne)) return sortir();
      if (/^(cls|clear)$/i.test(ligne)) { const z = document.getElementById("terminalContent"); if (z) z.innerHTML = ""; return; }
    }
    if (window.faderaReplOccupe) return;
    window.faderaReplOccupe = true;
    try {
      if (!mode && demande) {
        const quel = demande[1].toLowerCase() === "cmd" ? "cmd" : "powershell";
        if (demande[2]) { cwd = cwd || espace; await executer(demande[2], quel); }
        else entrer(quel);
      } else await executer(ligne);
    } finally {
      window.faderaReplOccupe = false;
      champ.focus();
    }
  }, true);

  // ---------------------------------------------------------------- démarrage
  let essais = 0;
  const attente = setInterval(() => {
    if (typeof FadFS !== "undefined" && typeof ecrireTerminal === "function" && document.getElementById("promptInput")) {
      clearInterval(attente);
      synchroniser();
      setInterval(() => { if (!document.hidden) synchroniser(); }, 3000);
      window.addEventListener("focus", () => synchroniser());
      const ps1 = document.querySelector(".prompt-ps1");
      if (ps1 && window.MutationObserver) new MutationObserver(majInvite).observe(ps1, { childList: true, characterData: true, subtree: true });
      window.faderaLocal = { synchroniser, executer: (ligne, shell) => api("POST", "/__terminal", { shell: shell || "powershell", commande: ligne, dossier: espace }) };
    } else if (++essais > 300) clearInterval(attente);
  }, 100);
})();

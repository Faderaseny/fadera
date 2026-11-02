// Correcteur : exécute le code de l'élève sur chaque test et compare les lignes affichées.
// `executer(code, saisies)` est la fonction du moteur Fadéra (WebAssembly) : elle renvoie un JSON
// {"sortie":[...], "erreur": null | "Ligne N : …", "source": "…"}.
(function (racine) {
  const normaliser = (l) => String(l).replace(/\s+$/, "");
  function corriger(exercice, code, executer) {
    const details = [];
    for (const t of exercice.tests) {
      let r;
      try { r = JSON.parse(executer(code, t.saisies.slice())); }
      catch (e) { return { reussi: false, erreur: "Le moteur n'a pas pu exécuter le programme (" + e + ").", details }; }
      if (r.erreur) return { reussi: false, erreur: String(r.erreur), source: r.source || "", details };
      const obtenu = (r.sortie || []).map(normaliser);
      const attendu = t.sortie.map(normaliser);
      const ok = obtenu.length === attendu.length && obtenu.every((l, i) => l === attendu[i]);
      details.push({ saisies: t.saisies, attendu, obtenu, ok });
    }
    return { reussi: details.every((d) => d.ok), erreur: null, details };
  }
  if (typeof module !== "undefined" && module.exports) module.exports = { corriger };
  else racine.Correcteur = { corriger };
})(typeof window !== "undefined" ? window : globalThis);

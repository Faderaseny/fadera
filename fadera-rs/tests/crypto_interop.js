#!/usr/bin/env node
// Interopérabilité du chiffrement : ce que le moteur JS (Web Crypto) chiffre, le moteur Rust le déchiffre, et inversement.
const fs = require("fs"), path = require("path"), cp = require("child_process"), os = require("os");
const racine = path.resolve(__dirname, "..");
// dossier contenant index.html, tests_fadera.js, … : FADERA_DOSSIER (défini par verifier_tout.js), sinon le dossier parent du projet
const DOSSIER_LIVRES = process.env.FADERA_DOSSIER ? path.resolve(process.env.FADERA_DOSSIER) : path.join(racine, "..");
const html = fs.readFileSync(path.resolve(process.argv[2] || path.join(DOSSIER_LIVRES, "index.html")), "utf8");
const bin = path.resolve(process.argv[3] || path.join(racine, "target", "release", "fadera"));
const a = html.indexOf("      const TYPE = {"), b = html.indexOf("      // ---------- MODALE / TOAST");
const M = new Function("module", "require", "window", "globalThis",
  html.slice(a, b) + "\nreturn {executerProgramme};")({ exports: {} }, require, undefined, globalThis);
const js = async (code) => { const o = []; await M.executerProgramme(code, { afficher: (t) => o.push(t) }); return o.join("\n"); };
const rs = (code) => cp.spawnSync(bin, ["-e", code], { encoding: "utf8" }).stdout.trim();
let ok = 0, ko = 0;
const verif = (nom, bon) => { bon ? ok++ : ko++; console.log((bon ? "✓ " : "✗ ") + nom); };
(async () => {
  const msg = "Message secret — éàü 🔐 " + "x".repeat(300);
  // AES-256-GCM
  const cJs = await js(`afficher chiffrer("${msg}", "mot de passe")`);
  verif("AES : JS chiffre → Rust déchiffre", rs(`afficher dechiffrer("${cJs}", "mot de passe")`) === msg);
  const cRs = rs(`afficher chiffrer("${msg}", "mot de passe")`);
  verif("AES : Rust chiffre → JS déchiffre", (await js(`afficher dechiffrer("${cRs}", "mot de passe")`)) === msg);
  // RSA-OAEP hybride
  const cles = JSON.parse(await js(`afficher json_texte(crypto_generer_cles_rsa())`));
  const c1 = await js(`afficher crypto_chiffrer_asymetrique("${msg}", "${cles.cle_publique}")`);
  verif("RSA : JS chiffre → Rust déchiffre", rs(`afficher crypto_dechiffrer_asymetrique("${c1}", "${cles.cle_privee}")`) === msg);
  const c2 = rs(`afficher crypto_chiffrer_asymetrique("${msg}", "${cles.cle_publique}")`);
  verif("RSA : Rust chiffre → JS déchiffre", (await js(`afficher crypto_dechiffrer_asymetrique("${c2}", "${cles.cle_privee}")`)) === msg);
  const kRs = JSON.parse(rs(`afficher json_texte(crypto_generer_cles_rsa())`));
  const c3 = await js(`afficher crypto_chiffrer_asymetrique("abc", "${kRs.cle_publique}")`);
  verif("RSA : clés générées par Rust utilisables par JS", rs(`afficher crypto_dechiffrer_asymetrique("${c3}", "${kRs.cle_privee}")`) === "abc");
  // hachages
  for (const [n, f] of [["sha256", 'crypto_hacher_sha256("é")'], ["sha512", 'crypto_hacher_sha512("é")'], ["hmac", 'crypto_hmac_sha256("m", "k")']])
    verif("Hachage " + n + " identique", (await js("afficher " + f)) === rs("afficher " + f));
  console.log(`\n${ok} réussi(s), ${ko} échoué(s).`);
  process.exit(ko ? 1 : 0);
})();

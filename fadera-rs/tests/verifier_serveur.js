#!/usr/bin/env node
// Vérifie le mini-site d'exemple (applications/mini-site) : un vrai serveur Fadéra, interrogé par HTTP.
//   usage : node verifier_serveur.js <fadera> [dossier du mini-site]    (code 3 = exemple absent : étape ignorée)
const cp = require("child_process"), fs = require("fs"), os = require("os"), path = require("path"), http = require("http"), net = require("net");
const bin = process.argv[2], app = path.resolve(process.argv[3] || path.join(__dirname, "..", "..", "applications", "mini-site"));
if (!fs.existsSync(path.join(app, "site.fa"))) { console.log("mini-site absent : " + app); process.exit(3); }
let ok = 0, ko = 0;
const verif = (c, msg) => { if (c) ok++; else { ko++; console.log("✗ " + msg); } };
const req = (port, method, p, body) => new Promise((r) => {
  const q = http.request({ port, method, path: p, agent: false }, (res) => { const c = []; res.on("data", (x) => c.push(x)); res.on("end", () => { const t = Buffer.concat(c).toString(); let j = null; try { j = JSON.parse(t); } catch (e) {} r({ st: res.statusCode, t, j, h: res.headers }); }); });
  q.on("error", () => r({ st: 0, t: "", j: null, h: {} })); if (body) q.write(body); q.end();
});
const libre = () => new Promise((r) => { const s = net.createServer(); s.listen(0, () => { const p = s.address().port; s.close(() => r(p)); }); });
(async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "minisite-")), fichier = path.join(dir, "taches.json"); let srv, port;
  const demarrer = async () => { port = await libre(); srv = cp.spawn(bin, ["site.fa"], { cwd: app, env: { ...process.env, PORT: String(port), DONNEES_TACHES: fichier } }); for (let i = 0; i < 100; i++) { if ((await req(port, "GET", "/")).st === 200) return true; await new Promise((r) => setTimeout(r, 100)); } return false; };
  verif(await demarrer(), "le serveur démarre");
  const page = await req(port, "GET", "/"); verif(/text\/html/.test(page.h["content-type"] || "") && page.t.includes("Mes tâches"), "la page d'accueil est servie (HTML)");
  verif(JSON.stringify((await req(port, "GET", "/api/taches")).j) === '{"taches":[]}', "liste vide au départ");
  verif((await req(port, "POST", "/api/taches", '{"texte":"  Acheter du pain "}')).st === 200, "ajout d'une tâche");
  verif((await req(port, "POST", "/api/taches", '{"texte":""}')).st === 400, "tâche vide refusée");
  verif((await req(port, "POST", "/api/taches", '{"texte":"' + "x".repeat(201) + '"}')).st === 400, "tâche trop longue refusée");
  let l = (await req(port, "GET", "/api/taches")).j.taches; verif(l.length === 1 && l[0].texte === "Acheter du pain" && l[0].fait === false && l[0].id === 1, "la tâche est enregistrée (espaces retirés)");
  verif((await req(port, "POST", "/api/taches/basculer", '{"id":1}')).st === 200, "tâche terminée");
  verif((await req(port, "POST", "/api/taches/basculer", '{"id":99}')).st === 404, "tâche inconnue : 404");
  verif((await req(port, "GET", "/nimporte/quoi")).st === 404, "page inconnue : 404");
  verif((await req(port, "POST", "/api/taches", "pas du json")).st === 400, "corps invalide : erreur 400, le serveur continue");
  verif((await req(port, "GET", "/api/taches")).st === 200, "le serveur répond encore après une erreur");
  srv.kill(); await new Promise((r) => setTimeout(r, 400));
  verif(await demarrer(), "redémarrage");
  l = (await req(port, "GET", "/api/taches")).j.taches; verif(l.length === 1 && l[0].fait === true, "les tâches survivent au redémarrage");
  srv.kill(); fs.rmSync(dir, { recursive: true, force: true });
  console.log(ko ? "✗ " + ko + " vérification(s) échouée(s), " + ok + " réussie(s)." : "✓ Mini-site en Fadéra : " + ok + " vérification(s) réussie(s), 0 échouée(s).");
  process.exit(ko ? 1 : 0);
})();

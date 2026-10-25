//! Requêtes HTTP(S) de la ligne de commande (le navigateur utilise fetch() à la place).
//! Sert aux fonctions http_obtenir / http_json / http_envoyer et au gestionnaire de paquets.
use std::io::Read;
use std::time::Duration;

/// Taille maximale d'une réponse lue (protège la mémoire contre un serveur qui n'en finit pas).
const MAX_OCTETS: u64 = 20 * 1024 * 1024;

/// Envoie une requête et renvoie (statut, corps). Un statut 4xx/5xx n'est pas une erreur ici : c'est une réponse
/// (comme avec fetch). Seules les pannes (réseau coupé, DNS, TLS, délai dépassé) sont des erreurs.
pub fn requete(methode: &str, url: &str, corps: Option<&str>, entetes: &[(String, String)]) -> Result<(u16, String), String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(20))
        .user_agent(concat!("fadera/", env!("CARGO_PKG_VERSION")))
        .build();
    let mut r = agent.request(methode, url);
    for (k, v) in entetes {
        r = r.set(k, v);
    }
    let rep = match corps {
        Some(c) => r.send_string(c),
        None => r.call(),
    };
    let rep = match rep {
        Ok(r) => r,
        Err(ureq::Error::Status(_, r)) => r,
        Err(e) => return Err(e.to_string()),
    };
    let statut = rep.status();
    let mut texte = String::new();
    rep.into_reader().take(MAX_OCTETS).read_to_string(&mut texte).map_err(|e| e.to_string())?;
    Ok((statut, texte))
}

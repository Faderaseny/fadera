//! Cryptographie. SHA-256 est implémenté ici (sans dépendance) ; le reste (AES-256-GCM, RSA-OAEP,
//! SHA-512, HMAC) utilise des bibliothèques auditées sous la fonctionnalité « crypto ».
//! Les formats de sortie sont identiques à ceux du moteur JavaScript (interopérables).

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98,
        0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
        0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8,
        0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819,
        0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
        0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] =
        [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut msg = data.to_vec();
    let bits = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bits.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (x, y) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *x = x.wrapping_add(y);
        }
    }
    let mut out = [0u8; 32];
    for (i, x) in h.iter().enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&x.to_be_bytes());
    }
    out
}

pub fn sha256_hex(s: &str) -> String {
    hex(&sha256(s.as_bytes()))
}

#[cfg(feature = "crypto")]
mod reel {
    use super::hex;
    use aes_gcm::aead::{Aead, KeyInit};
    use aes_gcm::{Aes256Gcm, Key, Nonce};
    use base64::engine::general_purpose::STANDARD as B64;
    use base64::Engine;
    use hmac::{Hmac, Mac};
    use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
    use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
    use sha2::{Digest, Sha256, Sha512};

    fn alea(n: usize) -> Result<Vec<u8>, String> {
        let mut b = vec![0u8; n];
        getrandom::getrandom(&mut b).map_err(|e| format!("générateur aléatoire indisponible ({}).", e))?;
        Ok(b)
    }
    fn derive(mdp: &str, sel: &[u8]) -> [u8; 32] {
        let mut k = [0u8; 32];
        pbkdf2::pbkdf2_hmac::<Sha256>(mdp.as_bytes(), sel, 600_000, &mut k);
        k
    }
    fn depuis_b64(s: &str) -> Result<Vec<u8>, String> {
        let t: String = s.chars().filter(|c| !c.is_whitespace() && *c != '=').collect();
        base64::engine::general_purpose::STANDARD_NO_PAD
            .decode(t)
            .map_err(|_| "données illisibles (base64 invalide).".to_string())
    }

    pub fn chiffrer_sym(texte: &str, cle: &str) -> Result<String, String> {
        if cle.is_empty() {
            return Err("une clé (mot de passe) non vide est requise.".into());
        }
        let sel = alea(16)?;
        let iv = alea(12)?;
        let k = derive(cle, &sel);
        let c = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&k));
        let ct = c.encrypt(Nonce::from_slice(&iv), texte.as_bytes()).map_err(|_| "chiffrement impossible.".to_string())?;
        let mut sortie = sel;
        sortie.extend_from_slice(&iv);
        sortie.extend_from_slice(&ct);
        Ok(format!("AES256GCM_{}", B64.encode(sortie)))
    }

    pub fn dechiffrer_sym(chiffre: &str, cle: &str) -> Result<String, String> {
        let reste = chiffre
            .strip_prefix("AES256GCM_")
            .ok_or("format invalide : ce texte n'a pas été produit par crypto_chiffrer_symetrique.")?;
        let data = depuis_b64(reste)?;
        if data.len() < 44 {
            return Err("données chiffrées tronquées.".into());
        }
        let k = derive(cle, &data[..16]);
        let c = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&k));
        let clair = c
            .decrypt(Nonce::from_slice(&data[16..28]), &data[28..])
            .map_err(|_| "déchiffrement impossible : mauvaise clé ou données altérées.".to_string())?;
        Ok(String::from_utf8_lossy(&clair).into_owned())
    }

    pub fn generer_cles_rsa() -> Result<(String, String), String> {
        let mut rng = rand::thread_rng();
        let sk = RsaPrivateKey::new(&mut rng, 2048).map_err(|e| e.to_string())?;
        let pk = RsaPublicKey::from(&sk);
        let pubd = pk.to_public_key_der().map_err(|e| e.to_string())?;
        let privd = sk.to_pkcs8_der().map_err(|e| e.to_string())?;
        Ok((format!("FAD_PUB_{}", B64.encode(pubd.as_bytes())), format!("FAD_PRIV_{}", B64.encode(privd.as_bytes()))))
    }

    pub fn chiffrer_asym(texte: &str, cle_pub: &str) -> Result<String, String> {
        let r = cle_pub.strip_prefix("FAD_PUB_").ok_or("clé publique invalide (elle doit commencer par FAD_PUB_).")?;
        let der = depuis_b64(r).map_err(|_| "clé publique illisible ou corrompue.".to_string())?;
        let pk = RsaPublicKey::from_public_key_der(&der).map_err(|_| "clé publique illisible ou corrompue.".to_string())?;
        let aes = alea(32)?;
        let iv = alea(12)?;
        let c = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&aes));
        let ct = c.encrypt(Nonce::from_slice(&iv), texte.as_bytes()).map_err(|_| "chiffrement impossible.".to_string())?;
        let mut rng = rand::thread_rng();
        let emb = pk.encrypt(&mut rng, Oaep::new::<Sha256>(), &aes).map_err(|e| e.to_string())?;
        let mut sortie = vec![(emb.len() >> 8) as u8, (emb.len() & 255) as u8];
        sortie.extend_from_slice(&emb);
        sortie.extend_from_slice(&iv);
        sortie.extend_from_slice(&ct);
        Ok(format!("RSAHYB_{}", B64.encode(sortie)))
    }

    pub fn dechiffrer_asym(chiffre: &str, cle_priv: &str) -> Result<String, String> {
        let r = chiffre.strip_prefix("RSAHYB_").ok_or("format invalide : ce texte n'a pas été produit par crypto_chiffrer_asymetrique.")?;
        let rp = cle_priv.strip_prefix("FAD_PRIV_").ok_or("clé privée invalide (elle doit commencer par FAD_PRIV_).")?;
        let data = depuis_b64(r)?;
        let der = depuis_b64(rp).map_err(|_| "clé privée illisible ou corrompue.".to_string())?;
        let sk = RsaPrivateKey::from_pkcs8_der(&der).map_err(|_| "clé privée illisible ou corrompue.".to_string())?;
        let ech = || "déchiffrement impossible : ce n'est pas la clé privée correspondante, ou les données sont altérées.".to_string();
        if data.len() < 2 {
            return Err(ech());
        }
        let lg = ((data[0] as usize) << 8) | data[1] as usize;
        if data.len() < 14 + lg {
            return Err(ech());
        }
        let aes = sk.decrypt(Oaep::new::<Sha256>(), &data[2..2 + lg]).map_err(|_| ech())?;
        if aes.len() != 32 {
            return Err(ech());
        }
        let c = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&aes));
        let clair = c.decrypt(Nonce::from_slice(&data[2 + lg..14 + lg]), &data[14 + lg..]).map_err(|_| ech())?;
        Ok(String::from_utf8_lossy(&clair).into_owned())
    }

    pub fn sha512_hex(s: &str) -> Result<String, String> {
        Ok(hex(&Sha512::digest(s.as_bytes())))
    }
    pub fn hmac_sha256_hex(texte: &str, cle: &str) -> Result<String, String> {
        let mut m = <Hmac<Sha256> as Mac>::new_from_slice(cle.as_bytes()).map_err(|e| e.to_string())?;
        m.update(texte.as_bytes());
        Ok(hex(&m.finalize().into_bytes()))
    }
    pub fn aleatoire_hex(n: usize) -> Result<String, String> {
        Ok(hex(&alea(n)?))
    }
}

#[cfg(feature = "crypto")]
pub use reel::*;

#[cfg(not(feature = "crypto"))]
mod absent {
    fn non<T>() -> Result<T, String> {
        Err("cette version de Fadéra a été compilée sans la fonctionnalité « crypto ».".into())
    }
    pub fn chiffrer_sym(_: &str, _: &str) -> Result<String, String> { non() }
    pub fn dechiffrer_sym(_: &str, _: &str) -> Result<String, String> { non() }
    pub fn generer_cles_rsa() -> Result<(String, String), String> { non() }
    pub fn chiffrer_asym(_: &str, _: &str) -> Result<String, String> { non() }
    pub fn dechiffrer_asym(_: &str, _: &str) -> Result<String, String> { non() }
    pub fn sha512_hex(_: &str) -> Result<String, String> { non() }
    pub fn hmac_sha256_hex(_: &str, _: &str) -> Result<String, String> { non() }
    pub fn aleatoire_hex(_: usize) -> Result<String, String> { non() }
}
#[cfg(not(feature = "crypto"))]
pub use absent::*;

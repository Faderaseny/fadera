use std::process::Command;

fn fadera() -> Command {
    Command::new(env!("CARGO_BIN_EXE_fadera"))
}

fn ecrire_tmp(nom: &str, contenu: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("fadera_cli_{}_{}", std::process::id(), nom));
    std::fs::write(&p, contenu).unwrap();
    p
}

#[test]
fn run_execute_un_fichier() {
    let f = ecrire_tmp("a.fa", "soit x = 40\r\nafficher \"é \" + (x + 2)\r\n");
    let s = fadera().arg("run").arg(&f).output().unwrap();
    assert!(s.status.success());
    assert_eq!(String::from_utf8_lossy(&s.stdout).trim(), "é 42");
}

#[test]
fn run_sans_fichier_echoue_proprement() {
    let s = fadera().arg("run").output().unwrap();
    assert_eq!(s.status.code(), Some(2));
}

#[test]
fn run_fichier_absent_code_2() {
    let s = fadera().args(["run", "/inexistant/zzz.fa"]).output().unwrap();
    assert_eq!(s.status.code(), Some(2));
}

#[test]
fn run_erreur_de_programme_code_non_nul() {
    let f = ecrire_tmp("b.fa", "afficher 1 / 0 +\n");
    let s = fadera().arg("run").arg(&f).output().unwrap();
    assert!(!s.status.success());
}

#[test]
fn run_importe_un_fichier_voisin() {
    let d = std::env::temp_dir().join(format!("fadera_cli_dir_{}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    std::fs::write(d.join("outils.fa"), "fonction double(x)\n renvoyer x * 2\nfin\n").unwrap();
    std::fs::write(d.join("main.fa"), "importer \"outils\"\nafficher double(21)\n").unwrap();
    let s = fadera().arg("run").arg(d.join("main.fa")).output().unwrap();
    assert_eq!(String::from_utf8_lossy(&s.stdout).trim(), "42");
}

#[test]
fn version_et_aide() {
    let v = fadera().arg("--version").output().unwrap();
    assert!(String::from_utf8_lossy(&v.stdout).contains(env!("CARGO_PKG_VERSION")));
    let h = fadera().arg("--aide").output().unwrap();
    assert!(String::from_utf8_lossy(&h.stdout).contains("fadera run"));
}

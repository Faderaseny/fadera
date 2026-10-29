// Icône et informations de version de « Fadera-IDE.exe » (Windows seulement).
fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../windows/fadera.ico");
        res.set("ProductName", "Fadéra IDE");
        res.set("FileDescription", "Fadéra IDE");
        res.set("CompanyName", "Faderaseny");
        if let Err(e) = res.compile() {
            // l'icône est un plus : ne jamais empêcher la compilation
            println!("cargo:warning=icône non intégrée : {}", e);
        }
    }
}

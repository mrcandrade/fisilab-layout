//! Windows only: embed the app icon and version info (VERSIONINFO) into `fisilab-layout.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `DESIGNCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/fisilab/fisilab-layout.ico");
    println!("cargo:rerun-if-env-changed=DESIGNCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/fisilab/fisilab-layout.ico")
        .set("ProductName", "FisiLab Layout")
        .set("FileDescription", "Layout — Diagramação e editoração (Suíte FisiLab)")
        .set("CompanyName", "FisiLab — LARFIC/UFRGS")
        .set(
            "LegalCopyright",
            "Baseado no DesignCraft (© ArtCraft Team e contribuidores). Modificações © 2026 FisiLab — LARFIC/UFRGS. MIT OR Apache-2.0.",
        )
        .set("OriginalFilename", "fisilab-layout.exe")
        .set("InternalName", "fisilab-layout");
    if let Err(e) = res.compile() {
        if std::env::var_os("DESIGNCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=fisilab-layout.exe built without icon/version resources: {e}");
    }
}

//! Windows only: embed the app icon and version info (VERSIONINFO) into `pulso.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PULSO_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/pulso.ico");
    println!("cargo:rerun-if-env-changed=PULSO_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/pulso.ico")
        .set("ProductName", "Pulso")
        .set("FileDescription", "Pulso motion graphics and visual effects")
        .set("LegalCopyright", "Copyright (c) 2026 The EffectCraft contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "pulso.exe")
        .set("InternalName", "pulso");
    if let Err(e) = res.compile() {
        if std::env::var_os("PULSO_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=pulso.exe built without icon/version resources: {e}");
    }
}

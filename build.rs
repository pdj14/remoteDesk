#[cfg(windows)]
fn build_windows() {
    let file = "src/platform/windows.cc";
    let file2 = "src/platform/windows_delete_test_cert.cc";
    cc::Build::new().file(file).file(file2).compile("windows");
    println!("cargo:rustc-link-lib=WtsApi32");
    println!("cargo:rerun-if-changed={}", file);
    println!("cargo:rerun-if-changed={}", file2);
}

#[cfg(windows)]
fn sync_sciter_dll() {
    use std::{env, fs, path::PathBuf};

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_owned());
    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.join("target"));
    let profile_dir = target_dir.join(&profile);

    let src_candidates = [
        profile_dir.join("sciter.dll"),
        manifest_dir.join("sciter.dll"),
        manifest_dir.join("res").join("sciter.dll"),
    ];
    let Some(src) = src_candidates.into_iter().find(|p| p.exists()) else {
        println!(
            "cargo:warning=sciter.dll not found. Place it at target/{profile}/sciter.dll or repo root."
        );
        return;
    };

    let dst_candidates = [
        profile_dir.join("sciter.dll"),
        profile_dir.join("deps").join("sciter.dll"),
    ];
    for dst in dst_candidates {
        if src == dst {
            continue;
        }
        if let Some(parent) = dst.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Err(err) = fs::copy(&src, &dst) {
            println!(
                "cargo:warning=Failed to copy sciter.dll from {} to {}: {}",
                src.display(),
                dst.display(),
                err
            );
        }
    }
}

#[cfg(target_os = "macos")]
fn build_mac() {
    let file = "src/platform/macos.mm";
    let mut b = cc::Build::new();
    if let Ok(os_version::OsVersion::MacOS(v)) = os_version::detect() {
        let v = v.version;
        if v.contains("10.14") {
            b.flag("-DNO_InputMonitoringAuthStatus=1");
        }
    }
    b.flag("-std=c++17").file(file).compile("macos");
    println!("cargo:rerun-if-changed={}", file);
}

#[cfg(windows)]
fn build_winres() {
    use std::io::Write;
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/icon.ico")
        .set_language(winapi::um::winnt::MAKELANGID(
            winapi::um::winnt::LANG_ENGLISH,
            winapi::um::winnt::SUBLANG_ENGLISH_US,
        ))
        .set("ProductName", "RemoteDesk")
        .set("FileDescription", "RemoteDesk Remote Desktop")
        .set("OriginalFilename", "remotedesk.exe")
        .set("InternalName", "remotedesk");

    #[cfg(feature = "inline")]
    if std::env::var("PROFILE").unwrap() == "release" {
        res.set_manifest_file("res/manifest.xml");
    }

    match res.compile() {
        Err(e) => {
            write!(std::io::stderr(), "{}", e).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

fn install_android_deps() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "android" {
        return;
    }
    let mut target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "x86_64" {
        target_arch = "x64".to_owned();
    } else if target_arch == "x86" {
        target_arch = "x86".to_owned();
    } else if target_arch == "aarch64" {
        target_arch = "arm64".to_owned();
    } else {
        target_arch = "arm".to_owned();
    }
    let target = format!("{}-android", target_arch);
    let vcpkg_root = std::env::var("VCPKG_ROOT").unwrap();
    let mut path: std::path::PathBuf = vcpkg_root.into();
    if let Ok(vcpkg_root) = std::env::var("VCPKG_INSTALLED_ROOT") {
        path = vcpkg_root.into();
    } else {
        path.push("installed");
    }
    path.push(target);
    println!(
        "cargo:rustc-link-search={}",
        path.join("lib").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=ndk_compat");
    println!("cargo:rustc-link-lib=oboe");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=OpenSLES");
}

fn main() {
    hbb_common::gen_version();
    install_android_deps();
    #[cfg(windows)]
    build_winres();
    #[cfg(windows)]
    sync_sciter_dll();
    #[cfg(windows)]
    build_windows();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        #[cfg(target_os = "macos")]
        build_mac();
        println!("cargo:rustc-link-lib=framework=ApplicationServices");
    }
    println!("cargo:rerun-if-changed=build.rs");
}

use std::{
    io::Write,
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use hbb_common::{bail, ResultType};
use librustdesk::{common, platform, VERSION};

const EXIT_OK: i32 = 0;
const EXIT_FAILED: i32 = 1;
const EXIT_USAGE: i32 = 2;
#[cfg(not(windows))]
const EXIT_UNSUPPORTED: i32 = 4;

fn print_help() {
    println!("RemoteDesk Install/Admin CLI");
    println!();
    println!("Usage:");
    println!("  remotedesk-admin.exe --status");
    println!("  remotedesk-admin.exe --install");
    println!("  remotedesk-admin.exe --uninstall");
    println!("  remotedesk-admin.exe --version");
    println!();
    println!("Notes:");
    println!("  - --install performs installed-mode setup (same flow as remotedesk.exe --silent-install).");
    println!("  - --uninstall removes installed RemoteDesk (GUI/CLI/Admin together).");
}

fn run_with_progress<F>(label: &str, task: F) -> i32
where
    F: FnOnce() -> ResultType<()>,
{
    let running = Arc::new(AtomicBool::new(true));
    let ticker_running = running.clone();
    let started = Instant::now();
    let label_text = label.to_owned();
    let ticker = thread::spawn(move || {
        while ticker_running.load(Ordering::Relaxed) {
            let elapsed = started.elapsed().as_secs();
            print!("\r[{label_text}] running... {elapsed}s");
            let _ = std::io::stdout().flush();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let result = task();
    running.store(false, Ordering::Relaxed);
    let _ = ticker.join();
    println!();

    match result {
        Ok(()) => {
            println!(
                "[{}] completed in {}s",
                label,
                started.elapsed().as_secs()
            );
            EXIT_OK
        }
        Err(err) => {
            eprintln!("[{}] failed: {}", label, err);
            EXIT_FAILED
        }
    }
}

#[cfg(windows)]
fn install_target_path_from_admin_dir() -> ResultType<String> {
    let exe = std::env::current_exe()?;
    let Some(parent) = exe.parent() else {
        bail!(
            "Cannot resolve installer directory from current executable: {}",
            exe.to_string_lossy()
        );
    };
    let path = parent.join(common::get_app_name());
    let Some(path) = path.to_str() else {
        bail!("Failed to convert install path to string");
    };
    Ok(path.to_owned())
}

#[cfg(windows)]
fn install() -> i32 {
    run_with_progress("install", || {
        let options = "desktopicon startmenu printer";
        let path = install_target_path_from_admin_dir()?;
        platform::install_me(options, path, true, false)
    })
}

#[cfg(windows)]
fn uninstall() -> i32 {
    run_with_progress("uninstall", || platform::uninstall_me(true))
}

#[cfg(windows)]
fn status() -> i32 {
    let (_, install_path, _, install_exe) = platform::get_install_info();
    println!("installed: {}", platform::is_installed());
    println!("service_running: {}", platform::is_self_service_running());
    println!("install_path: {}", install_path);
    println!("service_binary: {}", install_exe);
    EXIT_OK
}

#[cfg(not(windows))]
fn install() -> i32 {
    eprintln!("remotedesk-admin is only supported on Windows.");
    EXIT_UNSUPPORTED
}

#[cfg(not(windows))]
fn uninstall() -> i32 {
    eprintln!("remotedesk-admin is only supported on Windows.");
    EXIT_UNSUPPORTED
}

#[cfg(not(windows))]
fn status() -> i32 {
    eprintln!("remotedesk-admin is only supported on Windows.");
    EXIT_UNSUPPORTED
}

fn main() {
    if !common::global_init() {
        eprintln!("Global initialization failed.");
        exit(EXIT_FAILED);
    }

    let args: Vec<String> = std::env::args().skip(1).collect();
    let exit_code = if args.is_empty() {
        print_help();
        EXIT_USAGE
    } else {
        match args[0].as_str() {
            "--help" | "-h" => {
                print_help();
                EXIT_OK
            }
            "--version" => {
                println!("{}", VERSION);
                EXIT_OK
            }
            "--status" => status(),
            "--install" => install(),
            "--uninstall" => uninstall(),
            cmd => {
                eprintln!("Unsupported argument: {cmd}");
                print_help();
                EXIT_USAGE
            }
        }
    };

    common::global_clean();
    exit(exit_code);
}

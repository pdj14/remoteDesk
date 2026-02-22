use std::process::{exit, Command};

use librustdesk::{common, platform, VERSION};

const EXIT_OK: i32 = 0;
const EXIT_FAILED: i32 = 1;
const EXIT_USAGE: i32 = 2;
const EXIT_NOT_INSTALLED: i32 = 3;

fn print_help() {
    let app_name = common::get_app_name().to_lowercase();
    println!("RemoteDesk Service CLI");
    println!();
    println!("Usage:");
    println!("  remotedesk-cli.exe --status");
    println!("  remotedesk-cli.exe --install-service");
    println!("  remotedesk-cli.exe --uninstall-service");
    println!("  remotedesk-cli.exe --start-service");
    println!("  remotedesk-cli.exe --stop-service");
    println!("  remotedesk-cli.exe --version");
    println!();
    println!("Notes:");
    println!("  - This CLI only works from an installed path.");
    println!(
        "  - Install first with: {}-admin.exe --install (recommended) or {}.exe --silent-install",
        app_name, app_name
    );
}

#[cfg(windows)]
fn ensure_installed_only() -> Result<(), String> {
    if !platform::is_installed() {
        return Err(
            "This CLI only works for installed RemoteDesk. Run remotedesk-admin.exe --install first."
                .to_owned(),
        );
    }
    if !platform::is_cur_exe_the_installed() {
        let (_, install_path, _, _) = platform::get_install_info();
        let expected = format!(
            "{}\\{}-cli.exe",
            install_path,
            common::get_app_name().to_lowercase()
        );
        return Err(format!(
            "This CLI must be launched from installed path. Expected: {expected}"
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn run_sc(action: &str) -> i32 {
    let app_name = common::get_app_name();
    match Command::new("sc").arg(action).arg(&app_name).status() {
        Ok(status) => status.code().unwrap_or(EXIT_FAILED),
        Err(err) => {
            eprintln!("Failed to run 'sc {action} {app_name}': {err}");
            EXIT_FAILED
        }
    }
}

#[cfg(windows)]
fn run_command(cmd: &str) -> i32 {
    match cmd {
        "--status" => {
            let (_, install_path, _, install_exe) = platform::get_install_info();
            println!("installed: {}", platform::is_installed());
            println!("service_running: {}", platform::is_self_service_running());
            println!("install_path: {}", install_path);
            println!("service_binary: {}", install_exe);
            EXIT_OK
        }
        "--install-service" => {
            if platform::install_service() {
                eprintln!("Service install failed.");
                EXIT_FAILED
            } else {
                EXIT_OK
            }
        }
        "--uninstall-service" => {
            if platform::uninstall_service(false, true) {
                eprintln!("Service uninstall failed.");
                EXIT_FAILED
            } else {
                EXIT_OK
            }
        }
        "--start-service" => run_sc("start"),
        "--stop-service" => run_sc("stop"),
        _ => EXIT_USAGE,
    }
}

#[cfg(not(windows))]
fn run_command(_cmd: &str) -> i32 {
    eprintln!("remotedesk-cli is only supported on Windows.");
    4
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
        let cmd = args[0].as_str();
        match cmd {
            "--help" | "-h" => {
                print_help();
                EXIT_OK
            }
            "--version" => {
                println!("{}", VERSION);
                EXIT_OK
            }
            _ => {
                #[cfg(windows)]
                {
                    if let Err(msg) = ensure_installed_only() {
                        eprintln!("{msg}");
                        common::global_clean();
                        exit(EXIT_NOT_INSTALLED);
                    }
                }
                let code = run_command(cmd);
                if code == EXIT_USAGE {
                    eprintln!("Unsupported argument: {cmd}");
                    print_help();
                }
                code
            }
        }
    };

    common::global_clean();
    exit(exit_code);
}

use argh::FromArgs;
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(FromArgs)]
/// Clear KDE Clipboard. Artyom Nazarov, Orenburg, 2025
struct Cli {
    /// clear the clipboard
    #[argh(switch, short = 'c')]
    clear: bool,
    
    /// copy space character to clipboard
    #[argh(switch, short = 's')]
    use_space: bool,
}

fn wait_with_timeout(child: &mut std::process::Child, timeout_secs: u64) -> bool {
    let timeout = Duration::from_secs(timeout_secs);
    let start = Instant::now();
    
    while start.elapsed() < timeout {
        match child.try_wait() {
            Ok(Some(_status)) => return true,
            Ok(None) => {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(e) => {
                eprintln!("Error waiting for process: {}", e);
                return false;
            }
        }
    }
    
    let _ = child.kill();
    let _ = child.wait();
    false
}

fn main() {
    let cli: Cli = argh::from_env();
    
    if cli.clear {
        clear_clipboard();
    } else if cli.use_space {
        copy_space();
    } else {
        println!("No action specified.");
        println!();
        println!("Clear KDE Klipper clipboard");
        println!("Artyom Nazarov, Orenburg, 2025");
        println!("Dependencies: yay -S dbus");
        println!("Usage: clear_clipboard [--clear | --use-space]");
        println!("Options:");
        println!("  -c, --clear     clear the clipboard");
        println!("  -s, --use-space copy space character to clipboard");
    }
}

fn clear_clipboard() {
    // Try dbus-send for KDE
    if let Ok(mut child) = Command::new("dbus-send")
        .args(&[
            "--session",
            "--type=method_call",
            "--dest=org.kde.klipper",
            "/klipper",
            "org.kde.klipper.klipper.clearClipboardHistory"
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn() {
        if wait_with_timeout(&mut child, 8) {
            println!("Clipboard cleared (KDE)");
            return;
        } else {
            println!("KDE command timed out");
        }
    }
    
    eprintln!("Error: Could not clear clipboard. Make sure you have dbus-send installed and KDE klipper running.");
}

fn copy_space() {
    // Try dbus-send for KDE
    if let Ok(mut child) = Command::new("dbus-send")
        .args(&[
            "--session",
            "--type=method_call",
            "--dest=org.kde.klipper",
            "/klipper",
            "org.kde.klipper.klipper.setClipboardContents",
            "string: "
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn() {
        if wait_with_timeout(&mut child, 8) {
            println!("Space character copied to clipboard (KDE)");
            return;
        } else {
            println!("KDE command timed out");
        }
    }
    
    eprintln!("Error: Could not copy to clipboard. Make sure you have dbus-send installed and KDE klipper running.");
}
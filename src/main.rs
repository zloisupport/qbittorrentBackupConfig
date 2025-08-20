use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use clap::{Arg, Command};
fn main() {
        let commands = Command::new("qbittorrent settings backup")
        .version("1.0")
        .about("qbittorrent settings backup")
        .arg(
            Arg::new("remove")
                .short('r')
                .long("remove")
                .help("Remove the scheduled task")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("add")
                .short('a')
                .long("add")
                .help("Add the scheduled task")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if commands.get_flag("add") {
        if let Err(e) = add_task_sheduler() {
            eprintln!("{}", e)
        }
    } else if commands.get_flag("remove") {
        if let Err(e) = remove_task_seduler() {
            eprintln!("{}", e)
        }
    } else {
        if let Err(e) = backup_app_data() {
            eprintln!("{}", e)
        }

        if let Err(e) = backup_local_data() {
            eprintln!("{}", e)
        }

    }
}


fn add_task_sheduler() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    let task_name = "Backup_qbittorrent_Settings";
    let my_app = env::current_exe()?;
    let program_path = my_app.display().to_string();
    let work_dir = my_app
        .parent()
        .ok_or("Failed to get parent directory")?
        .display()
        .to_string();

    std::env::set_current_dir(work_dir)?;

    // Use schtasks.exe to create a scheduled task
    let output = Command::new("schtasks")
        .args(&[
            "/Create",
            "/SC",
            "DAILY",
            "/MO",
            "3",
            "/TN",
            task_name,
            "/TR",
            program_path.as_str(),
            "/ST",
            "01:32",
            "/F",
        ])
        .output()?;

    if output.status.success() {
        println!("Task '{}' created successfully.", task_name);
        Ok(())
    } else {
        eprintln!(
            "Failed to create task: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Err("Failed to create scheduled task".into())
    }
}

fn remove_task_seduler() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    let task_name = "Backup_qbittorrent_Settings";
    // Use schtasks.exe to create a scheduled task
    let output = Command::new("schtasks")
        .args(&["/DELETE", "/TN", task_name, "/F"])
        .output()?;

    if output.status.success() {
        println!("Task '{}' deleted successfully.", task_name);
        Ok(())
    } else {
        eprintln!(
            "Failed to delete task: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Err("Failed to delete scheduled task".into())
    }
}


fn backup_local_data() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;

    let user_profile_path = env::var("USERPROFILE")?;

    let q_bittorrent_app_data_path = Path::new(&user_profile_path)
        .join("AppData")
        .join("Local")
        .join("qBittorrent");

    if Path::exists(&q_bittorrent_app_data_path) {
        println!("{}", q_bittorrent_app_data_path.display())
    }

    let backupapp_data_path = Path::new(&current_dir).join("BackupAppData").join("Local");

    if !Path::exists(&backupapp_data_path) {
        fs::create_dir_all(&backupapp_data_path)?
    }

    let src = q_bittorrent_app_data_path;
    let dest = Path::new(&backupapp_data_path);
    copy_dir_all(src, dest)?;
    return Ok(());
}

fn backup_app_data() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;

    let user_profile_path = env::var("USERPROFILE")?;

    let q_bittorrent_app_data_path = Path::new(&user_profile_path)
        .join("AppData")
        .join("Roaming")
        .join("qBittorrent");

    if Path::exists(&q_bittorrent_app_data_path) {
        println!("{}", q_bittorrent_app_data_path.display())
    }

    let backupapp_data_path = Path::new(&current_dir)
        .join("BackupAppData")
        .join("Roaming");

    if !Path::exists(&backupapp_data_path) {
        fs::create_dir_all(&backupapp_data_path)?
    }

    let src = q_bittorrent_app_data_path;
    let dest = Path::new(&backupapp_data_path);
    copy_dir_all(src, dest)?;
    return Ok(());
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

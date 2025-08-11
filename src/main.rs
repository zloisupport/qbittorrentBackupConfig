use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
fn main() {
    if let Err(e) = backup_app_data() {
        eprintln!("{}", e)
    }

    if let Err(e) = backup_local_data() {
        eprintln!("{}", e)
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

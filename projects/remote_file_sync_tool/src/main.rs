use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
 
fn main() -> io::Result<()> {
    println!("🔁 File Sync Tool");
 
    let source = prompt("Enter source directory: ");
    let destination = prompt("Enter destination directory: ");
 
    sync_dirs(Path::new(&source), Path::new(&destination))?;
    println!("✅ Sync complete.");
    Ok(())
}
 
fn sync_dirs(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
 
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
 
        if src_path.is_dir() {
            sync_dirs(&src_path, &dst_path)?;
        } else if is_new_or_updated(&src_path, &dst_path)? {
            fs::copy(&src_path, &dst_path)?;
            println!("📁 Copied: {}", src_path.display());
        }
    }
 
    Ok(())
}
 
fn is_new_or_updated(src: &PathBuf, dst: &PathBuf) -> io::Result<bool> {
    if !dst.exists() {
        return Ok(true);
    }
 
    let src_meta = fs::metadata(src)?;
    let dst_meta = fs::metadata(dst)?;
 
    let src_time = src_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let dst_time = dst_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
 
    Ok(src_time > dst_time)
}
 
fn prompt(msg: &str) -> String {
    use std::io::{Write, stdin, stdout};
    print!("{}", msg);
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
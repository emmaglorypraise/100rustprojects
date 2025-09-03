use std::process::Command;
 
fn main() {
    println!("📜 Git Commit Message Helper\n");
 
    // Run `git diff --cached --name-only`
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
        .expect("❌ Failed to run git");
 
    if !output.status.success() {
        eprintln!("❌ Not a git repository or no staged files.");
        return;
    }
 
    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<&str> = stdout.lines().collect();
 
    if files.is_empty() {
        println!("🚫 No staged files found.");
        return;
    }
 
    println!("📂 Staged files:");
    for file in &files {
        println!("• {}", file);
    }
 
    println!("\n💡 Suggested commit types:");
    for file in &files {
        if file.contains("test") {
            println!("- test: add test related changes");
        } else if file.contains("README") || file.ends_with(".md") {
            println!("- docs: update documentation");
        } else if file.ends_with(".rs") {
            println!("- feat: add feature in {}", file);
        } else if file.contains("fix") || file.contains("bug") {
            println!("- fix: fix bug in {}", file);
        } else {
            println!("- chore: update {}", file);
        }
    }
 
    println!("\n✅ Example:");
    println!("git commit -m \"feat: add user profile API\"");
}

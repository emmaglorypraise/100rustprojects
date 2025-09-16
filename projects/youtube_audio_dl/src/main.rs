use std::io::{self, Write};
use std::process::Command;
use std::process::Stdio;

fn main() {
    println!("🎬 YouTube Audio/Video Downloader");
    println!("====================");

    println!("Select download type:");
    println!("1. Audio (MP3)");
    println!("2. Video (MP4)");
    print!("Enter choice (1 or 2): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    let download_type = match choice {
        "1" => DownloadType::Audio,
        "2" => DownloadType::Video,
        _ => {
            println!("❌ Invalid choice. Defaulting to Audio.");
            DownloadType::Audio
        }
    };

    print!("🔗 Enter YouTube URL: ");
    io::stdout().flush().unwrap();

    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    let url = url.trim();

    if url.is_empty() {
        println!("❌ No URL provided.");
        return;
    }

    // Check if yt-dlp is available
    if !check_command_available("yt-dlp") {
        println!("❌ yt-dlp not found. Please install it first.");
        println!("   Installation: pip install yt-dlp");
        return;
    }

    // Check if ffmpeg is available for audio conversion
    if matches!(download_type, DownloadType::Audio) && !check_command_available("ffmpeg") {
        println!("⚠️  ffmpeg not found. Audio conversion may fail.");
        println!("   Please install ffmpeg for MP3 conversion:");
        println!("   - Ubuntu/Debian: sudo apt install ffmpeg");
        println!("   - macOS: brew install ffmpeg");
        println!("   - Windows: Download from ffmpeg.org");
        println!("   Continuing download anyway...");
    }

    // Download based on user choice
    match download_type {
        DownloadType::Audio => download_audio(url),
        DownloadType::Video => download_video(url),
    }
}

enum DownloadType {
    Audio,
    Video,
}

fn check_command_available(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn download_audio(url: &str) {
    println!("🎧 Downloading audio as MP3...");

    let output = Command::new("yt-dlp")
        .args([
            "--extract-audio",
            "--audio-format", "mp3",
            "--audio-quality", "0", // best quality
            "--output", "%(title)s.%(ext)s",
            url,
        ])
        .output();

    handle_download_result(output, "audio");
}

fn download_video(url: &str) {
    println!("🎥 Downloading video as MP4...");

    let output = Command::new("yt-dlp")
        .args([
            "--format", "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best",
            "--merge-output-format", "mp4",
            "--output", "%(title)s.%(ext)s",
            url,
        ])
        .output();

    handle_download_result(output, "video");
}

fn handle_download_result(output: Result<std::process::Output, std::io::Error>, media_type: &str) {
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("✅ {} downloaded successfully!", media_type);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("❌ yt-dlp returned an error while downloading {}:", media_type);
                println!("{}", stderr);
                
                if stderr.contains("ffmpeg") || stderr.contains("ffprobe") {
                    println!("\n💡 Solution: Please install ffmpeg:");
                    println!("   - Ubuntu/Debian: sudo apt install ffmpeg");
                    println!("   - macOS: brew install ffmpeg");
                    println!("   - Windows: Download from ffmpeg.org");
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to run yt-dlp: {}", e);
            if e.kind() == io::ErrorKind::NotFound {
                println!("💡 Please install yt-dlp: pip install yt-dlp");
            }
        }
    }
}
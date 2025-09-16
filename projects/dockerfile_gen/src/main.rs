use clap::{Arg, ArgAction, Command};
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let matches = Command::new("cargo-dockerize")
        .version("0.1.0")
        .about("Generates an optimized Dockerfile for Rust projects")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output Dockerfile path")
                .default_value("Dockerfile"),
        )
        .arg(
            Arg::new("rust-version")
                .long("rust-version")
                .value_name("VERSION")
                .help("Sets the Rust version for the builder image")
                .default_value("1.74"),
        )
        .arg(
            Arg::new("base-image")
                .long("base-image")
                .value_name("IMAGE")
                .help("Sets the base image for the runtime stage")
                .default_value("debian:bookworm-slim"),
        )
        .arg(
            Arg::new("build-stage")
                .long("build-stage")
                .value_name("NAME")
                .help("Sets the name for the build stage")
                .default_value("builder"),
        )
        .arg(
            Arg::new("target-dir")
                .long("target-dir")
                .value_name("DIR")
                .help("Sets a custom target directory (to preserve local target dir)"),
        )
        .arg(
            Arg::new("no-cache")
                .long("no-cache")
                .action(ArgAction::SetTrue)
                .help("Disables Docker layer caching optimization"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Suppresses informational output"),
        )
        .get_matches();

    let config = Config {
        output: PathBuf::from(matches.get_one::<String>("output").unwrap()),
        rust_version: matches.get_one::<String>("rust-version").unwrap().clone(),
        base_image: matches.get_one::<String>("base-image").unwrap().clone(),
        build_stage: matches.get_one::<String>("build-stage").unwrap().clone(),
        target_dir: matches.get_one::<String>("target-dir").map(|s| s.clone()),
        no_cache: matches.get_flag("no-cache"),
        quiet: matches.get_flag("quiet"),
    };

    if let Err(e) = run(&config) {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}

struct Config {
    output: PathBuf,
    rust_version: String,
    base_image: String,
    build_stage: String,
    target_dir: Option<String>,
    no_cache: bool,
    quiet: bool,
}

fn run(config: &Config) -> io::Result<()> {
    let cargo_path = Path::new("Cargo.toml");
    if !cargo_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Not a Rust project (Cargo.toml not found).",
        ));
    }

    let project_info = parse_cargo_toml(cargo_path)?;
    let dockerfile_content = generate_dockerfile(&project_info, config);

    fs::write(&config.output, dockerfile_content)?;
    
    if !config.quiet {
        println!(
            "✅ Dockerfile generated at '{}' for `{}`!",
            config.output.display(),
            project_info.name
        );

        if project_info.is_workspace {
            println!("⚠️  Detected workspace. The Dockerfile builds all members. Specify a single binary for a leaner image.");
        }
    }

    Ok(())
}

struct ProjectInfo {
    name: String,
    is_workspace: bool,
    is_binary: bool,
    has_examples: bool,
}

fn parse_cargo_toml(path: &Path) -> io::Result<ProjectInfo> {
    let contents = fs::read_to_string(path)?;

    let is_workspace = contents.contains("[workspace]");
    let is_binary = Path::new("src/main.rs").exists();
    let has_examples = Path::new("examples").exists() && fs::read_dir("examples")?.next().is_some();

    let name = if is_workspace {
        "workspace".to_string()
    } else {
        infer_crate_name(&contents).unwrap_or_else(|| {
            if is_binary {
                std::env::current_dir()
                    .ok()
                    .and_then(|dir| dir.file_name().map(|n| n.to_string_lossy().into_owned()))
                    .unwrap_or_else(|| "rust_app".to_string())
            } else {
                "rust_lib".to_string()
            }
        })
    };

    Ok(ProjectInfo {
        name,
        is_workspace,
        is_binary,
        has_examples,
    })
}

fn infer_crate_name(contents: &str) -> Option<String> {
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("name =") {
            return trimmed
                .splitn(2, '=')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').to_string());
        }
    }
    None
}

fn generate_dockerfile(info: &ProjectInfo, config: &Config) -> String {
    let build_cmd = if info.is_workspace {
        "RUN cargo build --release --workspace".to_string()
    } else if info.is_binary {
        format!("RUN cargo build --release --bin {}", info.name)
    } else if info.has_examples {
        "RUN cargo build --release --examples".to_string()
    } else {
        "RUN cargo build --release".to_string()
    };

    // Add cache optimization if not disabled
    let cache_optimization = if !config.no_cache {
        "\n# Install dependencies separately to leverage Docker cache\nRUN cargo fetch"
    } else {
        ""
    };

    // Handle custom target directory
    let target_dir = config
        .target_dir
        .as_ref()
        .map(|dir| format!("ENV CARGO_TARGET_DIR={}\n", dir))
        .unwrap_or_default();

    let copy_instruction = if info.is_workspace {
        "# NOTE: For workspaces, you must manually specify the binary to copy.\n# COPY --from=builder /usr/src/app/target/release/your_binary_name /app/".to_string()
    } else if info.is_binary {
        format!(
            "COPY --from={} /usr/src/app/target/release/{} /app/",
            config.build_stage, info.name
        )
    } else if info.has_examples {
        "# NOTE: Copying all example binaries. Specify a single one for production.\nCOPY --from=builder /usr/src/app/target/release/examples/* /app/".to_string()
    } else {
        "# Library detected. Nothing to copy to runtime image. You may need to adjust this.".to_string()
    };

    let cmd_instruction = if info.is_workspace {
        "# CMD [\"./your_binary_name\"]".to_string()
    } else if info.is_binary {
        format!("CMD [\"./{}\"]", info.name)
    } else {
        "# No default CMD for libraries".to_string()
    };

    format!(
        r#"# syntax=docker/dockerfile:1

# ----------------------------
# Build Stage
# ----------------------------
FROM rust:{0} AS {1}
WORKDIR /usr/src/app
COPY . .
{2}{3}
{4}

# ----------------------------
# Runtime Stage
# ----------------------------
FROM {5}
WORKDIR /app

# Install runtime dependencies if needed (e.g., CA certificates, libssl)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
{6}

# Set the startup command
{7}
"#,
        config.rust_version,
        config.build_stage,
        target_dir,
        cache_optimization,
        build_cmd,
        config.base_image,
        copy_instruction,
        cmd_instruction
    )
}
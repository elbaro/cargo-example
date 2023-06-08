use ahash::RandomState;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    uri: String,
    #[clap(long)]
    example: Option<String>,

    #[arg(long, group = "git")]
    rev: Option<String>,
    #[arg(long, group = "git")]
    branch: Option<String>,

    #[arg(long, group = "build-mode")]
    debug: bool,
    #[arg(long, group = "build-mode")]
    release: bool,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() {
    let args = Args::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(args.verbose.log_level_filter())
        .init();

    log::debug!("{:?}", args);

    // 1. clone
    let mut dir = std::env::temp_dir();
    dir.push("cargo-example");
    {
        let hash_builder = RandomState::with_seeds(0, 0, 0, 0);
        let uri_hash = hash_builder.hash_one(&args.uri);
        dir.push(uri_hash.to_string());
    }
    log::info!("mkdir -p {}", dir.display());
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");

    let mut cmd = std::process::Command::new("git");
    cmd.args(["clone", &args.uri, dir.to_str().unwrap()]);
    log::info!("Cloning {} to {} ..", args.uri, dir.display());
    cmd.spawn()
        .expect("failed to clone")
        .wait()
        .expect("failed to clone");

    // 2. checkout
    if let Some(branch) = args.branch {
        let mut cmd = std::process::Command::new("git");
        cmd.args(["checkout", &branch]);
        log::info!("checkout {} ..", branch);
        cmd.spawn()
            .expect("git checkout failed")
            .wait()
            .expect("git checkout failed");
    }
    if let Some(rev) = args.rev {
        let mut cmd = std::process::Command::new("git");
        cmd.args(["checkout", &rev]);
        log::info!("checkout {} ..", rev);
        cmd.spawn()
            .expect("git checkout failed")
            .wait()
            .expect("git checkout failed");
    }

    // 3. run
    // cargo run --example [example] [--debug|--release]
    let mut cmd = std::process::Command::new("cargo");
    dir.push("Cargo.toml");
    cmd.args(["run", "--manifest-path", dir.to_str().unwrap(), "--example"]);
    if let Some(example) = args.example {
        cmd.arg(example);
    }
    if args.release {
        cmd.arg("--release");
    }
    log::info!("{:?}", cmd);
    cmd.spawn()
        .expect("example failed to run")
        .wait()
        .expect("example failed to run");
}

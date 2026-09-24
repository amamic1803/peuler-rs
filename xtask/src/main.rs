use std::error::Error;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use xshell::{cmd, Shell};
use xtask_wasm::{Dist, DevServer, WasmOpt};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

trait Executable {
    fn execute(self, sh: &Shell) -> Result<(), Box<dyn Error>>;
}

#[derive(Subcommand)]
enum Commands {
    /// CLI binary commands
    #[command(subcommand)]
    Cli(CliCmd),

    /// CI commands
    #[command(subcommand)]
    Ci(CiCmd),

    /// Web commands
    #[command(subcommand)]
    Web(WebCmd),
}
impl Executable for Commands {
    fn execute(self, sh: &Shell) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Cli(cli_cmd) => cli_cmd.execute(sh),
            Self::Ci(ci_cmd) => ci_cmd.execute(sh),
            Self::Web(web_cmd) => web_cmd.execute(sh),
        }
    }
}

#[derive(Subcommand)]
enum CliCmd {
    /// Build the CLI binary
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Run the CLI binary
    Run {
        /// Run in release mode
        #[arg(long)]
        release: bool,

        /// Arguments passed to the CLI binary
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}
impl Executable for CliCmd {
    fn execute(self, sh: &Shell) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Build { release } => {
                if release {
                    cmd!(sh, "cargo build --package peuler --release").run()?;
                } else {
                    cmd!(sh, "cargo build --package peuler").run()?;
                }
            }
            Self::Run { release, args } => {
                if release {
                    cmd!(sh, "cargo run --package peuler --release -- {args...}").run()?;
                } else {
                    cmd!(sh, "cargo run --package peuler -- {args...}").run()?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Subcommand)]
enum CiCmd {
    /// Run all CI tasks
    Run,
    /// Build the project
    Build,
    /// Run tests
    Test,
    /// Generate documentation
    Docs,
    /// Run clippy
    Clippy,
    /// Check formatting with rustfmt
    Format,
}
impl CiCmd {
    fn build(sh: &Shell) -> Result<(), Box<dyn Error>> {
        cmd!(sh, "cargo build --all-features").run()?;
        Ok(())
    }

    fn test(sh: &Shell) -> Result<(), Box<dyn Error>> {
        cmd!(sh, "cargo test --all-features").run()?;
        Ok(())
    }

    fn docs(sh: &Shell) -> Result<(), Box<dyn Error>> {
        cmd!(sh, "cargo doc --all-features").run()?;
        Ok(())
    }

    fn clippy(sh: &Shell) -> Result<(), Box<dyn Error>> {
        cmd!(sh, "cargo clippy --all-features --all-targets -- -D warnings").run()?;
        Ok(())
    }

    fn format(sh: &Shell) -> Result<(), Box<dyn Error>> {
        cmd!(sh, "cargo fmt --all --check").run()?;
        Ok(())
    }
}
impl Executable for CiCmd {
    fn execute(self, sh: &Shell) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Build => Self::build(sh),
            Self::Test => Self::test(sh),
            Self::Docs => Self::docs(sh),
            Self::Clippy => Self::clippy(sh),
            Self::Format => Self::format(sh),
            Self::Run => {
                println!("Running all CI tasks...");

                println!("Building the project...");
                Self::build(sh)?;

                println!("Running tests...");
                Self::test(sh)?;

                println!("Generating documentation...");
                Self::docs(sh)?;

                println!("Running clippy...");
                Self::clippy(sh)?;

                println!("Checking formatting...");
                Self::format(sh)?;

                println!("All CI tasks completed successfully.");
                Ok(())
            }
        }
    }
}

#[derive(Subcommand)]
enum WebCmd {
    /// Build the web distribution
    Dist(Dist),
    /// Start the development server
    Serve(DevServer),
}
impl Executable for WebCmd {
    fn execute(self, _sh: &Shell) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Dist(dist) => {
                dist.app_name("my-wasm-crate").optimize_wasm(WasmOpt::level(4).shrink(1)).build("web")?;
                Ok(())
            }
            Self::Serve(dev_server) => {
                dev_server.xtask("web dist").start()?;
                Ok(())
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    // change the current working directory to the root of the workspace
    let project_root = workspace_root();
    sh.change_dir(&project_root);

    // execute the command
    cli.command.execute(&sh)
}

/// Returns the root of the current workspace
fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR points to xtask/
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // go up to the workspace root
    path
}

use clap::{CommandFactory, Parser, Subcommand};
use std::{fs, process::exit};
use venv::VenvManager;
mod interactive;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[command()]
    activate: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(infer_subcommands = true)]
enum Commands {
    /// List your projects
    #[command(short_flag('l'))]
    List,

    /// Add an alias to a project
    #[command(short_flag('a'), arg_required_else_help(true))]
    Activate {
        #[arg(value_name = "NAME{:?}")]
        name: String,
    },

    /// Deletes an alias
    #[command(short_flag('c'), arg_required_else_help(true))]
    Create { name: String },

    /// Deletes an alias
    #[command(short_flag('d'), arg_required_else_help(true))]
    Delete { name: String },

    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

fn main() {
    // Get the program options
    let cli = CLI::parse();

    let venv_manager: venv::VenvManager;
    if let Ok(x) = VenvManager::new() {
        venv_manager = x;
    } else {
        exit(1);
    }

    // if there's no command, but an arg, try activating the env
    if let Some(env) = cli.activate {
        let path = venv_manager.venv_store.to_str().unwrap();
        write_cmd(path, env);
    }
    // if there's a command run that
    else if let Some(cmd) = cli.command {
        match cmd {
            Commands::List => venv_manager.list(),
            Commands::Activate { name } => {
                eprintln!("add {}", name);
                venv_manager.activate();
            }
            Commands::Create { name } => {
                eprintln!("create {}", name);
                venv_manager.create();
            }

            Commands::Delete { name } => {
                eprintln!("del{:?}", venv_manager.venv_store);
            }
            // e.g. `$ cli completions bash`
            Commands::Completions { shell } => {
                shell.generate(&mut CLI::command(), &mut std::io::stdout());
            }
        }
        exit(0);
    }
    // default to interactive mode
    else {
        //interactive mode
        // interactive(&venv_manager);
        let opt = venv_manager.interactive();
        if opt.is_none() {
            exit(0);
        }
        if let Some(cmd) = opt {
            let pth = venv_manager.venv_store.to_str().unwrap();
            write_cmd(pth, cmd);
        }
    }

    println!("\n***RUNNING LAST LINE IN HIST FILE***");
    let hist_path = format!("{}/history", venv_manager.venv_store.to_str().unwrap());
    let c = fs::read_to_string(hist_path).unwrap();
    println!("{}", c);
}

fn write_cmd(path: &str, cmd: String) {
    let hist_path = format!("{path}/history");
    fs::write(hist_path, cmd).expect(format!("Error writing to file {}", path).as_str());
}

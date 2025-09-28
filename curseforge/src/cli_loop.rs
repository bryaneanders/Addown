use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use clap::{Parser, Subcommand};
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Debug)]
pub enum InputEvent {
    Command(String),
    CtrlC,
    Exit,
}

#[derive(Debug, Clone)]
pub struct CtrlCState {
    last_time: Option<Instant>,
    showing_message: bool,
    command_in_progress: bool,
    interrupt_command: bool,
}

impl CtrlCState {
    pub fn new() -> Self {
        Self {
            last_time: None,
            showing_message: false,
            command_in_progress: false,
            interrupt_command: false,
        }
    }
}

impl Default for CtrlCState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Parser)]
// name of the program
#[command(name = "Addown")]
// description of the program
#[command(about = "A CLI WoW Addon Manager", long_about = None)]
struct Addown {
    // this field will hold the subcommands
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize wow installation
    Init,
    /// View installed addons
    View,
    /// Search for addons
    Search {
        /// The exact name of the addon to search for
        #[arg(short = 'n', long = "name")]
        name: Option<String>,
        /// The id of addons to search for
        #[arg(short = 'i', long = "id")]
        id: Option<String>,
    },
    /// Get addons
    Get {
        /// The exact name of the addon to get
        #[arg(short = 'n', long = "name")]
        name: Option<String>,
        /// The id(s) of addons to get
        #[arg(short = 'i', long = "ids")]
        ids: Option<String>,
    },
    /// Delete addons
    Delete {
        /// The exact name of the addon to delete
        #[arg(short = 'n', long = "name")]
        name: Option<String>,
        /// The id(s) of addons to delete
        #[arg(short = 'i', long = "ids")]
        ids: Option<String>,
    },
    /// Update addons
    Update {
        /// The exact name of the addon to update
        #[arg(short = 'n', long = "name")]
        name: Option<String>,
        /// The id(s) of addons to update
        #[arg(short = 'i', long = "ids")]
        ids: Option<String>,
        /// boolean to update all addons
        #[arg(short = 'a', long = "all")]
        all: bool,
    },
    /// Update all addons
    Exit,
}

pub async fn execute_command(
    command: Commands,
    ctrl_c_state: &Arc<Mutex<CtrlCState>>,
) -> Result<(), Box<dyn std::error::Error>> {

    match command {
        Commands::Init => {
            println!("Initializing WoW installation...");
            // Call your init function here
        }
        Commands::View => {
            println!("Viewing installed addons...");
            // Call your view function here
        }
        Commands::Search { name, id } => {
            if let Some(name) = name {
                println!("Searching for addon with name: {}", name);
                // Call your search by name function here
            } else if let Some(id) = id {
                println!("Searching for addon with id: {}", id);
                // Call your search by id function here
            } else {
                println!("Please provide either a name or an id to search for.");
            }
        }
        Commands::Get { name, ids } => {
            if let Some(name) = name {
                println!("Getting addon with name: {}", name);
                // Call your get by name function here
            } else if let Some(ids) = ids {
                println!("Getting addons with ids: {}", ids);
                // Call your get by ids function here
            } else {
                println!("Please provide either a name or ids to get.");
            }
        }
        Commands::Delete { name, ids } => {
            if let Some(name) = name {
                println!("Deleting addon with name: {}", name);
                // Call your delete by name function here
            } else if let Some(ids) = ids {
                println!("Deleting addons with ids: {}", ids);
                // Call your delete by ids function here
            } else {
                println!("Please provide either a name or ids to delete.");
            }
        }
        Commands::Update { name, ids, all } => {
            if all {
                println!("Updating all addons...");
                // Call your update all function here
            } else if let Some(name) = name {
                println!("Updating addon with name: {}", name);
                // Call your update by name function here
            } else if let Some(ids) = ids {
                println!("Updating addons with ids: {}", ids);
                // Call your update by ids function here
            } else {
                println!("Please provide either a name, ids, or use --all to update.");
            }
        }
        Commands::Exit => {
            println!("Exiting...");
            //
        }
    }
    Ok(())
}

/// The main cli parsing loop
pub async fn main_loop(
    ctrl_c_state: Arc<Mutex<CtrlCState>>,
    input_rx: &mut UnboundedReceiver<InputEvent>,
) {
    loop {
        tokio::select! {
            // Handle input from rustyline
            input_event = input_rx.recv() => {
                match input_event {
                    Some(InputEvent::Command(line)) => {
                        if line.trim().is_empty() {
                            continue;
                        }

                        // Reset Ctrl+C state on new command
                        {
                            let mut state = ctrl_c_state.lock().unwrap();
                            state.last_time = None;
                            state.interrupt_command = false;
                            if state.showing_message {
                                // Clear any existing message
                                print!("\x1b[2K\x1b[1A\x1b[2K\r\x1b[32mprompt-cli>\x1b[97m\x1b[?25h ");
                                io::stdout().flush().unwrap();
                                state.showing_message = false;
                                continue;
                            }
                        }

                        // Parse and execute command
                        let args = parse_quoted_args(&line);
                        if args.is_empty() {
                            continue;
                        }

                        let mut full_args = vec!["prompt-cli"];
                        full_args.extend(args.iter().map(|s| s.as_str()));

                        match Addown::try_parse_from(full_args) {
                            Ok(cli) => {

                                // Spawn command execution in separate task so main loop stays responsive
                                let ctrl_c_state_clone = ctrl_c_state.clone();
                                let mut command_handle = tokio::spawn(async move {
                                    execute_command(cli.command, &ctrl_c_state_clone).await
                                });

                                // Wait for either command completion or keep processing other events
                                let mut command_finished = false;
                                while !command_finished {
                                    tokio::select! {
                                        // Command completed
                                        result = &mut command_handle => {
                                            command_finished = true;

                                            match result {
                                                Ok(Ok(should_continue)) => {
                                                    if !should_continue {
                                                        return; // Exit main loop
                                                    }
                                                }
                                                Ok(Err(e)) => {
                                                    if e.to_string().contains("interrupted") {
                                                        print!("\r\x1b[2K\x1b[1A\x1b[2K");
                                                        io::stdout().flush().unwrap();
                                                        println!("\x1b[1ACommand was interrupted");
                                                        print!("\x1b[32mprompt-cli>\x1b[97m\x1b[?25h ");
                                                        io::stdout().flush().unwrap();
                                                    } else {
                                                        eprintln!("\r\x1b[2K❌ Error executing command: {}", e);
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!("\r\x1b[2K❌ Command task failed: {}", e);
                                                }
                                            }
                                        }

                                        // Handle more input while command is running
                                        input_event = input_rx.recv() => {

                                            match input_event {
                                                Some(InputEvent::CtrlC) => {

                                                    let mut state = ctrl_c_state.lock().unwrap();
                                                    if state.command_in_progress {
                                                        state.interrupt_command = true;
                                                        // Continue loop to wait for command to actually stop
                                                    }
                                                }
                                                Some(InputEvent::Command(_line)) => {
                                                    // User tried to run another command while one is running
                                                    print!("\r\x1b[2K\x1b[1A");
                                                    io::stdout().flush().unwrap();
                                                    //println!("⚠️ Command '{}' ignored - another command is still running. Press Ctrl+C to interrupt it.", line.trim());
                                                    continue;
                                                }
                                                Some(InputEvent::Exit) => {
                                                    println!("Goodbye!");
                                                    return; // Exit main loop
                                                }
                                                None => {
                                                    println!("Input channel closed, exiting...");
                                                    return; // Exit main loop
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                print!("\r\x1b[2K\x1b[?25l");
                                io::stdout().flush().unwrap();
                                if line == "help" {
                                    show_help();
                                } else if line == "exit" || line == "quit" {
                                    println!("Goodbye!");
                                    break;
                                } else {
                                    println!("Error: {}", e);
                                    println!("Type 'help' for available commands.");
                                }
                                print!("\x1b[32mprompt-cli>\x1b[97m\x1b[?25h ");
                                io::stdout().flush().unwrap();
                            }
                        }
                    }
                    Some(InputEvent::CtrlC) => {
                        // this is handled in the readline loop
                        continue;
                    }
                    Some(InputEvent::Exit) => {
                        println!("Goodbye!");
                        break;
                    }
                    None => {
                        println!("Input channel closed, exiting...");
                        break; // Channel closed
                    }
                }
            }
        }
    }
}

/// Handles commands like `prompt -p "what is 2+2?"`
fn parse_quoted_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                // switch to turn on or off quotes mode
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                // break into a new arg on space if not in quotes
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            '\\' if in_quotes => {
                // Handle escaped characters in quotes
                if let Some(next_ch) = chars.next() {
                    match next_ch {
                        'n' => current_arg.push('\n'),
                        't' => current_arg.push('\t'),
                        'r' => current_arg.push('\r'),
                        '\\' => current_arg.push('\\'),
                        '"' => current_arg.push('"'),
                        _ => {
                            // default case, just add the \\
                            current_arg.push('\\');
                            current_arg.push(next_ch);
                        }
                    }
                }
            }
            _ => {
                current_arg.push(ch);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

fn show_help() {
    println!("Available commands:");
    println!("  init                 Initialize WoW installation");
    println!("  view                 View installed addons");
    println!("  search -n <name>    Search for addon by name");
    println!("  search -i <id>      Search for addon by id");
    println!("  get -n <name>       Get addon by name");
    println!("  get -i <ids>        Get addons by ids (comma-separated)");
    println!("  delete -n <name>    Delete addon by name");
    println!("  delete -i <ids>     Delete addons by ids (comma-separated)");
    println!("  update -n <name>    Update addon by name");
    println!("  update -i <ids>     Update addons by ids (comma-separated)");
    println!("  update -a           Update all addons");
    println!("  help                Show this help message");
    println!("  exit, quit         Exit the CLI");
}
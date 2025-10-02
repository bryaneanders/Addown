use crate::{curseforge_api, game_version, installed_mods, mod_table};
use clap::{Parser, Subcommand};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

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
        #[arg(short = 'f', long = "filter")]
        name: Option<String>,
    },
    /// Get addons
    Get {
        /// The id(s) of addons to get
        #[arg(short = 'i', long = "ids")]
        ids: Option<String>,
    },
    /// Delete addons
    Delete {
        /// The id(s) of addons to delete
        #[arg(short = 'i', long = "ids")]
        ids: Option<String>,
    },
    /// Update addons
    Update {
        /// The id(s) of addons to update
        #[arg(short = 'i', long = "ids")]
        ids: Option<String>,
        /// boolean to update all addons
        #[arg(short = 'a', long = "all")]
        all: bool,
        /// boolean to force update addons
        #[arg(short = 'f', long = "force")]
        force: bool,
    },
    /// Update all addons
    Exit,
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
                                print!("\x1b[2K\x1b[1A\x1b[2K\r\x1b[32maddown>\x1b[97m\x1b[?25h ");
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

                        let mut full_args = vec!["addown"];
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
                                                        print!("\x1b[32maddown>\x1b[97m\x1b[?25h ");
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
                                print!("\x1b[32maddown>\x1b[97m\x1b[?25h ");
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

pub async fn execute_command(
    command: Commands,
    ctrl_c_state: &Arc<Mutex<CtrlCState>>,
) -> anyhow::Result<bool> {
    {
        let mut state = ctrl_c_state.lock().unwrap();
        state.command_in_progress = true;
        state.interrupt_command = false;
    }

    print!("\x1b[2K\r\x1b[?25l"); // Clear current line and move up
    io::stdout().flush()?;

    match command {
        Commands::Init => {
            println!("Initializing WoW installation...");
            // Call your init function here
            reset_prompt(ctrl_c_state).await;
            Ok(true)
        }
        Commands::View => {
            println!("Viewing installed addons...");
            installed_mods::get_installed_mods().await.unwrap();
            reset_prompt(ctrl_c_state).await;
            Ok(true)
        }
        Commands::Search { name: filter } => {
            if let Some(filter) = filter {
                println!("Searching for addon with filter: {}", filter);
                let game_mods = curseforge_api::search_mods(1, &filter).await.unwrap();
                println!("\nSearch Results ({} total):", game_mods.len());
                for game_mod in &game_mods {
                    println!(
                        "  - {} (ID: {}). About: {}",
                        game_mod.name, game_mod.id, game_mod.summary
                    );
                }

                let mut table = mod_table::ModTable::new();
                table.populate_mods_table(game_mods).unwrap();
                table.printstd();
            } else {
                println!("Please provide either a text filter to search for");
            }

            reset_prompt(ctrl_c_state).await;
            Ok(true)
        }
        Commands::Get { ids } => {
            if let Some(ids) = ids {
                println!("Getting addons with ids: {}", ids);
                for id in ids.split(',') {
                    if let Ok(id_num) = id.trim().parse::<u32>() {
                        let game_mod = curseforge_api::get_mod_info(id_num).await.unwrap();
                        // Get the right file for the game version
                        let mod_file =
                            game_version::get_mod_file_for_game_version(&game_mod).unwrap();
                        // Download the file
                        curseforge_api::get_mod_file(mod_file.id, &*mod_file.file_name)
                            .await
                            .unwrap();
                    } else {
                        println!("Invalid id: {}", id);
                    }
                }
            } else {
                println!("Please provide addon ids to get.");
            }

            reset_prompt(ctrl_c_state).await;
            Ok(true)
        }
        Commands::Delete { ids } => {
            if let Some(ids) = ids {
                println!("Deleting addons with ids: {}", ids);
                // Call your delete by ids function here
            } else {
                println!("Please provide addon ids to delete.");
            }

            reset_prompt(ctrl_c_state).await;
            Ok(true)
        }
        Commands::Update { ids, all, force } => {
            if all {
                println!("Updating all addons...");
                // Call your update all function here
            } else if let Some(ids) = ids {
                println!("Updating addons with ids: {}", ids);
                // Call your update by ids function here
            } else {
                println!("Please provide either ids or use --all to update addons.");
            }

            reset_prompt(ctrl_c_state).await;
            Ok(true)
        }
        Commands::Exit => {
            println!("Exiting...");
            Ok(false)
        }
    }
}

//async fn reset_prompt(progress_task: JoinHandle<()>, ctrl_c_state: &Arc<Mutex<CtrlCState>>) {
async fn reset_prompt(ctrl_c_state: &Arc<Mutex<CtrlCState>>) {
    //progress_task.abort();
    print!("\r\x1b[32mAddown>\x1b[97m\x1b[?25h "); // Show prompt and cursor≥
    io::stdout().flush().unwrap();

    {
        let mut state = ctrl_c_state.lock().unwrap();
        state.command_in_progress = false;
        state.interrupt_command = false;
    }
}

/// Handles commands like `get -n "Details! Damage Meter"`
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

/// Rustyline backround loop that handles Ctrl+C and other input events
pub fn crate_rustyline_background_loop(
    ctrl_c_timeout: Duration,
    input_tx_clone: UnboundedSender<InputEvent>,
    rusty_ctrl_c_state_clone: Arc<Mutex<CtrlCState>>,
) {
    std::thread::spawn(move || {
        let mut rl = DefaultEditor::new().unwrap();
        //load_history(&mut rl);

        loop {
            let state = rusty_ctrl_c_state_clone.lock().unwrap();
            // when I ctrl+c it prompts again before the state is set
            let prompt: &str =
                if !state.interrupt_command && !state.command_in_progress && !state.showing_message
                {
                    print!("\x1b[?25h"); // Show cursor
                    io::stdout().flush().unwrap();
                    "\x1b[32mAddown>\x1b[97m " // new promp value
                } else {
                    print!("\x1b[?25l"); // Hide cursor
                    io::stdout().flush().unwrap();
                    ""
                };
            drop(state);

            //save_history(&mut rl);
            match rl.readline(prompt) {
                Ok(line) => {
                    let mut state = rusty_ctrl_c_state_clone.lock().unwrap();
                    if state.showing_message {
                        // Clear the message and reset state
                        print!("\x1b[2K\x1b[1A\x1b[2K\r\x1b[32mAddown>\x1b[97m\x1b[?25h ");
                        io::stdout().flush().unwrap();
                        state.showing_message = false;
                        state.last_time = None;
                        continue;
                    }

                    rl.add_history_entry(line.as_str()).unwrap();
                    if input_tx_clone.send(InputEvent::Command(line)).is_err() {
                        break; // Main task has stopped
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    // Update state immediately in the rustyline thread
                    {
                        let mut state = rusty_ctrl_c_state_clone.lock().unwrap();
                        let now = Instant::now();

                        if !state.command_in_progress {
                            // Handle double Ctrl+C for exit
                            let within_timeout = state
                                .last_time
                                .map(|last| now.duration_since(last) < ctrl_c_timeout)
                                .unwrap_or(false);

                            if within_timeout {
                                std::process::exit(0);
                            } else {
                                // First Ctrl+C - immediately update state to hide prompt
                                state.last_time = Some(now);
                                state.showing_message = true;

                                // Clear the current line and show message
                                println!("\r\x1b[2K\x1b[1APress Ctrl+C again within 2 seconds to force exit...");
                            }
                        }
                    }

                    if input_tx_clone.send(InputEvent::CtrlC).is_err() {
                        break;
                    }
                }
                Err(ReadlineError::Eof) => {
                    let _ = input_tx_clone.send(InputEvent::Exit);
                    break;
                }
                Err(error) => {
                    eprintln!("Readline error: {}", error);
                    break;
                }
            }
        }
    });
}

/// Background loop that handles clearing out
pub fn create_ctrlc_background_loop(
    ctrl_c_timeout: Duration,
    ctrl_c_state_clone: Arc<Mutex<CtrlCState>>,
) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        loop {
            interval.tick().await;
            let mut state = ctrl_c_state_clone.lock().unwrap();
            if state.showing_message {
                if let Some(last_time) = state.last_time {
                    if Instant::now().duration_since(last_time) >= ctrl_c_timeout {
                        // Clear the message
                        print!("\x1b[2K\x1b[1A\x1b[2K\r\x1b[32mAddown>\x1b[97m\x1b[?25h "); // Show prompt and cursor
                        io::stdout().flush().unwrap();
                        state.showing_message = false;
                        state.last_time = None;
                    }
                }
            }
        }
    });
}

fn show_help() {
    println!("Available commands:");
    println!("  init                   Initialize WoW installation");
    println!("  view                   View installed addons");
    println!("  search -f <filter>     Search for addon by filter");
    println!("  get -i <ids>           Get addons with ids (comma-separated)");
    println!("  delete -i <ids>        Delete addons with ids (comma-separated)");
    println!("  update -i <ids> [-f]   Update addons with ids (comma-separated). Force to reinstall even if no update is needed");
    println!("  update -a [-f]         Update all addons. For to reinstall all addons even those that don't need updates.");
    println!("  help                   Show this help message");
    println!("  exit, quit             Exit the CLI");
}

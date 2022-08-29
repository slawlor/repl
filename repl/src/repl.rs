use anyhow::Result;
use log::{debug, error, info, warn};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{marker::PhantomData, path::PathBuf, str::FromStr};

use crate::commands::ReplCommandProcessor;

const DEFAULT_HISTORY_FILE_NAME: &str = ".repl_history";

/// Represents the REPL interface and processing loop
#[derive(Debug)]
pub struct Repl<C>
where
    C: clap::Parser,
{
    /// The REPL editor interface for the command prompt
    editor: Editor<()>,

    /// The history file
    history: Option<PathBuf>,

    /// User-provided command processor responsible for processing parsed command instructions and
    /// executing on them
    command_processor: Box<dyn ReplCommandProcessor<C>>,

    /// The prompt to the interface (defaults to ">>")
    prompt: String,

    /// Phantom holder for the command structure enum
    _command_type: PhantomData<C>,
}

impl<C> Drop for Repl<C>
where
    C: clap::Parser,
{
    fn drop(&mut self) {
        if let Some(history_path) = &self.history {
            match self.editor.save_history(&history_path) {
                Ok(_) => info!("REPL command history updated"),
                Err(err) => warn!("Failed to safe REPL command history with error '{}'", err),
            }
        }
    }
}

impl<C> Repl<C>
where
    C: clap::Parser,
{
    // =================== Private Functions =================== //

    /// Format the history file name to a full path for rustyline
    fn get_history_file_path(history_file_name: Option<String>) -> Result<Option<PathBuf>> {
        if let Some(history_file) = &history_file_name {
            let path = std::path::Path::new(history_file);
            if path.is_file() && path.exists() {
                // the file exists, utilize that
                Ok(Some(PathBuf::from_str(history_file)?))
            } else if path.is_dir() && path.exists() {
                let mut full_path = PathBuf::from_str(history_file)?;
                full_path.push(DEFAULT_HISTORY_FILE_NAME);
                Ok(Some(full_path))
            } else {
                // assume the provided history_file is a file name with no path, utilize the home directory
                Ok(dirs::home_dir().map(|mut home_dir| {
                    home_dir.push(history_file);
                    home_dir
                }))
            }
        } else {
            debug!("REPL history disabled as no history file provided");
            Ok(None)
        }
    }

    /// Retrieve the rustyline editor with history loaded (if possible)
    fn get_editor(history: &Option<PathBuf>) -> Result<Editor<()>> {
        let mut rl = Editor::<()>::new()?;

        if let Some(history_file) = history {
            match rl.load_history(history_file) {
                Ok(_) => info!("REPL command history file loaded"),
                Err(err) => warn!("Failed to load REPL command history {}", err),
            }
        }

        Ok(rl)
    }

    async fn process_command(&self, line: String) -> Result<()> {
        let mut cmd_parts: Vec<&str> = vec!["repl-interface"];
        cmd_parts.extend(line.split(' ').collect::<Vec<_>>().iter().copied());
        match C::try_parse_from(cmd_parts.into_iter()) {
            Ok(cli) => {
                self.command_processor.process_command(cli).await?;
            }
            Err(clap_err) => match clap::Error::kind(&clap_err) {
                clap::ErrorKind::DisplayHelp | clap::ErrorKind::DisplayVersion => {
                    println!("{}", clap_err);
                }
                _ => {
                    warn!(
                        "Invalid command (type 'help' for the help menu\r\n{}",
                        clap_err
                    );
                }
            },
        }
        Ok(())
    }

    // =================== Public API =================== //

    /// Construct a new REPL infterface.
    ///
    /// You can supply the (optional) history file for command history. Utilizing rustyline we can
    /// utilize the history for up & down arrow navigation of past commands. Having the history
    /// file be null will be no history is loaded nor stored
    ///
    /// * `history_file` - The optional command history file. Can be a full path, relative path, directory, or just the end filename to utilize
    /// * `prompt` - The prompt to display to the user to enter input. Defaults to ">>"
    pub fn new(
        command_processor: Box<dyn crate::commands::ReplCommandProcessor<C>>,
        history_file: Option<String>,
        prompt: Option<String>,
    ) -> Result<Self> {
        let history_path = Self::get_history_file_path(history_file)?;
        let editor = Self::get_editor(&history_path)?;
        Ok(Self {
            editor,
            history: history_path,
            command_processor,
            prompt: prompt.unwrap_or_else(|| ">>".to_string()),
            _command_type: PhantomData,
        })
    }

    /// Execute the REPL, prompting for user input and processing the results
    pub async fn process(&mut self) -> Result<()> {
        loop {
            let readline = self.editor.readline(&self.prompt);
            match readline {
                Ok(line) => {
                    let parts: Vec<&str> = line.split(' ').collect();
                    let mut command = String::new();
                    if let Some(head) = parts.first() {
                        command = String::from(*head);
                    }
                    match command.to_lowercase().as_ref() {
                        "" => {} // Loop, someone hit enter needlessly
                        maybe_quit if self.command_processor.is_quit(maybe_quit) => break, // check for quit/exit
                        _ => {
                            self.process_command(line).await?;
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    error!("Error: {:?}", err);
                    break;
                }
            }
        }
        Ok(())
    }
}

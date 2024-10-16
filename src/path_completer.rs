use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::{Helper, Context};

pub struct PathCompleter {
    filename_completer: FilenameCompleter,
    commands: Vec<String>,
}

impl PathCompleter {
    pub fn new() -> Self {
        Self {
            filename_completer: FilenameCompleter::new(),
            commands: vec![
                "generate_payload".into(),
                "list".into(),
                "kill".into(),
                "connect".into(),
                "resume".into(),
                "exit".into(),
                "help".into(),
            ],
        }
    }
}

impl Helper for PathCompleter {}

impl Completer for PathCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        
        let input = &line[..pos];
        if input.trim().is_empty() {
            return Ok((0, vec![]));
        }

        if input.trim().contains(' ') {
            return self.filename_completer.complete(line, pos, _ctx);
        };

        let completions: Vec<Pair> = self.commands
            .iter()
            .filter(|cmd| cmd.starts_with(input.trim()))
            .map(|cmd| Pair {display: cmd.clone(), replacement: cmd.clone()})
            .collect();

        Ok((0, completions))
    }
}

impl Highlighter for PathCompleter {}

impl Hinter for PathCompleter {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Validator for PathCompleter {}

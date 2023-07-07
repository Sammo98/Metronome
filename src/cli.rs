use std::{str::FromStr, process::Command};

use rustyline::{Editor, history::FileHistory};
use tokio_util::sync::CancellationToken;

const NANO_CONVERTER:f64 = 1_000_000_000.0;

#[derive(Clone)]
enum Token {
    Active(CancellationToken),
    Inactive
}

pub struct Metronome {
    editor:Editor<(), FileHistory>,
    current_token:Token,
    downbeat:bool
}

enum InputType {
    TempoChange(u16),
    Quit,
    ToggleDownbeat
}

impl FromStr for InputType{
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match (input.starts_with("q"), input.starts_with("bpm"), input == "db") {
            (true, false, false) => Ok(Self::Quit),
            (false, true, false) => {
                let tempo = input.split(" ").nth(1).unwrap().parse::<u16>().unwrap();
                Ok(Self::TempoChange(tempo))
            },
            (false, false, true) => Ok(Self::ToggleDownbeat),
            _ => Err(())
        }
    }
}

impl Metronome {

    pub fn new() -> Self {
        let editor = rustyline::DefaultEditor::new().expect("Failed to initiate CLI.");
        let current_token = Token::Inactive;
        let downbeat = false;
        Metronome { editor, current_token, downbeat }
    }

    pub fn start(&mut self) {

        // Start the metronome with default tempo
        self.start_with_new_tempo(100);

        // Start the input loop
        loop {

            let line = self.editor.readline(">>> ").unwrap();
            let input = InputType::from_str(line.trim().to_lowercase().as_str()).unwrap();
            match input {
                // If we get a tempo change request, we must cancel the previous task and initiate
                // a new task 
                InputType::TempoChange(tempo) => self.start_with_new_tempo(tempo),
                InputType::Quit => break,
                InputType::ToggleDownbeat =>{
                    self.downbeat = !self.downbeat;
                    self.start_with_new_tempo(100);
                }
            }
        }
    }

    fn start_with_new_tempo(&mut self, tempo:u16) {

        match &self.current_token {
            Token::Active(cancellation_token) => cancellation_token.clone().cancel(),
            Token::Inactive => {},
        }

        let new_token = CancellationToken::new();
        let cloned_token = new_token.clone();
        self.current_token = Token::Active(new_token);
        let db = self.downbeat.clone();
        let _ = tokio::spawn(async move {
            tokio::select! {
                // Step 3: Using cloned token to listen to cancellation requests
                _ = cloned_token.cancelled() => {
                    // The token was cancelled, task can shut down
                }
                _ =  Metronome::run(tempo, db)  => {
                    // Long work has completed
                }
            }
        });
    }

    async fn run(tempo:u16, with_downbeat:bool) {

        let pause = ((60_f64 / tempo as f64) * NANO_CONVERTER) as u64;
        match with_downbeat {
            true => {
                loop {
                    let _ = Command::new("afplay").arg("sounds/downbeat.wav").spawn().unwrap();
                    tokio::time::sleep(std::time::Duration::from_nanos(pause)).await;

                    let _ = Command::new("afplay").arg("sounds/pulse.wav").spawn().unwrap();
                    tokio::time::sleep(std::time::Duration::from_nanos(pause)).await;

                    let _ = Command::new("afplay").arg("sounds/pulse.wav").spawn().unwrap();
                    tokio::time::sleep(std::time::Duration::from_nanos(pause)).await;

                    let _ = Command::new("afplay").arg("sounds/pulse.wav").spawn().unwrap();
                    tokio::time::sleep(std::time::Duration::from_nanos(pause)).await;
                }
            },
            false => {
                loop {
                    let _ = Command::new("afplay").arg("sounds/pulse.wav").spawn().unwrap();
                    tokio::time::sleep(std::time::Duration::from_nanos(pause)).await;
                }
            }
        }
    }
}

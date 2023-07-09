
use crate::parse::{
    parse_input,
    InputType::{self, *},
};
use anyhow::Result;
use std::process::Command;
use tokio_util::sync::CancellationToken;

const NANO_CONVERTER: f64 = 1_000_000_000.0;

pub struct Metronome {
    current_token: CancellationToken,
    downbeat: bool,
    time_signature: Vec<u8>,
    tempo: u16,
}

impl Metronome {
    pub fn new() -> Self {
        let current_token = CancellationToken::new();
        let downbeat = false;
        let time_signature = vec![4];
        let tempo = 100_u16;
        Metronome {
            current_token,
            downbeat,
            time_signature,
            tempo,
        }
    }

    pub fn start(&mut self) {
        println!("{self}");
        // Start the metronome with default tempo
        self.restart();
        let mut editor = rustyline::DefaultEditor::new().expect("Failed to initialise Metronome.");

        // Start the input loop - This reads user input and handles accordingly to event type.
        loop {
            let line = editor.readline(">>> ").unwrap();
            let line_lower = line.to_lowercase();
            let input_type = parse_input(line_lower.trim());
            match input_type {
                Ok((_, valid_input)) => self.handle_input_event(valid_input),
                Err(e) => println!(
                    "Issue with command ({e:?})! Please enter 'help' for further instruction."
                ),
            }
        }
    }

    fn handle_input_event(&mut self, e: InputType) {
        // Input event handler. Takes the type of the input event and updates the state
        // accordingly or exits the application.
        match e {
            TempoChange(new_tempo) => self.tempo = new_tempo,
            TimeSignatureChange(new_time_signature) => {
                self.time_signature = new_time_signature
                    .iter()
                    .flat_map(|x| x.parse::<u8>().ok())
                    .collect::<Vec<u8>>();
            }
            DownbeatToggle(_) => self.downbeat = !self.downbeat,
            Help(_) => println!("{self}"),
            Quit(_) => {
                self.current_token.cancel();
                std::process::exit(0);
            }
        }
    }

    fn restart(&mut self) {
        // Cancel the current task and instantiate a new task with a fresh cancellation token
        self.current_token.cancel();
        self.current_token = CancellationToken::new();

        // Clone the current state post previous user input from which to construct the new task.
        let cloned_token = self.current_token.clone();
        let db = self.downbeat;
        let ts = self.time_signature.clone();
        let tempo = self.tempo;

        let _ = tokio::spawn(async move {
            tokio::select! {
                _ = cloned_token.cancelled() => {}
                _ =  Metronome::run(tempo, db, ts)  => {}
            }
        });
    }

    async fn run(tempo: u16, with_downbeat: bool, time_signature: Vec<u8>) -> Result<()> {

        // calculate the pause interval between playing sounds.
        let pause = ((60_f64 / tempo as f64) * NANO_CONVERTER) as u64;

        // If downbeat is active, we must use the time signature to know when to play the downbeat.
        // For each time signature given in the input (stored as a vec) we must initially play the
        // downbeat and follow that with time siganture -1 * pulse. E.g. The user inputs 'ts 4 3',
        // for alternating bars of 4/4 and 3/4, the loop is downbeat, 3*pulse, downbeat, 2*pulse.
        match with_downbeat {
            true => loop {
                for ts in time_signature.iter() {
                    Metronome::play_downbeat(pause).await?;

                    for _ in 0..*ts - 1 {
                        Metronome::play_pulse(pause).await?;
                    }
                }
            },
            false => loop {
                Metronome::play_pulse(pause).await?;
            },
        }
    }

    async fn play_downbeat(pause:u64) -> Result<()> {
        _ = Command::new("afplay").arg("sounds/downbeat.wav").spawn()?;
        tokio::time::sleep(std::time::Duration::from_nanos(pause)).await;
        Ok(())
    }

    async fn play_pulse(pause:u64) -> Result<()> {
        _ = Command::new("afplay").arg("sounds/pulse.wav").spawn()?;
        tokio::time::sleep(std::time::Duration::from_nanos(pause)).await;
        Ok(())
    }
}

impl std::fmt::Display for Metronome {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nWelcome to Metronome!\n\n")?;
        write!(f, "There are 4 available options:\n\n")?;
        writeln!(f, "1. Change the tempo. Using the command 'bpm' followed by your desired tempo. E.g. 'bpm 100' or 'BPM 101'")?;
        writeln!(f, "2. Toggle the downbeat. Using the command 'db' or 'downbeat'.")?;
        writeln!(f, "3. Enter a custom time signature. Using the command 'ts' followed by your desired combination of space separated numbers. This can handle complex combinations of time signatures. E.g. for 1 bar of 4/4 followed by 1 bar of 3/4 you can use 'ts 4 3'.")?;
        writeln!(f, "4. Quit! Using the command 'q' or 'quit' or 'exit'.")?;
        write!(f, "Enter 'help' at anytime to reshow these instructions!")
    }
}

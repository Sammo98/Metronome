use crate::parse::InputType::{self, *};
use rustyline::{Config, Editor};
use std::{collections::HashMap, process::Command, time::Duration};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

const NANO_CONVERTER: f64 = 1_000_000_000.0;

pub struct Metronome {
    current_token: CancellationToken,
    downbeat: bool,
    time_signature: Vec<(u8, u8)>,
    tempo: u16,
}

impl Metronome {
    pub fn start_metronome(&mut self) {
        // Show help info
        println!("{self}");

        // Start the metronome with default tempo
        self.restart_task();
        let mut editor = get_repl();

        // Start the input loop - This reads user input and handles accordingly to event type.
        loop {
            let line = editor.readline(">>> ").unwrap();
            let line_lower = line.to_lowercase();
            let input_type = InputType::parse(line_lower.trim());
            println!("{input_type:?}");
            match input_type {
                Ok((_, valid_input)) => self.handle_input_event(valid_input),
                Err(_) => {
                    println!("Issue with command! Please enter 'help' for further instruction.")
                }
            }
        }
    }

    fn handle_input_event(&mut self, e: InputType) {
        // Input event handler. Takes the type of the input event and updates the state
        // accordingly or exits the application.
        match e {
            TempoChange(new_tempo) => self.tempo = new_tempo,
            TimeSignatureChange(new_time_signature) => self.time_signature = new_time_signature,
            DownbeatToggle => self.downbeat = !self.downbeat,
            Help => println!("{self}"),
            Quit => {
                self.current_token.cancel();
                std::process::exit(0);
            }
        }
        self.restart_task();
    }

    fn restart_task(&mut self) {
        // Cancel the current task and instantiate a new task with a fresh cancellation token
        self.current_token.cancel();
        println!("cancelled token");
        self.current_token = CancellationToken::new();

        println!("new token");
        // Clone the current state post previous user input from which to construct the new task.
        let cloned_token = self.current_token.clone();
        let db = self.downbeat;
        let ts = self.time_signature.clone();
        let tempo = self.tempo;
        println!("{db:?}");
        println!("{ts:?}");
        println!("{tempo:?}");

        let _ = tokio::spawn(async move {
            tokio::select! {
                _ = cloned_token.cancelled() => {}
                _ =  Metronome::play(tempo, db, ts)  => {}
            }
        });
        
    }

    async fn play(tempo: u16, with_downbeat: bool, time_signature: Vec<(u8, u8)>) {
        // calculate the pause interval between playing sounds.
        let pause = ((60_f64 / tempo as f64) * NANO_CONVERTER) as u64;
        let durations_by_denominator = Metronome::calculate_pause_durations(&time_signature, pause);

        // If downbeat is active, we must use the time signature to know when to play the downbeat.
        // For each time signature given in the input (stored as a vec) we must initially play the
        // downbeat and follow that with time siganture -1 * pulse. E.g. The user inputs 'ts 4 3',
        // for alternating bars of 4/4 and 3/4, the loop is downbeat, 3*pulse, downbeat, 2*pulse.
        match with_downbeat {
            true => loop {
                for (numerator, denominator) in time_signature.iter() {
                    // Happy to panic here, time denominator should always be present
                    let pause = durations_by_denominator.get(denominator).unwrap();
                    Metronome::play_downbeat(*pause).await;

                    for _ in 0..*numerator - 1 {
                        Metronome::play_pulse(*pause).await;
                    }
                }
            },
            false => loop {
                Metronome::play_pulse(pause).await;
            },
        }
    }

    fn calculate_pause_durations(time_signatures: &[(u8, u8)], pause: u64) -> HashMap<&u8, u64> {
        // Create a hashmap of time signature denominator to recalculated pause length.
        // Default pause length is calculated based off of crochets/quarter notes, so we can bit
        // shift in either direction to double / half the pause value
        let mut durations_by_denominator = HashMap::new();
        for (_numerator, denominator) in time_signatures.iter() {
            let updated_pause = match denominator {
                1 => pause << 2,
                2 => pause << 1,
                4 => pause,
                8 => pause >> 1,
                16 => pause >> 2,
                32 => pause >> 3,
                _ => todo!(),
            };
            durations_by_denominator.insert(denominator, updated_pause);
        }
        durations_by_denominator
    }

    async fn play_downbeat(pause: u64) {
        _ = Command::new("afplay").arg("sounds/downbeat.wav").spawn();
        sleep(Duration::from_nanos(pause)).await;
    }

    async fn play_pulse(pause: u64) {
        _ = Command::new("afplay").arg("sounds/pulse.wav").spawn();
        sleep(Duration::from_nanos(pause)).await;
    }
}

impl Default for Metronome {
    fn default() -> Self {
        Self {
            current_token: Default::default(),
            downbeat: Default::default(),
            time_signature: vec![(4, 4)],
            tempo: 100,
        }
    }
}

impl std::fmt::Display for Metronome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nWelcome to Metronome!\n\n")?;
        write!(f, "There are 4 available options:\n\n")?;
        writeln!(f, "1. Change the tempo. Using the command 'bpm' followed by your desired tempo. E.g. 'bpm 100' or 'BPM 101'")?;
        writeln!(
            f,
            "2. Toggle the downbeat. Using the command 'db' or 'downbeat'."
        )?;
        writeln!(f, "3. Enter a custom time signature. Using the command 'ts' followed by your desired combination of space separated numbers. This can handle complex combinations of time signatures. E.g. for 1 bar of 4/4 followed by 1 bar of 3/4 you can use 'ts 4 3'.")?;
        writeln!(f, "4. Quit! Using the command 'q' or 'quit' or 'exit'.")?;
        write!(f, "Enter 'help' at anytime to reshow these instructions!")
    }
}

fn get_repl() -> Editor<(), rustyline::sqlite_history::SQLiteHistory> {
    let config = Config::builder().auto_add_history(true).build();
    let history = rustyline::sqlite_history::SQLiteHistory::with_config(config)
        .expect("Failed to initialise");
    let rl: Editor<(), _> = Editor::with_history(config, history).expect("failed to initialised");
    rl
}

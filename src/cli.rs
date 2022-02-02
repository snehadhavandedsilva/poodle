use crate::data::*;
use crate::day::*;
use crate::logs::*;
use clap::Parser;
use std::io;
use std::io::Write;

/// Word-guessing game from your terminal 🟩🟩🟩🟩🟩
#[derive(Parser, Debug)]
#[clap(author = "John Law <poyea@pm.me>")]
#[clap(version)]
#[clap(long_about = None)]
pub struct Cli {
    /// Instruction
    #[clap(arg_enum)]
    cmd: Instruction,
}

#[derive(clap::ArgEnum, Clone, Debug)]
pub enum Instruction {
    Start,
    Log,
}

fn start(today: String) {
    let ws: Words = get_words();
    let today_word = ws.data[&today].to_string();
    let mut today_state = DayState::new(today_word);

    let mut stdin = io::stdin();
    'game: while today_state.remaining != 0 {
        {
            print!("Your guess ({}) → ", today_state.remaining);
            io::stdout().flush().unwrap();
        }
        let mut buffer = String::new();
        {
            stdin.read_line(&mut buffer).unwrap();
            buffer = buffer.trim().to_string();
        }
        if DayState::validate_input(&buffer) {
            let attempt_fmt = today_state.guess(buffer);
            {
                print!("\t\t{}", attempt_fmt);
                io::stdout().flush().unwrap();
            }
            if today_state.finished() {
                break 'game;
            }
        } else {
            println!("Invalid input  ← {}", buffer);
        }
    }
    {
        println!("{}", &today_state);
        Logs::save_log(today_state);
    }
}

fn log() {
    let logs = Logs::get_logs();
    println!("{}", logs);
}

pub fn exec(today: String) {
    let args = Cli::parse();
    println!("[{}] Hello poodler!", &today);
    match args.cmd {
        Instruction::Start => start(today),
        Instruction::Log => log(),
    }
}
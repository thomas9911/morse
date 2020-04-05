use rodio::source::Source;
use rodio::source::{SineWave, Zero};
use rodio::Sink;

use morse_code::MorseString;

use std::convert::TryInto;
use std::time::Duration;

const FREQUENCY: u32 = 523; // C5
const VOLUME: f32 = 0.3;
const TICK_LENGTH: u64 = 150;

fn main() {
    let device = rodio::default_output_device().unwrap();
    let mut sink = Sink::new(&device);
    sink.set_volume(VOLUME);

    if let Some((text, print_bleeps)) = collect_arguments() {
        sink = match add_bleeps(sink, &text, print_bleeps) {
            Ok(x) => x,
            Err(x) => return println!("{}", x),
        };

        sink.sleep_until_end();
    }
}

fn add_bleeps(sink: Sink, input: &str, print_bleeps: bool) -> Result<Sink, &str> {
    let t: MorseString = match input.try_into() {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    if print_bleeps {
        println!("{}", t.print_morse());
    }

    for ch in t.print_morse_short().chars() {
        match ch {
            ' ' => sink.append(blank()),
            '.' => sink.append(beep(1)),
            '-' => sink.append(beep(3)),
            _ => (),
        }
    }

    sink.append(blank());
    Ok(sink)
}

fn blank() -> rodio::source::TakeDuration<Zero<f32>> {
    Zero::<f32>::new(1, 48000).take_duration(Duration::from_millis(TICK_LENGTH))
}

fn beep(n: u64) -> rodio::source::TakeDuration<SineWave> {
    SineWave::new(FREQUENCY).take_duration(Duration::from_millis(n * TICK_LENGTH))
}

fn collect_arguments() -> Option<(String, bool)> {
    let mut print_morse = false;
    let mut args = std::env::args();
    args.next().unwrap();

    let mut first_arg = fetch_first_argument(&mut args)?;

    if ["--print", "-p"].contains(&first_arg.as_ref()) {
        print_morse = true;
        first_arg = fetch_first_argument(&mut args)?;
    }

    if ["-h", "--help", "help"].contains(&first_arg.as_ref()) {
        print_help();
        return None;
    }

    let mut text = vec![first_arg];

    text.extend(args);

    Some((text.join(" "), print_morse))
}

fn fetch_first_argument(args: &mut std::env::Args) -> Option<String> {
    match args.next() {
        None => {
            print_help();
            None
        }
        Some(x) => Some(x),
    }
}

fn print_help() {
    println!(
        "Plays and prints morse code.
    
Usage: morse [OPTIONS] [ARGS ..]

OPTIONS:
    --help -h       prints help
    --print -p      also print the ARGS as morse code
    
ARGS:
    help            prints help
    ARGS            plays the ARGS as morse code
    "
    );
}

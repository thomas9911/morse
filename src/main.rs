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

    if let Some(text) = collect_arguments() {
        sink = add_bleeps(sink, &text, true);

        sink.sleep_until_end();
    }
}

fn add_bleeps(sink: Sink, input: &str, print_bleeps: bool) -> Sink {
    let t: MorseString = input.try_into().unwrap();

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
    sink
}

fn blank() -> rodio::source::TakeDuration<Zero<f32>> {
    Zero::<f32>::new(1, 48000).take_duration(Duration::from_millis(TICK_LENGTH))
}

fn beep(n: u64) -> rodio::source::TakeDuration<SineWave> {
    SineWave::new(FREQUENCY).take_duration(Duration::from_millis(n * TICK_LENGTH))
}

fn collect_arguments() -> Option<String> {
    let mut args = std::env::args();
    args.next().unwrap();

    let first_arg = args.next();

    if first_arg.is_none() {
        print_help();
        return None;
    }

    let first_arg = first_arg.unwrap();

    if ["-h", "--help", "help"].contains(&first_arg.as_ref()) {
        print_help();
        return None;
    }

    let mut text = vec![first_arg];

    text.extend(args);

    Some(text.join(" "))
}

fn print_help() {
    println!(
        "print and plays morse code.
    
    morse [ARGS]

    ARGS:
        help    prints help

        ARGS    prints and plays the ARGS as morse code

    "
    );
}

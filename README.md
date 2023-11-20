# Rust Synthesizer


## Requirements

* x86-64
* Linux/Unix
* [Rust](https://www.rust-lang.org/tools/install)

## Cargo dependencies
* rodio = "0.17.1"
* console = "0.15.7"
* rand = "0.8"

## Start

The script "up" builds and runs our application by executing the following:
```
1. cargo build
2. cargo run
```

## Usage
Once the application is running, you can interact with it using the following keys:

    Q, W, E, R, T, Y, U: Play musical notes A, B, C, D, E, F, G respectively.

    O: Decrease the octave.
    P: Increase the octave.

    F: Change the synthesizer waveform (sine, square, saw).

    3: Activate the low-pass filter.
    1: Increase the filter cutoff.
    2: Decrease the filter cutoff.
    4: Increase the filter resonance.
    5: Decrease the filter resonance.
     

    Z: Quit the application.
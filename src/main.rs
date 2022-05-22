mod chipp8;
use minifb;
use std::env;

fn main() {
    // let mut chip = chipp8::Chipp8::new(minifb::Scale::FitScreen);
    // let mut chip = chipp8::Chipp8::new(minifb::Scale::X1);
    // let mut chip = chipp8::Chipp8::new(minifb::Scale::X2);
    // let mut chip = chipp8::Chipp8::new(minifb::Scale::X4);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {panic!();}
    let mut chip = chipp8::Chipp8::new(minifb::Scale::X8);
    chip.init( &args[1]);
    while chip.window.is_open() {
        chip.run();
    }
}

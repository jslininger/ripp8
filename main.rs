mod chipp8;
use minifb;

fn main() {
    // let mut chip = chipp8::Chipp8::new(minifb::Scale::FitScreen);
    // let mut chip = chipp8::Chipp8::new(minifb::Scale::X1);
    // let mut chip = chipp8::Chipp8::new(minifb::Scale::X2);
    // let mut chip = chipp8::Chipp8::new(minifb::Scale::X4);
    let mut chip = chipp8::Chipp8::new(minifb::Scale::X8);

    chip.init();
    while chip.window.is_open() {
        chip.run();
    }
}

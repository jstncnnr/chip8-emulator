use chip8::rom::ROM;

fn main() {
    let rom = ROM::from_file("./data/IBM Logo.ch8").expect("Error opening ROM");
}

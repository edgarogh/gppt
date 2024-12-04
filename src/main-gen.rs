use gppt::Generator;

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let file = std::fs::read_to_string(arg).unwrap();
    let mut generator = Generator::new("../ppt");
    generator.update("unknown", &file).unwrap();
}

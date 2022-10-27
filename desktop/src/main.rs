extern crate sdl2;
use desktop::run;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: program [path_to_rom]");
        return;
    }
    run(&args[1]);
}

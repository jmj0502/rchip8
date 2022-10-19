pub mod chip8;
pub mod display;

pub fn hello_core() {
    println!("Hello from Chip8 core!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

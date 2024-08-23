fn main() {
    println!("Hello world.");
    println!("{}", "Hello rust");
    println!("{0} is {1}. {1} is not {0}", "Rust", 12);

    println!("{name} is a {type}", name="Lol", type="word"); // Rust Rover did not show up auto complete for named parameter?

    println!("Base 10:               {}",   2024); // 2024
    println!("Base 2 (binary):       {:b}", 2024); // 11111101000
    println!("Base 8 (octal):        {:o}", 2024); // 3750
    println!("Base 16 (hexadecimal): {:x}", 2024); // 7e8
    println!("Base 16 (hexadecimal): {:X}", 2024); // 7E8

    println!("Float is floaty \"{number:>5}\", isn't it?", number=1.2); // "  1.2"
    println!("{number:0>width$}", number=1, width=5); // 00001
}
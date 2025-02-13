use std::io;

fn main() {
    let mut nome = String::new();
    println! ("Digite seu nome:");

    io::stdin()
        .read_line(&mut nome)
        .expect("Error reading console");
        
    println!("Seu nome é {nome}");
}

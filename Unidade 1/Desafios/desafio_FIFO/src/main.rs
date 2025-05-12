use std::collections::VecDeque;

fn main() {
    let mut fila: VecDeque<i32> = VecDeque::new();

    println!("Adiciona 16 na fila");
    fila.push_back(16);
    println!("Adiciona 8 na fila");
    fila.push_back(8);
    println!("Adicionaa 420 na fila");
    fila.push_back(420);

    if let Some(primeiro) = fila.front() {
        println!("O primeiro numero da fila é {}", primeiro);
    }

    if let Some(ultimo) = fila.back() {
    println!("O ultimo numero da fila é {}", ultimo);
    }

    if fila.is_empty() {
        println!("A fila esta vazia")
    } else {
        println!("A fila tem {} elementos", fila.len());
    }
    
    while let Some(remover) = fila.pop_front() {
        println!("O primeiro elemento foi removido!");
        println!("A fila tem {} elementos", fila.len());
    } 

}

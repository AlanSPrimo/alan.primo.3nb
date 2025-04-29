use std::collections::HashMap;

fn main() {
    //Criando o hashmap
    let mut estoque = HashMap::new();

    //1. Inserção de valores
    estoque.insert("banana", 100);
    estoque.insert("morango", 50);
    estoque.insert("maracuja", 20);
    estoque.insert("kiwi", 10);

    println!("{:?}",estoque);

    //2. Acessando de forma segura os valores
    if let Some(qtde) = estoque.get("morango"){
        println!("Temos {:?} morangos em estoque", qtde);
    }

    //3. Atualizando o estoque com entry()
    estoque.entry("banana").and_modify(|qtde| *qtde += 90);
    estoque.entry("kiwi").and_modify(|qtde| *qtde += 20);

    println!("{:?}",estoque);

    estoque.remove("maracuja");

    //4. Remove da lista
    println!("Maracuja removido");
    println!("{:?}", estoque);

    //5. Filtrar todas as frutas acima de 100
    estoque.retain(|fruta, &mut qtde| qtde > 100);
    println!("{:?}",estoque);

    //6.Limpeza total
    estoque.clear();
    println!("{:?}", estoque);
}
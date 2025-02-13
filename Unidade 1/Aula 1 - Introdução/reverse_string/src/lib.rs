// Criando uma função publica em Rust
// Utilizarei "pub fn", desta forma tornando a função como publica e podendo acessar em outros módulos

//Referencia a função publica criada e classifica ela como String
pub fn reverse(s: &str) -> String { // Define o parametro s como um tipo String (string dinamica)
    s.chars().rev().collect() //Chama os iteradores desta função
    // chars iterador que classifica o parametro s dentro dos caracteres presentes no Unicode
    // rev é o iterador que inverte a string
    // collect é o iterador que coleta os caracteres invertidos e reescreve em uma nova string
} 

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn reverse(input: &str) -> String { // Aqui começamos a montar uma função publica conforme a atividade pede
    // Vale apena observar o uso do &str, que utiliza uma referencia da string no lugar da string em si... E o "-> String", indica que deve retornar um string
        input.chars().rev().collect()
    // Aqui convertemos a string de entrada em caracteres ".chars()"
    // Após isso utilizamos o ".rev()" para reverter este caracteres
    // E o ".collect()" para coletar o resultado
}

use reverse_string::reverse;
// Referenciamos que do módulo "reverse_string" iremos importar a função reverse

// Agora iremos começar a realizar os testes requisitados no exercicio

// Primeiro iremos testar subindo um função com uma palavra normal

#[test] //Aqui definimos uma macro, para que quando rodarmos o código seja lido individualmente.
fn palavra_invertida() {
    let input = "Teste"; // Entrada da String para reverter
    let expected = "etseT"; // Definimos o que seria experado voltar quando rodarmos o código
    assert_eq!(reverse(input), expected); // Aqui como pedido no exercicio, comparamos se o resultado da inversão é igual o "expected"
}

#[test] // Outro teste
fn palavra_vazio() {
    let input = " ";
    let expected = " ";
    assert_eq!(reverse(input), expected);
}

#[test]
fn palindromo() {
    let input = "radar";
    let expected = "radar";
    assert_eq!(reverse(input), expected);
}

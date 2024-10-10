fn main() {
    // Variáveis com operadores aritméticos
    let a = 10;
    let b = 20;
    let soma = a + b;          // Soma
    let subtracao = b - a;     // Subtração
    let multiplicacao = a * b; // Multiplicação
    let divisao = b / a;       // Divisão
    let resto = b % a;         // Resto da divisão

    // Variáveis booleanas com operadores lógicos
    let x = true;
    let y = false;

    let resultado_and = x && y; // Operador lógico AND
    let resultado_or = x || y;  // Operador lógico OR
    let resultado_not = !x;     // Operador lógico NOT

    // Exibindo resultados dos operadores aritméticos
    println!("Soma de a + b = {}", soma);
    println!("Subtração de b - a = {}", subtracao);
    println!("Multiplicação de a * b = {}", multiplicacao);
    println!("Divisão de b / a = {}", divisao);
    println!("Resto de b % a = {}", resto);

    // Exibindo resultados dos operadores lógicos
    println!("x AND y = {}", resultado_and);
    println!("x OR y = {}", resultado_or);
    println!("NOT x = {}", resultado_not);

    // Estrutura condicional if-else usando operadores aritméticos e lógicos
    if a < b && x {
        println!("a é menor que b e x é verdadeiro.");
    } else if a == b || y {
        println!("a é igual a b ou y é verdadeiro.");
    } else {
        println!("Nenhuma das condições foi satisfeita.");
    }
}

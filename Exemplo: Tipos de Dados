fn main() {
    //Tipos Primitivos
    
    // Declaração de inteiros com diferentes tamanhos
    let a: i32 = 10; // Inteiro com sinal de 32 bits
    let b: u8 = 20;  // Inteiro sem sinal de 8 bits
    let c: isize = -50; // Inteiro com sinal baseado na arquitetura (32 ou 64 bits)
    
    // Impressão de Inteiros
    println!("{}, {}, {}",a, b, c);
    
    //Declaração de um caractere
    let letra: char = 'A';  // Um único caractere
    let emoji: char = '😊'; // Caracteres podem ser emojis ou qualquer caractere Unicode

    // Impressão dos caracteres
    println!("Letra: {}", letra);
    println!("Emoji: {}", emoji);
    
    // Declaração de números de ponto flutuante
    let a: f32 = 3.14;  // f32 (32 bits)
    let b: f64 = 2.718; // f64 (64 bits)
    
    // Impressão dos resultados
    println!("{}", a);
    println!("{}", b);
    
    // Declaração de variáveis booleanas
    let verdadeiro: bool = true; // valor booleano 'true'
    let falso: bool = false;     // valor booleano 'false'

    // Uso em uma expressão condicional
    if verdadeiro {
        println!("Isso é verdade!");
    } else {
        println!("Isso é falso!");
    }
    
    if falso {
        println!("Isso é verdade!");
    } else {
        println!("Isso é falso!");
    }

    //Tipos Compostos
    
    // Declaração de Vetores
    let numeros: [i32; 5] = [1, 2, 3, 4, 5]; // Array de 5 inteiros
    let todos_iguais = [3; 4]; // Array de 4 elementos, todos com valor 3

    println!("Primeiro elemento: {}", numeros[0]); // Acessa o primeiro elemento
    println!("Array todos iguais: {:?}", todos_iguais); // Imprime todo o array
    
    let slice = &numeros[1..3]; // Slice dos elementos do índice 1 ao 2 (exclui o 3)

    println!("Slice: {:?}", slice); // Saída: [2, 3]
    
    // Declaracao de tupla
    let pessoa: (i32, f64, char) = (25, 72.5, 'A'); // Tupla com tipos diferentes

    // Acessando elementos da tupla por índice
    println!("Idade: {}", pessoa.0);
    println!("Altura: {}", pessoa.1);
    println!("Inicial: {}", pessoa.2);
    
     //Tipo Padrão

    // Declaracao de String
    let s1: String = String::from("Hello"); // String dinâmica
    let s2: &str = "World"; // String Imutavel
    
    println!("Dinamica: {}", s1);
    println!("imutavel: {}", s2);
}

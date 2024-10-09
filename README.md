# Rust
# Apresentação sobre Rust
Reposítorio da nossa apresentação sobre a linguagem de programação Rust, onde foi abordando sua origem, características e casos de uso.

# Este repositório inclui:
- **RUST.pdf**: O arquivo principal da apresentação, que inclui todos os slides e explicações detalhadas sobre o Rust, sua história, uso e conceitos técnicos.
- **Exemplos de Código**: Trechos de código demonstrando a sintaxe do Rust, tipos de dados e diversos paradigmas de programação, que podem ser testados em [Rust Playground](https://play.rust-lang.org/).

## Sumário
- [Sobre o Rust](#sobre-o-rust)
- [Por que usar Rust?](#por-que-usar-rust)
- [Tipos de Dados no Rust](#tipos-de-dados-no-rust)
- [Expressões e Operadores](#expressões-e-operadores)
- [Paradigmas de Programação no Rust](#paradigmas-de-programação-no-rust)
- [Concorrência no Rust](#concorrência-no-rust)
- [Tratamento de Erros](#tratamento-de-erros)
- [Contribuidores](#contribuidores)
- [Referências](#referências)

## Sobre o Rust
Rust foi criado em 2006 por Graydon Hoare, que desejava uma linguagem de programação que evitasse os erros de memória comuns em linguagens como C e C++. O objetivo do Rust é ser rápido, seguro e concorrente. A linguagem tem crescido rapidamente, especialmente em sistemas de baixo nível.

Rust foi lançado oficialmente em 2015 com a versão 1.0 e desde então começõu a ser implantado mais e mais por grandes empresas como Meta, Dropbox e Discord. Em 2021, foi fundada a Rust Foundation para apoiar desenvolvedores com bolsas e recursos comunitários.

## Por que usar Rust?
Rust tem potencial para ser implementado em diversas áreas como:

- **Desenvolvimento Web**: Utilizando frameworks como Rocket e Actix.
- **Desenvolvimento de Jogos**: Ideal para aplicações de alta performance, como jogos e processamento de dados.
- **Confiabilidade**: O modelo de ownership do Rust garante segurança de memória e diminuindo os erros de execução.
- **Produtividade**: O Rust oferece excelente documentação, um compilador amigável e suporte a diversos editores.
- **Desempenho**: O Rust é altamente eficiente em gerenciamento de memória, ideal para sistemas de baixo nível como kernels e drivers.

## Tipos de Dados no Rust
Rust suporta tanto tipos de dados primitivos quanto compostos. Exemplos incluem:

- **Inteiros**: `i8` a `u128` e `isize`
- **Ponto Flutuante**: `f32`, `f64`
- **Booleanos**: `true`, `false`
- **Arrays e Tuplas**: Rust permite arrays de tamanho fixo e tuplas com múltiplos tipos.
### Exemplo:
- fn main() {
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
![image](https://github.com/user-attachments/assets/18edf3a7-76f6-4346-a5f1-185b1f036f6a)

## Expressões e Operadores
Rust usa expressões para computar valores, como operadores aritméticos (+, -, *, /) e lógicos (&&, ||, !). Também suporta estruturas de controle de fluxo como if/else, while e for.

Rust não possui o operador ternário tradicional, mas utiliza expressões if para retornar valores.

### Exemplo:

## Paradigmas de Programação no Rust
Rust é uma linguagem multiparadigma, com suporte para:

- Programação Imperativa: Estrutura central com variáveis, loops e estruturas de controle.
- Programação Funcional: Suporte a closures, iteradores e pattern matching.
- Programação Orientada a Objetos (POO): Através de traits e estruturas (similares a classes).
- Programação Concorrente: Concorrência segura através de threads, async/await e canais.

### Exemplo:

## Concorrência no Rust
Rust fornece ferramentas robustas para programação concorrente, garantindo segurança de memória através do sistema de ownership. Alguns dos recursos de concorrência incluem:

- Threads
- Mutexes
- Canais Assíncronos

## Tratamento de Erros
Rust adota uma abordagem única para o tratamento de erros, oferecendo:

Result<T, E> para erros recuperáveis.
Panic!() para erros irrecuperáveis.

Rust não utiliza exceções como outras linguagens, e sim o tipo Result para a maioria dos cenários de tratamento de erros.

## Contribuidores
João Luis
Paulo Sérgio
Fabrício
Victor Daniel

## Referências
1.Site Oficial do Rust
2.Metz, C. (2023). Rust: A Linguagem de Programação que Mais Cresce no Mundo. MIT Technology Review.

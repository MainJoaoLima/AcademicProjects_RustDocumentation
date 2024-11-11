# Analise Lexica 

# Questão 1 - Expressão regular para o reconhecimento de índices de datasets do pandas.

Criamos uma expressão com REGEX para reconhecimento dos indices de datasets do Pandas com base nos seguintes exemplos:

O número da coluna, que é um número inteiro, positivo, negativo ou zero.
Ex: x[0], x[10], x[-2]

O nome da coluna entre aspas, simples ou duplas. Considere que os nomes das colunas são formados por letras maiúsculas, minúsculas ou espaços em branco.

Ex: x[‘Date’], x[“New Column”]

Um intervalo (slice) formado por dois números positivos (ou zero) ou dois números negativos, separados por “:”.

Ex: x[0:5], x[2:2], x[-1: -5]

Um intervalo (slice) formado por dois nomes de colunas entre aspas, simples ou duplas.

Ex: x[‘Data’:’State’], x[“District”:’Tested’]

No final a expressão ficou: `-?[0-9]+ | [' "] [a-z][a-z0-9]+ [' "] | -?[0-9]+ : - ?[0-9]+ | [' "] [a-z][a-z0-9]+ [' "] : [' "] [a-z][a-z0-9]+ [' "]`


# Questão 2 implementar um programa que utiliza a biblioteca e expressões regulares para verificar se uma cadeia informada é ou não reconhecida como um índice de dataset válido do pandas

Para fazer o programa utilizamos a linguagem Python e utilizamos a biblioteca 're' para reconhecimento das expresões regulares, abaixo o código em python que fizemos para a validação da expressão e das strings aceitas.






# Questão 3 - Conversão da expressão regular para um AFN






# Rust
# Apresentação sobre Rust
Reposítorio da nossa apresentação sobre a linguagem de programação Rust, onde foi abordando sua origem, características e casos de uso.

# Este repositório inclui:
- **RUST.pptx**: O arquivo da apresentação, que inclui todos os slides e explicações detalhadas sobre o Rust, sua história, uso e conceitos técnicos.
[Apresentação Rust](https://docs.google.com/presentation/d/13EIbcE18h3AD82ldQS1uh_MQkdh1UbAG1zkTpLF8v3w/edit?usp=sharing)

-  **Documentação.doc**: Arquivo da documentação dos conceitos técnicos do Rust.
[Documentação](https://docs.google.com/document/d/1h5WY7LeXUb-27CwiDJV8pJb8RfLrKX7zl2-M5RCiNOs/edit?usp=sharing)

- **Exemplos de Código.rs**: Trechos de código demonstrando a sintaxe do Rust, tipos de dados e diversos paradigmas de programação, que podem ser testados em
[Rust Playground](https://play.rust-lang.org/).

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
A linguagem de programação Rust, foi criada em 2006 por Graydon Hoare, que desejava uma linguagem de programação que evitasse os erros de memória comuns em linguagens como C e C++. O objetivo do Rust é ser rápido, seguro e competitivo. A linguagem tem crescido rapidamente, especialmente em sistemas de baixo nível.

Em 2015,  teve seu lançamento oficial com a versão 1.0 e desde então começou a ser implantado mais e mais por grandes empresas como Meta, Dropbox e Discord. Em 2021, foi fundada a Rust Foundation para apoiar e subsidiar desenvolvedores com bolsas e recursos.


## Por que usar Rust?
- **Desenvolvimento Web**: Utilizando frameworks como Rocket e Actix.
- **Desenvolvimento de Jogos**: Ideal para aplicações de alta performance, como jogos e realidade aumentada.
- **Confiabilidade**: O modelo de ownership do Rust garante segurança de memória e diminuindo os erros de execução.
- **Produtividade**: O Rust oferece excelente documentação, um compilador amigável e suporte a diversos editores.
- **Desempenho**: O Rust é altamente eficiente em gerenciamento de memória, ideal para sistemas de baixo nível como kernels e drivers.

## Tipos de Dados no Rust
Rust suporta tanto tipos de dados primitivos quanto compostos. Exemplos incluem:

- **Inteiros**: `i8` a `u128` e `isize` ou `usize`
- **Ponto Flutuante**: `f32`, `f64`
- **Booleanos**: `true`, `false`
- **Arrays e Tuplas**: Rust permite arrays de tamanho fixo e tuplas com múltiplos tipos.

### Exemplo:
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
![image](https://github.com/user-attachments/assets/18edf3a7-76f6-4346-a5f1-185b1f036f6a)

## Expressões e Operadores
Rust usa expressões para computar valores, como operadores aritméticos (+, -, *, /) e lógicos (&&, ||, !). Também suporta estruturas de controle de fluxo como if/else, while e for.

Rust não possui o operador ternário tradicional, mas utiliza expressões if para retornar valores.

### Exemplo:
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

![image](https://github.com/user-attachments/assets/d0e81cbe-9b58-4e8f-8359-47578b0d6762)


## Paradigmas de Programação no Rust
Rust é uma linguagem multiparadigma, com suporte para:

- Programação Imperativa: Estrutura central com variáveis, loops e estruturas de controle.
- Programação Funcional: Suporte a closures, iteradores e pattern matching.
- Programação Orientada a Objetos (POO): Através de traits e estruturas (similares a classes).
- Programação Concorrente: Concorrência segura através de threads, async/await e canais.

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
- João Luis
- Paulo Sérgio
- Fabrício
- Victor Daniel

## Referências
- 1.Site Oficial do Rust
- 2.Metz, C. (2023). Rust: A Linguagem de Programação que Mais Cresce no Mundo. MIT Technology Review.

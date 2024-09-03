/*
    entrada:
        - uma string com a primeira palavra
        - uma string com a segunda palavra
        - uma string com a terceira palavra
    saida:
        - uma string com a classificacao

        PSEUDOCODE:
        ler as 3 entradas de palavras
        verificar se a primeira palavra é vertebrado ou invertebrado
        verificar se a segunda palavra é ave, mamifero, inseto ou anelideo
        verificar se a terceira palavra é carnivoro, onivoro, herbivoro ou hematofago
        imprimir a classificacao

        Como resolver esse problema de forma mais eficiente ou complexa?
        - podemos usar uma estrutura de dados para armazenar as entradas
        - podemos usar uma estrutura de dados para armazenar as classificacoes
        
        - Dicionario ou HashMap para armazenar as entradas
        O que é hashmap?
        - é uma estrutura de dados que permite armazenar valores associados a chaves
        Exemplo:
        - chave: valor
        - "vertebrado": ["ave", "mamifero", "inseto", "anelideo"]
        - "invertebrado": ["inseto", "anelideo"]
        - "ave": ["carnivoro", "onivoro", "herbivoro", "hematofago"]
        - "mamifero": ["carnivoro", "onivoro", "herbivoro", "hematofago"]
        - "inseto": ["hematofago"]
        - "anelideo": ["hematofago"]

        entao hashmap é dicionario? 
        -> https://pt.stackoverflow.com/questions/429851/qual-a-diferen%C3%A7a-entre-um-mapa-um-dicion%C3%A1rio-um-array-associativo-e-uma-tabela

 
    // & é um alias para o tipo de referência em Rust. Ele permite que você crie uma referência para um valor, em vez de copiar o valor.
    // Some é um tipo de enum em Rust que representa um valor opcional. Ele é usado para lidar com valores que podem ou não existir.
    // unwrap é um método que desembrulha o valor de um Option<T>. Se o valor for Some, ele retorna o valor. Se o valor for None, ele gera um erro.
    // get é um método que retorna um valor opcional do HashMap. Se a chave existir, ele retorna o valor. Se a chave não existir, ele retorna None.
    // & é um alias para o tipo de referência em Rust. Ele permite que você crie uma referência para um valor, em vez de copiar o valor.

*/

use std::collections::HashMap;
use std::io;

fn main() {
    let mut animal_list = HashMap::new();

    animal_list.insert(
        "vertebrado",
        vec![
            ("ave", vec![("carnivoro", "aguia"), ("onivoro", "pomba")]),
            ("mamifero", vec![("onivoro", "homem"), ("herbivoro", "vaca")]),
        ]
    );
    animal_list.insert(
        "invertebrado",
        vec![
            ("inseto", vec![("hematofago", "pulga"), ("herbivoro", "lagarta")]),    
            ("anelideo", vec![("hematofago", "sanguessuga"), ("onivoro", "minhoca")]),
        ]
    );

    let mut first_word = String::new();
    io::stdin().read_line(&mut first_word).unwrap();
    let first_word = first_word.trim();

    let mut second_word = String::new();
    io::stdin().read_line(&mut second_word).unwrap();
    let second_word = second_word.trim();

    let mut third_word = String::new();
    io::stdin().read_line(&mut third_word).unwrap();
    let third_word = third_word.trim();

    if let Some(groups) = animal_list.get(first_word) {
        for (group_name, species_list) in groups {
            if group_name == &second_word {
                for (diet, animal) in species_list {
                    if diet == &third_word {
                        println!("{}", animal);
                        return;
                    }
                }
            }
        }
    }

    println!("Bilu bilu biluuuu nao ta na listaaaa.");
}
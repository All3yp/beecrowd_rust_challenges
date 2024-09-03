mod bee_1045;
use std::io;
use bee_1045::triangle_type;

fn main() {

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
    
    let mut values = input.trim().split_whitespace();
    
    let a: f64 = values.next().unwrap().parse().expect("Erro ao ler o lado a");
    let b: f64 = values.next().unwrap().parse().expect("Erro ao ler o lado b");
    let c: f64 = values.next().unwrap().parse().expect("Erro ao ler o lado c");

    let triangle_types = triangle_type(a, b, c);
    for t in triangle_types {
        println!("{}", t);
    }
}

#[test]
fn test_triangle_type() {
    assert_eq!(triangle_type(7.0, 5.0, 7.0), vec!["TRIANGULO ACUTANGULO", "TRIANGULO ISOSCELES"]);
    assert_eq!(triangle_type(6.0, 6.0, 10.0), vec!["TRIANGULO OBTUSANGULO", "TRIANGULO ISOSCELES"]);
    assert_eq!(triangle_type(6.0, 6.0, 6.0), vec!["TRIANGULO ACUTANGULO", "TRIANGULO EQUILATERO"]);
    assert_eq!(triangle_type(5.0, 7.0, 2.0), vec!["NAO FORMA TRIANGULO"]);
    assert_eq!(triangle_type(6.0, 8.0, 10.0), vec!["TRIANGULO RETANGULO"]);
}
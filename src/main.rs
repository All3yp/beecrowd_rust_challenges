use std::io;

mod bee_1045;
mod bee_1048;
use bee_1045::triangle_type;
use bee_1048::salary_increase;

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

    let salary: f64 = input.trim().parse().expect("Por favor, insira um número válido");
    let new_salary = salary_increase(salary);
    
    println!("Novo salario: {:.2}", new_salary);
    println!("Reajuste ganho: {:.2}", new_salary - salary);

    let percentual = (new_salary - salary) / salary * 100.0;
    println!("Em percentual: {:.0} %", percentual);
}

#[test]
fn test_triangle_type() {
    assert_eq!(triangle_type(7.0, 5.0, 7.0), vec!["TRIANGULO ACUTANGULO", "TRIANGULO ISOSCELES"]);
    assert_eq!(triangle_type(6.0, 6.0, 10.0), vec!["TRIANGULO OBTUSANGULO", "TRIANGULO ISOSCELES"]);
    assert_eq!(triangle_type(6.0, 6.0, 6.0), vec!["TRIANGULO ACUTANGULO", "TRIANGULO EQUILATERO"]);
    assert_eq!(triangle_type(5.0, 7.0, 2.0), vec!["NAO FORMA TRIANGULO"]);
    assert_eq!(triangle_type(6.0, 8.0, 10.0), vec!["TRIANGULO RETANGULO"]);
}
/*

Exemplo de Entrada	Exemplo de Saída
400.00

Novo salario: 460.00
Reajuste ganho: 60.00
Em percentual: 15 %

800.01

Novo salario: 880.01
Reajuste ganho: 80.00
Em percentual: 10 %

2000.00

Novo salario: 2140.00
Reajuste ganho: 140.00
Em percentual: 7 %


*/

// #[test]
// fn test_salary_increase() {
//     assert_eq!(salary_increase(400.00), 460.00);
//     assert_eq!(salary_increase(800.01), 880.01);
//     assert_eq!(salary_increase(2000.00), 2140.00);
// }
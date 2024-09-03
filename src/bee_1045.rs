pub fn triangle_type(a: f64, b: f64, c: f64) -> Vec<String> {
    let mut vec_of_triangle = vec![a, b, c];

    vec_of_triangle.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let a = vec_of_triangle[0];
    let b = vec_of_triangle[1];
    let c = vec_of_triangle[2];

    let mut results = Vec::new();

    if a >= b + c {
        results.push("NAO FORMA TRIANGULO".to_string());
    } else {
        if a * a == b * b + c * c {
            results.push("TRIANGULO RETANGULO".to_string());
        } else if a * a > b * b + c * c {
            results.push("TRIANGULO OBTUSANGULO".to_string());
        } else if a * a < b * b + c * c {
            results.push("TRIANGULO ACUTANGULO".to_string());
        }

        if a == b && b == c {
            results.push("TRIANGULO EQUILATERO".to_string());
        } else if a == b || b == c || a == c {
            results.push("TRIANGULO ISOSCELES".to_string());
        }
    }

    results
}

/*
    pseudocodigo

    ler 3 valores flutuantes
    ordenar os valores
    se a >= b + c
        println!("NAO FORMA TRIANGULO")
    senao se a^2 = b^2 + c^2
         println!("TRIANGULO RETANGULO")
    senao se a^2 > b^2 + c^2
        println!("TRIANGULO OBTUSANGULO")
    senao 
        println!("TRIANGULO ACUTANGULO")
    se a == b && b == c
        println!("TRIANGULO EQUILATERO")
    senao se a == b || b == c || a == c
        println!("TRIANGULO ISOSCELES")
    fim_se
*/
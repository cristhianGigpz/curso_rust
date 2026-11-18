fn main() {
    println!("Control de flujo en Rust - Bucles");

    println!("Bucle loop:");
    let mut contador = 1;

    loop {
        println!("Número: {}", contador);

        if contador == 5 {
            break;
        }

        contador += 1;
    }

    let mut numero = 0;

    let resultado = loop {
        numero += 1;

        if numero == 5 {
            break numero * 2;
        }
    };

    println!("Resultado: {}", resultado);
    
    ////////////////////////////////////////////////////////

    println!("Bucle while:");
    let mut contador = 1;

    while contador <= 5 {
        println!("Contador: {}", contador);
        contador += 1;
    }

    ////////////////////////////////////////////////////////

    println!("Bucle for:");
    for numero in 1..=6 {
        println!("Número: {}", numero);
    }
    
    let lenguajes = ["Rust", "Python", "Go"];

    for leng in lenguajes {
        println!("Lenguaje: {}", leng);
    }
     
    let frutas = ["Manzana", "Pera", "Uva"];

    for (indice, fruta) in frutas.iter().enumerate() {
        println!("{}: {}", indice, fruta);
    }

    ///////////////////////////////////////
    println!("break y continue");
    for numero in 1..=10 {
        if numero == 3 {
            continue;
        }

        if numero == 8 {
            break;
        }

        println!("{}", numero);
    }

    /////////////////////////////////////
    println!("Etiqueta en bucles");
    'externo: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                break 'externo;
            }

            println!("i: {}, j: {}", i, j);
        }
    }

    let numero = 7;

    println!("=== Tabla del {} ===", numero);

    for i in 1..=12 {
        let resultado = numero * i;
        println!("{} x {} = {}", numero, i, resultado);
    }

    
}

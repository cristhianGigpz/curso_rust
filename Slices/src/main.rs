fn main() {
    //[10, 20, 30, 40, 50, 60];
    // Slice: [20,30,40]
    //&coleccion[inicio..fin]
    let texto = String::from("Hola Mundo");

    let hola: &str = &texto[0..4];
    let mundo: &str = &texto[5..10];

    println!("{}", hola);
    println!("{}", mundo);
    println!("Original: {}", texto);
    ///////////////////////////////////////
    // Sintaxis	Descripción-----------------------
    // &texto[0..4]	Desde el índice 0 hasta antes del 4
    // &texto[..4]	Desde el inicio hasta antes del 4
    // &texto[5..]	Desde el índice 5 hasta el final
    // &texto[..]	Todo el contenido
    // &texto[0..=3]	Desde el índice 0 hasta el 3 inclusive

    let texto = String::from("Hola Rust");

    println!("{}", &texto[..4]);
    println!("{}", &texto[5..]);
    println!("{}", &texto[..]);
    ///////////////////////////////////////////////////

    let numeros = [10, 20, 30, 40, 50];

    let porcion: &[i32] = &numeros[1..4];

    println!("Arreglo: {:?}", numeros);
    println!("Slice: {:?}", porcion);
    println!("Longitud: {}", porcion.len());
    /////////////////////////////////////////////////

    let mut numeros = [10, 20, 30, 40, 50];

    let porcion = &mut numeros[1..4];

    porcion[0] = 100;
    porcion[1] = 200;

    println!("{:?}", numeros);

    //////////////Slices como parámetros de funciones//////////////////
    println!("-------------------------------");

    let numeros = [10, 20, 30, 40, 50];

    println!("Suma total: {}", sumar(&numeros));
    println!("Suma parcial: {}", sumar(&numeros[1..4]));
    
    /////////////////////////////////////////////////////////////////
    let texto = "Perú";

    println!("{}", &texto[..3]);  // Per
    println!("{}", &texto[3..5]); // ú
    //println!("{}", &texto[..4]); // Panic
    /////////////////////////////////////////////////////////////
    println!("{:?}", texto.get(0..3));
    println!("{:?}", texto.get(0..4));
    println!("{:?}", texto.get(0..5));
    
    ///////////////////////Slices Borrowing/////////////////////
    let mut texto = String::from("Hola Mundo");

    let palabra = &texto[..4];
    //texto.clear();
    println!("{}", palabra);
    /////////////////////////////////////////////////////////

    let mensaje = String::from("Perú Hola");

    let palabra = primera_palabra(&mensaje);

    println!("Mensaje: {}", mensaje);
    println!("Primera palabra: {}", palabra);

    ///////////RETO RESUELTO//////////////////
    let notas = [11, 18, 15, 19, 10];
    let promedio = calcular_promedio(&notas);

    println!("Promedio: {}", promedio);
}

fn sumar(numeros: &[i32]) -> i32 {
    let mut total = 0;

    for numero in numeros {
        total += *numero;
    }

    total
}

fn primera_palabra(texto: &str) -> &str {
    for (indice, caracter) in texto.char_indices() {
        if caracter.is_whitespace() {
            return &texto[..indice];
        }
    }

    texto
}

fn calcular_promedio(numeros: &[i32]) -> f64 {
    let total: i32 = numeros.iter().sum();
    let cantidad = numeros.len();

    if cantidad == 0 {
        return 0.0;
    }

    total as f64 / cantidad as f64
}
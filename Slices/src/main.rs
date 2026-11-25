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

}

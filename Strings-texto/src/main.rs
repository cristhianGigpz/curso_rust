fn main() {
    let saludo: &str = "Hola Mundo";
    let mut mensaje: String = String::from("Hola");

    mensaje.push_str(" Rust");

    println!("{}", saludo);
    println!("{}", mensaje);   
    /////////////////////////////////////////////

    let texto1 = String::new();
    let texto2 = String::from("Rust");
    let texto3 = "Hola Mundo".to_string();

    println!("Texto 1: {}", texto1);
    println!("Texto 2: {}", texto2);
    println!("Texto 3: {}", texto3);

    ////////////////////////////////////////////
    let mut mensaje2 = String::from("Hola");

    mensaje2.push_str(" Mundo");
    mensaje2.push('!');

    println!("{}", mensaje2);
    ////////////////////////////////////////////
    
    let nombre = String::from("Rust");
    let mensaje3 = String::from("Hola ") + &nombre;

    println!("{}", mensaje3);

    ///////////////////////////////////////////
    let nombre = "Ana";
    let edad = 25;

    let mensaje = format!("Hola {}, tienes {} años", nombre, edad);

    println!("{}", mensaje);

    ///////////////////////////////////////////////
    let texto = "Rust";
    let saludo = "Perú";

    println!("Bytes Rust: {}", texto.len());
    println!("Bytes Perú: {}", saludo.len());
    println!("Caracteres Perú: {}", saludo.chars().count());
    //////////////////////////////////////////////////////

    let texto = "Rust";

    for caracter in texto.chars() {
        println!("{}", caracter);
    }

    let primero = texto.chars().next();

    println!("Primer carácter: {:?}", primero);

    //////////////////////////////////////////////////////
    let texto = "  Hola Rust  ";

    println!("Mayúsculas: {}", texto.to_uppercase());
    println!("Minúsculas: {}", texto.to_lowercase());
    println!("Sin espacios: {}", texto.trim());
    println!("Contiene Rust: {}", texto.contains("Rust"));
    println!("Reemplazo: {}", texto.replace("Rust", "Mundo"));

    let lenguajes = "Rust Python Go";

    for lenguaje in lenguajes.split(" ") {
        println!("{}", lenguaje);
    }
    //split_whitespace()
    //////////////////////////////////////////////
    let texto = "150";

    let numero: i32 = texto.parse().unwrap();

    println!("Resultado: {}", numero + 50);
    
    /////////////////////////////////////////////

    let nombre = "  cristian  ";

    let mensaje = procesar_nombre(nombre);

    println!("{}", mensaje);
}

fn procesar_nombre(nombre: &str) -> String {
    let limpio = nombre.trim();
    let mayusculas = limpio.to_uppercase();

    format!("Bienvenido, {}!", mayusculas)
}
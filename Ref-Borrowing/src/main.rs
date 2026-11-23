fn main() {
    let usuario = String::from("Carlos");

    mostrar_nombre(&usuario);

    println!("Usuario original: {}", usuario);
    /////////////////////////////////////////////
    let mensaje = String::from("Hola Rust");

    let longitud = calcular_longitud(&mensaje);

    println!("Longitud: {}", longitud);
    println!("Mensaje: {}", mensaje);

    /////////////////////////////////////////
    let nombre = String::from("Rust");

    let referencia1 = &nombre;
    let referencia2 = &nombre;

    println!("{} - {}", referencia1, referencia2);
    //////////////////////////////////////////////
    let mut saludo = String::from("Hola");

    agregar_texto(&mut saludo);

    println!("{}", saludo);

    ////////////////////////////////////////
    let mut texto = String::from("Rust");

    // let r1 = &texto;
    // let r2 = &mut texto;

    // println!("{}", r1);
    // println!("{}", r2);
    let r1 = &texto;
    println!("{}", r1);

    let r2 = &mut texto;
    r2.push_str(" es genial");

    println!("{}", r2);
    ////////////////////////////////

    let nombre = String::from("Ana");

    saludar(&nombre);
    saludar("Carlos");

}

fn mostrar_nombre(nombre: &String) {
    println!("Nombre: {}", nombre);
}

fn calcular_longitud(texto: &String) -> usize {
    texto.len()
}

fn agregar_texto(mensaje: &mut String) {
    mensaje.push_str(" Mundo");
}

fn saludar(nombre: &str) {
    println!("Hola, {}!", nombre);
}
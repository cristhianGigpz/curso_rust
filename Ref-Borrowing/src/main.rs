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

    //////////////Desreferenciación//////////////////
    let numero = 10;
    let referencia = &numero;

    println!("Referencia: {}", referencia);
    println!("Valor: {}", *referencia);
    //////////////////////////////////////////////////

    let mut contador = 5;

    incrementar(&mut contador);

    println!("Contador: {}", contador);
    ////////////////////////////////////////////////
    let mensaje = crear_mensaje();

    println!("{}", mensaje);

    ///////////////////////////////////////////
    let mut stock = 20;

    mostrar_stock(&stock);

    vender_producto(&mut stock, 5);

    mostrar_stock(&stock);

    reponer_stock(&mut stock, 10);
    mostrar_stock(&stock);

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

/////////////////////////////////////////////////////

fn incrementar(numero: &mut i32) {
    *numero += 1;
}

//Dangling References (Referencias colgantes)//
// fn crear_referencia() -> &String {
//     let mensaje = String::from("Hola");
//     &mensaje
// }

fn crear_mensaje() -> String {
    String::from("Hola")
}

fn mostrar_stock(stock: &i32) {
    println!("Stock actual: {}", stock);
}

fn vender_producto(stock: &mut i32, cantidad: i32) {
    if cantidad > 0 && cantidad <= *stock {
        *stock -= cantidad;
        println!("Venta realizada: {} unidades", cantidad);
    } else {
        println!("Cantidad inválida o stock insuficiente");
    }
}

fn reponer_stock(stock: &mut i32, cantidad: i32) {
    if cantidad > 0 {
        *stock += cantidad;
        println!("Stock repuesto: {} unidades", cantidad);
    } else {
        println!("Cantidad inválida para reponer stock");
    }
}
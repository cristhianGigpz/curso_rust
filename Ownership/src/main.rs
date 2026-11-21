fn main() {
    //////////////AMBITO - PROPIEDAD///////////////////////
    {
        let mensaje = String::from("Hola Rust");
        println!("{}", mensaje);
    }

    println!("El mensaje ya no es accesible aquí");
    
    ///////////////////MOVE//////////////////////////////
    let nombre1 = String::from("Rust");
    let nombre2 = nombre1;

    println!("{}", nombre2);

    //println!("{}", nombre1); // Error

    ////////////////////COPY///////////////////////////
    let numero1 = 10;
    let numero2 = numero1;

    println!("Número 1: {}", numero1);
    println!("Número 2: {}", numero2);
    
    //////////////////CLONE////////////////////////////
    let nombre1 = String::from("Rust");
    let nombre2 = nombre1.clone();

    println!("Original: {}", nombre1);
    println!("Copia: {}", nombre2);

    println!("-------------------");
    
    //////////////TRNASFERENCIA CON STRING/////////////////////
    let usuario = String::from("Carlos");
    mostrar_nombre(usuario);
    //println!("{}", usuario); // Error
    
    //////////////FUNCION CON COPY I32////////////////////////
    let edad = 25;

    mostrar_edad(edad);

    println!("Edad original: {}", edad);
    ///////////////////////////////////////////////////////

    let texto = crear_mensaje();

    println!("{}", texto);
    //////////////////////////////////////////////////////

    let mensaje = String::from("Rust");

    let mensaje = devolver_texto(mensaje);

    println!("Recuperado: {}", mensaje);
    ////////////////////////////////////////

    let producto = String::from("Laptop");

    let respaldo = producto.clone();

    let producto = registrar_producto(producto);

    println!("Producto registrado: {}", producto);
    println!("Respaldo: {}", respaldo);

    
}

fn mostrar_nombre(nombre: String) {
    println!("Nombre: {}", nombre);
}

fn mostrar_edad(edad: i32) {
    println!("Edad: {}", edad);
}

fn crear_mensaje() -> String {
    let mensaje = String::from("Hola Mundo");
    mensaje
}

fn devolver_texto(texto: String) -> String {
    println!("Procesando: {}", texto);
    texto
}

fn registrar_producto(nombre: String) -> String {
    println!("Registrando producto: {}", nombre);
    nombre
}


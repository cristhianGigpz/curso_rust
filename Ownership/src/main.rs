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
}

const MAX_USUARIOS: u32 = 100;
fn main() {
    let nombre = "Cristhian";
    println!("Nombre: {}", nombre);
    //nombre = "Juan"; // This line will cause a compile-time error because `nombre` is immutable
    let mut contador = 0;
    println!("Contador: {}", contador);
    contador = 5;
    println!("Contador: {}", contador);
    println!("Máximo de usuarios: {}", MAX_USUARIOS);

    let numero = 5;
    let numero = numero + 10;
    let numero = numero * 2;

    println!("Resultado: {}", numero);

    let dato = "123";
    let dato = dato.len();

    println!("Longitud: {}", dato);
}

fn main() {
    // Tupla: colección de datos de diferentes tipos
    // let usuario: (&str, u8, bool) = ("Ana", 25, true);

    // println!("Nombre: {}", usuario.0);
    // println!("Edad: {}", usuario.1);
    // println!("Activo: {}", usuario.2);

    // let producto = ("Laptop", 2500.50, 10);

    // let (nombre, precio, stock) = producto;

    // println!("Producto: {}", nombre);
    // println!("Precio: S/ {}", precio);
    // println!("Stock: {}", stock);

    // //Array: colección de datos del mismo tipo
    // let mut notas: [i32; 4] = [15, 18, 12, 20];

    // println!("Primera nota: {}", notas[0]);
    // println!("Cantidad: {}", notas.len());

    // notas[2] = 16;

    // println!("Notas actualizadas: {:?}", notas);

    // let numeros = [0; 5];
    // println!("Arreglo de ceros: {:?}", numeros);

    //Conversión de tipos
    // let numero: i32 = 10;
    // let decimal: f64 = numero as f64;

    // let precio: f64 = 19.99;
    // let entero: i32 = precio as i32;

    // println!("Decimal: {:.1}", decimal);
    // println!("Entero: {}", entero);

    let nombre = "Carlos";
    let edad: u8 = 20;
    let mut promedio: f64 = 0.0;
    let aprobado: bool = true;
    let seccion: char = 'A';

    let notas: [i32; 4] = [16, 18, 17, 19];
    promedio = (notas[0] + notas[1] + notas[2] + notas[3]) as f64 / notas.len() as f64;

    let estudiante = (nombre, edad, promedio);

    println!("=== Registro de estudiante ===");
    println!("Nombre: {}", estudiante.0);
    println!("Edad: {}", estudiante.1);
    println!("Promedio: {}", estudiante.2);
    println!("Aprobado: {}", aprobado);
    println!("Sección: {}", seccion);
    println!("Notas: {:?}", notas);
}
fn saludar(nombre: &str, edad: u8) {
    println!("¡Hola, {}! Tienes {} años.", nombre, edad);
}

fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn verificar_edad(edad: u8) -> bool {
    if edad >= 18 {
        return true;
    }

    false
}

fn calcular(a: i32, b: i32) -> (i32, i32) {
    let suma = a + b;
    let producto = a * b;

    (suma, producto)
}


fn main() {
    saludar("Alice", 25);
    saludar("Bob", 30);
    
    // let resultado = sumar(5, 3);
    // println!("El resultado de la suma es: {}", resultado);
    
    // let es_adulto = verificar_edad(20);
    // println!("¿Es mayor de edad? {}", es_adulto);

    // let numero = 10;

    // let resultado = {
    //     let doble = numero * 2;
    //     doble + 5
    // };

    // println!("Resultado: {}", resultado);

    //  let (suma, producto) = calcular(5, 3);

    // println!("Suma: {}", suma);
    // println!("Producto: {}", producto);

    let precio = 200.0;
    let descuento = 15.0;

    let (total, desc) = calcular_descuento(precio, descuento);

    println!("Precio original: S/ {:.2}", precio);
    println!("Descuento: {}%", descuento);
    println!("Monto descontado: S/ {:.2}", desc);
    println!("Precio final: S/ {:.2}", total);
}

fn calcular_descuento(precio: f64, descuento: f64) -> (f64, f64) {
    let desc = (precio * descuento / 100.0);
    (precio - desc, desc)
    
}




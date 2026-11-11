
fn main() {
    ///Tamaño		Con signo	Sin signo
    // 8 bits		i8		u8
    // 16 bits		i16		u16
    // 32 bits		i32		u32
    // 64 bits		i64		u64
    // 128 bits	    i128	u128    
    
    let edad: u8 = 25;
    let temperatura: i32 = -10;
    let poblacion: u64 = 34_000_000;

    println!("Edad: {}", edad);
    println!("Temperatura: {}", temperatura);
    println!("Población: {}", poblacion);

    let precio: f64 = 19.99;
    let peso: f32 = 75.5;

    let cantidad = 3.0;
    let total = precio * cantidad;

    println!("Peso: {} kg", peso);
    println!("Total: S/ {:.2}", total);

    let pi_32: f32 = 3.141592653589793; 
    // Rust lo aproximará y guardará solo: 3.1415927 (7 dígitos útiles)

    let pi_64: f64 = 3.141592653589793; 
    // Rust guardará exactamente: 3.141592653589793 (16 dígitos útiles)
    println!("Valor de pi (f32): {}", pi_32);
    println!("Valor de pi (f64): {}", pi_64);

    let activo: bool = true;
    let es_admin = false;
    let mayor_edad = 25 <= 18;

    println!("Usuario activo: {}", activo);
    println!("Es administrador: {}", es_admin);
    println!("Mayor de edad: {}", mayor_edad);

    //4 bytes = 32 bits
    let letra: char = 'A';
    let simbolo = '@';
    let emoji = '🚀';

    println!("Letra: {}", letra);
    println!("Símbolo: {}", simbolo);
    println!("Emoji: {}", emoji);
}

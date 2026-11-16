fn main() {
    let a: u8 = 5; // 00000101
    let b: u8 = 3; // 00000011
    //128 | 64 | 32 | 16 | 8 | 4 | 2 | 1

    //  00000101  (5)
    //& 00000011  (3)
    //  00000001  (1)
    println!("AND: {}", a & b);

    //  00000101  (5)
    //| 00000011  (3)
    //  00000111  (7)
    println!("OR: {}", a | b);

    //  00000101  (5)
    //^ 00000011  (3)
    //  00000110  (6)
    println!("XOR: {}", a ^ b);

    //00000101
    //00001010
    println!("Izquierda: {}", a << 1);  

    //00000101
    //00000010
    println!("Derecha: {}", a >> 1);

    //  00000101  (5)
    //  11111010  (~5)
    println!("NOT: {}", !a);

    const LEER: u8     = 0b00000001; // 1
    const ESCRIBIR: u8 = 0b00000010; // 2
    const BORRAR: u8   = 0b00000100; // 4

    // 1. Asignar varios permisos a la vez usando OR (|)
    let mut permisos_usuario = LEER | ESCRIBIR; // 00000011 (3)

    // 2. Verificar si tiene un permiso usando AND (&)
    let puede_leer = (permisos_usuario & LEER) != 0;     // true
    let puede_borrar = (permisos_usuario & BORRAR) != 0; // false

    // 3. Alternar (activar/desactivar) un permiso usando XOR (^)
    permisos_usuario ^= ESCRIBIR; // Ahora vale 00000001 (solo LEER)

    println!("¿Puede leer?: {}, ¿Puede borrar?: {}", puede_leer, puede_borrar);
      
}

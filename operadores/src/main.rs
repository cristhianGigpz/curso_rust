fn main() {
    // Operadores aritméticos
    let a = 13;
    let b = 3;

    println!("Suma: {}", a + b);
    println!("Resta: {}", a - b);
    println!("Multiplicación: {}", a * b);
    println!("División: {}", a / b);
    println!("Residuo: {}", a % b);

    // Operadores de comparación
    // ==		Igual a		5 == 5 → true
    // !=		Diferente de	5 != 3 → true
    // >		Mayor que	10 > 5 → true
    // <		Menor que	3 < 5 → true
    // >=		Mayor o igual	5 >= 5 → true
    // <=		Menor o igual	3 <= 5 → true
    // let edad = 20;
    // let minimo = 18;

    // println!("¿Mayor de edad?: {}", edad >= minimo);
    // println!("¿Menor de edad?: {}", edad < minimo);
    // println!("¿Tiene 18 años?: {}", edad == minimo);

    // Operadores lógicos
    // &&		Y lógico		true && false → false
    // ||		O lógico		true || false → true
    // !		Negación lógica	!true → false

    let edad = 25;
    let tiene_licencia = true;

    let puede_conducir = edad >= 18 && tiene_licencia;
    let acceso_especial = edad >= 65 || tiene_licencia;
    let sin_licencia = !tiene_licencia;

    println!("Puede conducir: {}", puede_conducir);
    println!("Acceso especial: {}", acceso_especial);
    println!("Sin licencia: {}", sin_licencia);

    // Operadores de asignación
    // =		Asignación		x = 10
    // +=		Suma y asignación	x = x + 5
    // -=		Resta y asignación	x = x - 5
    // *=		Multiplica y asigna	x = x * 5
    // /=		Divide y asigna	x = x / 5
    // %=		Residuo y asignación	x = x % 5

    let mut contador = 10;

    contador += 1;
    println!("Suma: {}", contador);

    contador -= 3;
    println!("Resta: {}", contador);

    contador *= 2;
    println!("Multiplicación: {}", contador);

    contador /= 4;
    println!("División: {}", contador);

    //ejercicio////////////////////////////////////////////
    let precio = 50.0;
    let cantidad = 5.0;
    let cliente_vip = true;

    let subtotal = precio * cantidad;

    let tiene_descuento = subtotal >= 200.0 && cliente_vip;

    let descuento = if tiene_descuento {
        subtotal * 0.10
    } else {
        0.0
    };

    let total = subtotal - descuento;

    println!("Subtotal: S/ {:.2}", subtotal);
    println!("Descuento: S/ {:.2}", descuento);
    println!("Total: S/ {:.2}", total);
    //Reto: Modifica el programa para que los clientes VIP reciban un 15 % de descuento 
    //cuando su compra supere los S/ 300.
}

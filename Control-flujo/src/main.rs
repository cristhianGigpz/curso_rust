fn main() {
    // let edad = 65;

    // if edad < 18 {
    //     println!("Menor de edad");
    // } else if edad >= 65 {
    //     println!("Adulto mayor");
    // } else {
    //     println!("Adulto");
    // }

    let edad = 17;

    let estado = if edad >= 18 {
        "Mayor de edad"
    } else {
        "Menor de edad"
    };

    println!("Estado: {}", estado);

    
    let numero = 0;

    match numero {
        1 => println!("Uno"),
        2 => println!("Dos"),
        3 => println!("Tres"),
        _ => println!("Otro número"),
    }
}

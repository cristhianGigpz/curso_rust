struct Usuario {
    nombre: String,
    edad: u8,
    activo: bool,
}

struct Producto {
    nombre: String,
    precio: f64,
    stock: u32,
}


fn main() {
    let usuario = Usuario {
        nombre: String::from("Carlos"),
        edad: 25,
        activo: true,
    };

    println!("Nombre: {}", usuario.nombre);
    println!("Edad: {}", usuario.edad);
    println!("Activo: {}", usuario.activo);

    let mut producto = Producto {
        nombre: String::from("Laptop"),
        precio: 2500.0,
        stock: 10,
    };

    producto.precio = 2300.0;
    producto.stock -= 2;

    println!("Producto: {}", producto.nombre);
    println!("Precio: {}", producto.precio);
    println!("Stock: {}", producto.stock);

    let usuario = crear_usuario(String::from("Ana"), 25);

    println!("{} - {}", usuario.nombre, usuario.edad);
}

fn crear_usuario(nombre: String, edad: u8) -> Usuario {
    Usuario {
        nombre,
        edad,
        activo: true,
    }
}

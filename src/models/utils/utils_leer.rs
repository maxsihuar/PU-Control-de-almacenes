use crate::models::utils::utils_validaciones;
use std::io;
pub fn pedir_datos_categoria() -> (String, String) {
    let mut codigo = String::new();
    let mut nombre = String::new();
    println!("Ingrese el codigo de la categoria: ");
    std::io::stdin().read_line(&mut codigo).expect("Error al leer el codigo");
    let codigo = codigo.trim().to_string();
    println!("Ingrese el nombre de la categoria: ");
    std::io::stdin().read_line(&mut nombre).expect("Error al leer el nombre");
    let nombre = nombre.trim().to_string();
    return (codigo, nombre);
}

pub fn pedir_datos_articulo() -> (String, String, String) {
    let mut codigo = String::new();
    let mut nombre = String::new();
    let mut categoria = String::new();

    println!("Ingrese el codigo del articulo: ");
    std::io::stdin().read_line(&mut codigo).expect("Error al leer el codigo");
    let codigo = codigo.trim().to_string();

    println!("Ingrese el nombre del articulo: ");
    std::io::stdin().read_line(&mut nombre).expect("Error al leer el nombre");
    let nombre = nombre.trim().to_string();

    println!("Ingrese la categoria del articulo: ");
    std::io::stdin().read_line(&mut categoria).expect("Error al leer la categoria");
    let categoria = categoria.trim().to_string();

    return (codigo, nombre, categoria);
}
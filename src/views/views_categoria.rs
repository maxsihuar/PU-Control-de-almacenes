use std::collections::HashMap;

fn menu(){
    println!("--- Menu de Categorias ---");
    println!("1. Mostrar Categorias");
    println!("2. Agregar Categoria");
    println!("3. Eliminar Categoria");
    println!("4. Salir");
}

fn mostrar_categorias(dc : HashMap<String, String>) {
    for (clave, valor) in dc.iter() {
        println!("Clave: {}, Valor: {}", clave, valor);
    }
}
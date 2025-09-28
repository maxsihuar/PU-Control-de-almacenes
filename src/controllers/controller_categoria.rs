use models::obj::Categorias;
use views::views_categoria::mostrar_categorias;
use views::views_categoria::menu as Menu;
use std::collections::HashMap;
use std::io;
fn controller_categoria() {
    Menu();
    match get_input().trim() {
        "1" => mostrar_categorias(),
        "2" => agregar_categoria(),
        "3" => eliminar_categoria(),
        "4" => println!("Saliendo..."),
        _ => println!("Opcion invalida, intente de nuevo."),
    }
}


fn agregar_categoria() {
    let mut categorias = Categorias();
    categorias.insert("NT".to_string(), "Nutricion".to_string());
    return categorias;
}
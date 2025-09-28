use crate::views::view_menu;

pub fn principal(){
    view_menu::menu_principal();

    match 1 {
        1 => println!("Categorias"),
        2 => println!("Productos"),
        3 => println!("Salir"),
        _ => println!("Opcion no valida"),
    }
}
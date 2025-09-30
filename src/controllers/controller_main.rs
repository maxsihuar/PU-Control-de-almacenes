use crate::views::view_menu;
use crate::models::obj;

pub fn run(){

    let mut dc_categorias = obj::obj_categoria::crear_categorias();
    let mut dc_articulos = obj::obj_articulo::crear_articulos();

    loop{
        let opcion = 1;

        match opcion{
            1 => controller_categorias::run(&dc_categorias),
            2 => break,
            _ => println!("Opcion no valida") // view_error_opcion();
        }
    }

    view_menu::menu_principal();
}
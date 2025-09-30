use crate::views::view_menu;
use crate::models::services;
use std::collections::HashMap;

pub fn run_articulo(dc : &HashMap<String, (String, String, u32)>) {
    loop{
        view::menu_secundario("Articulos".to_string());
        match opcion{
            1 => services::service_agregar::agregar_articulo(),
                //util::utils_data::pedir_datos_categoria(),
                //dc
            2 => services::service_modificar::modificar_categoria(),
                //util::utils_data::pedir_datos_categoria(),
                //dc
            3 => services::service_eliminar::eliminar_categoria(),
                //util::utils_data::pedir_datos_categoria(), 
                //dc
            4 => services::service_listar::listar_categorias(),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}
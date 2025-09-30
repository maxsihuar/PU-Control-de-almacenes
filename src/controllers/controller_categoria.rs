use crate::views::view_menu;
use crate::models::services;

use std::collections::HashMap;

pub fn run_categoria(dc : &HashMap<String, String>) {
    loop{
        view::menu_secundario("Categorias".to_string());
        match opcion{
            1 => services::service_agregar::agregar_categoria(
                //util::utils_data::pedir_datos_categoria(),
                //dc
            ),
        }
    }
}

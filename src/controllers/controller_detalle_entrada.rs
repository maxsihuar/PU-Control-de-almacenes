use crate::views::view_menu;
use crate::models::services;
use std::collections::HashMap;

pub fn run_detalle_entrada(dc : &HashMap<String, HashMap<String, (u32,u32)>>) {
    loop{
        view::menu_secundario("Detalles de Entrada".to_string());
        match opcion{
            1 => services::service_agregar::agregar(),
                //util::utils_data::pedir_datos_categoria(),
                //dc
            2 => services::service_modificar::modificar(),
                //util::utils_data::pedir_datos_categoria(),
                //dc
            3 => services::service_eliminar::eliminar(),
                //util::utils_data::pedir_datos_categoria(), 
                //dc
            4 => services::service_listar::listar(),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}

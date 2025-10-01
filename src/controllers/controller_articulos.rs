use crate::views::view_menu;
use crate::models::services;
use std::collections::HashMap;
pub fn leer_datos_articulo() -> (String, (String, String, u32)) {
    // Lógica para leer datos del artículo
    let codigo = utils::utils_leer::leer_string();
    let descripcion = utils::utils_leer::leer_string();
    let precio = utils::utils_leer::leer_u32();

<<<<<<< HEAD

    return (codigo, (descripcion, categoria, precio));

}
pub fn run_articulo(dc : &HashMap<String, Vec<String,String,u32>>) {
    loop{
        view::menu_secundario("Articulos".to_string());
        match opcion{

            1 => services::service_agregar(dc, leer_datos_articulo()),

            1 => services::service_agregar::agregar(),
                //util::utils_data::pedir_datos_articulo(),
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
use crate::views::view_menu;
use crate::models::services;
use std::collections::HashMap;
/// Lee desde la entrada estándar los datos de un artículo.
///
/// Retorna una tupla con la siguiente estructura:
/// - `String`: código del artículo
/// - `(String, String, u32)`: una tupla interna con:
///   - descripción del artículo (`String`)
///   - categoría del artículo (`String`)
///   - precio del artículo (`u32`)
pub fn leer_datos_articulo() -> (String, (String, String, u32)) {
    // Lógica para leer datos del artículo
    let codigo = utils::utils_leer::leer_string();
    let descripcion = utils::utils_leer::leer_string();
    let precio = utils::utils_leer::leer_u32();
    return (codigo, (descripcion, categoria, precio));

}


pub fn run_articulo(dc : &HashMap<String, (String, String, u32)>) {

    loop{
        view::menu_secundario("Articulos".to_string());
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::service_agregar(dc, leer_datos_articulo()),
            2 => services::service_modificar::modificar(dc,leer_datos_articulo()),
            3 => services::service_eliminar::eliminar(dc,utils::utils_leer::leer_string()),
            4 => services::service_listar::listar(dc),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}
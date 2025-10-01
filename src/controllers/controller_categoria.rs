use crate::views::view_menu;
use crate::view::view_leer;
use crate::models::services;
use std::collections::HashMap;
/// Lee desde la entrada estándar los datos de una categoría.
///
/// Retorna una tupla con:
/// - `String`: código de la categoría.
/// - `String`: descripción de la categoría.
pub fn leer_datos_categoria() -> (String, String) {
    // Lógica para leer datos del artículo
    view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA CATEGORIA");
    let codigo = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la descripcion de la categoria:");
    let descripcion = utils::utils_leer::leer_string();
    return (codigo, descripcion);
}
/// Ejecuta el menú secundario para la gestión de categorías.
///
/// Permite al usuario realizar operaciones básicas de un CRUD:
/// 
/// 1. **Agregar** una categoría nueva  
/// 2. **Modificar** una categoría existente  
/// 3. **Eliminar** una categoría por su código  
/// 4. **Listar** todas las categorías registradas  
/// 5. **Salir** del menú  
///
/// # Parámetros
/// - `dc`: referencia mutable a un `HashMap` donde:
///   - La clave (`String`) es el **código de la categoría**.
///   - El valor (`String`) es la **descripción de la categoría**.
pub fn run_categoria(dc : &Hashmap<String, String>) {
    loop{
        view::menu_secundario("Categorias".to_string());
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::service_agregar(dc, leer_datos_categoria()),
            2 => services::service_modificar::modificar(dc,leer_datos_categoria()),
            3 => services::service_eliminar::eliminar(dc,utils::utils_leer::leer_string()),
            4 => services::service_listar::listar(dc),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}
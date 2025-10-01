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
/// Ejecuta el menú secundario para la gestión de artículos.
///
/// Este menú permite al usuario interactuar con el inventario de artículos,
/// brindando las operaciones básicas de un CRUD:
///
/// 1. **Agregar** un nuevo artículo.  
/// 2. **Modificar** un artículo existente.  
/// 3. **Eliminar** un artículo por su código.  
/// 4. **Listar** todos los artículos registrados.  
/// 5. **Salir** del menú.  
///
/// # Parámetros
/// - `dc`: referencia mutable a un `HashMap` donde:
///   - La clave (`String`) es el **código del artículo**.
///   - El valor es una tupla con:
///     - `String`: descripción del artículo.  
///     - `String`: categoría del artículo.  
///     - `u32`: precio del artículo. 
pub fn run_articulo(dc : &mut HashMap<String, (String, String, u32)>) {
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
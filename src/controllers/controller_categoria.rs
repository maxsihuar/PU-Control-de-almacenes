use crate::views::view_menu;
use crate::models::utils;
use crate::views::view_listar;
use crate::views::view_error;
use crate::views::view_leer;
use crate::models::services;
use std::collections::HashMap;
/// Lee desde la entrada estándar los datos de una categoría.
///
/// Retorna una tupla con:
/// - `String`: código de la categoría.
/// - `String`: descripción de la categoría.
pub fn leer_datos_categoria(dc : &mut HashMap<String, String>) -> (String, String) {

    let mut codigo : String;
    loop{
        // Lógica para leer datos del artículo
        view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA CATEGORIA");
        codigo = utils::utils_leer::leer_string();
        if utils::utils_validaciones::validar_existencia_s(codigo.clone(), dc) {
            view_error::error_existencia();
        } else {
            break;
        }
    }

    view_leer::mostrar_mensaje("Ingrese la descripcion de la categoria:");
    let descripcion = utils::utils_leer::leer_string();
    return (codigo, descripcion);
}

pub fn leer_datos_categoria_m(dc : &mut HashMap<String, String>) -> (String, String) {

    let mut codigo : String;
    loop{
        // Lógica para leer datos del artículo
        view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA CATEGORIA");
        codigo = utils::utils_leer::leer_string();
        if !utils::utils_validaciones::validar_existencia_s(codigo.clone(), dc) {
            view_error::error_no_existencia();
        } else {
            break;
        }
    }

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
pub fn run_categoria(dc : &mut HashMap<String, String>) {
    loop{
        view_menu::menu_secundario("Categorias".to_string());
        view_leer::mostrar_mensaje("Ingrese una opcion:");
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::services_agregar::agregar_s(dc, leer_datos_categoria),
            2 => services::services_modificar::modificar_s(dc,leer_datos_categoria_m),
            3 => services::services_eliminar::eliminar_s(dc,utils::utils_leer::leer_string()),
            4 => view_listar::mostrar_listado(services::services_listar::listar_s(dc, vec!["Codigo", "Descripcion"])),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}
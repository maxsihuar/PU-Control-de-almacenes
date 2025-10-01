use crate::views::view_menu;
use crate::views::view_leer;
use crate::views::view_listar;
use crate::views::view_error;
use crate::models::services;
use crate::models::utils;
use std::collections::HashMap;
/// Lee desde la entrada estándar los datos de un artículo.
///
/// Retorna una tupla con la siguiente estructura:
/// - `String`: código del artículo
/// - `(String, String, u32)`: una tupla interna con:
///   - descripción del artículo (`String`)
///   - categoría del artículo (`String`)
///   - precio del artículo (`u32`)
pub fn leer_datos_articulo(dc : &mut HashMap<String, (String, String, u32)>) -> (String, (String, String, u32)) {

    let mut codigo: String;
    loop {
        // Lógica para leer datos del artículo
        view_leer::mostrar_titulo("INGRESE EL CODIGO DEL ARTICULO");
        codigo = utils::utils_leer::leer_string();

        if utils::utils_validaciones::validar_existencia_t1(codigo.clone(), dc) {
            view_error::error_existencia();
        } else {
            break;
        }

    }


    view_leer::mostrar_mensaje("Ingrese la descripcion del articulo:");
    let descripcion = utils::utils_leer::leer_string();

    view_leer::mostrar_mensaje("Ingrese el codigo de categoria del articulo:");
    let categoria = utils::utils_leer::leer_string();

    view_leer::mostrar_mensaje("Ingrese el precio del articulo:");
    let precio = utils::utils_leer::leer_u32();
    
    return (codigo, (descripcion, categoria, precio));

}

pub fn leer_datos_articulo_m(dc : &mut HashMap<String, (String, String, u32)>) -> (String, (String, String, u32)) {

    let mut codigo: String;
    loop {
        // Lógica para leer datos del artículo
        view_leer::mostrar_titulo("INGRESE EL CODIGO DEL ARTICULO");
        codigo = utils::utils_leer::leer_string();

        if !utils::utils_validaciones::validar_existencia_t1(codigo.clone(), dc) {
            view_error::error_no_existencia();
        } else {
            break;
        }

    }


    view_leer::mostrar_mensaje("Ingrese la descripcion del articulo:");
    let descripcion = utils::utils_leer::leer_string();

    view_leer::mostrar_mensaje("Ingrese el codigo de categoria del articulo:");
    let categoria = utils::utils_leer::leer_string();

    view_leer::mostrar_mensaje("Ingrese el precio del articulo:");
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
        view_menu::menu_secundario("Articulos".to_string());
        view_leer::mostrar_mensaje("Ingrese una opcion:");
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::services_agregar::agregar_t1(dc,leer_datos_articulo),
            2 => services::services_modificar::modificar_t1(dc,leer_datos_articulo_m),
            3 => services::services_eliminar::eliminar_t1(dc,utils::utils_leer::leer_string()),
            4 => view_listar::mostrar_listado(services::services_listar::listar_t1(dc, vec!["Codigo", "Descripcion", "Categoria", "Precio"])),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}
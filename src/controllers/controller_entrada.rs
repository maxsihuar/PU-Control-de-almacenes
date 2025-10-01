use crate::views::view_menu;
use crate::views::view_leer;
use crate::views::view_error;
use crate::models::services;
use crate::views::view_listar;
use crate::models::utils;
use std::collections::HashMap;

/// Lee desde la entrada estándar los datos de una **Entrada**.
///
/// Retorna una tupla con:
/// - `String`: Código de la entrada.
/// - `(String, String)`: Datos asociados a la entrada:
///   - Nombre del cliente.
///   - Fecha de la entrada.
///
/// # Flujo de lectura
/// 1. Se solicita el código de la entrada.
/// 2. Se pide el nombre del cliente.
/// 3. Se pide la fecha de la entrada.
/// 4. Se retorna una tupla `(codigo, (cliente, fecha))`.
pub fn leer_datos_entrada(dc : &mut HashMap<String, (String, String)>) -> (String, (String, String)) {

    let mut codigo:String;
    loop{
        view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA ENTRADA");
        codigo = utils::utils_leer::leer_string();

        if utils::utils_validaciones::validar_existencia_t3(codigo.clone(), dc){
            view_error::error_existencia();
        } else {
            break;
        }
    }
    view_leer::mostrar_mensaje("Ingrese el código del proveedor:");
    let proveedor = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la fecha de la entrada:");
    let fecha = utils::utils_leer::leer_string();
    return (codigo,(proveedor, fecha));
}

pub fn leer_datos_entrada_m(dc : &mut HashMap<String, (String, String)>) -> (String, (String, String)) {

    let mut codigo:String;
    loop{
        view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA ENTRADA");
        codigo = utils::utils_leer::leer_string();

        if !utils::utils_validaciones::validar_existencia_t3(codigo.clone(), dc){
            view_error::error_no_existencia();
        } else {
            break;
        }
    }
    view_leer::mostrar_mensaje("Ingrese el código del proveedor:");
    let proveedor = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la fecha de la entrada:");
    let fecha = utils::utils_leer::leer_string();
    return (codigo,(proveedor, fecha));
}

/// Ejecuta el menú de gestión de **Entradas**.
///
/// Este procedimiento muestra un menú secundario en un bucle y 
/// permite al usuario interactuar con las opciones disponibles
/// sobre el registro de entradas.
///
/// Cada entrada está representada en un `HashMap`:
/// - La **clave** es un `String` que corresponde al código de la entrada.
/// - El **valor** es una tupla `(String, String)` donde:
///   - `String` → Proveedor
///   - `String` → Fecha
///
/// # Parámetros
/// - `dc`: Referencia inmutable a un `HashMap<String, (String, String)>`
///   que almacena las entradas.
///
/// # Opciones del menú
/// 1. **Agregar** una nueva entrada (usa `service_agregar`).
/// 2. **Modificar** una entrada existente (usa `service_modificar`).
/// 3. **Eliminar** una entrada por código (usa `service_eliminar`).
/// 4. **Listar** todas las entradas con sus campos:
///    - Código
///    - Proveedor
///    - Fecha
/// 5. **Salir** del menú.
pub fn run_entrada(dc : &mut HashMap<String, (String, String)>) {
    loop{
        view_menu::menu_secundario("Entrada".to_string());
        view_leer::mostrar_mensaje("Ingrese una opcion:");
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::services_agregar::agregar_t3(dc,leer_datos_entrada),
            2 => services::services_modificar::modificar_t3(dc,leer_datos_entrada_m),
            3 => services::services_eliminar::eliminar_t3(dc,utils::utils_leer::leer_string()),
            4 => view_listar::mostrar_listado(services::services_listar::listar_t3(dc, vec!["Codigo", "Proveedor", "Fecha"])),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}

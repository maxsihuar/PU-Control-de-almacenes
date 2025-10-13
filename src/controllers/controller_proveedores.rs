use crate::views::view_menu;
use crate::models::services;
use crate::views::view_leer;
use crate::views::view_error;
use crate::views::view_listar;
use crate::models::utils;
use std::collections::HashMap;
/// Lee los datos de un proveedor desde la entrada estándar.
///
/// Esta función solicita al usuario que ingrese la información necesaria para
/// registrar un proveedor en el sistema. Los datos se devuelven en una tupla
/// que luego puede insertarse en una estructura de almacenamiento como un
/// `HashMap`.
///
/// # Retorno
/// Retorna una tupla con el siguiente formato:
/// ```
/// (codigo, (razon_social, ruc, direccion, ciudad))
/// ```
/// - `codigo`: Identificador único del proveedor.
/// - `razon_social`: Nombre o razón social del proveedor.
/// - `ruc`: Número de RUC del proveedor (11 dígitos).
/// - `direccion`: Dirección del proveedor.
/// - `ciudad`: Ciudad donde se ubica el proveedor.
pub fn leer_datos_proveedores(dc: &mut HashMap<String, (String, String, String, String)>) -> (String, (String, String,String, String)) {
    let mut codigo:String;
    loop {
        view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA SALIDA");
        codigo = utils::utils_leer::leer_string();

        if utils::utils_validaciones::validar_existencia_t2(codigo.clone(), &mut HashMap::new()){
            view_error::error_existencia();
        } else {
            break;
        }
    }

    view_leer::mostrar_mensaje("Ingrese la razón social del proveedor:");
    let rs = utils::utils_leer::leer_string();
    let mut ruc:String;
    loop {
        view_leer::mostrar_mensaje("Ingrese el RUC del proveedor:");
        ruc = utils::utils_leer::leer_string();
        if  utils::utils_validaciones::validar_RUC(ruc.clone(), dc){
            break;
        }else{
             view_error::error_RUC_existencia();
        }

    }
    view_leer::mostrar_mensaje("Ingrese la dirección del proveedor:");
    let direccion = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la ciudad del proveedor:");
    let ciudad = utils::utils_leer::leer_string();
    return (codigo,(rs, ruc, direccion, ciudad));
}

pub fn leer_datos_proveedores_m(dc: &mut HashMap<String, (String, String, String, String)>) -> (String, (String, String,String, String)) {
    let mut codigo:String;
    loop {
        view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA SALIDA");
        codigo = utils::utils_leer::leer_string();

        if !utils::utils_validaciones::validar_existencia_t2(codigo.clone(), &mut HashMap::new()){
            view_error::error_no_existencia();
        } else {
            break;
        }
    }

    view_leer::mostrar_mensaje("Ingrese la razón social del proveedor:");
    let rs = utils::utils_leer::leer_string();
    let mut ruc:String;
    loop {
        view_leer::mostrar_mensaje("Ingrese el RUC del proveedor:");
        ruc = utils::utils_leer::leer_string();
        if  utils::utils_validaciones::validar_RUC(ruc.clone(), dc){
            break;
        }else{
             view_error::error_RUC_existencia();
        }

    }

    view_leer::mostrar_mensaje("Ingrese la dirección del proveedor:");
    let direccion = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la ciudad del proveedor:");
    let ciudad = utils::utils_leer::leer_string();
    return (codigo,(rs, ruc, direccion, ciudad));
}

/// Controlador principal para la gestión de proveedores.
///
/// Muestra un menú secundario con las operaciones CRUD disponibles:
/// 1. Agregar proveedor  
/// 2. Modificar proveedor  
/// 3. Eliminar proveedor  
/// 4. Listar proveedores  
/// 5. Salir del menú  
///
/// # Parámetros
/// - `dc`: Referencia mutable a un `HashMap` que almacena los proveedores,
///   donde:
///   - `String`: código del proveedor.
///   - `(String, String, String, String)`: razón social, RUC, dirección y ciudad.
///
pub fn run_proveedores(dc: &mut HashMap<String, (String, String, String, String)>) {
    loop {
        view_menu::menu_secundario("Proveedores".to_string());
        view_leer::mostrar_mensaje("Ingrese una opcion:");
        let opcion = utils::utils_leer::leer_u32();

        match opcion {
            1 => services::services_agregar::agregar_t2(dc, leer_datos_proveedores),
            2 => services::services_modificar::modificar_t2(dc, leer_datos_proveedores_m),
            3 => services::services_eliminar::eliminar_t2(dc, utils::utils_leer::leer_string()),
            4 => view_listar::mostrar_listado(services::services_listar::listar_t2(dc,vec!["Codigo", "Razon Social", "RUC", "Direccion", "Ciudad"])),
            5 => break,
            _ => println!("Opcion no valida"),
        }
    }
}

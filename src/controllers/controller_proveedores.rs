use crate::views::view_menu;
use crate::models::services;
use crate::view::view_leer;
use std::collections::HashMap;
pub fn leer_datos_proveedores() -> (String, (String, String,String, String)) {
    // Lógica para leer datos del artículo
    view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA SALIDA");
    let codigo = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la razón social del proveedor:");
    let rs = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese el RUC del proveedor:");
    let ruc = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la dirección del proveedor:");
    let direccion = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la ciudad del proveedor:");
    let ciudad = utils::utils_leer::leer_string();
    return (codigo,(rs, ruc, direccion, ciudad));
}
pub fn run_proveedores(dc : &HashMap<String, (String, String, String, String)>) {
    loop{
        view::menu_secundario("Proveedores".to_string());
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
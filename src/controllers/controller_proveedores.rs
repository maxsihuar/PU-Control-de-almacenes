use crate::views::view_menu;
use crate::models::services;
use crate::view::view_leer;
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
use std::collections::HashMap;

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
/// # Ejemplo
/// ```ignore
/// let mut proveedores: HashMap<String, (String, String, String, String)> = HashMap::new();
/// run_proveedores(&mut proveedores);
/// ```
pub fn run_proveedores(dc: &mut HashMap<String, (String, String, String, String)>) {
    loop {
        view::menu_secundario("Proveedores".to_string());
        let opcion = utils::utils_leer::leer_u32();

        match opcion {
            1 => services::service_agregar::agregar(dc, leer_datos_proveedores()),
            2 => services::service_modificar::modificar(dc, leer_datos_proveedores()),
            3 => services::service_eliminar::eliminar(dc, utils::utils_leer::leer_string()),
            4 => services::service_listar::listar(dc,vec!["Codigo", "Razon Social", "RUC", "Direccion", "Ciudad"]),
            5 => break,
            _ => println!("Opcion no valida"),
        }
    }
}

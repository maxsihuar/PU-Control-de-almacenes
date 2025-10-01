use crate::views::view_menu;
use crate::view::view_leer;
use crate::models::services;
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
pub fn leer_datos_entrada() -> (String, (String, String)) {
    // Lógica para leer datos del artículo
    view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA ENTRADA");
    let codigo = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese el código del proveedor:");
    let proveedor = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la fecha de la entrada:");
    let fecha = utils::utils_leer::leer_string();
    return (codigo,(cliente, fechas));
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
pub fn run_entrada(dc : &HashMap<String, (String, String)>) {
    loop{
        view::menu_secundario("Entrada".to_string());
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::service_agregar(dc,leer_datos_entrada()),
            2 => services::service_modificar::modificar(dc,leer_datos_entrada()),
            3 => services::service_eliminar::eliminar(dc,utils::utils_leer::leer_string()),
            4 => services::service_listar::listar(dc, vec!["Codigo", "Proveedor", "Fecha"]),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}

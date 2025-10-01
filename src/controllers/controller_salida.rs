use crate::views::view_menu;
use crate::view::view_leer;
use crate::models::services;
use std::collections::HashMap;
/// Solicita y devuelve los datos necesarios para registrar una **salida**.
///
/// Este procedimiento interactúa con el usuario para capturar la información
/// de una salida específica. Los datos recolectados se empaquetan en una tupla
/// lista para ser insertada en la estructura de almacenamiento.
///
/// # Retorno
/// Devuelve una tupla `(String, (String, String))` donde:
/// - El primer `String` es el **código de la salida**.
/// - La tupla `(String, String)` contiene:
///   - El **DNI del cliente**.
///   - La **fecha de la salida**.
///
/// # Flujo de interacción
/// 1. Muestra un título solicitando el código de la salida.
/// 2. Solicita al usuario el DNI del cliente.
/// 3. Solicita la fecha de la salida.
/// 4. Retorna los datos empaquetados.
pub fn leer_datos_salida() -> (String, (String, String)) {
    // Lógica para leer datos del artículo
    view_leer::mostrar_titulo("INGRESE EL CODIGO DE LA SALIDA");
    let codigo = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese el DNI del cliente:");
    let dni = utils::utils_leer::leer_string();
    view_leer::mostrar_mensaje("Ingrese la fecha de la salida:");
    let fecha = utils::utils_leer::leer_string();
    return (codigo,(dni, fechas));
}
/// Ejecuta el menú principal de gestión de **salidas**.
///
/// Este bucle interactivo permite al usuario gestionar los registros de salidas
/// en la aplicación. Recibe una referencia mutable al diccionario (`HashMap`) 
/// donde se almacenan los datos.
///
/// # Parámetros
/// - `dc`: Referencia mutable a un `HashMap<String, (String, String)>` que representa:
///   - `String`: Código de la salida.
///   - `(String, String)`: Tupla con el **DNI del cliente** y la **fecha de la salida**.
///
/// # Opciones del menú
/// 1. **Agregar**: Inserta una nueva salida solicitando datos al usuario.
/// 2. **Modificar**: Permite editar una salida existente.
/// 3. **Eliminar**: Elimina una salida a partir de su código.
/// 4. **Listar**: Muestra todas las salidas con columnas `["Codigo", "Cliente", "Fecha"]`.
/// 5. **Salir**: Finaliza la ejecución del menú.
pub fn run_salida(dc : &mut HashMap<String, (String, String)>) {
    loop{
        view::menu_secundario("Entrada".to_string());
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::service_agregar(dc,leer_datos_salida()),
            2 => services::service_modificar::modificar(dc,leer_datos_salida()),
            3 => services::service_eliminar::eliminar(dc,utils::utils_leer::leer_string()),
            4 => services::service_listar::listar(dc, vec!["Codigo", "Cliente", "Fecha"]),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}

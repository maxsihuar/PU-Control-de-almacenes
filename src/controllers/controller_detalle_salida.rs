use crate::views::view_menu;
use crate::view::view_leer;
use crate::models::services;
use std::collections::HashMap;
/// Lee desde la entrada estándar los datos de un **Detalle de Salida**.
///
/// Retorna:
/// - `String`: Código del detalle de salida.
/// - `HashMap<String, (u32, u32)>`: Diccionario de artículos incluidos en la salida.
///   - **Clave (`String`)**: Código del artículo.
///   - **Valor (`(u32, u32)`)**: `(cantidad, precio)`
///
/// # Flujo de lectura
/// 1. Se pide el código del detalle de salida.
/// 2. Se pregunta cuántos artículos se van a registrar.
/// 3. Para cada artículo:
///    - Se solicita el código.
///    - Se solicita la cantidad.
///    - Se solicita el precio.
/// 4. Se construye un `HashMap` con todos los artículos registrados.
/// 5. Se retorna la tupla `(codigo, diccionario_articulos)`.
pub fn leer_datos_detalle_salida() -> (String, HashMap<String, (u32,u32)>) {
    // Lógica para leer datos del artículo
    view_leer::mostrar_titulo("INGRESE EL CODIGO DEL DETALLE DE LA SALIDA");
    let codigo = utils::utils_leer::leer_string();
    let dictemp : HashMap<String, (u32,u32)> = HashMap::new();
    view_leer::mostrar_mensaje("Ingrese la cantidad de articulos a ingresar:");
    let mut n : u32 = utils::utils_leer::leer_u32();
    for _i in 0..n {
        view_leer::mostrar_mensaje("Ingrese el codigo del articulo:");
        let codigo_articulo = utils::utils_leer::leer_string();
        view_leer::mostrar_mensaje("Ingrese la cantidad del articulo:");
        let cantidad = utils::utils_leer::leer_u32();
        view_leer::mostrar_mensaje("Ingrese el precio del articulo:");
        let precio = utils::utils_leer::leer_u32();
        dictemp.insert(codigo_articulo, (cantidad,precio));
    }
    return (codigo, dictemp);
}
/// Ejecuta el submenú de **Gestión de Detalles de Salida**.
///
/// Permite al usuario realizar operaciones CRUD sobre la estructura de datos
/// `HashMap<String, HashMap<String, (u32, u32)>>`, donde:
/// - **Clave externa (`String`)**: Código del detalle de salida.
/// - **Valor (`HashMap<String, (u32, u32)`)**: Diccionario de artículos asociados a ese detalle.
///   - **Clave interna (`String`)**: Código del artículo.
///   - **Valor interno (`(u32, u32)`)**: `(cantidad, precio)`
///
/// # Opciones del menú
/// 1. **Agregar**: Solicita datos con [`leer_datos_detalle_salida`] y los inserta en la colección.
/// 2. **Modificar**: Permite actualizar un detalle existente usando [`leer_datos_detalle_salida`].
/// 3. **Eliminar**: Borra un detalle a partir de su código.
/// 4. **Listar**: Muestra todos los detalles de salida con sus artículos.
/// 5. **Salir**: Cierra el submenú y retorna el control al menú principal.
///
/// # Parámetros
/// - `dc`: Referencia mutable al `HashMap` que contiene los detalles de salida.
pub fn run_detalle_salida(dc: &mut HashMap<String, HashMap<String, (u32, u32)>>){
    loop{
        view::menu_secundario("Detalles de Salida".to_string());
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::service_agregar(dc, leer_datos_detalle_salida()),
            2 => services::service_modificar::modificar(dc,leer_datos_detalle_salida()),
            3 => services::service_eliminar::eliminar(dc,utils::utils_leer::leer_string()),
            4 => services::service_listar::listar(dc, vec!["Codigo", "Codigo Articulo", "Cantidad", "Precio"]),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}


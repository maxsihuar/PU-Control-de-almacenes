use crate::views::view_menu;
use crate::view::view_leer;
use crate::models::services;
use std::collections::HashMap;
/// Lee desde la entrada estándar los datos de un **detalle de entrada**.
///
/// Retorna una tupla con:
/// - `String`: código del detalle de entrada.
/// - `HashMap<String, (u32, u32)>`: diccionario temporal donde:
///   - La clave (`String`) es el código del artículo.
///   - El valor es una tupla con:
///     - `u32`: cantidad ingresada del artículo.
///     - `u32`: precio del artículo.
///
/// # Flujo de entrada
/// 1. Se solicita el **código del detalle de entrada**.  
/// 2. Se pide la cantidad de artículos a registrar (`n`).  
/// 3. Para cada artículo:
///    - Código del artículo.  
///    - Cantidad.  
///    - Precio.  
pub fn leer_datos_detalle_entrada() -> (String, HashMap<String, (u32,u32)>) {
    // Lógica para leer datos del artículo
    view_leer::mostrar_titulo("INGRESE EL CODIGO DEL DETALLE DE LA ENTRADA");
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
/// Menú interactivo para la gestión de Detalles de Entrada.
///
/// - `dc`: Diccionario principal de detalles de entrada.
///   - **Clave**: Código del detalle de entrada (`String`)
///   - **Valor**: `HashMap` de artículos
///       - **Clave**: Código de artículo (`String`)
///       - **Valor**: `(cantidad, precio)`
///
/// Opciones del menú:
/// 1. Agregar detalle de entrada
/// 2. Modificar detalle existente
/// 3. Eliminar detalle
/// 4. Listar detalles
/// 5. Salir
pub fn run_detalle_entrada(dc: &mut HashMap<String, HashMap<String, (u32, u32)>>){
    loop{
        view::menu_secundario("Detalles de Entrada".to_string());
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::service_agregar(dc, leer_datos_detalle_entrada()),
            2 => services::service_modificar::modificar(dc,leer_datos_detalle_entrada()),
            3 => services::service_eliminar::eliminar(dc,utils::utils_leer::leer_string()),
            4 => services::service_listar::listar(dc, vec!["Codigo", "Codigo Articulo", "Cantidad", "Precio"]),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}

use crate::views::view_menu;
use crate::views::view_leer;
use crate::views::view_error;
use crate::models::services;
use crate::views::view_listar;
use crate::models::utils;
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
pub fn leer_datos_detalle_entrada(dc: &mut HashMap<String, HashMap<String, (u32, u32)>>) -> (String, HashMap<String, (u32,u32)>) {
    let mut codigo:String;
    loop{
        view_leer::mostrar_titulo("INGRESE EL CODIGO DEL DETALLE DE LA ENTRADA");
        codigo = utils::utils_leer::leer_string();

        if utils::utils_validaciones::validar_existencia_h(codigo.clone(), dc){
            view_error::error_existencia();
        } else {
            break;
        }
    }
    // Lógica para leer datos del artículo

    let mut dictemp : HashMap<String, (u32,u32)> = HashMap::new();
    view_leer::mostrar_mensaje("Ingrese la cantidad de articulos a ingresar:");
    let n : u32 = utils::utils_leer::leer_u32();
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

pub fn leer_datos_detalle_entrada_m(dc: &mut HashMap<String, HashMap<String, (u32, u32)>>) -> (String, HashMap<String, (u32,u32)>) {
    let mut codigo:String;
    loop{
        view_leer::mostrar_titulo("INGRESE EL CODIGO DEL DETALLE DE LA ENTRADA");
        codigo = utils::utils_leer::leer_string();

        if !utils::utils_validaciones::validar_existencia_h(codigo.clone(), dc){
            view_error::error_no_existencia();
        } else {
            break;
        }
    }
    // Lógica para leer datos del artículo

    let mut dictemp : HashMap<String, (u32,u32)> = HashMap::new();
    view_leer::mostrar_mensaje("Ingrese la cantidad de articulos a ingresar:");
    let n : u32 = utils::utils_leer::leer_u32();
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
        view_menu::menu_secundario("Detalles de Entrada".to_string());
        view_leer::mostrar_mensaje("Ingrese una opcion:");
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::services_agregar::agregar_h_e(dc, leer_datos_detalle_entrada),
            2 => services::services_modificar::modificar_h_e(dc,leer_datos_detalle_entrada_m),
            3 => services::services_eliminar::eliminar_h(dc,utils::utils_leer::leer_string()),
            4 => view_listar::mostrar_listado(services::services_listar::listar_h(dc, vec!["Codigo", "Codigo Articulo", "Cantidad", "Precio"])),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}

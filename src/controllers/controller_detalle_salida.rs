use crate::views::view_menu;
use crate::views::view_leer;
use crate::views::view_error;
use crate::views::view_listar;
use crate::models::utils;
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
pub fn leer_datos_detalle_salida(dc: &mut HashMap<String, HashMap<String, (u32, u32)>>, dc_detalle_entrada : &mut HashMap<String, HashMap<String, (u32,u32)>>) -> (String, HashMap<String, (u32,u32)>) {
    let mut codigo:String;
    loop{
        view_leer::mostrar_titulo("INGRESE EL CODIGO DEL DETALLE DE LA SALIDA");
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
                let mut cantidad : u32;
        loop {
            view_leer::mostrar_mensaje("Ingrese la cantidad del articulo:");
            cantidad = utils::utils_leer::leer_u32();
            if utils::utils_validaciones::validar_stock(dc,dc_detalle_entrada, codigo_articulo.clone(), cantidad){
                break;
            }else {
                view_error::error_stock_insuficiente();
            }
        }
        view_leer::mostrar_mensaje("Ingrese el precio del articulo:");
        let precio = utils::utils_leer::leer_u32();
        dictemp.insert(codigo_articulo, (cantidad,precio));
    }
    return (codigo, dictemp);
}

pub fn leer_datos_detalle_salida_m(dc: &mut HashMap<String, HashMap<String, (u32, u32)>>, dc_detalle_entrada : &mut HashMap<String, HashMap<String, (u32,u32)>>) -> (String, HashMap<String, (u32,u32)>) {
    let mut codigo:String;
    loop{
        view_leer::mostrar_titulo("INGRESE EL CODIGO DEL DETALLE DE LA SALIDA");
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
        let mut cantidad : u32;
        loop {
            view_leer::mostrar_mensaje("Ingrese la cantidad del articulo:");
            cantidad = utils::utils_leer::leer_u32();
            if utils::utils_validaciones::validar_stock(dc,dc_detalle_entrada, codigo_articulo.clone(), cantidad){
                break;
            }else {
                view_error::error_stock_insuficiente();
            }
        }
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
pub fn run_detalle_salida(dc: &mut HashMap<String, HashMap<String, (u32, u32)>>, dc_detalle_entrada : &mut HashMap<String, HashMap<String, (u32,u32)>>){
    loop{
        view_menu::menu_secundario("Detalles de Salida".to_string());
        view_leer::mostrar_mensaje("Ingrese una opcion:");
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => services::services_agregar::agregar_h_s(dc,dc_detalle_entrada,leer_datos_detalle_salida),
            2 => services::services_modificar::modificar_h_s(dc,dc_detalle_entrada,leer_datos_detalle_salida_m),
            3 => services::services_eliminar::eliminar_h(dc,utils::utils_leer::leer_string()),
            4 => view_listar::mostrar_listado(services::services_listar::listar_h(dc, vec!["Codigo", "Codigo Articulo", "Cantidad", "Precio"])),
            5 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}


use std::collections::HashMap;

/// Modifica o agrega una categoría en el HashMap.
///
/// # Parámetros
/// - `dc`: HashMap mutable de categorías (`clave: código`, `valor: nombre`).
/// - `leer`: Función que devuelve `(clave, valor)` de la categoría a modificar.

//Categoria
pub fn modificar_s(dc: &mut HashMap<String, String>, leer : fn(dc: &mut HashMap<String, String>) -> (String,String)){

    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

/// Modifica o agrega un artículo en el HashMap.
///
/// # Parámetros
/// - `dc`: HashMap mutable de artículos (`clave: código`, `valor: (nombre, categoría, precio)`).
/// - `leer`: Función que devuelve `(clave, valor)` del artículo a modificar.

//Articulo
pub fn modificar_t1(dc: &mut HashMap<String, (String,String,u32)>, leer : fn(dc: &mut HashMap<String, (String,String,u32)>) -> (String,(String,String,u32))){

    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

/// Modifica o agrega un proveedor en el HashMap.
///
/// # Parámetros
/// - `dc`: HashMap mutable de proveedores (`clave: código`, `valor: (nombre, RUC, dirección, teléfono)`).
/// - `leer`: Función que devuelve `(clave, valor)` del proveedor a modificar.

//proveedor
pub fn modificar_t2(dc: &mut HashMap<String,(String,String,String,String)>, leer : fn(dc: &mut HashMap<String,(String,String,String,String)>) -> (String,(String,String,String,String))){

    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

/// Modifica o agrega un documento de entrada o salida.
///
/// # Parámetros
/// - `dc`: HashMap mutable de documentos (`clave: número de documento`, `valor: (fecha, proveedor/cliente)`).
/// - `leer`: Función que devuelve `(clave, valor)` del documento a modificar.

//Entrada y Salida
pub fn modificar_t3(dc: &mut HashMap<String,(String,String)>, leer : fn(dc: &mut HashMap<String,(String,String)>) -> (String,(String,String))){

    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

/// Modifica o agrega los detalles de un documento de entrada o salida.
///
/// # Parámetros
/// - `dc`: HashMap mutable de detalles (`clave: número de documento`, `valor: HashMap de artículos (cantidad, precio)`).
/// - `leer`: Función que devuelve `(clave, valor)` de los detalles a modificar.
///`

//Detalle Entrada y Detalle Salida
pub fn modificar_h(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>, leer : fn(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>) -> (String,HashMap<String,(u32,u32)>)){

    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}
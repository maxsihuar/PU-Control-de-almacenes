use std::collections::HashMap;


/// Elimina un registro de un HashMap de categorías.
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = código de categoría, valor = nombre de categoría.
/// - `clave`: Código de la categoría que se desea eliminar.
//Categoria
pub fn eliminar_s(dc: &mut HashMap<String, String>, clave: String){

    dc.remove(&clave);
}


/// Elimina un registro de un HashMap de artículos.
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = código de artículo, valor = (nombre, categoría, precio).
/// - `clave`: Código del artículo que se desea eliminar.
//Articulo
pub fn eliminar_t1(dc: &mut HashMap<String, (String,String,u32)>, clave: String){

    dc.remove(&clave);
}


/// Elimina un registro de un HashMap de proveedores.
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = código de proveedor, valor = (nombre, RUC, dirección, teléfono).
/// - `clave`: Código del proveedor que se desea eliminar.
//proveedor
pub fn eliminar_t2(dc: &mut HashMap<String,(String,String,String,String)>, clave: String){

    dc.remove(&clave);
}


/// Elimina un registro de un HashMap de documentos de entrada o salida.
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = número de documento, valor = (fecha, proveedor/cliente).
/// - `clave`: Número de documento que se desea eliminar.
//Entrada y Salida
pub fn eliminar_t3(dc: &mut HashMap<String,(String,String)>, clave: String){

    dc.remove(&clave);
}


/// Elimina un registro de un HashMap de detalles de documentos (entrada o salida).
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = número de documento, valor = HashMap de artículos (cantidad, precio).
/// - `clave`: Número de documento que se desea eliminar.
//Detalle Entrada y Detalle Salida
pub fn eliminar_h(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>, clave: String){

    dc.remove(&clave);
}


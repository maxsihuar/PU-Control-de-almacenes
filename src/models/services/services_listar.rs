use std::collections::HashMap;


/// Lista el contenido de un HashMap de categorías.
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = código de categoría, valor = nombre de categoría.
/// - `titulos`: Vector de títulos para el encabezado de la tabla.
///
/// # Retorna
/// Una tupla `(encabezado, filas)` donde `encabezado` es la fila de títulos y `filas` contiene
/// cada fila formateada de la tabla.

//Categoria
pub fn listar_s(dc: &mut HashMap<String, String>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        let fila = format!("{:<width$}{:<width$}", clave, valor, width = ancho_columna);
        filas.push(fila);

    }

    return (encabezado, filas);


}

/// Lista el contenido de un HashMap de artículos.
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = código de artículo, valor = (nombre, categoría, precio).
/// - `titulos`: Vector de títulos para el encabezado.
///
/// # Retorna
/// `(encabezado, filas)` formateadas.

//Articulo
pub fn listar_t1(dc: &mut HashMap<String, (String,String,u32)>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        let fila = format!(
            "{:<width$}{:<width$}{:<width$}{:<width$}",
            clave, valor.0, valor.1, valor.2,
            width = ancho_columna
        );
        filas.push(fila);
    }

    return (encabezado, filas);

}

/// Lista el contenido de un HashMap de proveedores.
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = código de proveedor, valor = (nombre, RUC, dirección, teléfono).
/// - `titulos`: Vector de títulos.
///
/// # Retorna
/// `(encabezado, filas)` formateadas.

//proveedor
pub fn listar_t2(dc: &mut HashMap<String,(String,String,String,String)>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        let fila = format!(
            "{:<width$}{:<width$}{:<width$}{:<width$}{:<width$}",
            clave, valor.0, valor.1, valor.2, valor.3,
            width = ancho_columna
        );
        filas.push(fila);
    }

    return (encabezado, filas);
}

/// Lista documentos de entrada o salida.
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = número de documento, valor = (fecha, proveedor/cliente).
/// - `titulos`: Vector de títulos.
///
/// # Retorna
/// `(encabezado, filas)` con cada documento formateado.

//Entrada y Salida
pub fn listar_t3(dc: &mut HashMap<String,(String,String)>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        let fila = format!(
            "{:<width$}{:<width$}{:<width$}",
            clave, valor.0, valor.1,
            width = ancho_columna
        );
        filas.push(fila);
    }

    return (encabezado, filas);
}

/// Lista los detalles de documentos de entrada o salida.
///
/// # Parámetros
/// - `dc`: HashMap mutable con clave = número de documento, valor = HashMap de artículos (cantidad, precio).
/// - `titulos`: Vector de títulos.
///
/// # Retorna
/// `(encabezado, filas)` donde cada fila contiene: documento, código de artículo, cantidad y precio

//Detalle Entrada y Detalle Salida
pub fn listar_h(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        for (clave2, valor2) in valor.iter() {
            let fila = format!(
                "{:<width$}{:<width$}{:<width$}{:<width$}",
                clave, clave2, valor2.0, valor2.1,
                width = ancho_columna
            );
            filas.push(fila);
        }
    }

    return (encabezado, filas);
}



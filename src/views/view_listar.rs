use super::utils::utils_view;

/// Muestra por pantalla un listado con encabezado y filas de datos.
///
/// # Parámetros
/// - `datos`: una tupla `(String, Vec<String>)` donde:
///   - `String` corresponde al **encabezado** (normalmente generado con los títulos de las columnas).
///   - `Vec<String>` contiene las **filas de contenido**, cada una representada como una cadena ya formateada.
///
pub fn mostrar_listado(datos : (String, Vec<String>)) {
    println!();
    println!("{}" , datos.0);
    println!("{}", "-".repeat(datos.0.len()));
    println!();
    for fila in datos.1 {
        println!("{}", fila);
    }

    utils_view::pausar();
}
use std::collections::HashMap;

/// Crea un nuevo diccionario (`HashMap`) para almacenar las salidas.
///
/// # Retorno
/// Retorna un `HashMap<String, Vec<String, String>>` donde:
///
/// - La **clave** (`String`) representa el codigo del documento de salida.
/// - El **valor** es una **vector** con la siguiente estructura:
///   1. `String` — Fecha de la salida.
///   2. `String` — Codigo del cliente.


pub fn crear_salida() -> HashMap<String, Vec<String, String>> {
    return HashMap::new()
}

use std::collections::HashMap;

/// Crea un nuevo diccionario (`HashMap`) para almacenar las entradas.
///
/// # Retorno
/// Retorna un `HashMap<String, HashMap<String, (String, String)>>` vacío donde:
///
/// - La **clave** (`String`) representa el codigo del documento de entrada.
/// - El **valor** es una **tupla** con la siguiente estructura:
///   1. `String` — Fecha de la entrada.
///   2. `String` — Codigo del proveedor.


pub fn crear_entrada() -> HashMap<String, (String, String)> {
    return HashMap::new()
}

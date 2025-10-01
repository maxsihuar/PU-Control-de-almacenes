use std::collections::HashMap;
/// Crea un nuevo diccionario (`HashMap`) para almacenar proveedores.
///
/// # Retorno
/// Retorna un `HashMap<String, Vec<String, String, String, String>>` donde:
///
/// - La **clave** (`String`) representa el codigo del proveedor.
/// - El **valor** es una **vector** con la siguiente estructura:
///   1. `String` — Razon social del proveedor.
///   2. `String` — RUC del proveedor.
///   4. `String` — Direccion.
///   5. `String` — Ciudad.}
pub fn crear_proveedor() -> HashMap<String, Vec<String, String, String, String>> {
    return HashMap::new()
}

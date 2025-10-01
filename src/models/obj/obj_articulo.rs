use std::collections::HashMap;
/// Crea un nuevo diccionario (`HashMap`) para almacenar artículos.
///
/// # Retorno
/// Retorna un `HashMap<String, (String, String, u32)>` donde:
///
/// - La **clave** (`String`) representa el código del artículo.
/// - El **valor** es una **tupla** con la siguiente estructura:
///   1. `String` — Descripción del artículo.
///   2. `String` — Código de la categoría.
///   3. `u32` — Precio.

pub fn crear_articulo() -> HashMap<String, (String, String, u32)>{
    return HashMap::new();
}

use std::collections::HashMap;

/// Crea un nuevo diccionario (`HashMap`) para almacenar las entradas.
///
/// # Retorno
/// Retorna un `HashMap<String, HashMap<String, (u32, String)>>` vacío donde:
///
/// El mapa tiene la siguiente estructura anidada:
/// - **Clave externa (`String`)**: Representa el codigo del documento de salida.
/// - **Valor externo (`HashMap<String, (u32, String)>`)**: Otro hashmapa, donde:
///   - **Clave interna (`String`)**: Subclave asociada al codigo del articulo.
///   - **Valor interno (`(u32, String)`)**: Una tupla que contiene:
///       - `u32`: La cantidad unidades de un producto.
///       - `u32`: El precio unitario del producto.


pub fn crear_detalle_salida() -> HashMap<String, HashMap<String, (u32,u32)>> {
    return HashMap::new()
}

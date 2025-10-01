use std::collections::HashMap;

/// Crea un nuevo diccionario (`HashMap`) para almacenar las entradas.
///
/// # Retorno
/// Retorna un `HashMap<String, Vec<String, String, String, String>>` donde:
///
/// El mapa tiene la siguiente estructura anidada:
/// - **Clave externa (`String`)**: Representa el codigo del documento de entrada.
/// - **Valor externo (`HashMap<String, (u32, String)>`)**: Otro hashmapa, donde:
///   - **Clave interna (`String`)**: Subclave asociada al codigo del proveedor.
///   - **Valor interno (`(u32, String)`)**: Un vector que contiene:
///       - `u32`: La cantidad unidades de un producto.
///       - `u32`: El precio unitario del producto.


pub fn crear_detalle_entrada() -> HashMap<String, HashMap<String, Vec<u32,u32>>> {
    return HashMap::new()
}

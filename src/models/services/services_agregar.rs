use std::collections::HashMap;

enum Valor {
    /// Variante que guarda un String simple
    Texto(String),

    /// Variante que guarda una tupla (código, nombre, cantidad)
    Tupla(String, String, u32),

    /// Variante que guarda un HashMap
    /// - La clave es String
    /// - El valor es una tupla (String, String, u32)
    Mapa(HashMap<String, (String, String, u32)>),
}


/// Inserta un nuevo par `(clave, valor)` en el `HashMap`.
///
/// # Parámetros
/// - `dc`: Referencia mutable a un `HashMap<String, Valor>` donde se guardarán los datos.
/// - `leer`: Función (o closure) que no recibe parámetros y devuelve una tupla `(String, Valor)`.
///           - El `String` representa la clave de la entrada.
///           - `Valor` es la variante del enum que se desea almacenar como valor.
///
/// # Comportamiento
/// 1. Llama a la función `leer()` para obtener la clave y el valor.
/// 2. Inserta el par en el `HashMap`.
/// 3. Si la clave ya existía, reemplaza el valor anterior.
///
/// # Retorno
/// - Devuelve `None` si la clave era nueva.
/// - Devuelve `Some(valor_anterior)` si ya existía un valor para esa clave.

pub fn agregar(dc: &mut HashMap<String, Valor>, leer : fn() -> (String,Valor)){

    let clave_valor = leer();
    dc.insert(clave_valor.0, clave_valor.1);
}

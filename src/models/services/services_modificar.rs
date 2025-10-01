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

/// Modifica o inserta un elemento en un `HashMap<String, Valor>`.
///
/// # Parámetros
/// - `dc`: Referencia mutable al `HashMap<String, Valor>` donde se actualizarán los datos.
/// - `leer`: Función (o closure) que devuelve una tupla `(String, Valor)`.
///     - El `String` representa la **clave** de la entrada.
///     - `Valor` es el nuevo valor asociado a esa clave.
///
/// # Comportamiento
/// 1. Llama a la función `leer()` para obtener `(clave, valor)`.
/// 2. Inserta la entrada en el `HashMap`.
///     - Si la clave ya existía, el valor anterior es reemplazado.
///     - Si la clave no existía, se crea una nueva entrada.
///
/// # Retorno
/// - Devuelve `None` si la clave era nueva.
/// - Devuelve `Some(valor_anterior)` si ya existía y fue reemplazado.
///   *(esto gracias a [`HashMap::insert`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert))*.
///

pub fn modificar(dc: &mut HashMap<String, Valor>, leer : fn() -> (String,Valor)){

    let clave_valor = leer();
    dc.insert(clave_valor.0, clave_valor.1);
}

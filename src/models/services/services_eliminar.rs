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

/// Elimina un elemento de un `HashMap<String, Valor>` dada su clave.
///
/// # Parámetros
/// - `dc`: Referencia mutable al `HashMap<String, Valor>` donde se eliminará el elemento.
/// - `leer`: Función que en esta versión no se utiliza (puede retirarse o redefinirse 
///           si más adelante se necesita interacción para obtener la clave).
/// - `clave`: `String` que identifica la entrada que se quiere eliminar.
///
/// # Comportamiento
/// - Busca la clave en el `HashMap`.
/// - Si existe, elimina la entrada asociada.
/// - Si no existe, no realiza ningún cambio.
///
/// # Retorno
/// - No devuelve nada directamente.
/// - El método interno [`remove`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.remove) 
///   retorna un `Option<Valor>`, pero aquí se ignora.
///   - `Some(valor)` → el valor eliminado.
///   - `None` → no había un elemento con esa clave.


pub fn eliminar(dc: &mut HashMap<String, Valor>, leer : fn(), clave: String){

    dc.remove(&clave);
}
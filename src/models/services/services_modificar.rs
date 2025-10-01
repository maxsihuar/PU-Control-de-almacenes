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



pub fn modificar(dc: &mut HashMap<String, Valor>, leer : fn() -> (String,Valor)){

    let clave_valor = leer();
    dc.insert(clave_valor.0, clave_valor.1);
}

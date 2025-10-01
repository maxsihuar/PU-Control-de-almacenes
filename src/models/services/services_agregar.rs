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

pub fn agregar(dc: &mut HashMap<String, Valor>, leer : fn() -> (String,Valor)){

    loop{
        let codigo = datos().0; // util.validarcodigo();
        let nombre = datos().1; // util.validarnombre();
        if !validar_existencia(dc, &codigo) {
            break;
        }
    }

    dc.insert(codigo, nombre);
}

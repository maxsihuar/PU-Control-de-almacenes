use std::collections::HashMap;


enum Valor {
    /// Variante que guarda un String simple
    Texto(String),

    /// Variante que guarda una tupla (código, nombre, cantidad)
    TuplaProveedor(String, String, String, String),

    /// Variante que guarda una tupla (fecha, codigo_documento)
    TuplaES(String, String),

    ///
    TuplaArticulo(String, String, u32),

    /// Variante que guarda un HashMap
    /// - La clave es String
    /// - El valor es una tupla (String, String, u32)
    Mapa(HashMap<String, (String, String, u32)>),
}


/// Genera una tabla en formato de texto a partir de títulos y datos.
/// 
/// # Parámetros
/// - `dc`: referencia mutable a un `HashMap<String, Valor>`. 
///   Actualmente no se está utilizando dentro de la función, pero la idea
///   podría ser que los datos de este diccionario se conviertan en las filas.
/// - `leer`: función que se pasa como parámetro (tipo `fn()`), 
///   aunque tampoco se usa en la versión actual.
/// - `titulos`: un `Vec<&str>` que contiene los nombres de las columnas de la tabla.
///
/// # Retorna
/// Una tupla `(String, Vec<String>)` donde:
/// - `String`: representa el **encabezado** de la tabla (con los títulos alineados).
/// - `Vec<String>`: representa las **filas** de la tabla, cada fila es una cadena con columnas formateadas.
///
/// # Detalles de implementación
/// - `ancho_columna`: define el ancho fijo de cada columna (20 caracteres).
/// - El encabezado se construye alineando los títulos a la izquierda dentro de ese ancho fijo.
/// - El contenido (`contenido`) se llena recorriendo cada fila y formateando cada valor con el mismo ancho.
///


pub fn listar(dc: &mut HashMap<String, Valor>, titulos : Vec<&str>) -> (String, Vec<String>) {

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    let mut contenido: Vec<String> = Vec::new();
    for (clave, valor) in dc.iter() {
        match valor {
            Valor::Texto(texto) => {
                contenido.push(format!("{:<width$}", texto, width = ancho_columna));
            },
            Valor::Tupla(codigo, nombre, cantidad) => {
                let fila = vec![clave.as_str(), codigo.as_str(), nombre.as_str(), &cantidad.to_string()];
                let mut linea = String::new();
                for valor in fila {
                    linea.push_str(&format!("{:<width$}", valor, width = ancho_columna));
                }
                contenido.push(linea);
            },
            Valor::Mapa(mapa) => {
                for (sub_clave, (codigo, nombre, cantidad)) in mapa.iter() {
                    let fila = vec![clave.as_str(), sub_clave.as_str(), codigo.as_str(), nombre.as_str(), &cantidad.to_string()];
                    let mut linea = String::new();
                    for valor in fila {
                        linea.push_str(&format!("{:<width$}", valor, width = ancho_columna));
                    }
                    contenido.push(linea);
                }
            },
        }
    }

    (encabezado, contenido)
    
}


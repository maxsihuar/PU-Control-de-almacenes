use std::collections::HashMap;


//Categoria
pub fn listar_s(dc: &mut HashMap<String, String>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        let fila = format!("{:<width$}{:<width$}", clave, valor, width = ancho_columna);
        filas.push(fila);

    }

    return (encabezado, filas);


}


//Articulo
pub fn listar_t1(dc: &mut HashMap<String, (String,String,u32)>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        let fila = format!(
            "{:<width$}{:<width$}{:<width$}{:<width$}",
            clave, valor.0, valor.1, valor.2,
            width = ancho_columna
        );
        filas.push(fila);
    }

    return (encabezado, filas);

}

//proveedor
pub fn listar_t2(dc: &mut HashMap<String,(String,String,String,String)>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        let fila = format!(
            "{:<width$}{:<width$}{:<width$}{:<width$}{:<width$}",
            clave, valor.0, valor.1, valor.2, valor.3,
            width = ancho_columna
        );
        filas.push(fila);
    }

    return (encabezado, filas);
}

//Entrada y Salida
pub fn listar_t3(dc: &mut HashMap<String,(String,String)>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        let fila = format!(
            "{:<width$}{:<width$}{:<width$}",
            clave, valor.0, valor.1,
            width = ancho_columna
        );
        filas.push(fila);
    }

    return (encabezado, filas);
}

//Detalle Entrada y Detalle Salida
pub fn listar_h(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>, titulos : Vec<&str>) -> (String, Vec<String>){

    let ancho_columna = 20;
    let mut encabezado: String = String::new();
    let mut filas: Vec<String> = Vec::new();
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (clave, valor) in dc.iter() {
        for (clave2, valor2) in valor.iter() {
            let fila = format!(
                "{:<width$}{:<width$}{:<width$}{:<width$}",
                clave, clave2, valor2.0, valor2.1,
                width = ancho_columna
            );
            filas.push(fila);
        }
    }

    return (encabezado, filas);
}



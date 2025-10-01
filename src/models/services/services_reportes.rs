use std::collections::HashMap;
use std::io::{self, Write};
use crate::views::view_leer;
use crate::views::view_error;
use crate::models::utils;


pub fn listar_articulos_por_categoria(d_categorias: &HashMap<String, String>,d_articulos: &HashMap<String, (String, String, u32)>,titulos: Vec<&str>,) -> (String, Vec<String>) {
    let ancho_columna = 20;
    let mut encabezado = String::new();
    let mut filas: Vec<String> = Vec::new();

    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    for (cod_cat, nom_cat) in d_categorias {
        filas.push(format!("Categoría: {} (Código: {})", nom_cat, cod_cat));
        filas.push("-".repeat(ancho_columna * titulos.len()));

        for (cod_art, (nom_art, cat_art, precio_art)) in d_articulos {
            if cat_art == cod_cat {
                let fila = format!(
                    "{:<width$}{:<width$}{:<width$}",
                    cod_art, nom_art, precio_art,
                    width = ancho_columna
                );
                filas.push(fila);
            }
        }
        filas.push(String::new());
    }

    (encabezado, filas)
}


pub fn listar_documentos_fechas(
    d_entrada: &HashMap<String, (String, String)>,
    d_salida: &HashMap<String, (String, String)>,
    d_proveedores: &HashMap<String, (String, String, String, String)>,
    titulos_entrada: Vec<&str>,
    titulos_salida: Vec<&str>
) -> (String, Vec<String>) {
    let ancho_columna = 20;
    let mut encabezado = String::new();
    let mut filas: Vec<String> = Vec::new();

    view_leer::mostrar_mensaje("Ingrese la fecha de inicio (YYYY-MM-DD):");
    let fecha_inicio: String = utils::utils_leer::leer_string();

    view_leer::mostrar_mensaje("Ingrese la fecha de fin (YYYY-MM-DD):");
    let fecha_fin: String = utils::utils_leer::leer_string();

    // Encabezado de entrada
    for titulo in &titulos_entrada {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    filas.push("Documentos de Entrada:".to_string());
    filas.push("-".repeat(ancho_columna * titulos_entrada.len()));

    let default_val = ("Desconocido".to_string(), "".to_string(), "".to_string(), "".to_string());

    for (doc, (fecha, cod_prov)) in d_entrada {
        if fecha_inicio <= *fecha && *fecha <= fecha_fin {
            let proveedor = d_proveedores.get(cod_prov).unwrap_or(&default_val);
            let fila = format!(
                "{:<width$}{:<width$}{:<width$}",
                doc, fecha, proveedor.0,
                width = ancho_columna
            );
            filas.push(fila);
        }
    }

    filas.push(String::new()); // separación

    // Encabezado de salida
    let mut encabezado_salida = String::new();
    for titulo in &titulos_salida {
        encabezado_salida.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    filas.push("Documentos de Salida:".to_string());
    filas.push("-".repeat(ancho_columna * titulos_salida.len()));

    for (doc, (fecha, nombre)) in d_salida {
        if fecha_inicio <= *fecha && *fecha <= fecha_fin {
            let fila = format!(
                "{:<width$}{:<width$}{:<width$}",
                doc, fecha, nombre,
                width = ancho_columna
            );
            filas.push(fila);
        }
    }

    (encabezado + &encabezado_salida, filas)
}

pub fn listar_articulos_proveedor(
    d_proveedores: &mut HashMap<String, (String,String,String,String)>,
    d_entrada: &HashMap<String, (String, String)>,
    d_entrada_detalle: &HashMap<String, HashMap<String, (u32, u32)>>,
    d_articulos: &HashMap<String, (String, String, u32)>,
) -> (String, Vec<String>) {
    let ancho_columna = 20;
    let mut encabezado = String::new();
    let mut filas: Vec<String> = Vec::new();

    // Leer código del proveedor
    let cod_prov: String = loop {
        let input = utils::utils_leer::leer_string();
        if !utils::utils_validaciones::validar_existencia_t2(input.clone(), d_proveedores) {
            view_error::error_no_existencia();
        } else {
            break input;
        }
    };

    // Construir encabezado
    let titulos = vec!["Código", "Nombre del Artículo", "Cantidad", "Precio Unitario"];
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    filas.push(format!("Artículos comprados al proveedor {} ", cod_prov));
    filas.push("-".repeat(ancho_columna * titulos.len()));

    // Valor por defecto compatible con la tupla de d_articulos
    let default_val = ("Desconocido".to_string(), "".to_string(), 0u32);

    // Construir filas
    for (doc, items) in d_entrada_detalle {
        if let Some((_, prov_doc)) = d_entrada.get(doc) {
            if prov_doc == &cod_prov {
                for (cod_art, (cantidad, precio)) in items {
                    let nom_art = d_articulos.get(cod_art).unwrap_or(&default_val);
                    let fila = format!(
                        "{:<width$}{:<width$}{:<width$}{:<width$}",
                        cod_art, nom_art.0, cantidad, precio,
                        width = ancho_columna
                    );
                    filas.push(fila);
                }
            }
        }
    }

    (encabezado, filas)
}



pub fn listar_stock_actual(
    d_entrada_detalle: &HashMap<String, HashMap<String, (u32, u32)>>,
    d_salida_detalle: &HashMap<String, HashMap<String, (u32, u32)>>,
    d_articulos: &HashMap<String, (String, String, u32)>,
) -> (String, Vec<String>) {
    let ancho_columna = 20;
    let mut encabezado = String::new();
    let mut filas: Vec<String> = Vec::new();

    // Construir encabezado
    let titulos = vec!["Código", "Nombre del Artículo", "Stock Actual"];
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    // Calcular stock
    let mut stock: HashMap<String, i32> = HashMap::new();

    for items in d_entrada_detalle.values() {
        for (cod_art, (cantidad, _)) in items {
            *stock.entry(cod_art.clone()).or_insert(0) += *cantidad as i32;
        }
    }

    for items in d_salida_detalle.values() {
        for (cod_art, (cantidad, _)) in items {
            *stock.entry(cod_art.clone()).or_insert(0) -= *cantidad as i32;
        }
    }

    // Valor por defecto compatible con la tupla
    let default_val = ("Desconocido".to_string(), "".to_string(), 0u32);

    // Construir filas
    for (cod_art, cantidad) in &stock {
        let nom_art = d_articulos.get(cod_art).unwrap_or(&default_val);
        let fila = format!(
            "{:<width$}{:<width$}{:<width$}",
            cod_art,
            nom_art.0, // solo el nombre del artículo
            cantidad,
            width = ancho_columna
        );
        filas.push(fila);
    }

    (encabezado, filas)
}
pub fn listar_historial_articulos(
    d_articulos: &mut HashMap<String, (String,String,u32)>,
    d_entrada: &mut HashMap<String, (String, String)>,
    d_entrada_detalle: &mut HashMap<String, HashMap<String, (u32, u32)>>,
    d_salida: &mut HashMap<String, (String, String)>,
    d_salida_detalle: &mut HashMap<String, HashMap<String, (u32, u32)>>
) -> (String, Vec<String>) {
    let ancho_columna = 20;
    let mut encabezado = String::new();
    let mut filas: Vec<String> = Vec::new();

    let mut cod_art:String;
    loop{
        cod_art = utils::utils_leer::leer_string();
        if !utils::utils_validaciones::validar_existencia_t1(cod_art.clone(), d_articulos){
            view_error::error_no_existencia();
        } else {
            break;
        }
    }
        // Construir encabezado
    let titulos = vec!["Tipo", "Documento", "Fecha", "Cantidad", "Precio Unitario"];
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    filas.push(format!("Historial del artículo {} ", cod_art));
    filas.push("-".repeat(ancho_columna * titulos.len()));

    let default_val = "Desconocido".to_string();

    // Entradas
    for (doc, items) in d_entrada_detalle {
        if let Some((cantidad, precio)) = items.get(&cod_art) {
            let fecha = d_entrada.get(doc).map(|(fecha, _)| fecha).unwrap_or(&default_val);
            let fila = format!(
                "{:<width$}{:<width$}{:<width$}{:<width$}{:<width$}",
                "Entrada", doc, fecha, cantidad, precio,
                width = ancho_columna
            );
            filas.push(fila);
        }
    }

    let default_val = "Desconocido".to_string();

        // Salidas
    for (doc, items) in d_salida_detalle {
        if let Some((cantidad, precio)) = items.get(&cod_art) {
            let fecha = d_salida.get(doc).map(|(fecha, _)| fecha).unwrap_or(&default_val);
            let fila = format!(
                "{:<width$}{:<width$}{:<width$}{:<width$}{:<width$}",
                "Salida", doc, fecha, cantidad, precio,
                width = ancho_columna
            );
            filas.push(fila);
        }
    }

    (encabezado, filas)
}


pub fn listar_valor_total(
    d_entrada_detalle: &HashMap<String, HashMap<String, (u32, u32)>>,
    d_salida_detalle: &HashMap<String, HashMap<String, (u32, u32)>>,
    d_articulos: &HashMap<String, (String, String, u32)>,
) -> (String, Vec<String>) {
    let ancho_columna = 20;
    let mut encabezado = String::new();
    let mut filas: Vec<String> = Vec::new();

    // Construir encabezado
    let titulos = vec!["Código", "Nombre del Artículo", "Valor en Stock"];
    for titulo in &titulos {
        encabezado.push_str(&format!("{:<width$}", titulo, width = ancho_columna));
    }

    // Calcular valor en stock
    let mut stock: HashMap<String, f64> = HashMap::new();

    for items in d_entrada_detalle.values() {
        for (cod_art, (cantidad, precio)) in items {
            *stock.entry(cod_art.clone()).or_insert(0.0) += (*cantidad as f64) * (*precio as f64);
        }
    }

    for items in d_salida_detalle.values() {
        for (cod_art, (cantidad, precio)) in items {
            *stock.entry(cod_art.clone()).or_insert(0.0) -= (*cantidad as f64) * (*precio as f64);
        }
    }

    // Valor por defecto compatible con la tupla
    let default_val = ("Desconocido".to_string(), "".to_string(), 0u32);

    // Construir filas
    for (cod_art, valor) in &stock {
        let nom_art = d_articulos.get(cod_art).unwrap_or(&default_val);
        let fila = format!(
            "{:<width$}{:<width$}{:<width$.2}",
            cod_art,
            nom_art.0, // solo el nombre del artículo
            valor,
            width = ancho_columna
        );
        filas.push(fila);
    }

    (encabezado, filas)
}
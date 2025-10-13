use std::collections::HashMap;

/// Agrega una nueva categoría al diccionario.
/// 
/// - `dc`: Diccionario de categorías (`Código → Nombre`)
/// - `leer`: Función que devuelve la clave y el valor a insertar.
pub fn agregar_s(
    dc: &mut HashMap<String, String>,
    leer: fn(dc: &mut HashMap<String, String>) -> (String, String)
) {
    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

/// Agrega un nuevo artículo al diccionario.
/// 
/// - `dc`: Diccionario de artículos (`Código → (Nombre, CódigoCategoría, Precio)`)
/// - `leer`: Función que devuelve la clave y el valor a insertar.
pub fn agregar_t1(
    dc: &mut HashMap<String, (String, String, u32)>,
    leer: fn(dc: &mut HashMap<String, (String, String, u32)>) -> (String, (String, String, u32))
) {
    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

/// Agrega un nuevo proveedor al diccionario.
/// 
/// - `dc`: Diccionario de proveedores (`RUC → (Nombre, Teléfono, Dirección, Email)`)
/// - `leer`: Función que devuelve la clave y el valor a insertar.
pub fn agregar_t2(
    dc: &mut HashMap<String, (String, String, String, String)>,
    leer: fn(dc: &mut HashMap<String, (String, String, String, String)>) -> (String, (String, String, String, String))
) {
    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

/// Agrega una nueva entrada o salida general al diccionario.
/// 
/// - `dc`: Diccionario (`Código → (Fecha, CódigoRelacionado)`)
/// - `leer`: Función que devuelve la clave y el valor a insertar.
pub fn agregar_t3(
    dc: &mut HashMap<String, (String, String)>,
    leer: fn(dc: &mut HashMap<String, (String, String)>) -> (String, (String, String))
) {
    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

/// Agrega un nuevo detalle de entrada al diccionario.
/// 
/// - `dc`: Diccionario (`CódigoEntrada → {CódigoArtículo → (Cantidad, Precio)}`)
/// - `leer`: Función que devuelve la clave y el valor a insertar.
pub fn agregar_h_e(
    dc: &mut HashMap<String, HashMap<String, (u32, u32)>>,
    leer: fn(dc: &mut HashMap<String, HashMap<String, (u32, u32)>>) -> (String, HashMap<String, (u32, u32)>)
) {
    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

/// Agrega un nuevo detalle de salida al diccionario, validando con las entradas.
/// 
/// - `dc`: Diccionario de detalles de salida (`CódigoSalida → {Artículo → (Cantidad, Precio)}`)
/// - `dc_detalle_e`: Diccionario de detalles de entrada (para validar stock)
/// - `leer`: Función que devuelve la clave y el valor a insertar.
pub fn agregar_h_s(
    dc: &mut HashMap<String, HashMap<String, (u32, u32)>>,
    dc_detalle_e: &mut HashMap<String, HashMap<String, (u32, u32)>>,
    leer: fn(
        dc: &mut HashMap<String, HashMap<String, (u32, u32)>>,
        dc_detalle_e: &mut HashMap<String, HashMap<String, (u32, u32)>>
    ) -> (String, HashMap<String, (u32, u32)>)
) {
    let clave_valor = leer(dc, dc_detalle_e);
    dc.insert(clave_valor.0, clave_valor.1);
}


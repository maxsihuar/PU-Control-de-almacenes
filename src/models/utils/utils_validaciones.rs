
/// Verifica si un proveedor con el código especificado existe en el HashMap.
///
/// # Parámetros
/// - `codigo`: Código del proveedor a buscar.
/// - `dc`: HashMap que almacena los proveedores. 
///    La clave es el código del proveedor (`String`) 
///    y el valor es una tupla con los datos: `(rs, ruc, direccion, ciudad)`.
///
/// # Retorno
/// - `true` si existe un proveedor con ese código.
/// - `false` en caso contrario.
pub fn validar_existencia(codigo: String,dc: HashMap)->bool{
    for (codigo,(rs,ruc,direccion, ciudad)) in &dc{
        if codigo == &codigo {
            return true;
        }
    }
    return false;
}
/// Verifica si un proveedor con el código especificado **no existe** en el HashMap.
///
/// # Parámetros
/// - `codigo`: Código del proveedor a verificar.
/// - `dc`: HashMap que almacena los proveedores. 
///    La clave es el código del proveedor (`String`) 
///    y el valor es una tupla con los datos: `(rs, ruc, direccion, ciudad)`.
///
/// # Retorno
/// - `true` si el proveedor con ese código **no existe**.
/// - `false` si el proveedor **ya existe**.

pub fn validar_no_existencia(codigo: String, dc: HashMap)->bool{
    for (codigo,(rs,ruc,direccion, ciudad)) in &dc{
        if codigo == &codigo {
            //view::view_error::error_codigo_duplicado();
            return false;
        }
    }
    return true;
}

/// Valida un número de RUC antes de registrar un proveedor.
///
/// # Reglas de validación
/// 1. El RUC debe tener exactamente **11 caracteres**.
/// 2. El RUC no debe estar ya registrado en el `HashMap`.
///
/// # Parámetros
/// - `ruc`: Número de RUC a validar.
/// - `dc`: HashMap que almacena los proveedores. 
///    La clave es el código del proveedor (`String`) 
///    y el valor es una tupla con los datos: `(rs, ruc, direccion, ciudad)`.
///
/// # Retorno
/// - `true` si el RUC es válido y no existe en el sistema.
/// - `false` si el RUC tiene una longitud incorrecta o ya existe.
pub fn validar_RUC(ruc: String, dc: &HashMap<String, (String, String, String, String)>) -> bool {
    if ruc.len() != 11 {
        return false;
    }

    for (_codigo, (_rs, ruc_, _direccion, _ciudad)) in dc {
        if ruc == *ruc_ {
            return false;
        }
    }

    true
}

/// Valida un número de DNI.
///
/// Reglas:
/// - Debe tener exactamente 8 caracteres.
/// - No debe estar duplicado en la colección.
///
/// # Parámetros
/// - `dni`: DNI a validar.
/// - `dc`: HashMap de registros, donde:
///   - La clave es un `String` (código).
///   - El valor es `(String, String)` donde el primer campo es el DNI.
///
/// # Retorno
/// - `true` si el DNI es válido.
/// - `false` si no cumple las condiciones.
pub fn validar_DNI(dni: String, dc: &HashMap<String, (String, String)>) -> bool {
    if dni.len() != 8 {
        return false;
    }

    for (_codigo, (dni_existente, _fecha)) in dc {
        if dni == *dni_existente {
            return false;
        }
    }

    true
}
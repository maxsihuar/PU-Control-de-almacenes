
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
pub fn validar_RUC(ruc: String,dc: &HashMap)->bool{
    if ruc.len() != 11{
        //view::view_error::error_
        return false
    }
    for (codigo,(rs,ruc_,direccion, ciudad)) in &dc{
        if ruc == *ruc_ {
            //view::view_error::error_ruc_duplicado();
            return false
        }
    }
    return true
}
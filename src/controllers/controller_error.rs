use crate::src::views::view_error;

/// Controlador que gestiona el error cuando un código no existe.
///
/// # Descripción
/// Llama a la función correspondiente en la capa de **vista (`view_error`)**
/// para mostrar un mensaje de error de "código no existente".
pub fn controller_error_no_existencia() {
    view_error::error_no_existencia();
}

/// Controlador que gestiona el error cuando un código ya existe.
///
/// # Descripción
/// Llama a la función de la capa de **vista (`view_error`)**
/// para mostrar un mensaje de error de "código ya existente".
pub fn controller_error_existencia() {
    view_error::error_existencia();
}

/// Controlador que gestiona el error cuando el RUC no tiene el tamaño correcto.
///
/// # Descripción
/// Invoca la función de la capa de **vista (`view_error`)**
/// para mostrar el mensaje de error de "RUC debe tener 11 dígitos".
pub fn controller_error_RUC_tamaño() {
    view_error::error_RUC_tamaño();
}
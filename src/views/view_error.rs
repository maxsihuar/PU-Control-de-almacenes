use super::utils::utils_view;

/// Muestra un mensaje de error cuando el código ya existe en el sistema.
///
/// # Descripción
/// Limpia la pantalla, muestra un mensaje enmarcado de error
/// y luego pausa la ejecución hasta que el usuario presione una tecla.
pub fn error_existencia() {
    utils_view::limpiar_pantalla();
    println!("********************************************");
    println!("*         ERROR: EL CODIGO  YA EXISTE      *");
    println!("********************************************");
    utils_view::pausar();
}

/// Muestra un mensaje de error cuando el código no existe en el sistema.
///
/// # Descripción
/// Limpia la pantalla, imprime un mensaje enmarcado
/// y luego pausa la ejecución.
pub fn error_no_existencia() {
    utils_view::limpiar_pantalla();
    println!("********************************************");
    println!("*      ERROR: EL CODIGO NO EXISTE          *");
    println!("********************************************");
    utils_view::pausar();
}

/// Muestra un mensaje de error cuando el RUC no tiene el tamaño correcto.
///
/// # Descripción
/// Limpia la pantalla y muestra que el RUC debe tener exactamente 11 dígitos.
/// A diferencia de otros errores, aquí **no se pausa** la ejecución.
pub fn error_RUC_tamaño() {
    utils_view::limpiar_pantalla();
    println!("********************************************");
    println!("*   ERROR: EL RUC DEBE TENER 11 DIGITOS    *");
    println!("********************************************");
    utils_view::pausar();
}

/// Muestra un mensaje de error cuando un RUC ya pertenece a un proveedor.
///
/// # Descripción
/// Limpia la pantalla, imprime un mensaje enmarcado de error
/// y luego pausa la ejecución.
pub fn error_RUC_existencia() {
    utils_view::limpiar_pantalla();
    println!("*********************************************");
    println!("* ERROR: EL RUC YA PERTENECE A UN PROVEEDOR *");
    println!("*********************************************");
    utils_view::pausar();
}

/// Muestra un mensaje de error cuando se ingresa una opción inválida.
///
/// # Descripción
/// Limpia la pantalla, imprime el mensaje enmarcado de error
/// y pausa la ejecución.
pub fn view_error_opcion() {
    utils_view::limpiar_pantalla();
    println!("********************************************");
    println!("*         ERROR: OPCION NO VALIDA          *");
    println!("********************************************");
    utils_view::pausar();
}
pub fn error_stock_insuficiente(){
    utils_view::limpiar_pantalla();
    println!("********************************************");
    println!("*         ERROR: STOCK INSUFICIENTE          *");
    println!("********************************************");
    utils_view::pausar();
}
use super::utils::utils_view;

/// Muestra un título centrado dentro de un marco de asteriscos.
/// 
/// # Parámetros
/// - `titulo`: Texto que se desea mostrar centrado.
/// 
/// # Comportamiento
/// 1. Limpia la pantalla llamando a `utils_view::limpiar_pantalla()`.
/// 2. Calcula el espacio disponible dentro de un marco de ancho fijo (`ancho_total = 44`).
/// 3. Centra el `titulo` agregando espacios a la izquierda y derecha.
/// 4. Imprime tres líneas:
///    - Una línea superior de asteriscos.
///    - Una línea con el título centrado.
///    - Una línea inferior de asteriscos.
/// 
pub fn mostrar_titulo(titulo: &str) {
    utils_view::limpiar_pantalla();

    let ancho_total: usize = 44;
    let ancho_interno:usize = ancho_total - 2;

    let padding = ancho_interno.saturating_sub(titulo.len());
    let izquierda = padding / 2;
    let derecha = padding - izquierda;

    println!();
    println!("{}", "*".repeat(ancho_total));
    println!("*{}{}{}*", " ".repeat(izquierda), titulo, " ".repeat(derecha));
    println!("{}", "*".repeat(ancho_total));
    println!();
}

/// Muestra un mensaje simple en consola.
/// 
/// # Parámetros
/// - `mensaje`: Texto que se desea imprimir.
/// 
pub fn mostrar_mensaje(mensaje: &str) {
    println!();
    println!("{}", mensaje);
}
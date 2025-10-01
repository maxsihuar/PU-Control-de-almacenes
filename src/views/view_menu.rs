use super::utils::utils_view;
/// Muestra en pantalla el menú principal del sistema de gestión.
///
/// Este menú permite al usuario navegar por las distintas
/// secciones del programa, como categorías, artículos, proveedores,
/// movimientos de entrada y salida, etc.
///
/// # Opciones
/// 1. Categorías  
/// 2. Artículos  
/// 3. Proveedores  
/// 4. Entradas  
/// 5. Detalles de Entrada  
/// 6. Salidas  
/// 7. Detalles de Salida  
/// 8. Salir

pub fn menu_principal(){

    utils_view::limpiar_pantalla();

    println!("--- Menu Principal ---");
    println!("1. Categorias");
    println!("2. Artículos");
    println!("3. Proveedores");
    println!("4. Entradas");
    println!("5. Detalles de Entrada");
    println!("6. Salidas");
    println!("7. Detalles de Salida");
    println!("0. Salir");
} 
/// Muestra un menú secundario genérico con opciones de gestión.
///
/// Este menú se utiliza para operaciones comunes como
/// **agregar, modificar, eliminar, listar y salir**.  
/// Recibe un texto que se usa para personalizar el título.
///
/// # Parámetros
/// - `texto`: Texto descriptivo que se concatenará al título del menú.
pub fn menu_secundario(texto: String){

    utils_view::limpiar_pantalla();

    let base = "Menu ";
    let texto_final: String = base.to_string() + &texto;
    println!("---{}---",texto_final);
    println!("1. Agregar");
    println!("2. Modificar");
    println!("3. Eliminar");
    println!("4. Listar");
    println!("5. Salir");                                              
}

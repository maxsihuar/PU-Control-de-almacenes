use std::io;
pub fn menu_principal(){
    println!("--- Menu Principal ---");
    println!("1. Categorias");
    println!("2. Artículos");
    println!("3. Proveedores");
    println!("4. Entradas");
    println!("5. Detalles de Entrada");
    println!("6. Salidas");
    println!("7. Detalles de Salida");
    println!("8. Salir");
} 
pub fn menu_secundario(texto: String){
    let base = "Menu ";
    let texto_final: String = base.to_string() + &texto;
    println!("---{}---",texto_final);
    println!("1. Agregar");
    println!("2. Modificar");
    println!("3. Eliminar");
    println!("4. Listar");
    println!("5. Salir");                                              
}

use std::io;

/// Lee una línea desde la entrada estándar y devuelve un `String` sin espacios al inicio ni al final.
///
/// # Retorno
/// Devuelve la cadena ingresada por el usuario, ya recortada (`trim`).
pub fn leer_string() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_) => {},
        Err(e) => println!("Error al leer la entrada: {}", e),
    }
    input.trim().to_string()
}



/// Lee un valor numérico de tipo `u32` desde la entrada estándar.
///
/// La función solicita al usuario un número hasta que ingrese un valor válido.
///
/// # Retorno
/// Devuelve el número ingresado por el usuario como `u32`.
pub fn leer_u32() -> u32 {
    loop {
        let input = leer_string();
        match input.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Por favor, ingrese un número válido."),
        }
    }
}

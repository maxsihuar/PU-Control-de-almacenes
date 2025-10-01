use std::io;


pub fn leer_string() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_) => {},
        Err(e) => println!("Error al leer la entrada: {}", e),
    }
    input.trim().to_string()
}

pub fn leer_u32() -> u32 {
    loop {
        let input = leer_string();
        match input.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Por favor, ingrese un número válido."),
        }
    }
}

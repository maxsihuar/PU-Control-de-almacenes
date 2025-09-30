

pub fn limpiar_pantalla(){
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn pausar(){
    use std::io::{self, Write};
    let mut stdout = io::stdout();
    write!(stdout, "Presiona Enter para continuar...").unwrap();
    stdout.flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}
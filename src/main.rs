pub mod controllers;
pub mod models;
pub mod views;

/// Punto de entrada del programa.
/// 
/// Esta función invoca la función `run` del módulo `controller_main` dentro de `controllers`,
/// que se encarga de iniciar la ejecución de la aplicación y mostrar los menús principales.
fn main() {
    controllers::controller_main::run();
}
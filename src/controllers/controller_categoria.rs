use crate::views::view_menu;
use crate::models::services;

pub fn run_categoria(dc : &HashMap<String, String>) {
    loop{
        view::menu_secundario("Categorias".to_string());
        match opcion{
            1 => services::service_agregar::agregar_categoria(
                //util::utils_data::pedir_datos_categoria(),
                //dc
            ),
        }
    }
}

fn agregar_categoria() {
    let mut categorias = Categorias();
    categorias.insert("NT".to_string(), "Nutricion".to_string());
    return categorias;
}
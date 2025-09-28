use crate::models::obj::Categoria;

fn agregar_categoria(){
    let categoria = Categorias();
    categoria.insert("NT".to_string(), "Nutricion".to_string());
    return categoria;
}
fn 
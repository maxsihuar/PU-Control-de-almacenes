use std::HashMap;

fn Categorias() {
    let mut categorias: HashMap<String, String> = HashMap::from([
        ("EL".to_string(), "Electronica".to_string()),
        ("RP".to_string(), "Ropa".to_string()),
        ("HG".to_string(), "Hogar".to_string()),
        ("JG".to_string(), "Juguetes".to_string()),
        ("DP".to_string(), "Deportes".to_string()),
    ]);
    return categorias;
}



pub fn validación_entrada(){

}

pub fn validar_opcion(){

}

pub fn validar_existencia() -> bool{
    if dc.contains_key(&codigo) {
        return true;
    }else{
        view::view_error::error_existencia();
        return false;
    }

}

pub fn validar_no_existencia(){

}

pub fn validar_RUC(){
    if ruc.len() != 11 {
        view::view_error::error_RUC_tamaño();
        return false;
    }
} 

pub fn validar_existencia(codigo: String,dc: HashMap)->bool{
    for (codigo,(rs,ruc,direccion, ciudad)) in &dc{
        if codigo == &codigo {
            return true;
        }
    }
    return false;
}
pub fn validar_no_existencia(codigo: String, dc: HashMap)->bool{
    for (codigo,(rs,ruc,direccion, ciudad)) in &dc{
        if codigo == &codigo {
            //view::view_error::error_codigo_duplicado();
            return false;
        }
    }
    return true;
}
pub fn validar_RUC(ruc: String,dc: &HashMap)->bool{
    if ruc.len() != 11{
        //view::view_error::error_
        return false
    }
    for (codigo,(rs,ruc_,direccion, ciudad)) in &dc{
        if ruc == *ruc_ {
            //view::view_error::error_ruc_duplicado();
            return false
        }
    }
    return true
}
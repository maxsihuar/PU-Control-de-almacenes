use std:: collections::HashMap;

pub fn validar_existencia_s(codigo: String,dc: &mut HashMap<String,String>)->bool{
    for clave in dc.keys(){
        if &codigo == clave {
            return true;
        }
    }
    return false;
}

pub fn validar_existencia_t1(codigo: String,dc: &mut HashMap<String, (String,String,u32)>)->bool{
    for clave in dc.keys(){
        if &codigo == clave {
            return true;
        }
    }
    return false;
}

pub fn validar_existencia_t2(codigo: String,dc: &mut HashMap<String,(String,String,String,String)>)->bool{
    for clave in dc.keys(){
        if &codigo == clave {
            return true;
        }
    }
    return false;
}

pub fn validar_existencia_t3(codigo: String,dc: &mut HashMap<String,(String,String)>)->bool{
    for clave in dc.keys(){
        if &codigo == clave {
            return true;
        }
    }
    return false;
}

pub fn validar_existencia_h(codigo: String,dc: &mut HashMap<String, HashMap<String,(u32,u32)>>)->bool{
    for clave in dc.keys(){
        if &codigo == clave {
            return true;
        }
    }
    return false;
}



pub fn validar_RUC(ruc: String, dc: &mut HashMap<String, (String, String, String, String)>) -> bool {
    if ruc.len() != 11 {
        return false;
    }

    for (_codigo, (_rs, ruc_, _direccion, _ciudad)) in dc {
        if ruc == *ruc_ {
            return false;
        }
    }

    true
}

/// Valida un número de DNI.
///
/// Reglas:
/// - Debe tener exactamente 8 caracteres.
/// - No debe estar duplicado en la colección.
///
/// # Parámetros
/// - `dni`: DNI a validar.
/// - `dc`: HashMap de registros, donde:
///   - La clave es un `String` (código).
///   - El valor es `(String, String)` donde el primer campo es el DNI.
///
/// # Retorno
/// - `true` si el DNI es válido.
/// - `false` si no cumple las condiciones.
pub fn validar_DNI(dni: String, dc: &HashMap<String, (String, String)>) -> bool {
    if dni.len() != 8 {
        return false;
    }

    for (_codigo, (dni_existente, _fecha)) in dc {
        if dni == *dni_existente {
            return false;
        }
    }

    true
}

pub fn validar_stock(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>,cod_arti:String,cantidad:u32) -> bool{
    for (_codigo,m) in dc{
        for (_cod_art,(cnt,precio)) in m{
            if cod_arti == *_cod_art && cantidad <=*cnt{
                return true
            }
        }
    }
    return false;
}
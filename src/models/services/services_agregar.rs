use std::collections::HashMap;


//Categoria
pub fn agregar_s(dc: &mut HashMap<String, String>, leer : fn(dc: &mut HashMap<String, String>) -> (String,String)){

    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}


//Articulo
pub fn agregar_t1(dc: &mut HashMap<String, (String,String,u32)>, leer : fn(dc: &mut HashMap<String, (String,String,u32)>) -> (String,(String,String,u32))){

    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

//proveedor
pub fn agregar_t2(dc: &mut HashMap<String,(String,String,String,String)>, leer : fn(dc: &mut HashMap<String,(String,String,String,String)>) -> (String,(String,String,String,String))){
    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

//Entrada y Salida
pub fn agregar_t3(dc: &mut HashMap<String,(String,String)>, leer : fn(dc: &mut HashMap<String,(String,String)>) -> (String,(String,String))){
    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);

}

//Detalle Entrada y Detalle Salida

pub fn agregar_h_e(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>, leer : fn(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>) -> (String,HashMap<String,(u32,u32)>)){

    let clave_valor = leer(dc);
    dc.insert(clave_valor.0, clave_valor.1);
}

//Detalle Entrada y Detalle Salida
pub fn agregar_h_s(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>,dc_detalle_e: &mut HashMap<String,HashMap<String,(u32,u32)>>, leer : fn(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>, dc_detalle_e: &mut HashMap<String,HashMap<String,(u32,u32)>>) -> (String,HashMap<String,(u32,u32)>)){

    let clave_valor = leer(dc,dc_detalle_e);
    dc.insert(clave_valor.0, clave_valor.1);
}



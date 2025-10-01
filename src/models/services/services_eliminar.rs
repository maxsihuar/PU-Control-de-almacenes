use std::collections::HashMap;



//Categoria
pub fn eliminar_s(dc: &mut HashMap<String, String>, clave: String){

    dc.remove(&clave);
}


//Articulo
pub fn eliminar_t1(dc: &mut HashMap<String, (String,String,u32)>, clave: String){

    dc.remove(&clave);
}

//proveedor
pub fn eliminar_t2(dc: &mut HashMap<String,(String,String,String,String)>, clave: String){

    dc.remove(&clave);
}

//Entrada y Salida
pub fn eliminar_t3(dc: &mut HashMap<String,(String,String)>, clave: String){

    dc.remove(&clave);
}

//Detalle Entrada y Detalle Salida
pub fn eliminar_h(dc: &mut HashMap<String, HashMap<String,(u32,u32)>>, clave: String){

    dc.remove(&clave);
}


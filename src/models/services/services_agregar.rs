use std::collections::HashMap;

pub fn agregar(datos : fn() -> (String,String) ,dc : &Ha){
    let codigo = datos().0; // util.validarcodigo();
    let nombre = datos().1; // util.validarnombre();

    dc.insert(codigo, nombre);
}

use std::collections::HashMap;

pub fn agregar(datos : fn() -> (String,String) ,dc : &Ha){

    loop{
        let codigo = datos().0; // util.validarcodigo();
        let nombre = datos().1; // util.validarnombre();
        if !validar_existencia(dc, &codigo) {
            break;
        }
    }

    dc.insert(codigo, nombre);
}

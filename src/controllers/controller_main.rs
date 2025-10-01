use crate::views::view_menu;
use crate::controllers::controller_error;
use crate::models::obj;

pub fn run(){

    let mut dc_categorias = obj::obj_categoria::crear_categorias();
    let mut dc_articulos = obj::obj_articulo::crear_articulo();

    let mut dc_cliente = obj::obj_cliente::crear_cliente();
    let mut dc_proveedor = obj::obj_proveedor::crear_proveedor();

    let mut dc_salida = obj::obj_salida::crear_salida();
    let mut dc_entrada = obj::obj_entrada::crear_entrada();

    let mut dc_detalle_salida = obj::obj_detalle_salida::crear_detalle_salida();
    let mut dc_detalle_entrada = obj::obj_detalle_entrada::crear_detalle_entrada();

    loop{
        view_menu::menu_principal();


        
        match opcion{
            1 => controller_categorias::run(),
            2 => controller_articulos::run(),
            3 => controller_cliente::run(),
            4 => controller_proveedor::run(),
            5 => controller_salida::run(),
            6 => controller_entrada::run(),
            7 => controller_detalle_salida::run(),
            8 => controller_detalle_entrada::run(),
            9 => controller_reportes::run(),
            0 => break,
            _ => controller_error::opcion_no_valida(),
        }
    } 
}
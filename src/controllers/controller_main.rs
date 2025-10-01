use crate::views::view_menu;
use crate::views::view_leer;
use crate::models::utils;
use crate::controllers::controller_error;
use crate::controllers;
use crate::models::obj;

use crate::tests::test1;

pub fn run(){

    let mut dc_categorias = obj::obj_categoria::crear_categorias();
    let mut dc_articulos = obj::obj_articulo::crear_articulo();

    let mut dc_cliente = obj::obj_cliente::crear_cliente();
    let mut dc_proveedor = obj::obj_proveedor::crear_proveedor();

    let mut dc_salida = obj::obj_salida::crear_salida();
    let mut dc_entrada = obj::obj_entrada::crear_entrada();

    let mut dc_detalle_salida = obj::obj_detalle_salida::crear_detalle_salida();
    let mut dc_detalle_entrada = obj::obj_detalle_entrada::crear_detalle_entrada();

    //test
    dc_categorias = test1::test_categoria();
    dc_articulos = test1::test_articulo();
    dc_proveedor = test1::proveedor();
    dc_entrada = test1::test_entrada();
    dc_detalle_entrada = test1::test_entrada_detalle();
    dc_salida = test1::test_salida();
    dc_detalle_salida = test1::test_salida_detalle();

    loop{
        view_menu::menu_principal();

        view_leer::mostrar_mensaje("Ingrese una opcion:");
        let opcion = utils::utils_leer::leer_u32();
        
        match opcion{
            1 => controllers::controller_categoria::run_categoria(&mut dc_categorias),
            2 => controllers::controller_articulos::run_articulo(&mut dc_articulos),
            3 => controllers::controller_proveedores::run_proveedores(&mut dc_proveedor),
            4 => controllers::controller_entrada::run_entrada(&mut dc_entrada),
            5 => controllers::controller_detalle_entrada::run_detalle_entrada(&mut dc_detalle_entrada),
            6 => controllers::controller_detalle_salida::run_detalle_salida(&mut dc_detalle_salida),
            7 => controllers::controller_salida::run_salida(&mut dc_salida),
            8 => controllers::controller_reporte::run_reportes(&mut dc_categorias, &mut dc_articulos, &mut dc_proveedor, &mut dc_entrada,&mut dc_detalle_entrada,&mut  dc_salida,&mut dc_detalle_salida),
            0 => break,
            _ => controller_error::opcion_no_valida(),
        }
    } 
}
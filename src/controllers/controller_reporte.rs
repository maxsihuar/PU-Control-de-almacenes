use crate::views::view_menu;
use crate::views::view_leer;
use crate::views::view_error;
use crate::views::view_listar;
use crate::models::utils;
use crate::models::services;
use std::collections::HashMap;

pub fn run_reportes(
    d_categorias: &mut HashMap<String, String>,
    d_articulos: &mut HashMap<String, (String, String, u32)>,
    d_proveedores: &mut HashMap<String, (String, String,String,String)>,
    d_entrada: &mut HashMap<String, (String, String)>,
    d_entrada_detalle: &mut HashMap<String, HashMap<String, (u32, u32)>>,
    d_salida: &mut HashMap<String, (String, String)>,
    d_salida_detalle: &mut HashMap<String, HashMap<String, (u32, u32)>>,
) {
    loop{
        view_menu::menu_secundario("Detalles de Salida".to_string());
        view_leer::mostrar_mensaje("Ingrese una opcion:");
        let opcion = utils::utils_leer::leer_u32();
        match opcion{
            1 => view_listar::mostrar_listado(services::services_reportes::listar_articulos_por_categoria(d_categorias, d_articulos, vec!["Codigo Categoria", "Nombre Categoria", "Codigo Articulo", "Nombre Articulo", "Stock"])),
            2 => view_listar::mostrar_listado(services::services_reportes::listar_documentos_fechas(d_entrada, d_salida, d_proveedores, vec!["Documento", "Fecha", "Proveedor"],vec!["Documento", "Fecha", "Cliente"])),
            3 => view_listar::mostrar_listado(services::services_reportes::listar_articulos_proveedor(d_proveedores, d_entrada, d_entrada_detalle, d_articulos)),
            4 => view_listar::mostrar_listado(services::services_reportes::listar_stock_actual(d_entrada_detalle, d_salida_detalle, d_articulos)),
            5 => view_listar::mostrar_listado(services::services_reportes::listar_historial_articulos(d_articulos, d_entrada, d_entrada_detalle, d_salida, d_salida_detalle)),
            6 => view_listar::mostrar_listado(services::services_reportes::listar_valor_total(d_entrada_detalle, d_salida_detalle, d_articulos)),
            0 => break,
            _ => println!("Opcion no valida"
            ),
        }
    }
}
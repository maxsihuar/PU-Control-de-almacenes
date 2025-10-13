use std::collections::HashMap;


pub fn test_categoria() -> HashMap<String, String>{
    let mut d_categorias:HashMap<String, String> = HashMap::new();

    d_categorias.insert("LB".to_string(), "Línea Blanca".to_string());
    d_categorias.insert("VI".to_string(), "Video".to_string());
    d_categorias.insert("TEC".to_string(), "Equipos Tecnológicos".to_string());
    d_categorias.insert("AU".to_string(), "Audio".to_string());
    d_categorias.insert("MO".to_string(), "Muebles de Oficina".to_string());
    d_categorias.insert("CL".to_string(), "Climatización".to_string());
    d_categorias.insert("EL".to_string(), "Electrodomésticos pequeños".to_string());

    return d_categorias;

}

pub fn test_articulo() -> HashMap<String, (String, String, u32)>{
    let mut d_articulos: HashMap<String, (String, String, u32)> = HashMap::new();

    d_articulos.insert("L025".to_string(), ("Laptop HP 215".to_string(), "TEC".to_string(), 2500));
    d_articulos.insert("I010".to_string(), ("Impresora HP".to_string(), "TEC".to_string(), 1200));
    d_articulos.insert("S020".to_string(), ("Smartphone Samsung A54".to_string(), "TEC".to_string(), 1800));
    d_articulos.insert("PR01".to_string(), ("Proyector Epson XGA".to_string(), "TEC".to_string(), 2800));
    d_articulos.insert("C100".to_string(), ("Cámara Canon EOS".to_string(), "TEC".to_string(), 3000));

    d_articulos.insert("R045".to_string(), ("Refrigeradora LG".to_string(), "LB".to_string(), 4000));
    d_articulos.insert("M045".to_string(), ("Microondas Panasonic".to_string(), "LB".to_string(), 650));
    d_articulos.insert("L080".to_string(), ("Lavadora Samsung".to_string(), "LB".to_string(), 3500));
    d_articulos.insert("H060".to_string(), ("Horno Eléctrico Oster".to_string(), "LB".to_string(), 800));
    d_articulos.insert("D030".to_string(), ("Dishwasher LG".to_string(), "LB".to_string(), 2500));

    d_articulos.insert("TV32".to_string(), ("Televisor JVC 32".to_string(), "VI".to_string(), 700));
    d_articulos.insert("TV50".to_string(), ("Televisor LG 50".to_string(), "VI".to_string(), 2500));
    d_articulos.insert("TV75".to_string(), ("Televisor Samsung 75".to_string(), "VI".to_string(), 4800));
    d_articulos.insert("BD01".to_string(), ("Blu-ray Sony".to_string(), "VI".to_string(), 350));
    d_articulos.insert("DV10".to_string(), ("DVD Player Philips".to_string(), "VI".to_string(), 300));

    d_articulos.insert("A015".to_string(), ("Parlante Sony".to_string(), "AU".to_string(), 950));
    d_articulos.insert("H020".to_string(), ("Headphones Bose".to_string(), "AU".to_string(), 1200));
    d_articulos.insert("SP30".to_string(), ("Subwoofer JBL".to_string(), "AU".to_string(), 1800));
    d_articulos.insert("MC12".to_string(), ("Micrófono Rode".to_string(), "AU".to_string(), 650));
    d_articulos.insert("RG07".to_string(), ("Radio Panasonic".to_string(), "AU".to_string(), 400));

    d_articulos.insert("E099".to_string(), ("Escritorio Gerencial".to_string(), "MO".to_string(), 2200));
    d_articulos.insert("S110".to_string(), ("Silla Oficina Ergonomica".to_string(), "MO".to_string(), 950));
    d_articulos.insert("CA01".to_string(), ("Cajonera Oficina".to_string(), "MO".to_string(), 850));
    d_articulos.insert("ME22".to_string(), ("Mesa de Centro".to_string(), "MO".to_string(), 1500));
    d_articulos.insert("LB33".to_string(), ("Lámpara de Pie".to_string(), "MO".to_string(), 300));

    d_articulos.insert("AC12".to_string(), ("Aire Acondicionado Daikin 12BTU".to_string(), "CL".to_string(), 3000));
    d_articulos.insert("VC05".to_string(), ("Ventilador Oster".to_string(), "CL".to_string(), 350));
    d_articulos.insert("HC15".to_string(), ("Humidificador Philips".to_string(), "CL".to_string(), 700));
    d_articulos.insert("DE40".to_string(), ("Deshumidificador LG".to_string(), "CL".to_string(), 1200));
    d_articulos.insert("AC24".to_string(), ("Aire Acondicionado Samsung 24BTU".to_string(), "CL".to_string(), 4500));

    d_articulos.insert("BL01".to_string(), ("Batidora Oster".to_string(), "EL".to_string(), 400));
    d_articulos.insert("OL10".to_string(), ("Olla Arrocera Imaco".to_string(), "EL".to_string(), 300));
    d_articulos.insert("MJ22".to_string(), ("Mezcladora KitchenAid".to_string(), "EL".to_string(), 1800));
    d_articulos.insert("TO09".to_string(), ("Tostadora Philips".to_string(), "EL".to_string(), 500));
    d_articulos.insert("CF03".to_string(), ("Cafetera Nespresso".to_string(), "EL".to_string(), 1500));

    return d_articulos;
}

pub fn proveedor() -> HashMap<String, (String, String, String, String)>{
    let mut d_proveedores: HashMap<String, (String, String, String, String)> = HashMap::new();
    d_proveedores.insert("P01".to_string(), ("Proveedor A".to_string(), "20141312171".to_string(), "Av. Perú 120".to_string(), "Lima".to_string()));
    d_proveedores.insert("P02".to_string(), ("Proveedor B".to_string(), "20451111233".to_string(), "Av. Brasil 455".to_string(), "Arequipa".to_string()));
    d_proveedores.insert("P03".to_string(), ("Proveedor C".to_string(), "20561231289".to_string(), "Av. Javier Prado 999".to_string(), "Cusco".to_string()));
    d_proveedores.insert("P04".to_string(), ("Proveedor D".to_string(), "20678945612".to_string(), "Av. Larco 1500".to_string(), "Trujillo".to_string()));
    d_proveedores.insert("P05".to_string(), ("Proveedor E".to_string(), "20789045123".to_string(), "Av. Grau 700".to_string(), "Chiclayo".to_string()));
    d_proveedores.insert("P06".to_string(), ("Proveedor F".to_string(), "20890123456".to_string(), "Av. Bolognesi 456".to_string(), "Tacna".to_string()));

    return d_proveedores;
}

pub fn test_entrada_detalle() -> HashMap<String, HashMap<String, (u32, u32)>>{
    let mut d_entrada_detalle: HashMap<String, HashMap<String, (u32, u32)>> = HashMap::new();
    d_entrada_detalle.insert("PI-12345".to_string(), HashMap::from([
        ("L025".to_string(), (4, 3000)),
        ("I010".to_string(), (3, 1200))
    ]));
    d_entrada_detalle.insert("PI-12346".to_string(), HashMap::from([
        ("R045".to_string(), (1, 4000)),
        ("A015".to_string(), (5, 1000))
    ]));
    d_entrada_detalle.insert("PI-12347".to_string(), HashMap::from([
        ("S020".to_string(), (4, 1850)),
        ("E099".to_string(), (2, 2300)),
        ("M045".to_string(), (3, 700))
    ]));
    d_entrada_detalle.insert("PI-12348".to_string(), HashMap::from([
        ("TV50".to_string(), (3, 2600)),
        ("VC05".to_string(), (10, 370))
    ]));
    d_entrada_detalle.insert("PI-12349".to_string(), HashMap::from([
        ("AC12".to_string(), (2, 3000)),
        ("BL01".to_string(), (6, 420))
    ]));
    d_entrada_detalle.insert("PI-12350".to_string(), HashMap::from([
        ("OL10".to_string(), (8, 310)),
        ("PR01".to_string(), (2, 2900))
    ]));
    //


    d_entrada_detalle.insert("PI-12351".to_string(), HashMap::from([
        ("AC12".to_string(), (1, 3000)),
        ("I010".to_string(), (3, 1200)),
        ("C100".to_string(), (2, 3000))  // Cámara Canon EOS (nuevo)
    ]));

    d_entrada_detalle.insert("PI-12352".to_string(), HashMap::from([
        ("VC05".to_string(), (1, 4300)),
        ("A015".to_string(), (3, 1000)),
        ("L080".to_string(), (1, 3500))  // Lavadora Samsung (nuevo)
    ]));

    d_entrada_detalle.insert("PI-12353".to_string(), HashMap::from([
        ("S020".to_string(), (2, 1850)),
        ("A015".to_string(), (2, 2300)),
        ("M045".to_string(), (3, 700)),
        ("TV75".to_string(), (1, 4800))  // Televisor Samsung 75 (nuevo)
    ]));

    d_entrada_detalle.insert("PI-12354".to_string(), HashMap::from([
        ("TV32".to_string(), (3, 700)),
        ("VC05".to_string(), (3, 370)),
        ("H020".to_string(), (2, 1200))  // Headphones Bose (nuevo)
    ]));

    d_entrada_detalle.insert("PI-12355".to_string(), HashMap::from([
        ("AC12".to_string(), (2, 3000)),
        ("BL01".to_string(), (2, 420)),
        ("MJ22".to_string(), (1, 1800))  // Mezcladora KitchenAid (nuevo)
    ]));

    d_entrada_detalle.insert("PI-12356".to_string(), HashMap::from([
        ("AC12".to_string(), (1, 3000)),
        ("PR01".to_string(), (10, 2900))
    ]));

    return d_entrada_detalle;
}

pub fn test_entrada() -> HashMap<String, (String, String)>{
    let mut d_entrada: HashMap<String, (String, String)> = HashMap::new();
    d_entrada.insert("PI-12345".to_string(), ("02/02/2025".to_string(), "P01".to_string()));
    d_entrada.insert("PI-12346".to_string(), ("03/02/2025".to_string(), "P02".to_string()));
    d_entrada.insert("PI-12347".to_string(), ("05/02/2025".to_string(), "P03".to_string()));
    d_entrada.insert("PI-12348".to_string(), ("07/02/2025".to_string(), "P04".to_string()));
    d_entrada.insert("PI-12349".to_string(), ("10/02/2025".to_string(), "P05".to_string()));
    d_entrada.insert("PI-12350".to_string(), ("11/02/2025".to_string(), "P06".to_string()));
   
    //
    d_entrada.insert("PI-12351".to_string(), ("02/03/2025".to_string(), "P02".to_string()));
    d_entrada.insert("PI-12352".to_string(), ("04/04/2025".to_string(), "P01".to_string()));
    d_entrada.insert("PI-12353".to_string(), ("06/05/2025".to_string(), "P06".to_string()));
    d_entrada.insert("PI-12354".to_string(), ("08/05/2025".to_string(), "P03".to_string()));
    d_entrada.insert("PI-12355".to_string(), ("12/02/2025".to_string(), "P05".to_string()));
    d_entrada.insert("PI-12356".to_string(), ("13/01/2025".to_string(), "P04".to_string()));
    return d_entrada;
}

pub fn test_salida() -> HashMap<String, (String, String)>{
    let mut d_salida: HashMap<String, (String, String)> = HashMap::new();
    d_salida.insert("NS-45678".to_string(), ("03/02/2025".to_string(), "Ana Paz".to_string()));
    d_salida.insert("NS-45679".to_string(), ("04/01/2025".to_string(), "Carlos Ramos".to_string()));
    d_salida.insert("NS-45680".to_string(), ("06/08/2025".to_string(), "María Torres".to_string()));
    d_salida.insert("NS-45681".to_string(), ("08/07/2025".to_string(), "Luis Fernández".to_string()));
    d_salida.insert("NS-45682".to_string(), ("10/06/2025".to_string(), "Claudia Rojas".to_string()));
    d_salida.insert("NS-45683".to_string(), ("12/03/2025".to_string(), "Pedro Sánchez".to_string()));
    //
    d_salida.insert("NS-45684".to_string(), ("04/05/2025".to_string(), "Carlos Quispe".to_string()));
    d_salida.insert("NS-45685".to_string(), ("06/02/2025".to_string(), "Yeyson Huarhua".to_string()));

    return d_salida;
}

pub fn test_salida_detalle() -> HashMap<String, HashMap<String, (u32, u32)>>{
        let mut d_salida_detalle: HashMap<String, HashMap<String, (u32, u32)>> = HashMap::new();
    d_salida_detalle.insert("NS-45678".to_string(), HashMap::from([
        ("L025".to_string(), (1, 3200)),
        ("TV32".to_string(), (2, 700)),
        ("S020".to_string(), (2, 1800))
    ]));
    d_salida_detalle.insert("NS-45679".to_string(), HashMap::from([
        ("TV50".to_string(), (1, 2700)),
        ("M045".to_string(), (1, 800)),
        ("OL10".to_string(), (2, 300))
    ]));
    d_salida_detalle.insert("NS-45680".to_string(), HashMap::from([
        ("S020".to_string(), (2, 2000)),
        ("A015".to_string(), (1, 1100)),
        ("E099".to_string(), (1, 2500))
    ]));
    d_salida_detalle.insert("NS-45681".to_string(), HashMap::from([
        ("VC05".to_string(), (2, 400)),
        ("R045".to_string(), (1, 4000)),
        ("PR01".to_string(), (4, 2800))
    ]));
    d_salida_detalle.insert("NS-45682".to_string(), HashMap::from([
        ("AC12".to_string(), (1, 3000)),
        ("OL10".to_string(), (2, 350)),
        ("L025".to_string(), (1, 3200)),
        ("TV75".to_string(), (1, 4800)),  // Televisor Samsung 75
    ]));

    d_salida_detalle.insert("NS-45683".to_string(), HashMap::from([
        ("PR01".to_string(), (1, 3100)),
        ("BL01".to_string(), (1, 450)),
        ("I010".to_string(), (2, 1200)),
        ("MJ22".to_string(), (1, 1800)),  // Mezcladora KitchenAid
    ]));

    d_salida_detalle.insert("NS-45684".to_string(), HashMap::from([
        ("AC12".to_string(), (4, 3000)),
        ("OL10".to_string(), (2, 350)),
        ("L025".to_string(), (1, 3200)),
        ("H020".to_string(), (2, 1200)),  // Headphones Bose
    ]));

    d_salida_detalle.insert("NS-45685".to_string(), HashMap::from([
        ("PR01".to_string(), (1, 3100)),
        ("BL01".to_string(), (1, 450)),
        ("VC05".to_string(), (1, 400)),
        ("C100".to_string(), (1, 3000)),  // Cámara Canon EOS
    ]));

    return d_salida_detalle
}

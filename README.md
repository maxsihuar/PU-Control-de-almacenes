# PU‑Control-de-almacenes

Proyecto de control de almacenes desarrollado en **Rust**  
Unidad 1, dirigido por “papi Carrasco”

---

## Descripción

Este proyecto es una aplicación para gestionar el control de almacenes (entradas, salidas, inventario, etc.) escrita en Rust. Permite llevar un registro básico de las operaciones de almacén con un sistema simple pero funcional, ideal para propósitos educativos o de prototipo.

---

## Características

- Registro de entradas de mercancía al almacén.  
- Registro de salidas de mercancía.  
- Mantenimiento de inventario.  
- Interfaz en consola (línea de comandos).  
- Estructura modular en Rust.

---

## Tecnologías

- **Lenguaje:** Rust  
- **Gestión de dependencias:** Cargo (`Cargo.toml`)  
- **Licencia:** GPL‑3.0

---

## Instalación

1. Clona el repositorio:  
   ```bash
   git clone https://github.com/maxsihuar/PU-Control-de-almacenes.git
2. Compila el proyecto:
   cargo build --release
3. Ejecuta la aplicación:
   cargo run --release
   
## Uso

Al correr la aplicación, se mostrarán opciones para:

- **Registrar una entrada:** añadir artículo, cantidad, precio, fecha, etc.
- **Registrar una salida:** seleccionar artículo, cantidad, fecha, etc.
- **Ver inventario actual:** consultar existencias de cada artículo.
- **Generar reportes simples** (según lo implementado).

- 
## Estructura del repositorio

PU‑Control-de-almacenes/
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
├── .gitignore
├── FETCH_HEAD
├── target/
└── src/
├── main.rs
├── controllers/
├── models/
├── tests/
│ ├── mod.rs
│ └── test1.rs
└── views/
├── mod.rs
├── utils/
├── view_error.rs
├── view_leer.rs
├── view_listar.rs
└── view_menu.rs

## Contacto

### Autores: maxsihuar
###          Ocramgit0

### Repositorio: maxsihuar/PU-Control-de-almacenes

# Si tienes preguntas o sugerencias: abre un issue en el repo.

# rust, pdf and python (django)
Crear un acceso a la BD de postgres para generar un pdf que se instancia desde microservicio de python (django)

El pdf se crea en el microservicio en rust

## crear un proyecto en rust
$ cargo new msrust --bin

## compilar y correr rust
$ cargo run

## documentacion postgres 0.19.2
https://docs.rs/crate/postgres/0.19.2
https://docs.rs/postgres/0.19.2/postgres/config/struct.Config.html


## create forlder assets/fonts

## documentacion printpdf 0.3.2
https://docs.rs/printpdf/0.4.1/printpdf/types/pdf_layer/struct.PdfLayerReference.html#implementations
https://github.com/fschutt/printpdf
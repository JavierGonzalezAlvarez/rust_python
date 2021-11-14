//require tthe module
use postgres::{Client, NoTls};

struct Cliente {
    id: i32,
    name: String,
    surname: String,   
    //img: String,
}

fn main() {

    let database_url = "host=localhost user=javier password=2525_ap dbname=rustpython";
    let mut client = Client::connect(database_url, NoTls).unwrap();    
           
    for row in client.query("SELECT id, name, surname FROM pdf_cliente", &[]).unwrap() {
        let client = Cliente {
            id: row.get(0),
            name: row.get(1),
            surname: row.get(2),
            //img: row.get(3),
        };
        println!("Cliente => id: {}, nombre: {}, apellido: {}", client.id, client.name, client.surname);
        //println!("Cliente => id: {}, nombre: {}, apellido: {}, img: {}", client.id, client.name, client.surname, client.img);
    }
    
}

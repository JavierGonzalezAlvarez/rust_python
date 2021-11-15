//mod handlers;

//import file pdf.rs (module)
mod pdf;

//require the module. before adding it to cargo.toml
use postgres::{Client, NoTls};

struct Cliente {
    id: i32,
    name: String,
    surname: String,   
    img: String,
}

fn main() {

    let database_url = "host=localhost user=javier password=2525_ap dbname=rustpython";
    let mut client = Client::connect(database_url, NoTls).unwrap();    
           
    for row in client.query("SELECT id, name, surname, img FROM pdf_cliente", &[]).unwrap() {
        let client = Cliente {
            id: row.get(0),
            name: row.get(1),
            surname: row.get(2),
            img: row.get(3),
        };        

        //println!("Cliente => id: {}, nombre: {}, apellido: {}, img: {}", client.id, client.name, client.surname, client.img);
        println!("Client");
        println!("--------");
        println!("id: {}", client.id);
        println!("Name: {}", client.name);
        println!("Surname: {}", client.surname);
        println!("Path img: {}", client.img);
        println!("--------");    
        
        //from the module pdf.rs, import get_pdf funtion
        use pdf::get_pdf;   
        //let name = String::from("test");
        let id = client.id;
        let name = client.name;
        let surname = client.surname;
        get_pdf(&id, &name, &surname);

    }
}

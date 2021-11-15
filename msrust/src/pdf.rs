use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn get_pdf(
    id: &i32,
    name: &String,
    surname: &String
) {
    //title: PDF_Rust"    
    //size A4: Mm(210.0), Mm(297.0)    
    //set header: page 1, layer 1
    let (doc, page1, layer1) = PdfDocument::new("PDF_Rust", Mm(210.0), Mm(297.0), "Page 1 Layer 1");
    
    //layer 1
    let current_layer = doc.get_page(page1).get_layer(layer1);    

    //parse to string => fn use_text only support string
    let id = id.to_string();
    //string
    let name = name;
    let surname = surname;
    
    //indicate src/
    let font = doc.add_external_font(File::open("src/assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();    
    
    //section
    current_layer.begin_text_section();        
        //text, font size, x from left edge, y from bottom edge, font    
        current_layer.use_text("Client", 14.0, Mm(10.0), Mm(290.0), &font);            
        current_layer.use_text("--------", 12.0, Mm(10.0), Mm(285.0), &font);            
        current_layer.use_text(id, 12.0, Mm(10.0), Mm(280.0), &font);
        current_layer.use_text(name, 12.0, Mm(10.0), Mm(275.0), &font);
        current_layer.use_text(surname, 12.0, Mm(10.0), Mm(270.0), &font);    
        current_layer.use_text("--------", 12.0, Mm(10.0), Mm(265.0), &font);     
        current_layer.add_line_break();
    current_layer.end_text_section();
    
    //save
    doc.save(&mut BufWriter::new(File::create("test.pdf").unwrap())).unwrap();
}
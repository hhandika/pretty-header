mod handler;

use handler::PrettyHeader;

fn main() {
    let title = "PROCESSING LSUMZ1234";
    let title_2 = "PROCESS";
    let title_3 = "PROCESSORY";
    let length = 98;
    let sym = '=';
    let mut header = PrettyHeader::new(title, sym, length);
    header.print_header().unwrap();

    let mut header = PrettyHeader::new(title_2, sym, length);
    header.print_header().unwrap();

    let mut header = PrettyHeader::new(title_3, sym, length);
    header.print_header().unwrap();

    println!("HELLO WORLD");
}




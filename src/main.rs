use std::io::{self, Result, Write};

fn main() {
    let title = "PROCESSING LSUMZ1234";
    let title_2 = "PROCESS";
    let title_3 = "PROCESSORY";
    let length = 100;
    let sym = '=';
    let mut header = PrettyHeader::new(title, sym, length);
    header.print_header().unwrap();

    let mut header = PrettyHeader::new(title_2, sym, length);
    header.print_header().unwrap();

    let mut header = PrettyHeader::new(title_3, sym, length);
    header.print_header().unwrap();
}

struct PrettyHeader {
    text: String,
    sym: char,
    len: usize,
    text_len: usize,
    sym_len: usize,
}

impl PrettyHeader {
    pub fn new(text: &str, sym: char, len: usize) -> Self {
        Self {
            text: String::from(text),
            sym, 
            len,
            text_len: 0,
            sym_len: 0, 
        }
    }

    fn print_header(&mut self) -> Result<()> {
        self.get_len();
        let io = io::stdout();
        let mut handle = io::BufWriter::new(io);

        if self.text_len > self.len {
            writeln!(handle, "{}", self.text)?;
            Ok(())
        } else {
            self.print_with_symbol(&mut handle)?;
            Ok(())
        }
    }

    fn print_with_symbol<W: Write>(&mut self, handle: &mut W) -> Result<()> {
        self.print_symbols(handle);
        write!(handle, " {} ", self.text)?;
        self.print_symbols(handle);

        if self.text_len % 2 != 0 {
            write!(handle,"{}", self.sym)?;
        }

        writeln!(handle)?;
        Ok(())
    }

    fn get_len(&mut self) {
        self.text_len = self.text.len();
        self.sym_len = (self.len - self.text_len) / 2;
    }

    fn print_symbols<W: Write>(&self, io: &mut W) {
        (0..=self.sym_len).for_each(|_| {
            write!(io, "{}", self.sym).unwrap();
        });
    }
}


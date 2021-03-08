use std::io::{self, Result, Write};

pub struct PrettyHeader {
    pub text: String,
    pub sym: char,
    pub len: usize,
    text_len: usize,
    sym_len: usize,
    color: String,
}

impl PrettyHeader {
    pub fn new(text: &str, sym: char, len: usize) -> Self {
        Self {
            text: String::from(text),
            sym, 
            len,
            text_len: 0,
            sym_len: 0,
            color: String::from("\x1b[0;33m"), 
        }
    }

    pub fn print_header(&mut self) -> Result<()> {
        self.get_len();
        let io = io::stdout();
        let mut handle = io::BufWriter::new(io);
        write!(handle,"{}", self.color)?;
        
        if self.text_len > self.len {
            writeln!(handle, "{}", self.text)?;
        } else {
            self.print_with_symbol(&mut handle)?;
        }
        write!(handle,"\x1b[0m")?;
        Ok(())
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
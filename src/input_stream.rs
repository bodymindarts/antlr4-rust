use super::tokens::TOKEN_EOF;

pub struct InputStream<'a>{
    name: String,
    index: i32,
    size: i32,
    data: &'a [u8]
}

impl<'a> InputStream<'a> {
    pub fn new(s: &str) -> InputStream {
        InputStream {
            name: "<empty>".to_string(),
            index: 0,
            size: s.len() as i32,
            data: s.as_bytes(),
        }
    }

    pub fn la(&self, mut offset: i32) -> i32 {

        if offset == 0 {
            return 0 // nil
        }

        if offset < 0 {
            offset = offset + 1 // e.g., translate LA(-1) to use offset=0
        }
        let pos = self.index + offset - 1;

        if pos < 0 || pos >= self.size { // invalid
            return TOKEN_EOF
        }

        self.data[pos as usize] as i32
    }
}


use super::tokens::{Token,TOKEN_EOF};
use std::str;

fn intMin(a: i32, b: i32) -> i32 {
    if a < b {
        return a
    }
    else {
        return b
    }
}

pub struct InputStream<'a>{
    name: &'a str,
    index: i32,
    size: i32,
    data: &'a [u8]
}

impl<'a> InputStream<'a> {
    pub fn new(s: &str) -> InputStream {
        InputStream {
            name: "<empty>",
            index: 0,
            size: s.len() as i32,
            data: s.as_bytes(),
        }
    }

    pub fn consume(&mut self) {
        if self.index >= self.size {
            panic!("cannot consume EOF")
        }

        self.index += 1
    }

    pub fn la(&self, mut offset: i32) -> i32 {

        if offset == 0 {
            return 0 // nil
        }

        if offset < 0 {
            offset += 1 // e.g., translate LA(-1) to use offset=0
        }
        let pos = self.index + offset - 1;

        if pos < 0 || pos >= self.size { // invalid
            return TOKEN_EOF
        }

        self.data[pos as usize] as i32
    }

    pub fn lt(&self, offset: i32) -> i32 {
        self.la(offset)
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn size(&self) -> i32 {
        return self.size
    }

    // // mark/release do nothing we have entire buffer
    pub fn mark(&self) -> i32 {
        -1
    }

    pub fn release(&self, marker: i32) {
    }

    pub fn seek(&mut self, index: i32) {
        if index <= self.index {
            self.index = index; // just jump don't update stream state (line,...)
            return
        }
        // seek forward
        self.index = intMin(index, self.size)
    }

    pub fn get_text(&self, start: i32, mut stop: i32) -> &str {
        if stop >= self.size {
            stop = self.size - 1
        }
        if start >= self.size {
            return ""
        }

        let section = &self.data[start as usize .. (stop+1) as usize];
        unsafe {
            str::from_utf8_unchecked(section)
        }
    }

    // pub fn get_text_from_tokens(&self, start: Token, stop: Token) -> &str {
    //     if !start.is_null() && !stop.is_null() {
    //         return self.GetTextFromi32(NewInterval(start.GetTokenIndex(), stop.GetTokenIndex()))
    //     }

    //     return ""
    // }

    // pub fn get_text_from_interval(&self, i *i32) string {
    //     return self.GetText(i.start, i.stop)
    // }

    // pub fn (*InputStream) GetSourceName() string {
    //     return "Obtained from string"
    // }

    // pub fn (is *InputStream) String() string {
    //     return string(self.data)
    // }
}


use interval_set::Interval;
use int_stream::IntStream;
use char_stream::CharStream;
use std::str;
use std::cmp;

pub struct InputStream<'a> {
    name: &'a str,
    index: i32,
    size: i32,
    data: &'a [u8],
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

    fn lt(&self, offset: i32) -> i32 {
        self.la(offset)
    }

    // fn get_text_from_tokens(&self, start: &Token, stop: &Token) -> &str {
    //     return self.get_text_from_interval(Interval::new(start.get_token_index(), stop.get_token_index()))
    // }

    // fn get_text_from_interval(&self, i: Interval) -> &str {
    //     return self.get_text(i.start, i.stop)
    // }


    // pub fn (is *InputStream) String() string {
    //     return string(self.data)
    // }
}

impl<'a> IntStream for InputStream<'a> {
    fn consume(&mut self) {
        if self.index >= self.size {
            panic!("cannot consume EOF")
        }

        self.index += 1
    }

    fn la(&self, offset: i32) -> i32 {
        if offset == 0 {
            return 0; // nil
        }

        let i = if offset < 0 { offset + 1 } else { offset };// e.g., translate LA(-1) to use offset=0
        let pos = self.index + i - 1;

        if pos < 0 || pos >= self.size {
            // invalid
            return super::int_stream::EOF;
        }

        self.data[pos as usize] as i32
    }

    fn index(&self) -> i32 {
        self.index
    }

    fn size(&self) -> i32 {
        return self.size;
    }

    // // mark/release do nothing we have entire buffer
    fn mark(&self) -> i32 {
        -1
    }

    fn release(&self, marker: i32) {}

    fn seek(&mut self, index: i32) {
        if index <= self.index {
            self.index = index; // just jump don't update stream state (line,...)
            return;
        }
        // seek forward
        self.index = cmp::min(index, self.size)
    }

    fn get_source_name(&self) -> &str {
        return "Obtained from string";
    }
}

impl<'a> CharStream for InputStream<'a> {
    fn get_text(&self, interval: Interval) -> &str {
        let start = interval.start;
        if start >= self.size {
            return "";
        }
        let stop = if interval.stop >= self.size {
            self.size - 1
        } else {
            interval.stop
        };

        let section = &self.data[start as usize..(stop + 1) as usize];
        unsafe { str::from_utf8_unchecked(section) }
    }
}

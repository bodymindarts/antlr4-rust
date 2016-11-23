pub const EOF: i32 = -1;

pub trait IntStream {
    fn consume(&mut self);
    fn la(&self, i32) -> i32;
    fn mark(&self) -> i32;
    fn release(&self, marker: i32);
    fn index(&self) -> i32;
    fn seek(&mut self, index: i32);
    fn size(&self) -> i32;
    fn get_source_name(&self) -> &str;
}

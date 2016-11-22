use super::int_stream::IntStream;
use super::interval_set::Interval;

pub trait CharStream<'a>: IntStream<'a> {
    fn get_text(&self, Interval) -> &str;
}

use super::int_stream::IntStream;
use super::interval_set::Interval;

pub trait CharStream: IntStream {
    fn get_text(&self, Interval) -> &str;
}

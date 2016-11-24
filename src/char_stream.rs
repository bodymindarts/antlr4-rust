use int_stream::IntStream;
use interval_set::Interval;

pub trait CharStream: IntStream {
    fn get_text(&self, Interval) -> &str;
}

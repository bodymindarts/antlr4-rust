pub struct Interval {
    pub start: i32,
    pub stop: i32,
}

impl Interval {
    pub fn new(start: i32, stop: i32) -> Interval {
        Interval {
            start: start,
            stop: stop,
        }
    }
}

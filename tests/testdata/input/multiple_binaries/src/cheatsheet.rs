/*
Some code are not proper to be used added a library. Put cheatsheet here
*/


use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Block{
    pos:usize,
    temperature:i32,
}

impl Ord for Block {
    fn cmp(&self, other:&Self) -> Ordering {
        other.temperature.cmp(&self.temperature)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}
impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

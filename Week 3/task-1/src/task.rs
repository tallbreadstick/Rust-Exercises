/*

    1.1 - TIME CONVERSION

    Given the defined struct [Duration], use the [From] trait of the standard library to add
    conversion methods from [&Duration] to [u32] and [u32] to [Duration], assuming that any
    given [u32] is an integer representing seconds and duration is in (hh:mm:ss) format.

*/

#[derive(Debug, PartialEq, Eq)]
pub struct Duration {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32
}

impl Duration {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Duration { hours, minutes, seconds }
    }
}
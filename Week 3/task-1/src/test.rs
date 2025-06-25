use rstest::rstest;

use crate::task::Duration;

#[rstest]
#[case(Duration { hours: 28, minutes: 32, seconds: 17 }, 102737)]
#[case(Duration { hours: 1, minutes: 15, seconds: 45 }, 4545)]
#[case(Duration { hours: 12, minutes: 45, seconds: 56 }, 45956)]
#[case(Duration { hours: 18, minutes: 55, seconds: 24 }, 68124)]
#[case(Duration { hours: 21, minutes: 0, seconds: 54 }, 75654)]
#[case(Duration { hours: 0, minutes: 58, seconds: 9 }, 3489)]
#[case(Duration { hours: 2, minutes: 59, seconds: 12 }, 10752)]
#[case(Duration { hours: 5, minutes: 12, seconds: 16 }, 18736)]
#[case(Duration { hours: 7, minutes: 9, seconds: 18 }, 25758)]
pub fn test_duration_to_seconds(#[case] duration: Duration, #[case] output: u32) {
    assert_eq!(u32::from(&duration), output);
}

#[rstest]
#[case(6450, Duration { hours: 1, minutes: 47, seconds: 30 })]
#[case(7890, Duration { hours: 2, minutes: 11, seconds: 30 })]
#[case(12, Duration { hours: 0, minutes: 0, seconds: 12 })]
#[case(60, Duration { hours: 0, minutes: 1, seconds: 0 })]
#[case(195, Duration { hours: 0, minutes: 3, seconds: 15 })]
#[case(1430, Duration { hours: 0, minutes: 23, seconds: 50 })]
#[case(23076, Duration { hours: 6, minutes: 24, seconds: 36 })]
#[case(77, Duration { hours: 0, minutes: 1, seconds: 17 })]
pub fn test_seconds_to_duration(#[case] seconds: u32, #[case] output: Duration) {
    assert_eq!(Duration::from(seconds), output);
}
fn main() {
    println!("Hello, world!");
}

pub fn parse_roll(roll: char) -> i32 {
    match roll {
        'X' => 10,
        '-' => 0,
        '/' => -1,
        number => number.to_string().parse::<i32>().expect("invalid string number"),
    }
}


pub fn calc_frame(frame: &str, mut last_is_spare: bool) -> (i32, bool) {
    let mut bonus_score_frame = 0;
    let mut score_frame = 0;
    let mut frame_is_spare = false;
    for roll in frame.chars() {
        let mut roll_value = parse_roll(roll);
        roll_value = match roll_value {
            -1 => {
                frame_is_spare = true;
                10 - score_frame
            }
            _ => roll_value,
        };
        score_frame += roll_value;
        if last_is_spare {
            bonus_score_frame += roll_value;
            last_is_spare = false;
        }
    }
    (score_frame + bonus_score_frame, frame_is_spare)
}


pub fn score_game(bowling_line: &str) -> i32 {
    let frames: Vec<&str> = bowling_line.split(" ").collect();
    if frames.iter().all(|frame| frame.contains("X")) {
        300
    } else {
        let mut score = 0;
        let mut last_is_spare = false;
        for frame in frames {
            let (score_frame, is_spare) = calc_frame(frame, last_is_spare);
            last_is_spare = is_spare;
            score += score_frame;
        }
        score
    }
}

enum Frame {
    Strike,
    Spare(i32),
    Value((i32, i32)),
    Last((i32, i32, i32)),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const ALL_MISS: &str = "-- -- -- -- -- -- -- -- -- --";
    const ALL_NINE_MISS: &str = "9- 9- 9- 9- 9- 9- 9- 9- 9- 9-";
    const ALL_NINE_MISS: &str = "11 11 11 11 11 11 11 11 11 11";
    const ALL_SPARE_MISS_LAST: &str = "9/ -- -- -- -- -- -- -- -- --";
    const ALL_SPARE_MISS_LAST: &str = "9/ 9- -- -- -- -- -- -- -- --";
    const ALL_SPARE_MISS_LAST: &str = "9/ 52 4- -- -- -- -- -- -- --";
    const ALL_SPARE_NINE_LAST: &str = "9/ 9/ 9/ 9/ 9/ 9/ 9/ 9/ 9/ 9/9";
    const STRIKE_MISS_MISS: &str = "X -- -- -- -- -- -- -- -- --";
    const STRIKE_STRIKE_MISS: &str = "X X -- -- -- -- -- -- -- --";
    const MIX_STRIKE_MISS: &str = "X -- X -- X -- X -- X --";
    const ALL_STRIKE: &str = "X X X X X X X X X XXX";
    // const STRIKE_STRIKE_STRIKE: &str = "X X X -- -- -- -- -- -- --";

    #[test]
    fn test_all_strike() {
        assert_eq!(score_game(ALL_STRIKE), 300);
    }
    #[test]
    fn test_all_miss() {
        assert_eq!(score_game(ALL_MISS), 0);
    }
    #[test]
    fn test_mix_strike_miss() {
        assert_eq!(score_game(MIX_STRIKE_MISS), 50);
    }
    #[test]
    fn test_all_nine_miss() {
        assert_eq!(score_game(ALL_NINE_MISS), 90);
    }
    #[test]
    fn test_all_spare_miss_last() {
        assert_eq!(score_game(ALL_SPARE_MISS_LAST), 181);
    }
    #[test]
    fn test_all_spare_nine_last() {
        assert_eq!(score_game(ALL_SPARE_NINE_LAST), 190);
    }
    #[test]
    fn test_strike_strike_miss() {
        assert_eq!(score_game(STRIKE_STRIKE_MISS), 30);
    }
    #[test]
    fn test_strike_strike_strike() {
        assert_eq!(score_game(STRIKE_STRIKE_STRIKE), 60);
    }
}

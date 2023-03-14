const ORDINALS: [&'static str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const DAY_NUMBERS: [&'static str; 12] = [
    "a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve",
];

const VERSE: [&'static str; 12] = [
    "partridge in a pear tree",
    "turtle doves",
    "french hends",
    "calling birds",
    "gold rings",
    "geese a-laying",
    "swans a-swimming",
    "maids a-milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming",
];

const FIRST_DAY: usize = 0;
const TWELFTH_DAY: usize = 12;

fn generate_day_lyric(current_day: usize) {
    for day in (FIRST_DAY..=current_day).rev() {
        if day == FIRST_DAY {
            println!("{} {}", DAY_NUMBERS[day], VERSE[day]);
        } else {
            println!("{} {},", DAY_NUMBERS[day], VERSE[day]);
        }
    }
}

fn main() {
    for day in FIRST_DAY..TWELFTH_DAY {
        println!(
            "On the {} day of Christmas my true love sent to me,",
            ORDINALS[day]
        );

        generate_day_lyric(day);
        println!();
    }
}

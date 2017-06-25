use std::ops::Range;

pub fn verse(n: i32) -> String {
    let next = n - 1;
    if n != 0 {
        format!("{0} of beer on the wall, {0} of beer.\n\
                 Take {1} down and pass it around, {2} of beer on the wall.\n",
                if n == 1 { "1 bottle".to_string() } else { format!("{} bottles", n) },
                if n == 1 { "it" } else { "one" },
                match next {
                    0 => "no more bottles".to_string(),
                    1 => "1 bottle".to_string(),
                    _ => format!("{} bottles", next)
                }
        )
    } else {
        "No more bottles of beer on the wall, no more bottles of beer.\n\
         Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let range = Range { start: end, end: start + 1};
    range.rev().map(|n| verse(n)).collect::<Vec<_>>().join("\n")
}

pub fn verse(n: u32) -> String {
    let variable_part_1;
    let variable_part_2;

    match n {
        0 => {
            variable_part_1 = "No more bottles".to_string();
            variable_part_2 =
                "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
        }
        1 => {
            variable_part_1 = "1 bottle".to_string();
            variable_part_2 =
                "Take it down and pass it around, no more bottles of beer on the wall.\n"
                    .to_string();
        }
        2..=99 => {
            variable_part_1 = format!("{} bottles", n);
            if n == 2 {
                variable_part_2 =
                    "Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string();
            } else {
                variable_part_2 = format!(
                    "Take one down and pass it around, {} bottles of beer on the wall.\n",
                    n - 1
                );
            }
        }
        _ => panic!("Invalid bottle number"),
    }

    format!(
        "{} of beer on the wall, {} of beer.\n{}",
        variable_part_1,
        variable_part_1.to_lowercase(),
        variable_part_2
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(|x| verse(x)).collect::<Vec<_>>().join("\n")
}

pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    [3, 5, 7]
        .iter()
        .filter(|&x| n % x == 0)
        .for_each(|&x| match x {
            3 => result.push_str("Pling"),
            5 => result.push_str("Plang"),
            7 => result.push_str("Plong"),
            _ => (),
        });

    if result.is_empty() {
        result = n.to_string();
    }

    result
}

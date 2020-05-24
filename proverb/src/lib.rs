pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if !list.is_empty() {
        if list.len() > 1 {
            proverb.push_str(&format!("For want of a {} ", list.first().unwrap()));

            list.iter().enumerate().skip(1).for_each(|(i, x)| {
                proverb.push_str(&format!("the {} was lost.\n", x));
                if i != list.len() - 1 {
                    proverb.push_str(&format!("For want of a {} ", x));
                }
            });
        }

        proverb.push_str(&format!(
            "And all for the want of a {}.",
            list.first().unwrap()
        ));
    }

    proverb
}
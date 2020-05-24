pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    
    if minefield.is_empty() {
        return result;
    }

    for (y, row_value) in minefield.iter().enumerate() {
        let mut mines_string = String::new();
        
        for x in 0..row_value.len() {
            if minefield[y].chars().nth(x).unwrap() == '*' {
                mines_string.push_str("*");
                continue;
            }

            let x = x as isize;
            let y = y as isize;
            let mut sum = 0;
            let coordenates_list: [(isize, isize); 8] = [(x-1, y-1), (x, y-1), (x+1, y-1), (x+1, y), (x+1, y+1), (x, y+1), (x-1, y+1), (x-1, y)];
            for (a,b) in coordenates_list.iter() {
                if *a < 0 || *b < 0 || *a as usize >= minefield[0].len() || *b as usize >= minefield.len() {
                    continue;
                }

                if minefield[*b as usize].chars().nth(*a as usize).unwrap() == '*' {
                    sum += 1;
                }
            }

            if sum != 0 {
                mines_string.push_str(&sum.to_string());
            } else {
                mines_string.push_str(" ");
            }
        }
        result.push(mines_string);
    }

    result
}

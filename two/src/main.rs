fn main() {
    // Parse table
    let contents = include_str!("../input").trim_end();

    let mut safes: Vec<Option<()>> = vec![None; contents.split("\n").count()];
    let mut unsafe_rows = Vec::new();

    // Part 1

    contents.split("\n").for_each(|row| {
        let mut row_items = Vec::new();

        row.split(" ").for_each(|item| {
            row_items.push(item.parse::<usize>().unwrap());
        });

        let mut pos: Option<bool> = None;
        for i in 0..(row_items.len() - 1) {
            let comp_a = row_items[i];
            let comp_b = row_items[i + 1];

            if i == 0 {
                pos = Some(comp_a > comp_b);
            } else if (pos.unwrap() && !(comp_a > comp_b)) || (!pos.unwrap() && (comp_a > comp_b)) {
                safes.pop();
                unsafe_rows.push(row);
                break;
            }

            if comp_a.abs_diff(comp_b) < 1 || comp_a.abs_diff(comp_b) > 3 {
                safes.pop();
                unsafe_rows.push(row);
                break;
            }
        }
    });

    println!("Answer part 1: {}", safes.len());

    // Part 2

    let mut unsafes: Vec<Option<()>> = vec![None; unsafe_rows.len()];

    for row in unsafe_rows.iter() {
        let mut row_items = Vec::new();

        row.split(" ").for_each(|item| {
            row_items.push(item.parse::<usize>().unwrap());
        });

        'outer: for i1 in 0..row_items.len() {
            let mut rem_row_items = row_items.clone();
            rem_row_items.remove(i1);

            let mut success = true;

            let mut pos: Option<bool> = None;

            for i2 in 0..(rem_row_items.len() - 1) {
                let comp_a = rem_row_items[i2];
                let comp_b = rem_row_items[i2 + 1];

                if i2 == 0 {
                    pos = Some(comp_a > comp_b);
                } else if (pos.unwrap() && !(comp_a > comp_b))
                    || (!pos.unwrap() && (comp_a > comp_b))
                {
                    success = false;
                    break;
                }

                if comp_a.abs_diff(comp_b) < 1 || comp_a.abs_diff(comp_b) > 3 {
                    success = false;
                    break;
                }

                continue;
            }
            if success {
                unsafes.pop();
                break 'outer;
            }
        }
    }

    println!(
        "Answer part 2: {}",
        safes.len() + (unsafe_rows.len() - unsafes.len())
    );
}

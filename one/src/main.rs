fn main() {
    // Parse table
    let contents = include_str!("../input").trim_end();

    let mut total_items = 0;

    let mut first_col_str = Vec::new();
    let mut second_col_str = Vec::new();

    contents.split("\n").for_each(|thing| {
        thing.split("   ").for_each(|thing| {
            if first_col_str.len() < (total_items + 1) {
                first_col_str.push(thing);
            } else {
                second_col_str.push(thing);
            }
        });
        total_items += 1;
    });

    let first_col_int: Vec<usize> = first_col_str
        .iter()
        .map(|thing| thing.parse::<usize>().unwrap())
        .collect();
    let second_col_int: Vec<usize> = second_col_str
        .iter()
        .map(|thing| thing.parse::<usize>().unwrap())
        .collect();

    // Part 1

    let mut first_col_int_p1 = first_col_int.clone();
    let mut second_col_int_p1 = second_col_int.clone();
    let mut differences = Vec::new();

    for _ in 0..total_items {
        let first_min = first_col_int_p1.iter().min().unwrap();
        let second_min = second_col_int_p1.iter().min().unwrap();

        differences.push(first_min.abs_diff(*second_min));

        first_col_int_p1.remove(
            first_col_int_p1
                .iter()
                .position(|item| item == first_min)
                .unwrap(),
        );
        second_col_int_p1.remove(
            second_col_int_p1
                .iter()
                .position(|item| item == second_min)
                .unwrap(),
        );
    }

    println!(
        "Answer part 1: {}",
        differences.iter().copied().reduce(|a, b| a + b).unwrap()
    );

    // Part 2

    let mut similarity = 0;

    first_col_int.iter().for_each(|item| {
        similarity += item * second_col_int.iter().filter(|item2| item == *item2).count();
    });

    println!("Answer part 2: {}", similarity);
}

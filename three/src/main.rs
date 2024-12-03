fn main() {
    // Parse table
    let contents = include_str!("../input").trim_end();

    // Part 1
    let mut split1: Vec<Vec<&str>> = Vec::new();
    contents.split("mul(").for_each(|x| {
        split1.push(x.split(")").collect::<Vec<&str>>()[0].split(",").collect::<Vec<&str>>());
    });

    println!("Answer part 1: {}", split1.iter().filter(|f| f.iter().all(|f| f.parse::<usize>().is_ok())).map(|f| f.iter().map(|f| f.parse::<usize>().unwrap())).map(|f| f.reduce(|acc,e| acc * e).unwrap()).reduce(|acc,e| acc + e).unwrap());

    // Part 2

    let new_contents = ("do()".to_owned() + contents).split("do(").map(|x| x.split("don't(").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>().iter().map(|f| f[0]).collect::<Vec<&str>>().join("");
    let mut split2: Vec<Vec<&str>> = Vec::new();
    
    new_contents.split("mul(").for_each(|x| {
        split2.push(x.split(")").collect::<Vec<&str>>()[0].split(",").collect::<Vec<&str>>());
    });

    println!("Answer part 2: {}", split2.iter().filter(|f| f.iter().all(|f| f.parse::<usize>().is_ok())).map(|f| f.iter().map(|f| f.parse::<usize>().unwrap())).map(|f| f.reduce(|acc,e| acc * e).unwrap()).reduce(|acc,e| acc + e).unwrap());
}

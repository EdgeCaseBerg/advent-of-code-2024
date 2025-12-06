use std::fs;

fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    p1(raw_data);
    p2(raw_data);
}

fn p1(raw_data: &str) {
    let lines: Vec<Vec<&str>> = raw_data.lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.trim().split(" ").map(|raw| raw.trim()).filter(|raw| !raw.is_empty()).collect()
        }).collect();
        println!("{:?}", lines);

    let copy = lines.clone();
    let ops = copy.last().expect("cant get op line");
    let rows_of_numbers_count = lines.len() - 1;

    let mut cursors: Vec<_> = 
        lines.into_iter()
            .take(rows_of_numbers_count)
            .map(|split_line| split_line.into_iter())
            .collect();

    let mut sum = 0;
    for op in ops {
        match *op {
            "+" => {
                for i in 0..rows_of_numbers_count {
                    let num: usize = cursors[i].next().expect("no next number").parse().expect("couldnt parse it");
                    sum += num;
                }
            }
            "*" => {
                let mut tmp = 1;
                for i in 0..rows_of_numbers_count {
                    let num: usize = cursors[i].next().expect("no next number").parse().expect("couldnt parse it");
                    tmp *= num;
                }
                sum += tmp;
            }
            _ => unreachable!()
        }
    }
    println!("{:?}", sum);
}

fn p2(_raw_data: &str) {

}

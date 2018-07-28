struct Item {
    value: f64,
    weight: f64,
}

// https://doc.rust-lang.org/std/cmp/trait.Ord.html#how-can-i-implement-ord
impl Item {
    fn ratio(&self) -> f64 {
        return self.value / self.weight;
    }
}

impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> std::cmp::Ordering {
        let compared = self.ratio().partial_cmp(&other.ratio());
        return compared.unwrap();
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        return self.ratio() == self.ratio();
    }
}


fn main() {
    let DEBUG = false;
    let first_line = read_input();
    let max_count: i32 = first_line[0] as i32;
    let max_weight = first_line[1];

    let mut items = Vec::new();
    loop {
        let input = read_input();
        if input.len() == 0 {
            break;
        }

        items.push(Item { value: input[0], weight: input[1] })
    }


    items.sort_by(|a, b| b.cmp(a));
    let result = maximum_value(max_count, max_weight, &items);
    println!("{:.4}", result);

    std::process::exit(0);
}


fn maximum_value(max_count: i32, max_weight: f64, items: &Vec<Item>) -> f64 {
    let mut ix: usize = 0;
    let mut value: f64 = 0.0;
    let mut weight: f64 = 0.0;
    let mut count: i32 = 0;

    loop {
        if ix >= items.len() {
            return value;
        }

        loop {
            if count >= max_count {
                return value;
            }

            if weight >= max_weight {
                return value;
            }

            if (max_weight - weight) >= items[ix].weight // try to fit full object in
                {
                    count += 1;
                    value = value + items[ix].value;
                    weight = weight + items[ix].weight;
                    println!("c: {}\tv: {}\t w: {}", count, value, weight);
                }

            let fractional_weight = max_weight / items[ix].weight;
            if fractional_weight <= 1.0 {
                let fractional_value = items[ix].value * fractional_weight;
                if (max_weight - fractional_weight) >= fractional_weight {
                    count += 1;
                    value += fractional_value;
                    weight += fractional_weight;
                    println!("c: {}\tv: {}\t w: {}", count, fractional_value, fractional_weight);
                }
            } else {
                break; // continue to item with next lowest value/weight ratio
            }
        }
        ix += 1;
    }
}


fn read_input() -> Vec<f64> {
    let mut vector: Vec<f64> = Vec::new();
    let mut stdin = String::new();

    std::io::stdin().read_line(&mut stdin);
    stdin.replace("\n", "");
    let mut items = stdin.split_whitespace().peekable();
    if items.peek().is_none() {
        return vector;
    }

    for item in items {
        let num: f64 = item.parse().unwrap();
        vector.push(num);
    }

    if vector.len() != 2 {
        println!("inputs should be pairs of integers");
        std::process::exit(-1);
    }

    return vector;
}

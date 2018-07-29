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
    println!("{:.6}", result);

    std::process::exit(0);
}


/*
while knapsack is not full
chose item with max value/weight
if item fits into knapsack, take all of it
otherwise, take so much as to fill the knapsack
return total value and amounts taken
*/
fn maximum_value(max_count: i32, max_weight: f64, items: &Vec<Item>) -> f64 {
    let DEBUG = false;
    let mut ix: usize = 0;
    let mut value: f64 = 0.0;
    let mut weight: f64 = 0.0;
    let mut count: i32 = 0;

    for i in items {
        if DEBUG {
            println!("current value: {}\nremaining weight: {}\nremaining count: {}\n",
                     value, max_weight - weight, max_count - count,
            );
        }
        if max_weight - weight <= 0.0 {
            return value;
        }

        if max_count - count <= 0 {
            return value;
        }

        let remaining_weight = max_weight - weight;
        let remaining_count = max_count - count;
        let fractional_count = 1.0 / (i.weight / (max_weight - weight));
        let fractional_weight = i.weight * fractional_count;
        let fractional_value = i.value * fractional_count;

        // take whole item if it fits, starting with highest density of value
        if (max_weight - weight) >= i.weight {
            count += 1;
            value += i.value;
            weight += i.weight;

            if DEBUG {
                println!("took all of item with w:{} and v:{} (ratio: {})",
                         i.weight, i.value, i.ratio());
            }
        } else {
            // otherwise, take (1 / (i.weight / remaining_weight) of the item
            count += 1;
            value += fractional_value;
            weight += fractional_weight;

            if DEBUG {
                println!("took {} of item with w:{} and v:{} (ratio: {})",
                         fractional_count, i.weight, i.value, i.ratio());
            }
        }
    }

    return value;
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

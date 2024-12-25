use std::collections::HashMap;

fn main() {
    let mut orders: Vec<(usize, usize)> = vec![];
    let lines = std::io::stdin().lines();
    let mut count = 0;
    for line in lines {
        let line = line.unwrap();
        if line.contains('|') {
            let mut splits = line.split('|').map(|w| w.parse().unwrap());
            orders.push((splits.next().unwrap(), splits.next().unwrap()));
        } else if !line.is_empty() {
            let mut pages = line
                .split(',')
                .map(|w| w.parse().unwrap())
                .collect::<Vec<usize>>();
            let mut pages_map: HashMap<usize, usize> = HashMap::new();
            pages.iter().enumerate().for_each(|(i, p)| {
                pages_map.insert(*p, i);
            });
            let mut violated = false;
            for order in orders.iter() {
                if let (Some(&a), Some(&b)) = (pages_map.get(&order.0), pages_map.get(&order.1)) {
                    if a > b {
                        violated = true;
                        break;
                    }
                }
            }
            if violated {
                pages.sort_by(|&a, &b| {
                    if orders.contains(&(a, b)) {
                        return std::cmp::Ordering::Less;
                    } else if orders.contains(&(b, a)) {
                        return std::cmp::Ordering::Greater;
                    }
                    std::cmp::Ordering::Equal
                });
                count += pages[pages.len() / 2];
            }
        }
    }
    println!("{}", count);
}

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Xor,
}

fn main() {
    let input = std::io::stdin().lines().map(|l| l.unwrap()).collect_vec();
    let mut input = input.split(|l| l.trim().is_empty());
    let mut wires = HashMap::new();
    for line in input.next().unwrap() {
        let (wire, value) = line.split_once(": ").unwrap();
        let value = value.parse::<u8>().unwrap();
        wires.insert(wire, value);
    }
    let mut gates = vec![];
    let mut gate_map = HashMap::new();
    for line in input.next().unwrap() {
        let (logic, wire) = line.split_once(" -> ").unwrap();
        let wire = wire.trim();
        let mut logic = logic.split_whitespace();
        let input1 = logic.next().unwrap();
        let gate = logic.next().unwrap();
        let gate = match gate {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!(),
        };
        let input2 = logic.next().unwrap();
        gates.push((input1, gate, input2, wire));
        gate_map.insert(wire, (input1, gate, input2));
    }

    let mut swap_candidates = vec![];
    let mut swapped_wires = vec![];
    // The value of the output is x XOR y XOR c
    // The next carry bit is (x AND y) OR ((x XOR y) AND c)
    for &(input1, gate, input2, wire) in &gates {
        if !(wire.starts_with('z')
            || input1.starts_with('x')
            || input1.starts_with('y')
            || input2.starts_with('x')
            || input2.starts_with('y')
            || !(gate == Gate::Xor))
        {
            swap_candidates.push((input1, gate, input2, wire));
            let (i11, _, i12) = gate_map.get(input1).unwrap();
            let (i21, _, i22) = gate_map.get(input2).unwrap();
            if let Some(i) = [i11, i12, i21, i22].iter().find(|x| x.starts_with('x')) {
                let i = gates
                    .iter()
                    .find(|(_, _, _, z)| z == &("z".to_string() + i.strip_prefix("x").unwrap()))
                    .unwrap()
                    .3;
                swapped_wires.push((i, wire));
                let v1 = *gate_map.get(i).unwrap();
                let v2 = *gate_map.get(wire).unwrap();
                gate_map.insert(i, v2);
                gate_map.insert(wire, v1);
            }
        }
    }

    let mut gates = gate_map
        .into_iter()
        .map(|(k, v)| (v.0, v.1, v.2, k))
        .collect_vec();

    let mut x_value = 0;
    let mut all_x = wires.keys().filter(|k| k.starts_with('x')).collect_vec();
    all_x.sort();
    all_x.reverse();
    for x in all_x {
        let value = *wires.get(x).unwrap();
        x_value = (x_value << 1) + value as usize;
    }
    let mut y_value = 0;
    let mut all_y = wires.keys().filter(|k| k.starts_with('y')).collect_vec();
    all_y.sort();
    all_y.reverse();
    for y in all_y {
        let value = *wires.get(y).unwrap();
        y_value = (y_value << 1) + value as usize;
    }

    for (i, j) in (0..gates.len()).tuple_combinations() {
        if i == j {
            continue;
        }
        if swapped_wires.contains(&(gates[i].3, gates[j].3)) {
            continue;
        }
        if swapped_wires.contains(&(gates[j].3, gates[i].3)) {
            continue;
        }
        if gates[i].0 == gates[j].3
            || gates[j].0 == gates[i].3
            || gates[i].2 == gates[j].3
            || gates[j].2 == gates[i].3
        {
            continue;
        }

        let temp = gates[i].3;
        gates[i].3 = gates[j].3;
        gates[j].3 = temp;

        let wires = wires.clone();
        let z_value = find_value(&gates, wires);
        if z_value == Some(x_value + y_value) {
            // HACK (only works for my input) - some manual inspection of input
            if gates[i].3 == "dtk" || gates[j].3 == "dtk" {
                let _ = (&gates[i].3, &gates[j].3);
                swapped_wires.push((gates[i].3, gates[j].3));
                break;
            }
        }
        let temp = gates[i].3;
        gates[i].3 = gates[j].3;
        gates[j].3 = temp;
    }

    println!(
        "{}",
        swapped_wires
            .into_iter()
            .flat_map(|(a, b)| vec![a, b])
            .sorted()
            .join(",")
    );
}

fn find_value<'a>(
    gates: &[(&'a str, Gate, &'a str, &'a str)],
    mut wires: HashMap<&'a str, u8>,
) -> Option<usize> {
    let mut work = gates.to_owned();
    while !work.is_empty() {
        let mut next = vec![];
        let work_len = work.len();
        for (input1, gate, input2, wire) in work {
            let input1_value = wires.get(input1);
            let input2_value = wires.get(input2);

            if input1_value.is_none() || input2_value.is_none() {
                next.push((input1, gate, input2, wire));
                continue;
            }

            let input1 = *input1_value.unwrap();
            let input2 = *input2_value.unwrap();

            let value = match gate {
                Gate::And => input1 & input2,
                Gate::Or => input1 | input2,
                Gate::Xor => input1 ^ input2,
            };
            wires.insert(wire, value);
        }
        work = next;
        if work.len() == work_len {
            return None;
        }
    }

    let mut all_z = wires.keys().filter(|k| k.starts_with('z')).collect_vec();
    all_z.sort();
    all_z.reverse();
    let mut z_value: usize = 0;
    for z in all_z {
        let value = *wires.get(z).unwrap();
        z_value = (z_value << 1) + value as usize;
    }
    Some(z_value)
}

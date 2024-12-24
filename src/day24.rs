use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque}, process::Output};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day24)]
fn parse_input(text: &str) -> (Vec<(String, u32)>, Vec<(String, String, String, String)>) {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(
        section(
            lines(string(any_char+) ": " u32)
        )
        section(
            lines(string(any_char+))
        )
    );

    let (initial, ops): (Vec<(String, u32)>, Vec<String>) =
        pairs.parse(text).unwrap();

    let mut operations: Vec<(String, String, String, String)> = Vec::new();
    for op in ops {
        let [left, operator, right, _, output] = &op.split(" ").collect::<Vec<_>>()[..] else {
            panic!("asd")
        };

        operations.push((left.to_string(), operator.to_string(), right.to_string(), output.to_string()));
    }

    (initial, operations)
}


#[aoc(day24, part1)]
fn part1((initial, operations): &(Vec<(String, u32)>, Vec<(String, String, String, String)>)) -> u64 {
    let mut gate_values: BTreeMap<String, u32> = BTreeMap::new();

    for (gate, value) in initial {
        gate_values.insert(gate.clone(), *value);
    }

    let mut ops = VecDeque::new();
    ops.extend(operations);


    while let Some(op) = ops.pop_front() {
        // println!("checking: {}, {}", op.0, op.2);
        if !gate_values.contains_key(&op.0) || !gate_values.contains_key(&op.2) {
            ops.push_back(op);
            continue;
        }
        // println!("proceeding: {} {}", op.0, op.2);

        let (left, operator, right, output) = op;
        let res = match operator.as_str() {
            "AND" => gate_values[left] & gate_values[right],
            "OR" => gate_values[left] | gate_values[right],
            "XOR" => gate_values[left] ^ gate_values[right],
            _ => unreachable!("asd")
        };

        gate_values.insert(output.clone(), res);
    }

    println!("Getting the output");
    let mut ans = Vec::new();
    for (gate, value) in gate_values.iter().rev() {
        if !gate.starts_with("z") {
            break;
        }

        ans.push(value.to_string());
    }
    let ans = ans.into_iter().collect::<String>();

    println!("ans: {}", ans);
    let ans = u64::from_str_radix(&ans, 2).unwrap();
    ans
}


#[aoc(day24, part2)]
fn part2((initial, operations): &(Vec<(String, u32)>, Vec<(String, String, String, String)>)) -> u64 {
    // let mut gate_values: BTreeMap<String, u32> = BTreeMap::new();

    // for (gate, value) in initial {
    //     gate_values.insert(gate.clone(), *value);
    // }

    println!("digraph g {{");
    for (left, operator, right, output) in operations {
        println!("{left} -> {operator}X{left}X{right};");
        println!("{right} -> {operator}X{left}X{right};");
        println!("{operator}X{left}X{right} -> {output};");
    }
    println!("}}");

    let mut initial_x = vec![
        ("x00".to_string(),  0),
        ("x01".to_string(),  0),
        ("x02".to_string(),  0),
        ("x03".to_string(),  0),
        ("x04".to_string(),  0),
        ("x05".to_string(),  0),
        ("x06".to_string(),  0),
        ("x07".to_string(),  0),
        ("x08".to_string(),  0),
        ("x09".to_string(),  0),
        ("x10".to_string(),  0),
        ("x11".to_string(),  0),
        ("x12".to_string(),  0),
        ("x13".to_string(),  0),
        ("x14".to_string(),  0),
        ("x15".to_string(),  0),
        ("x16".to_string(),  0),
        ("x17".to_string(),  0),
        ("x18".to_string(),  0),
        ("x19".to_string(),  0),
        ("x20".to_string(),  0),
        ("x21".to_string(),  0),
        ("x22".to_string(),  0),
        ("x23".to_string(),  0),
        ("x24".to_string(),  0),
        ("x25".to_string(),  0),
        ("x26".to_string(),  0),
        ("x27".to_string(),  0),
        ("x28".to_string(),  0),
        ("x29".to_string(),  0),
        ("x30".to_string(),  0),
        ("x31".to_string(),  0),
        ("x32".to_string(),  0),
        ("x33".to_string(),  0),
        ("x34".to_string(),  0),
        ("x35".to_string(),  0),
        ("x36".to_string(),  0),
        ("x37".to_string(),  0),
        ("x38".to_string(),  0),
        ("x39".to_string(),  0),
        ("x40".to_string(),  0),
        ("x41".to_string(),  0),
        ("x42".to_string(),  0),
        ("x43".to_string(),  0),
        ("x44".to_string(),  0),
    ];
    let mut initial_y = vec![
        ("y00".to_string(),  0),
        ("y01".to_string(),  0),
        ("y02".to_string(),  0),
        ("y03".to_string(),  0),
        ("y04".to_string(),  0),
        ("y05".to_string(),  0),
        ("y06".to_string(),  0),
        ("y07".to_string(),  0),
        ("y08".to_string(),  0),
        ("y09".to_string(),  0),
        ("y10".to_string(),  0),
        ("y11".to_string(),  0),
        ("y12".to_string(),  0),
        ("y13".to_string(),  0),
        ("y14".to_string(),  0),
        ("y15".to_string(),  0),
        ("y16".to_string(),  0),
        ("y17".to_string(),  0),
        ("y18".to_string(),  0),
        ("y19".to_string(),  0),
        ("y20".to_string(),  0),
        ("y21".to_string(),  0),
        ("y22".to_string(),  0),
        ("y23".to_string(),  0),
        ("y24".to_string(),  0),
        ("y25".to_string(),  0),
        ("y26".to_string(),  0),
        ("y27".to_string(),  0),
        ("y28".to_string(),  0),
        ("y29".to_string(),  0),
        ("y30".to_string(),  0),
        ("y31".to_string(),  0),
        ("y32".to_string(),  0),
        ("y33".to_string(),  0),
        ("y34".to_string(),  0),
        ("y35".to_string(),  0),
        ("y36".to_string(),  0),
        ("y37".to_string(),  0),
        ("y38".to_string(),  0),
        ("y39".to_string(),  0),
        ("y40".to_string(),  0),
        ("y41".to_string(),  0),
        ("y42".to_string(),  0),
        ("y43".to_string(),  0),
        ("y44".to_string(),  0),
    ];

    // for i in 0..9 {
    // x08 (256) + y08 (256) = 1024
    // x09 (512) + y09 (512) = 512
        initial_x[44].1 = 1;
        initial_y[44].1 = 1;
    // }

    let mut initial = initial_x;
    initial.extend(initial_y);

    let input = (initial.clone(), operations.clone());
   
    part1(&input)
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";


    #[test]
    fn input() {
        let op = parse_input(r"x01: 0

x01 AND x02 -> asd");

        assert_eq!(op.0[0].0, "x01");
        assert_eq!(op.0[0].1, 0);

        assert_eq!(op.1[0].0, "x01");
        assert_eq!(op.1[0].1, "AND");
        assert_eq!(op.1[0].2, "x02");
        assert_eq!(op.1[0].3, "asd");
    }

    #[test]
    fn p1() {
        assert_eq!(2024, part1(&parse_input(TEST_INPUT)));
    }
}
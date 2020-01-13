use crate::input::get_lines;
use std::collections::HashMap;

pub fn run() {
    let lines = get_lines("day7");
    let mut circuit = Circuit::new(lines.as_slice());

    let part1 = circuit.resolve_signal("a");
    println!("part1: {}", part1);

    let mut second_circuit = Circuit::new(lines.as_slice());
    second_circuit
        .wires
        .insert("b".to_string(), Source::Value(part1));
    let part2 = second_circuit.resolve_signal("a");
    println!("part2: {}", part2);
}

#[derive(Clone)]
enum Source {
    Instruction(String),
    Value(u16),
}

struct Circuit {
    wires: HashMap<String, Source>,
}

impl Circuit {
    fn new(lines: &[String]) -> Self {
        let mut wires: HashMap<String, Source> = HashMap::new();
        for line in lines {
            let parts = line.split(" -> ").collect::<Vec<_>>();
            if let Ok(resolved) = parts[0].parse::<u16>() {
                wires.insert(parts[1].to_string(), Source::Value(resolved));
            } else {
                wires.insert(
                    parts[1].to_string(),
                    Source::Instruction(parts[0].to_string()),
                );
            }
        }
        Self { wires }
    }

    fn resolve_signal(&mut self, wire: &str) -> u16 {
        let result = if let Ok(resolved) = wire.parse::<u16>() {
            resolved
        } else {
            let source = self.wires[wire].clone();
            match source {
                Source::Value(v) => v,
                Source::Instruction(i) => {
                    if let Ok(value) = i.parse::<u16>() {
                        value
                    } else {
                        let parts = i.split(' ').collect::<Vec<_>>();
                        match parts.len() {
                            1 => self.resolve_signal(parts[0]),
                            2 => match parts[0] {
                                "NOT" => !self.resolve_signal(parts[1]),
                                _ => panic!("unknown unary operator in '{}'", i),
                            },
                            3 => match parts[1] {
                                "AND" => {
                                    self.resolve_signal(parts[0]) & self.resolve_signal(parts[2])
                                }
                                "OR" => {
                                    self.resolve_signal(parts[0]) | self.resolve_signal(parts[2])
                                }
                                "LSHIFT" => {
                                    self.resolve_signal(parts[0]) << self.resolve_signal(parts[2])
                                }
                                "RSHIFT" => {
                                    self.resolve_signal(parts[0]) >> self.resolve_signal(parts[2])
                                }
                                _ => panic!("unknown binary operator in '{}'", i),
                            },
                            _ => panic!("bad source definition: '{}'", i),
                        }
                    }
                }
            }
        };
        self.wires.insert(wire.to_string(), Source::Value(result));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let lines = [
            "123 -> x".to_string(),
            "456 -> y".to_string(),
            "x AND y -> d".to_string(),
            "x OR y -> e".to_string(),
            "x LSHIFT 2 -> f".to_string(),
            "y RSHIFT 2 -> g".to_string(),
            "NOT x -> h".to_string(),
            "NOT y -> i".to_string(),
        ];
        let mut circuit = Circuit::new(&lines);

        assert_eq!(circuit.resolve_signal("d"), 72);
        assert_eq!(circuit.resolve_signal("e"), 507);
        assert_eq!(circuit.resolve_signal("f"), 492);
        assert_eq!(circuit.resolve_signal("g"), 114);
        assert_eq!(circuit.resolve_signal("h"), 65412);
        assert_eq!(circuit.resolve_signal("i"), 65079);
        assert_eq!(circuit.resolve_signal("x"), 123);
        assert_eq!(circuit.resolve_signal("y"), 456);
    }
}

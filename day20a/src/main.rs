use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Clone)]
enum Pulse {
    H,
    L,
}

#[derive(Debug)]
enum Module {
    FlipFlop {
        on: bool,
        output: Vec<String>,
    },
    Conjunction {
        memory: HashMap<String, Pulse>,
        output: Vec<String>,
    },
    Broadcast {
        output: Vec<String>,
    },
}

fn parse_module(line: &str) -> Option<(String, Module)> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("^([%&]?)([a-z]+) -> (.*)$").unwrap());

    let captures = PATTERN.captures(line)?;
    let type_char = captures.get(1)?.as_str();
    let name = captures.get(2)?.as_str().to_string();
    let output_str = captures.get(3)?.as_str();

    let output: Vec<_> = output_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    match type_char {
        "" => Some((name, Module::Broadcast { output })),
        "%" => Some((name, Module::FlipFlop { on: false, output })),
        "&" => Some((
            name,
            Module::Conjunction {
                memory: HashMap::new(),
                output,
            },
        )),
        _ => None,
    }
}

fn init_conjunctions(modules: &mut HashMap<String, Module>) -> Option<()> {
    // find names of conjunction modules

    let conjunctions: Vec<_> = modules
        .iter()
        .filter_map(|(n, m)| match m {
            Module::Conjunction { .. } => Some(n.to_string()),
            _ => None,
        })
        .collect();

    // for each name:
    // 1. collect names of modules that point to it
    // 2. set the initial state

    for c_name in conjunctions {
        let inputs: Vec<_> = modules
            .iter()
            .filter_map(|(n, m)| match m {
                Module::FlipFlop { output, .. }
                | Module::Conjunction { output, .. }
                | Module::Broadcast { output } => {
                    if output.contains(&c_name) {
                        Some(n.to_string())
                    } else {
                        None
                    }
                }
            })
            .collect();

        if let Module::Conjunction { ref mut memory, .. } = modules.get_mut(&c_name)? {
            for i in inputs {
                memory.insert(i, Pulse::L);
            }
        }
    }

    Some(())
}

fn parse_modules(input: &str) -> HashMap<String, Module> {
    let modules = input.lines().flat_map(parse_module).collect();
    modules
}

fn receive_pulse(
    pulse_queue: &mut VecDeque<(String, String, Pulse)>,
    modules: &mut HashMap<String, Module>,
    from: String,
    to: String,
    pulse: Pulse,
) {
    if let Some(m) = modules.get_mut(&to) {
        match m {
            Module::Broadcast { output } => {
                for o in output {
                    pulse_queue.push_back((to.to_string(), o.to_string(), pulse.clone()));
                }
            }
            Module::FlipFlop { ref mut on, output } => {
                if let Pulse::L = pulse {
                    *on = !(*on);
                    for o in output {
                        pulse_queue.push_back((
                            to.to_string(),
                            o.to_string(),
                            if *on { Pulse::H } else { Pulse::L },
                        ))
                    }
                }
            }
            Module::Conjunction {
                ref mut memory,
                output,
            } => {
                memory.insert(from, pulse);
                let pulse_to_send = if memory.iter().all(|(_, p)| match p {
                    Pulse::H => true,
                    Pulse::L => false,
                }) {
                    Pulse::L
                } else {
                    Pulse::H
                };
                for o in output {
                    pulse_queue.push_back((to.to_string(), o.to_string(), pulse_to_send.clone()));
                }
            }
        }
    }
}

fn send_pulse(modules: &mut HashMap<String, Module>) -> (u32, u32) {
    let mut h_sent = 0;
    let mut l_sent = 0;
    let mut pulse_queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
    pulse_queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::L));

    // println!("");
    while let Some((from, to, pulse)) = pulse_queue.pop_front() {
        // println!("{from} -{:?}-> {to}", pulse);

        match pulse {
            Pulse::H => h_sent += 1,
            Pulse::L => l_sent += 1,
        }

        receive_pulse(&mut pulse_queue, modules, from, to, pulse);
    }

    (h_sent, l_sent)
}

fn main() {
    let input = fs::read_to_string("input").expect("unable to read input");
    let mut modules = parse_modules(&input);
    init_conjunctions(&mut modules).expect("unable to initialize conjunctions");

    // println!("");
    // for m in &modules {
    //     eprintln!("{:?}", m);
    // }

    let mut h_sent: u32 = 0;
    let mut l_sent: u32 = 0;
    for _ in 0..1000 {
        let (h, l) = send_pulse(&mut modules);
        // println!("\nlow: {l}\nhigh: {h}");
        h_sent += h;
        l_sent += l;
    }

    println!("\ntotal low: {l_sent}\ntotal high: {h_sent}\n");
    let answer = h_sent * l_sent;
    println!("answer: {answer}");
}

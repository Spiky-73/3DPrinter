struct Code {
    key: char,
    value: f32
}

impl Code {
    fn new(raw: &str) -> Code {
        Code{ key: raw.chars().next().unwrap(), value: raw[1..].parse().unwrap()}
    }
}

struct GcodeLine {
    code: Code,
    args: Vec<Code>

}

fn parse_line(line: &str) -> GcodeLine {
    
    let mut line: Vec<&str> = line.split_whitespace().collect();

    let code = Code::new(line.remove(0));

    let mut args: Vec<Code> = Vec::new();
    for l in line {
        args.push(Code::new(l));
    }
    GcodeLine {code, args }
}
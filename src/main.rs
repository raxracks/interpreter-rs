use std::collections::HashMap;

fn s(str: &str) -> String {
    return str.to_string();
}

fn getsym(
    _stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    return s(symbols
        .get::<String>(
            &code[{
                *index += 1;
                *index
            }],
        )
        .unwrap());
}

fn parse_param(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    let value = s(&code[*index]);

    return match value.as_str() {
        "getsym" => getsym(stack, index, code, symbols),
        "stacktop" => s(stack.last().unwrap()),
        _ => value,
    };
}

fn push(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
) {
    *index += 1;
    let value = parse_param(stack, index, code, symbols);

    stack.push(value);
}

fn pop(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
) {
    stack.pop();
}

fn pops(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
) {
    *index += 1;
    let symbol_name = parse_param(stack, index, code, symbols);

    symbols.insert(symbol_name, stack.pop().unwrap());
}

fn label(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
) {
    *index += 1;
    let label_name = parse_param(stack, index, code, symbols);

    labels.insert(label_name.to_string(), *index);
}

fn sym(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
) {
    *index += 1;
    let symbol_name = parse_param(stack, index, code, symbols);

    *index += 1;
    let symbol_value = parse_param(stack, index, code, symbols);

    symbols.insert(symbol_name, symbol_value);
}

fn jmp(
    _stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    _symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
) {
    *index = *(labels
        .get::<String>(
            &code[{
                *index += 1;
                *index
            }],
        )
        .unwrap());
}

fn je(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l == r {
        jmp(stack, index, code, symbols, labels);
    } else {
        *index += 1;
    }
}

fn jne(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l == r {
        jmp(stack, index, code, symbols, labels);
    } else {
        *index += 1;
    }
}

fn jlt(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l < r {
        jmp(stack, index, code, symbols, labels);
    } else {
        *index += 1;
    }
}

fn jgt(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l > r {
        jmp(stack, index, code, symbols, labels);
    } else {
        *index += 1;
    }
}

fn jle(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l <= r {
        jmp(stack, index, code, symbols, labels);
    } else {
        *index += 1;
    }
}

fn jge(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l >= r {
        jmp(stack, index, code, symbols, labels);
    } else {
        *index += 1;
    }
}

fn print(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
) {
    *index += 1;
    let value = parse_param(stack, index, code, symbols);

    print!("{}", value);
}

fn inc(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
) {
    let value = stack.pop().unwrap().parse::<i32>().unwrap() + 1;

    stack.push(value.to_string());
}

fn main() {
    let mut stack: Vec<String> = Vec::new();
    let code = [
        s("sym"),s("i"),s("0"),
        
        s("label"),s("main"),
        
        s("print"),s("getsym"),s("i"),
        s("print"),s("\n"),
        
        s("push"),s("100000"),
        
        s("push"),s("getsym"),s("i"),
        s("inc"),
        s("sym"),s("i"),s("stacktop"),
        
        s("jlt"),s("main"),
    ];

    let mut i = 0;
    let mut symbols: HashMap<String, String> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();

    let mut ops: HashMap<
        String,
        &dyn Fn(
            &mut Vec<String>,
            &mut usize,
            &[String],
            &mut HashMap<String, String>,
            &mut HashMap<String, usize>,
        ),
    > = HashMap::new();

    ops.insert(s("push"), &push);
    ops.insert(s("pop"), &pop);
    ops.insert(s("pops"), &pops);
    ops.insert(s("label"), &label);
    ops.insert(s("sym"), &sym);
    ops.insert(s("jmp"), &jmp);
    ops.insert(s("je"), &je);
    ops.insert(s("jne"), &jne);
    ops.insert(s("jlt"), &jlt);
    ops.insert(s("jgt"), &jgt);
    ops.insert(s("jle"), &jle);
    ops.insert(s("jge"), &jge);
    ops.insert(s("print"), &print);
    ops.insert(s("inc"), &inc);

    while i < code.len() {
        ops[&code[i]](&mut stack, &mut i, &code, &mut symbols, &mut labels);
        i += 1;
    }
}

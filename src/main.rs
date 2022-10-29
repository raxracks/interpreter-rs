use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn s(str: &str) -> String {
    return str.to_string();
}

fn getsym(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    *index += 1;

    let value = parse_param(stack, index, code, symbols);

    return s(symbols.get::<String>(&value).unwrap());
}

fn getstack(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
) -> String {
    return stack.join(",");
}

fn len(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    *index += 1;
    return parse_param(stack, index, code, symbols).len().to_string();
}

fn idx(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    *index += 1;
    let value = parse_param(stack, index, code, symbols);

    *index += 1;
    let idx = parse_param(stack, index, code, symbols)
        .parse::<usize>()
        .unwrap();

    return (value.as_bytes()[idx] as char).to_string();
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
        "getstack" => getstack(stack, index, code, symbols),
        "stacktop" => s(stack.last().unwrap()),
        "len" => len(stack, index, code, symbols),
        "idx" => idx(stack, index, code, symbols),
        "null" => s(""),
        _ => value,
    };
}

fn push(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
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
    _ret_i: &mut Vec<usize>,
) {
    stack.pop();
}

fn pops(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let symbol_name = parse_param(stack, index, code, symbols);

    symbols.insert(symbol_name, stack.pop().unwrap());
}

fn label(
    _stack: &mut Vec<String>,
    index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;
}

fn sym(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
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
    ret_i: &mut Vec<usize>,
) {
    ret_i.push(*index + 1);

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
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l == r {
        jmp(stack, index, code, symbols, labels, ret_i);
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
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l == r {
        jmp(stack, index, code, symbols, labels, ret_i);
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
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l < r {
        jmp(stack, index, code, symbols, labels, ret_i);
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
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l > r {
        jmp(stack, index, code, symbols, labels, ret_i);
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
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l <= r {
        jmp(stack, index, code, symbols, labels, ret_i);
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
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    if l >= r {
        jmp(stack, index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

fn dumps(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;

    let symbol_name = parse_param(stack, index, code, symbols);

    symbols.insert(symbol_name, stack.join("|SEPARATOR|"));
}

fn res(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;

    let symbol_name = parse_param(stack, index, code, symbols);

    let str_stack: Vec<&str> = symbols
        .get(&symbol_name)
        .unwrap()
        .split("|SEPARATOR|")
        .collect();

    let mut string_stack = Vec::<String>::with_capacity(str_stack.len());
    for str_value in &str_stack {
        string_stack.push(str_value.to_string());
    }

    *stack = string_stack;
}

fn print(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let value = parse_param(stack, index, code, symbols);

    print!("{}", value);
}

fn exit(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let code: i32 = parse_param(stack, index, code, symbols).parse().unwrap();

    process::exit(code);
}

fn clears(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    stack.clear();
}

fn ret(
    _stack: &mut Vec<String>,
    index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    *index = ret_i.pop().unwrap();
}

fn add(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    stack.push((l + r).to_string());
}

fn sub(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    stack.push((l - r).to_string());
}

fn mul(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    stack.push((l * r).to_string());
}

fn div(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    stack.push((l / r).to_string());
}

fn modulo(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<i32>().unwrap();

    stack.push((l % r).to_string());
}

fn pow(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<i32>().unwrap();
    let r = stack.pop().unwrap().parse::<u32>().unwrap();

    stack.push(l.pow(r).to_string());
}

fn inc(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let value = stack.pop().unwrap().parse::<i32>().unwrap() + 1;

    stack.push(value.to_string());
}

fn dec(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let value = stack.pop().unwrap().parse::<i32>().unwrap() - 1;

    stack.push(value.to_string());
}

fn unescape(string: String) -> String {
    return string
        .replace("\\b", r"\b")
        .replace("\\t", "\t")
        .replace("\\n", "\n")
        .replace("\\f", r"\f")
        .replace("\\r", "\r")
        .replace("\\'", "'")
        .replace("\\\"", "\"");
}

fn load(filename: String, code: &mut Vec<String>) {
    let text = fs::read_to_string(filename).unwrap();
    let chars = text.as_bytes();
    let mut val: String = s("");
    let mut i = 0;

    while i < chars.len() {
        if (chars[i] as char) == '"' {
            i += 1;
            while (chars[i] as char) != '"' {
                val.push(chars[i] as char);
                i += 1;
            }

            if val.len() > 0 {
                code.push(unescape(val.clone()));
            }

            val = s("");

            i += 1;
            continue;
        }

        if (chars[i] as char) == ';' {
            while (chars[i] as char) != '\n' {
                i += 1;
            }

            i += 1;
            continue;
        }

        if (chars[i] as char) == ' ' || (chars[i] as char) == '\n' {
            if val.len() > 0 {
                code.push(unescape(val.clone()));
            }

            val = s("");

            i += 1;
            continue;
        }

        val.push(chars[i] as char);
        i += 1;
    }

    if val.len() > 0 {
        code.push(unescape(val.clone()));
    }
}

fn find_labels(code: &[String], labels: &mut HashMap<String, usize>) {
    let mut i = 0;

    while i < code.len() {
        if code[i] == "label" {
            labels.insert(
                (&code[{
                    i += 1;
                    i
                }])
                    .to_string(),
                i,
            );

            i += 1;
        }
        i += 1;
    }
}

fn main() {
    let mut stack: Vec<String> = Vec::new();

    let mut code: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();

    load(args[1].clone(), &mut code);

    let mut i = 0;
    let mut ret_i: Vec<usize> = vec![usize::MAX - 1];
    let mut symbols: HashMap<String, String> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();

    find_labels(&code, &mut labels);

    let mut ops: HashMap<
        String,
        &dyn Fn(
            &mut Vec<String>,
            &mut usize,
            &[String],
            &mut HashMap<String, String>,
            &mut HashMap<String, usize>,
            &mut Vec<usize>,
        ),
    > = HashMap::new();

    // ugly but tuples werent working
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
    ops.insert(s("dumps"), &dumps);
    ops.insert(s("res"), &res);
    ops.insert(s("print"), &print);
    ops.insert(s("exit"), &exit);
    ops.insert(s("clears"), &clears);
    ops.insert(s("ret"), &ret);

    ops.insert(s("add"), &add);
    ops.insert(s("sub"), &sub);
    ops.insert(s("mul"), &mul);
    ops.insert(s("div"), &div);
    ops.insert(s("mod"), &modulo);
    ops.insert(s("pow"), &pow);
    ops.insert(s("inc"), &inc);
    ops.insert(s("dec"), &dec);

    while i < code.len() {
        ops[&code[i]](
            &mut stack,
            &mut i,
            &code,
            &mut symbols,
            &mut labels,
            &mut ret_i,
        );
        i += 1;
    }
}

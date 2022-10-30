use std::collections::HashMap;
use std::process;

use crate::params::*;

pub fn push(
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

pub fn pop(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    stack.pop();
}

pub fn pops(
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

pub fn label(
    _stack: &mut Vec<String>,
    index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;
}

pub fn sym(
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

pub fn jmp(
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

pub fn je(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    if l == r {
        jmp(stack, index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jne(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    if l == r {
        jmp(stack, index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jlt(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    if l < r {
        jmp(stack, index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jgt(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    if l > r {
        jmp(stack, index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jle(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    if l <= r {
        jmp(stack, index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jge(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    if l >= r {
        jmp(stack, index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn dumps(
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

pub fn res(
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

pub fn print(
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

pub fn exit(
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

pub fn clears(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    stack.clear();
}

pub fn ret(
    _stack: &mut Vec<String>,
    index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    *index = ret_i.pop().unwrap();
}

pub fn add(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    stack.push((l + r).to_string());
}

pub fn sub(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    stack.push((l - r).to_string());
}

pub fn mul(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    stack.push((l * r).to_string());
}

pub fn div(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    stack.push((l / r).to_string());
}

pub fn modulo(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    stack.push((l % r).to_string());
}

pub fn pow(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let l = stack.pop().unwrap().parse::<f64>().unwrap();
    let r = stack.pop().unwrap().parse::<f64>().unwrap();

    stack.push(l.powf(r).to_string());
}

pub fn inc(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let value = stack.pop().unwrap().parse::<f64>().unwrap() + 1.0;

    stack.push(value.to_string());
}

pub fn dec(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    let value = stack.pop().unwrap().parse::<f64>().unwrap() - 1.0;

    stack.push(value.to_string());
}

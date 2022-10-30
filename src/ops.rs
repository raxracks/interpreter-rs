use std::collections::HashMap;
use std::process;

use crate::params::*;

pub fn label(
    index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;
}

pub fn sym(
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let symbol_name = parse_param(index, code, symbols);

    *index += 1;
    let symbol_value = parse_param(index, code, symbols);

    symbols.insert(symbol_name, symbol_value);
}

pub fn jmp(
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
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    if l == r {
        jmp(index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jne(
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    if l == r {
        jmp(index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jlt(
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    if l < r {
        jmp(index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jgt(
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    if l > r {
        jmp(index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jle(
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    if l <= r {
        jmp(index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn jge(
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    if l >= r {
        jmp(index, code, symbols, labels, ret_i);
    } else {
        *index += 1;
    }
}

pub fn print(
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let value = parse_param(index, code, symbols);

    print!("{}", value);
}

pub fn exit(
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    *index += 1;
    let code: i32 = parse_param(index, code, symbols).parse().unwrap();

    process::exit(code);
}

pub fn ret(
    index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    ret_i: &mut Vec<usize>,
) {
    *index = ret_i.pop().unwrap();
}

pub fn inc(
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    // let value = stack.pop().unwrap().parse::<f64>().unwrap() + 1.0;

    // stack.push(value.to_string());
}

pub fn dec(
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
    _labels: &mut HashMap<String, usize>,
    _ret_i: &mut Vec<usize>,
) {
    // let value = stack.pop().unwrap().parse::<f64>().unwrap() - 1.0;

    // stack.push(value.to_string());
}

use std::{collections::HashMap, time::SystemTime};

use crate::s;

fn getsym(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    *index += 1;

    let value = parse_param(stack, index, code, symbols);

    return s!(symbols.get::<String>(&value).unwrap());
}

fn getstack(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
) -> String {
    return stack.join(",");
}

fn getepoch(
    _stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
) -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .unwrap();

    return now.as_millis().to_string();
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

pub fn parse_param(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    let value = s!(&code[*index]);

    return match value.as_str() {
        "getsym" => getsym(stack, index, code, symbols),
        "getstack" => getstack(stack, index, code, symbols),
        "getepoch" => getepoch(stack, index, code, symbols),
        "stacktop" => s!(stack.last().unwrap()),
        "len" => len(stack, index, code, symbols),
        "idx" => idx(stack, index, code, symbols),
        "null" => s!(""),
        _ => value,
    };
}

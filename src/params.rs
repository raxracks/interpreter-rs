use std::collections::HashMap;

use crate::s;

pub fn getsym(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    *index += 1;

    let value = parse_param(stack, index, code, symbols);

    return s!(symbols.get::<String>(&value).unwrap());
}

pub fn getstack(
    stack: &mut Vec<String>,
    _index: &mut usize,
    _code: &[String],
    _symbols: &mut HashMap<String, String>,
) -> String {
    return stack.join(",");
}

pub fn len(
    stack: &mut Vec<String>,
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    *index += 1;
    return parse_param(stack, index, code, symbols).len().to_string();
}

pub fn idx(
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
        "stacktop" => s!(stack.last().unwrap()),
        "len" => len(stack, index, code, symbols),
        "idx" => idx(stack, index, code, symbols),
        "null" => s!(""),
        _ => value,
    };
}

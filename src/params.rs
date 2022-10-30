use std::{collections::HashMap, time::SystemTime};

use crate::s;

fn getsym(index: &mut usize, code: &[String], symbols: &mut HashMap<String, String>) -> String {
    *index += 1;

    let value = parse_param(index, code, symbols);

    return s!(symbols.get::<String>(&value).unwrap());
}

fn getepoch(
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

fn len(index: &mut usize, code: &[String], symbols: &mut HashMap<String, String>) -> String {
    *index += 1;
    return parse_param(index, code, symbols).len().to_string();
}

fn idx(index: &mut usize, code: &[String], symbols: &mut HashMap<String, String>) -> String {
    *index += 1;
    let value = parse_param(index, code, symbols);

    *index += 1;
    let idx = parse_param(index, code, symbols).parse::<usize>().unwrap();

    return (value.as_bytes()[idx] as char).to_string();
}

fn add(index: &mut usize, code: &[String], symbols: &mut HashMap<String, String>) -> String {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    return (l + r).to_string();
}

fn sub(index: &mut usize, code: &[String], symbols: &mut HashMap<String, String>) -> String {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    return (l - r).to_string();
}

fn mul(index: &mut usize, code: &[String], symbols: &mut HashMap<String, String>) -> String {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    return (l * r).to_string();
}

fn div(index: &mut usize, code: &[String], symbols: &mut HashMap<String, String>) -> String {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    return (l / r).to_string();
}

fn modulo(index: &mut usize, code: &[String], symbols: &mut HashMap<String, String>) -> String {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    return (l % r).to_string();
}

fn pow(index: &mut usize, code: &[String], symbols: &mut HashMap<String, String>) -> String {
    *index += 1;
    let l = parse_param(index, code, symbols).parse::<f64>().unwrap();

    *index += 1;
    let r = parse_param(index, code, symbols).parse::<f64>().unwrap();

    return (l.powf(r)).to_string();
}

pub fn parse_param(
    index: &mut usize,
    code: &[String],
    symbols: &mut HashMap<String, String>,
) -> String {
    let value = s!(&code[*index]);

    return match value.as_str() {
        "getsym" => getsym(index, code, symbols),
        "getepoch" => getepoch(index, code, symbols),
        "len" => len(index, code, symbols),
        "idx" => idx(index, code, symbols),
        "add" => add(index, code, symbols),
        "sub" => sub(index, code, symbols),
        "mul" => mul(index, code, symbols),
        "div" => div(index, code, symbols),
        "mod" => modulo(index, code, symbols),
        "pow" => pow(index, code, symbols),
        "null" => s!(""),
        _ => value,
    };
}

use std::{collections::HashMap, fs};

use crate::helpers::*;
use crate::s;

pub fn load(filename: String, code: &mut Vec<String>) {
    let text = fs::read_to_string(filename).unwrap();
    let chars = text.as_bytes();
    let mut val: String = s!("");
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

            val = s!("");

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

            val = s!("");

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

pub fn find_labels(code: &[String], labels: &mut HashMap<String, usize>) {
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

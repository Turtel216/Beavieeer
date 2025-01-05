// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

use crate::evaluator::object::*;
use std::{collections::HashMap, io};

pub fn new_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("read"), Object::Builtin(1, lang_read));
    builtins.insert(String::from("len"), Object::Builtin(1, lang_len));
    builtins.insert(String::from("first"), Object::Builtin(1, lang_first));
    builtins.insert(String::from("last"), Object::Builtin(1, lang_last));
    builtins.insert(String::from("rest"), Object::Builtin(1, lang_rest));
    builtins.insert(String::from("get"), Object::Builtin(2, lang_get));
    builtins.insert(String::from("push"), Object::Builtin(2, lang_push));
    builtins.insert(String::from("map"), Object::Builtin(2, lang_map));
    builtins.insert(String::from("filter"), Object::Builtin(2, lang_filter));
    builtins.insert(String::from("sort"), Object::Builtin(2, lang_fold));
    builtins.insert(String::from("fold"), Object::Builtin(2, lang_fold));
    builtins.insert(String::from("trim"), Object::Builtin(1, lang_trim));
    builtins.insert(String::from("explode"), Object::Builtin(1, lang_explode));
    builtins.insert(
        String::from("replaceString"),
        Object::Builtin(3, lang_replace_substring),
    );
    builtins.insert(
        String::from("replaceN"),
        Object::Builtin(4, lang_replace_n_substring),
    );
    builtins.insert(
        String::from("lowercase"),
        Object::Builtin(1, lang_to_lowercase),
    );
    builtins.insert(
        String::from("uppercase"),
        Object::Builtin(1, lang_to_uppercase),
    );
    builtins
}

fn lang_len(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(s) => Object::Int(s.len() as i64),
        Object::Array(o) => Object::Int(o.len() as i64),
        o => Object::Error(format!("argument to `len` not supported, got {}", o)),
    }
}

fn lang_first(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(o) => {
            if let Some(ao) = o.first() {
                ao.clone()
            } else {
                Object::Null
            }
        }
        o => Object::Error(format!("argument to `first` must be array. got {}", o)),
    }
}

fn lang_last(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(o) => {
            if let Some(ao) = o.last() {
                ao.clone()
            } else {
                Object::Null
            }
        }
        o => Object::Error(format!("argument to `last` must be array. got {}", o)),
    }
}

fn lang_rest(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(o) => {
            if o.len() > 0 {
                Object::Array(o[1..].to_vec())
            } else {
                Object::Null
            }
        }
        o => Object::Error(format!("argument to `rest` must be array. got {}", o)),
    }
}

fn lang_get(args: Vec<Object>) -> Object {
    match (&args[0], &args[1]) {
        (Object::Array(o), Object::Int(i)) => {
            if let Some(ao) = o.get(*i as usize) {
                ao.clone()
            } else {
                Object::Null
            }
        }
        (o1, o2) => Object::Error(format!(
            "argument to `get` must be Array, Int. got {}, {}",
            o1, o2
        )),
    }
}

fn lang_push(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::Array(o) => {
            let mut arr = o.clone();
            arr.push(args[1].clone());
            Object::Array(arr)
        }
        o => {
            let arr = vec![o.clone()];
            Object::Array(arr)
        }
    }
}

// Replaces all substrings
fn lang_replace_substring(args: Vec<Object>) -> Object {
    match (&args[0], &args[1], &args[2]) {
        (Object::String(s1), Object::String(s2), Object::String(s3)) => {
            let new_string = s1.replace(s2, s3);
            Object::String(new_string)
        }
        (o1, o2, o3) => Object::Error(format!(
            "argument to `replaceString` must be a String, String, String. got {}, {}, {}",
            o1, o2, o3
        )),
    }
}

// Replaces first N substrings
fn lang_replace_n_substring(args: Vec<Object>) -> Object {
    match (&args[0], &args[1], &args[2], &args[3]) {
        (Object::String(s1), Object::String(s2), Object::String(s3), Object::Int(i)) => {
            let new_string = s1.replacen(s2, s3, *i as usize);
            Object::String(new_string)
        }
        (o1, o2, o3, o4) => Object::Error(format!(
            "argument to `replaceN` must be a String, String, String, Int. got {}, {}, {}, {}",
            o1, o2, o3, o4
        )),
    }
}

// trim String
fn lang_trim(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(s) => {
            let new_string = s.trim();
            Object::String(new_string.to_string())
        }
        o => Object::Error(format!("argument to `trim` must be a String. got {}", o)),
    }
}

fn lang_explode(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(s) => {
            let new_vec = s.chars().map(|c| Object::String(String::from(c))).collect();
            Object::Array(new_vec)
        }
        o => Object::Error(format!("argument to `expload` must be a String. got {}", o)),
    }
}

// String to lowercase
fn lang_to_lowercase(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(s) => {
            let new_string = s.to_lowercase();
            Object::String(new_string)
        }
        o => Object::Error(format!(
            "argument to `lowercase` must be a String. got {}",
            o
        )),
    }
}

// String to lowercase
fn lang_to_uppercase(args: Vec<Object>) -> Object {
    match &args[0] {
        Object::String(s) => {
            let new_string = s.to_uppercase();
            Object::String(new_string)
        }
        o => Object::Error(format!(
            "argument to `uppercase` must be a String. got {}",
            o
        )),
    }
}

// Read from stdin
fn lang_read(args: Vec<Object>) -> Object {
    return Object::Error(String::from("TODO: read is not implemented yet"));
    //match &args[0] {
    //    Object::String(s) => {
    //        print!("{}", s);
    //        match io::stdin().lines().next() {
    //            Some(input) => Object::String(input.expect("Error reading from stdio")),
    //            None => Object::Error(String::from("invalid sdtio input")),
    //        }
    //    }
    //    o => Object::Error(format!(
    //        "argument to `uppercase` must be a String. got {}",
    //        o
    //    )),
    //}
}

// TODO
fn lang_map(args: Vec<Object>) -> Object {
    Object::Error(String::from("TODO: read is not implemented yet"))
}

// TODO
fn lang_filter(args: Vec<Object>) -> Object {
    Object::Error(String::from("TODO: filter is not implemented yet"))
}

// TODO
fn lang_fold(args: Vec<Object>) -> Object {
    Object::Error(String::from("TODO: fold is not implemented yet"))
}

// TODO
fn lang_sort(args: Vec<Object>) -> Object {
    Object::Error(String::from("TODO: sort is not implemented yet"))
}

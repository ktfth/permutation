use neon::prelude::*;
use itertools::Itertools;
use std::char;

fn vec_to_nested_number_array<'a, C: Context<'a>>(vec: &Vec<Vec<usize>>, cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as u32);

    for (i, s) in vec.iter().enumerate() {
        let v = JsArray::new(cx, s.len() as u32);
        for (j, n) in s.iter().enumerate() {
            let w = cx.number(*n as f64);
            v.set(cx, j as u32, w)?;
        }
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}

fn vec_to_string_array<'a, C: Context<'a>>(vec: &Vec<String>, cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as u32);

    for (i, s) in vec.iter().enumerate() {
        let v = cx.string(s);
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}

fn pure(mut cx: FunctionContext) -> JsResult<JsArray> {
    let pairs = cx.argument::<JsNumber>(0)?;
    let start = cx.argument::<JsNumber>(1)?;
    let end = cx.argument::<JsNumber>(2)?;
    let perms = ((start.value(&mut cx) as usize)..(end.value(&mut cx) as usize)).permutations(pairs.value(&mut cx) as usize).collect::<Vec<Vec<usize>>>();
    Ok(vec_to_nested_number_array(&perms, &mut cx).unwrap())
}

fn lower_alphabet(mut cx: FunctionContext) -> JsResult<JsArray> {
    let pairs = cx.argument::<JsNumber>(0)?;
    let perms = (10..36).permutations(pairs.value(&mut cx) as usize).collect::<Vec<Vec<u32>>>();
    let perms_chars = perms
        .iter()
        .map(|x| x.iter().map(|x| char::from_digit(*x, 36).unwrap()).collect::<String>())
        .collect::<Vec<String>>();
    Ok(vec_to_string_array(&perms_chars, &mut cx).unwrap())
}

fn upper_alphabet(mut cx: FunctionContext) -> JsResult<JsArray> {
    let pairs = cx.argument::<JsNumber>(0)?;
    let perms = (10..36).permutations(pairs.value(&mut cx) as usize).collect::<Vec<Vec<u32>>>();
    let perms_chars = perms
        .iter()
        .map(|x| x.iter().map(|x| char::from_digit(*x, 36).unwrap().to_uppercase().collect::<String>()).collect::<String>())
        .collect::<Vec<String>>();
    Ok(vec_to_string_array(&perms_chars, &mut cx).unwrap())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("pure", pure)?;
    cx.export_function("lowerAlphabet", lower_alphabet)?;
    cx.export_function("upperAlphabet", upper_alphabet)?;
    Ok(())
}

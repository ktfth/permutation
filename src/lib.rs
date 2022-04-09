use neon::prelude::*;
use itertools::Itertools;
use std::char;

fn vec_to_array<'a, C: Context<'a>>(vec: &Vec<String>, cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as u32);

    for (i, s) in vec.iter().enumerate() {
        let v = cx.string(s);
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}

fn lower_alphabet(mut cx: FunctionContext) -> JsResult<JsArray> {
    let pairs = cx.argument::<JsNumber>(0)?;
    let perms = (10..36).permutations(pairs.value(&mut cx) as usize).collect::<Vec<Vec<u32>>>();
    let perms_chars = perms
        .iter()
        .map(|x| x.iter().map(|x| char::from_digit(*x, 36).unwrap()).collect::<String>())
        .collect::<Vec<String>>();
    Ok(vec_to_array(&perms_chars, &mut cx).unwrap())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("lowerAlphabet", lower_alphabet)?;
    Ok(())
}

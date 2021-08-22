// use crate::EXAMPLE;
use neon::prelude::*;

pub fn fib(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // println!("EXAMPLE {}", *EXAMPLE);
    let num: Handle<JsNumber> = cx.argument(0)?;
    let num = num.downcast::<JsNumber, _>(&mut cx).unwrap().value(&mut cx);
    let r = cal(num as usize);
    Ok(cx.number(r))
}

fn cal(i: usize) -> f64 {
    if i <= 1 {
        return 1_f64;
    };
    cal(i - 1) + cal(i - 2)
}

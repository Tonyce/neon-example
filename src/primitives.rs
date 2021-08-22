use neon::prelude::*;

pub fn strings(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

pub fn numbers(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(88))
}

pub fn booleans(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    Ok(cx.boolean(true))
}

pub fn undefined(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    Ok(cx.undefined())
}

pub fn null(mut cx: FunctionContext) -> JsResult<JsNull> {
    Ok(cx.null())
}

pub fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let num = num_cpus::get();
    Ok(cx.number(num as f64))
}

pub fn max_num(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let num: f64 = 19007199254740992_f64;
    Ok(cx.number(num))
}

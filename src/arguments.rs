use neon::prelude::*;

pub fn check_args(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _arg0: Handle<JsString> = cx.argument(0)?;
    Ok(cx.undefined())
}

pub fn args_len(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let args_len = cx.len();
    Ok(cx.number(args_len as f64))
}

pub fn args_opt(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let args_opt = cx.argument_opt(2);
    match args_opt {
        Some(args) => {
            let s = args.downcast::<JsString, _>(&mut cx);
            if let Ok(s) = s {
                println!("{}", s.value(&mut cx));
                Ok(cx.undefined())
            } else {
                panic!("s.to_string()")
            }
        }
        None => panic!("err hanpend"),
    }
}

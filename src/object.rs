use neon::prelude::*;

struct Foo {
    pub bar: u64,
    pub baz: String,
}

pub fn convert_struct_to_js_object(mut cx: FunctionContext) -> JsResult<JsObject> {
    let thefoo = Foo {
        bar: 1234,
        baz: "baz".to_string(),
    };
    let object = JsObject::new(&mut cx);
    let js_string = cx.string(&thefoo.baz);
    let js_number = cx.number(thefoo.bar as f64);
    object.set(&mut cx, "myStringProperty", js_string).unwrap();
    object.set(&mut cx, "myNumberProperty", js_number).unwrap();
    Ok(object)
}

pub fn convert_js_object_to_struct(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let js_object: Handle<JsObject> = cx.argument(0)?;
    let key_value = js_object.get(&mut cx, "key")?;
    let key_value = key_value
        .downcast::<JsString, _>(&mut cx)
        .unwrap()
        .value(&mut cx);
    println!("{}", key_value);
    // .get(&mut cx, "myProperty")?
    // .downcast::<JsFunction, _>()
    // .or_throw(&mut cx)?;

    // let js_array_handle: Handle<JsArray> = cx.argument(0)?;
    // let vec: Vec<Handle<JsValue>> = js_array_handle.to_vec(&mut cx)?;
    Ok(cx.undefined())
}

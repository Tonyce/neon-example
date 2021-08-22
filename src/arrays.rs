use neon::prelude::*;

pub fn array_buffer(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut b: Handle<JsBuffer> = cx.argument(0)?;
    // let (x, y) = cx.borrow(&b, |data| {
    //     let slice = data.as_slice::<u8>();
    //     println!("{:?}", slice);
    //     (slice[0], slice[1])
    // });
    cx.borrow_mut(&mut b, |data| {
        let slice = data.as_mut_slice::<u8>();
        slice[0] /= 2;
        slice[1] *= 2;
    });
    // println!("({}, {})", x, y);
    // b.try_borrow(lock)
    Ok(cx.undefined())
}

pub fn convert_vec_to_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let size = 100;
    let mut vec: Vec<String> = Vec::with_capacity(size);
    for _i in 0..size {
        vec.push("0".to_string());
    }
    let js_array = JsArray::new(&mut cx, vec.len() as u32);
    // println!("vec: {:?}", vec);
    for (i, obj) in vec.iter().enumerate() {
        println!("--");
        let js_string = cx.string(obj);
        js_array.set(&mut cx, i as u32, js_string).unwrap();
    }
    Ok(js_array)
}

pub fn convert_array_to_vec(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let js_array_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_array_handle.to_vec(&mut cx)?;
    for (i, &v) in vec.iter().enumerate() {
        if v.is_a::<JsString, _>(&mut cx) {
            let s = v.downcast::<JsString, _>(&mut cx).unwrap();
            let s = s.value(&mut cx);
            println!("index {}: {}", i, s);
        };
    }
    Ok(cx.number(vec.len() as f64))
}

pub fn empty_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    Ok(cx.empty_array())
}

pub fn return_js_array_with_string(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array = JsArray::new(&mut cx, 1);
    let s = cx.string("hello world");
    array.set(&mut cx, 0, s)?;
    Ok(array)
}

pub fn shared_binary_data(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let buffer = JsArrayBuffer::new(&mut cx, 8)?;
    Ok(buffer)
}

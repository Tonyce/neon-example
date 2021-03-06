// use crate::THREAD_POOL;
use neon::prelude::*;

pub fn pool_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let queue = cx.channel();
    // let result = THREAD_POOL.(|| fib(45));
    // let pool = THREAD_POOL.lock().unwrap();
    // pool.execute(move || {
    //     let result = fib(45);
    //     // std::thread::spawn(move || {
    //     queue.send(move |mut cx| {
    //         let callback = callback.into_inner(&mut cx);
    //         let this = cx.undefined();

    //         let args = vec![
    //             cx.number(result as f64).upcast::<JsValue>(),
    //             cx.null().upcast::<JsValue>(),
    //         ];
    //         callback.call(&mut cx, this, args)?;
    //         Ok(())
    //     });
    // });
    Ok(cx.undefined())
}

pub fn rayon_pool_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let num = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let queue = cx.channel();
    // println!("-- {}", THREAD_POOL.current_num_threads());
    // let result = THREAD_POOL.(|| fib(45));
    // let pool = THREAD_POOL.lock().unwrap();
    std::thread::spawn(move || {
        // rayon::spawn(move || {
        let result = fib(num as _);
        // std::thread::spawn(move || {
        queue.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            // let err = JsError::error(&mut cx, "fsd")?;

            let args = vec![
                cx.null().upcast::<JsValue>(),
                cx.number(result as f64).upcast::<JsValue>(),
            ];
            let args = vec![
                cx.error("msg")?.upcast(),
                // err.as_value(&mut cx),
                // err.upcast::<JsValue>(),
                cx.null().upcast::<JsValue>(),
            ];
            // let args: Vec<Handle<JsValue>> = match result {
            //     Ok(id) => vec![cx.null().upcast(), cx.number(id as f64).upcast()],
            //     Err(err) => vec![cx.error(err.to_string())?.upcast()],
            // };

            // let args = cx.error("msg")?;

            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });
    Ok(cx.undefined())
}

fn fib(n: usize) -> f64 {
    if n == 0 || n == 1 {
        return 1_f64;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of `pool`
    a + b
}

pub fn start_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let queue = cx.channel();

    std::thread::spawn(move || {
        // compute the result...
        let result = cal(45);

        queue.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();

            let args = vec![
                cx.number(result).upcast::<JsValue>(),
                cx.null().upcast::<JsValue>(),
            ];
            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });

    Ok(cx.undefined())
}

fn cal(i: usize) -> f64 {
    if i <= 1 {
        return 1_f64;
    };
    cal(i - 1) + cal(i - 2)
}

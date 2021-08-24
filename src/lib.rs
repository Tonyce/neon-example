mod arguments;
mod arrays;
mod asynctask;
mod fib;
mod image_gen;
mod mythreadpool;
mod object;
mod primitives;

use lazy_static::lazy_static;
// use mythreadpool::ThreadPool;
use neon::prelude::*;
// use threadpool::ThreadPool;
// use std::sync::{mpsc, Arc, Mutex};

lazy_static! {
    static ref EXAMPLE: u8 = 42;
//     // static ref THREAD_POOL: Arc<ThreadPool> = init_thread_pool();
    static ref THREAD_POOL: rayon::ThreadPool = init_pool();
}

// fn init_thread_pool() -> Arc<ThreadPool> {
//     let pool = ThreadPool::new(8);
//     Arc::new(pool)
// }

fn init_pool() -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap()
    // ThreadPool::new(8)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // let pool = init_thread_pool();
    // let _ = init_pool();
    cx.export_function(
        "pool_task",
        move |mut cx: FunctionContext| -> JsResult<JsString> { Ok(cx.string("hello node")) },
    )?;

    cx.export_function("max_num", primitives::max_num)?;
    cx.export_function("strings", primitives::strings)?;
    cx.export_function("numbers", primitives::numbers)?;
    cx.export_function("booleans", primitives::booleans)?;
    cx.export_function("undefined", primitives::undefined)?;
    cx.export_function("err_gen", primitives::err_gen)?;
    cx.export_function("null", primitives::null)?;
    cx.export_function("get", primitives::get_num_cpus)?;

    //
    cx.export_function("arrays", arrays::convert_vec_to_array)?;
    cx.export_function("convert_array_to_vec", arrays::convert_array_to_vec)?;
    cx.export_function("empty_array", arrays::empty_array)?;
    cx.export_function(
        "return_js_array_with_string",
        arrays::return_js_array_with_string,
    )?;
    cx.export_function("shared_binary_data", arrays::shared_binary_data)?;
    cx.export_function("array_buffer", arrays::array_buffer)?;

    //
    cx.export_function(
        "convert_struct_to_js_object",
        object::convert_struct_to_js_object,
    )?;
    cx.export_function(
        "convert_js_object_to_struct",
        object::convert_js_object_to_struct,
    )?;

    //
    cx.export_function("check_args", arguments::check_args)?;
    cx.export_function("args_len", arguments::args_len)?;
    cx.export_function("args_opt", arguments::args_opt)?;

    // fib
    cx.export_function("fib", fib::fib)?;

    // async
    cx.export_function("start_task", asynctask::start_task)?;
    cx.export_function("pool_task", asynctask::pool_task)?;
    cx.export_function("rayon_pool_task", asynctask::rayon_pool_task)?;

    //
    cx.export_function("rayon_image_gen", image_gen::rayon_image_gen)?;

    Ok(())
}

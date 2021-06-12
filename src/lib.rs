extern crate napi;
#[macro_use]
extern crate napi_derive;

use napi::{CallContext, Env, JsObject, JsUndefined, Result};

struct Test {
  data: Vec<u8>
}

impl Test {
  fn new() -> Self {
    let mut data = Vec::new();
    for _ in 0..100000 {
      data.push(0);
    }
    Test {
      data
    }
  }
}

#[js_function(1)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    let mut this: JsObject = ctx.this_unchecked();
    let test = Test::new();
    ctx.env.wrap(&mut this, test)?;
    ctx.env.get_undefined()
}

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
  let sourcemap_class = env.define_class(
    "SourceMap",
    constructor,
    &[]
  )?;
  exports.set_named_property("SourceMap", sourcemap_class)?;
  Ok(())
}

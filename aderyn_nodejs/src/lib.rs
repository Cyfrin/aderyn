use aderyn_driver::driver::{self, Args};
use neon::prelude::*;

fn drive(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let root = cx.argument::<JsString>(0)?;
    let output = cx.argument::<JsString>(1)?;
    let no_snippets = cx.argument::<JsBoolean>(2)?;
    let exclude = cx.argument::<JsArray>(3)?;
    let scope = cx.argument::<JsArray>(4)?;

    let exclude_vector = exclude.to_vec(&mut cx).unwrap();
    let mut exclude_rust_vector = vec![];
    for handle in exclude_vector {
        let s = handle
            .downcast::<JsString, FunctionContext>(&mut cx)
            .unwrap()
            .value(&mut cx);
        exclude_rust_vector.push(s);
    }

    let scope_vector = scope.to_vec(&mut cx).unwrap();
    let mut scope_rust_vector = vec![];
    for handle in scope_vector {
        let s = handle
            .downcast::<JsString, FunctionContext>(&mut cx)
            .unwrap()
            .value(&mut cx);
        scope_rust_vector.push(s);
    }

    driver::drive(Args {
        root: root.value(&mut cx),
        output: output.value(&mut cx),
        exclude: if exclude_rust_vector.is_empty() {
            None
        } else {
            Some(exclude_rust_vector)
        },
        scope: if scope_rust_vector.is_empty() {
            None
        } else {
            Some(scope_rust_vector)
        },
        no_snippets: no_snippets.value(&mut cx),
    });

    Ok(cx.boolean(true))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("drive", drive)?;
    Ok(())
}

use aderyn_driver::driver;
use neon::prelude::*;

fn generate_report(mut cx: FunctionContext) -> JsResult<JsString> {
    let root = cx.argument::<JsString>(0).unwrap();
    let output = cx.argument::<JsString>(1).unwrap();
    driver::drive(driver::Args {
        root: root.value(&mut cx),
        output: output.value(&mut cx),
    });
    Ok(cx.string("Done processing!"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generate_report", generate_report)?;
    Ok(())
}

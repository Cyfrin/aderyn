use aderyn_driver::{
    detector::request_issue_detector_by_name,
    driver::{self, Args},
};
use neon::prelude::*;

fn drive(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let root = cx.argument::<JsString>(0)?;
    let output = cx.argument::<JsString>(1)?;
    let no_snippets = cx.argument::<JsBoolean>(2)?;
    let exclude = cx.argument::<JsArray>(3)?;
    let scope = cx.argument::<JsArray>(4)?;

    driver::drive(Args {
        root: root.value(&mut cx),
        output: output.value(&mut cx),
        exclude: parse(exclude, &mut cx),
        scope: parse(scope, &mut cx),
        no_snippets: no_snippets.value(&mut cx),
    });

    Ok(cx.boolean(true))
}

fn drive_with(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let root = cx.argument::<JsString>(0)?;
    let output = cx.argument::<JsString>(1)?;
    let no_snippets = cx.argument::<JsBoolean>(2)?;
    let exclude = cx.argument::<JsArray>(3)?;
    let scope = cx.argument::<JsArray>(4)?;
    let js_detectors_names = cx.argument::<JsArray>(5)?;

    if let Some(detectors_names) = parse(js_detectors_names, &mut cx) {
        let detectors = detectors_names
            .iter()
            .flat_map(|x| request_issue_detector_by_name(x))
            .collect::<Vec<_>>();

        if detectors.len() != detectors_names.len() {
            // At least 1 detectors' name has been passed incorrectly
            return Ok(cx.boolean(false));
        }

        driver::drive_with(
            Args {
                root: root.value(&mut cx),
                output: output.value(&mut cx),
                exclude: parse(exclude, &mut cx),
                scope: parse(scope, &mut cx),
                no_snippets: no_snippets.value(&mut cx),
            },
            detectors,
        );

        return Ok(cx.boolean(true));
    }

    Ok(cx.boolean(false))
}

fn parse(handle: Handle<'_, JsArray>, cx: &mut FunctionContext) -> Option<Vec<String>> {
    let js_vector = handle.to_vec(cx).unwrap();
    let mut native_string_vector = vec![];
    for handle in js_vector {
        let s = handle
            .downcast::<JsString, FunctionContext>(cx)
            .unwrap()
            .value(cx);
        native_string_vector.push(s);
    }
    if native_string_vector.is_empty() {
        None
    } else {
        Some(native_string_vector)
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("drive", drive)?;
    cx.export_function("drive_with", drive_with)?;
    Ok(())
}

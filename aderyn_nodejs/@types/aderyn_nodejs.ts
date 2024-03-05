declare module 'aderyn_nodejs' {

    export function drive(
        root: string,
        output: string,
        no_snippets: boolean,
        exclude: string[],
        scope: string[],
    ): boolean;

    export function drive_with(
        root: string,
        output: string,
        no_snippets: boolean,
        exclude: string[],
        scope: string[],
        js_detector_names: string[]
    ): boolean;

    export function get_all_issue_detectors_names(): string[]

}
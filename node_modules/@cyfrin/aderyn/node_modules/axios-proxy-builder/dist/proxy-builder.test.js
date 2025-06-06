"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const proxy_builder_1 = require("./proxy-builder");
const test_fixtures_1 = require("./test-fixtures");
describe("Test configureProxy", () => {
    afterEach(() => {
        process.env.HTTP_PROXY = "";
        process.env.HTTPS_PROXY = "";
        process.env.NO_PROXY = "";
    });
    test("with env", () => {
        process.env.HTTPS_PROXY = test_fixtures_1.https_env;
        const result = (0, proxy_builder_1.configureProxy)("https://test.com:8000");
        expect(result.proxy).toEqual({
            hostname: "testproxy.com",
            port: 8000,
            protocol: "https:",
            auth: {
                username: "",
                password: "",
            },
        });
        expect(result.httpsAgent).toBeUndefined();
    });
    test("with tunnel agent - with http env in proxy", () => {
        process.env.HTTPS_PROXY = test_fixtures_1.http_env;
        const result = (0, proxy_builder_1.configureProxy)("https://test.com:8000");
        expect(result.proxy).toEqual(false);
        expect(!!result.httpsAgent).toEqual(true);
    });
    test("with no env", () => {
        const result = (0, proxy_builder_1.configureProxy)("https://test.com:8000");
        expect(result).toBeNull();
    });
});

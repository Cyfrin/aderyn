"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const test_fixtures_1 = require("./test-fixtures");
const utils_1 = require("./utils");
describe("Test the utility functions", () => {
    afterEach(() => {
        process.env.HTTP_PROXY = "";
        process.env.HTTPS_PROXY = "";
        process.env.NO_PROXY = "";
    });
    test("Test proxy env - https", () => {
        process.env.HTTPS_PROXY = test_fixtures_1.https_env;
        const result = (0, utils_1.getProxyEnv)(test_fixtures_1.https_request);
        expect(result).toEqual(test_fixtures_1.https_env);
    });
    test("Test proxy env - http", () => {
        process.env.HTTP_PROXY = test_fixtures_1.http_env;
        const result = (0, utils_1.getProxyEnv)(test_fixtures_1.http_request);
        expect(result).toEqual(test_fixtures_1.http_env);
    });
    test("Test proxy env - https - no env", () => {
        const result = (0, utils_1.getProxyEnv)(test_fixtures_1.https_request);
        expect(result).toBeNull();
    });
    test("Test proxy env - http - no env", () => {
        const result = (0, utils_1.getProxyEnv)(test_fixtures_1.http_request);
        expect(result).toBeNull();
    });
    test("Test noProxy - no matches", () => {
        process.env.NO_PROXY = "internal.example.com, internal2.example.com";
        process.env.HTTP_PROXY = test_fixtures_1.http_env;
        const result = (0, utils_1.getProxyEnv)(test_fixtures_1.http_request);
        expect(result).toEqual(test_fixtures_1.http_env);
    });
    test("Test noProxy - noProxy match", () => {
        process.env.NO_PROXY = "test.com, internal2.example.com";
        process.env.HTTP_PROXY = test_fixtures_1.http_env;
        const result = (0, utils_1.getProxyEnv)(test_fixtures_1.http_request);
        expect(result).toBeNull();
    });
    test("Test noProxy - noProxy wildcard", () => {
        process.env.NO_PROXY = "*";
        process.env.HTTP_PROXY = test_fixtures_1.http_env;
        const result = (0, utils_1.getProxyEnv)(test_fixtures_1.http_request);
        expect(result).toBeNull();
    });
    test("Test proxy env - bad request protocol", () => {
        const result = (0, utils_1.getProxyEnv)(test_fixtures_1.bad_proto_request);
        expect(result).toBeNull();
    });
});

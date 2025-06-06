"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.http_env = exports.https_env = exports.bad_proto_request = exports.http_request = exports.https_request = void 0;
exports.https_request = new URL("https://test.com:8000");
exports.http_request = new URL("http://test.com:8000");
exports.bad_proto_request = new URL("rss://test.com:8000");
exports.https_env = "https://testproxy.com:8000";
exports.http_env = "http://testproxy.com:8000";

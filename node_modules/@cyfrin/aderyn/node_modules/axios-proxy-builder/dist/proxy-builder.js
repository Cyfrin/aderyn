"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.configureProxy = void 0;
const utils_1 = require("./utils");
const tunnel_1 = require("tunnel");
const configureProxy = (requestURL) => {
    const requestURLObject = new URL(requestURL);
    const proxyUrl = (0, utils_1.getProxyEnv)(requestURLObject);
    // short circuit if null
    if (!proxyUrl)
        return null;
    // parse proxy url
    const { hostname, port, protocol, username, password } = new URL(proxyUrl);
    // axios proxy implementation for https over http doesn't work. hence, this implementation
    if (requestURLObject.protocol === "https:" && protocol === "http:") {
        const agent = (0, tunnel_1.httpsOverHttp)({
            proxy: {
                host: hostname,
                port: parseInt(port),
            },
        });
        return {
            proxy: false,
            httpsAgent: agent,
        };
    }
    // return proxy object for axios request
    return {
        proxy: {
            protocol,
            hostname,
            port: parseInt(port),
            auth: {
                username,
                password,
            },
        },
    };
};
exports.configureProxy = configureProxy;

"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getProxyEnv = void 0;
const formatHostName = (hostname) => hostname.replace(/^\.*/, ".").toLowerCase();
const parseNoProxyZone = (zone) => {
    zone = zone.trim();
    const zoneParts = zone.split(":", 2);
    const zoneHost = formatHostName(zoneParts[0]);
    const zonePort = zoneParts[1];
    const hasPort = zone.indexOf(":") > -1;
    return { hostname: zoneHost, port: zonePort, hasPort: hasPort };
};
const urlInNoProxy = (requestURL, noProxy) => {
    const port = requestURL.port || (requestURL.protocol === "https:" ? "443" : "80");
    const hostname = formatHostName(requestURL.hostname);
    //testing: internal.example.com,internal2.example.com
    const noProxyList = noProxy.split(",");
    return noProxyList.map(parseNoProxyZone).some((noProxyZone) => {
        const isMatchedAt = hostname.indexOf(noProxyZone.hostname);
        const hostnameMatched = isMatchedAt > -1 &&
            isMatchedAt === hostname.length - noProxyZone.hostname.length;
        if (noProxyZone.hasPort) {
            return port === noProxyZone.port && hostnameMatched;
        }
        return hostnameMatched;
    });
};
const getProxyEnv = (requestURL) => {
    const noProxy = process.env.NO_PROXY || process.env.no_proxy || "";
    // if the noProxy is a wildcard then return null
    if (noProxy === "*") {
        return null;
    }
    // if the noProxy is not empty and the uri is found, return null
    if (noProxy !== "" && urlInNoProxy(requestURL, noProxy)) {
        return null;
    }
    // get proxy based on request url's protocol
    if (requestURL.protocol == "http:") {
        return process.env.HTTP_PROXY || process.env.http_proxy || null;
    }
    if (requestURL.protocol == "https:") {
        return process.env.HTTPS_PROXY || process.env.https_proxy || null;
    }
    // not a supported protocol...
    return null;
};
exports.getProxyEnv = getProxyEnv;

/// <reference types="node" />
import { Agent } from "http";
export interface RequestProxy {
    proxy: {
        protocol: string;
        hostname: string;
        port: number;
        auth: {
            username: string;
            password: string;
        };
    } | boolean;
    httpsAgent?: Agent;
}
export declare const configureProxy: (requestURL: string) => RequestProxy;

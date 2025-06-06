# axios-proxy-builder

A simple utility to build an axios proxy request object from standard http proxy environmental variables.

NoProxy logic inspired by the (now deprecated) Request project:
https://github.com/request/request/blob/3c0cddc7c8eb60b470e9519da85896ed7ee0081e/lib/getProxyFromURI.js

## Usage

### Set environmental variables

Set proxy address as well as no proxy whitelist to your environmental variables:

```sh
HTTP_PROXY-http://test.com:8000
http_proxy

HTTPS_PROXY=https://test.com:8000
https_proxy

NO_PROXY=example.test.com,example2.test.com
```

### Add to your project

Add the `configureProxy` method to your project and pass in the request url to get back a proxy object that can be added to any axios request.

```typescript
import { configureProxy } from "axios-proxy-builder";

const requestURL = "https://request-url.com/resource";
const proxy = configureProxy(requestURL);

// make REST call
axios({ ...proxy, url: requestURL });
```

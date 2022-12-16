console.log("preparing to shim");

import 'text-encoding-polyfill'

import * as crypto from 'expo-crypto';
import * as random from 'expo-random';
import * as pbkdf2 from 'pbkdf2';

// const MAX_RANDOM_BYTES = 65536;

function getRandomValues(values) {
    if (arguments.length < 1) {
        throw new TypeError(
            `An ArrayBuffer view must be specified as the destination for the random values`
        );
    }
    if (
        !(values instanceof Int8Array) &&
        !(values instanceof Uint8Array) &&
        !(values instanceof Int16Array) &&
        !(values instanceof Uint16Array) &&
        !(values instanceof Int32Array) &&
        !(values instanceof Uint32Array) &&
        !(values instanceof Uint8ClampedArray)
    ) {
        throw new TypeError(
            `The provided ArrayBuffer view is not an integer-typed array`
        );
    }
    // if (values.byteLength > MAX_RANDOM_BYTES) {
    //     throw new QuotaExceededError(
    //         `The ArrayBuffer view's byte length (${values.byteLength}) exceeds the number of bytes of entropy available via this API (${MAX_RANDOM_BYTES})`
    //     );
    // }

    // NOTE: Consider implementing `fillRandomBytes` to populate the given TypedArray directly
    let randomBytes = random.getRandomBytes(values.byteLength);

    // Create a new TypedArray that is of the same type as the given TypedArray but is backed with the
    // array buffer containing random bytes. This is cheap and copies no data.
    const TypedArrayConstructor = values.constructor;
    const randomValues = new TypedArrayConstructor(
        randomBytes.buffer,
        randomBytes.byteOffset,
        values.length
    );
    // Copy the data into the given TypedArray, letting the VM optimize the copy if possible
    values.set(randomValues);
    return values;
}

const expoCrypto = {
    ...crypto,
    ...random,
    ...pbkdf2,
    getRandomValues: getRandomValues,
}

Object.defineProperty(globalThis, "crypto", {
    configurable: true,
    enumerable: true,
    get: () => expoCrypto,
});

Object.defineProperty(window, "crypto", {
    configurable: true,
    enumerable: true,
    get: () => expoCrypto,
});

console.log("shim crypto:", Object.keys(expoCrypto))

if (typeof __dirname === "undefined") global.__dirname = "/";
if (typeof __filename === "undefined") global.__filename = "";
if (typeof process === "undefined") {
    global.process = require("process");
} else {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const bProcess = require("process");
    for (const p in bProcess) {
        if (!(p in process)) {
            process[p] = bProcess[p];
        }
    }
}

process.browser = false;
// eslint-disable-next-line @typescript-eslint/no-var-requires
if (typeof Buffer === "undefined") global.Buffer = require("buffer").Buffer;

if (!global.atob || !global.btoa) {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const Buffer = require("buffer").Buffer;
    global.atob = (data) => {
        return Buffer.from(data, "base64").toString();
    };

    global.btoa = (data) => {
        return Buffer.from(data).toString("base64");
    };
}

// const isDev = typeof __DEV__ === "boolean" && __DEV__;
// process.env["NODE_ENV"] = isDev ? "development" : "production";

import EventEmitter from "eventemitter3";

const eventListener = new EventEmitter();

window.addEventListener = (type, fn, options) => {
    if (options && options.once) {
        eventListener.once(type, fn);
    } else {
        eventListener.addListener(type, fn);
    }
};

window.removeEventListener = (type, fn) => {
    eventListener.removeListener(type, fn);
};

window.dispatchEvent = (event) => {
    eventListener.emit(event.type);
};

import registerRootComponent from 'expo/build/launch/registerRootComponent';
import App from './App';

registerRootComponent(App);
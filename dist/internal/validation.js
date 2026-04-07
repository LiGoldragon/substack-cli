"use strict";
/**
 * Utility functions for runtime validation using io-ts and fp-ts
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.decodeOrThrow = decodeOrThrow;
exports.decodeEither = decodeEither;
const function_1 = require("fp-ts/function");
const Either_1 = require("fp-ts/Either");
const PathReporter_1 = require("io-ts/PathReporter");
/**
 * Decode and validate data using an io-ts codec
 * @param codec - The io-ts codec to use for validation
 * @param data - The raw data to validate
 * @param errorContext - Context information for error messages
 * @returns The validated data
 * @throws {Error} If validation fails
 */
function decodeOrThrow(codec, data, errorContext) {
    const result = codec.decode(data);
    return (0, function_1.pipe)(result, (0, Either_1.fold)((_errors) => {
        const errorMessage = PathReporter_1.PathReporter.report(result).join(', ');
        throw new Error(`Invalid ${errorContext}: ${errorMessage}`);
    }, (parsed) => parsed));
}
/**
 * Safely decode data with io-ts, returning either an error or success
 * @param codec - The io-ts codec to use for validation
 * @param data - The raw data to validate
 * @returns Either validation errors or the parsed data
 */
function decodeEither(codec, data) {
    return codec.decode(data);
}

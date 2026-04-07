/**
 * Utility functions for runtime validation using io-ts and fp-ts
 */
import * as t from 'io-ts';
/**
 * Decode and validate data using an io-ts codec
 * @param codec - The io-ts codec to use for validation
 * @param data - The raw data to validate
 * @param errorContext - Context information for error messages
 * @returns The validated data
 * @throws {Error} If validation fails
 */
export declare function decodeOrThrow<A>(codec: t.Type<A, unknown, unknown>, data: unknown, errorContext: string): A;
/**
 * Safely decode data with io-ts, returning either an error or success
 * @param codec - The io-ts codec to use for validation
 * @param data - The raw data to validate
 * @returns Either validation errors or the parsed data
 */
export declare function decodeEither<A>(codec: t.Type<A, unknown, unknown>, data: unknown): t.Validation<A>;

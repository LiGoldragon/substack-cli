"use strict";
/**
 * Internal types - not exported from the public API
 * These represent raw API response shapes and internal structures
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.SubstackSearchResultCodec = exports.SubstackCommentResponseCodec = exports.SubstackCommentCodec = exports.SubstackFullPostCodec = exports.SubstackPostCodec = exports.SubstackPublicationCodec = void 0;
// Export io-ts codecs for runtime validation
var api_responses_1 = require("./api-responses");
Object.defineProperty(exports, "SubstackPublicationCodec", { enumerable: true, get: function () { return api_responses_1.SubstackPublicationCodec; } });
Object.defineProperty(exports, "SubstackPostCodec", { enumerable: true, get: function () { return api_responses_1.SubstackPostCodec; } });
Object.defineProperty(exports, "SubstackFullPostCodec", { enumerable: true, get: function () { return api_responses_1.SubstackFullPostCodec; } });
Object.defineProperty(exports, "SubstackCommentCodec", { enumerable: true, get: function () { return api_responses_1.SubstackCommentCodec; } });
Object.defineProperty(exports, "SubstackCommentResponseCodec", { enumerable: true, get: function () { return api_responses_1.SubstackCommentResponseCodec; } });
Object.defineProperty(exports, "SubstackSearchResultCodec", { enumerable: true, get: function () { return api_responses_1.SubstackSearchResultCodec; } });

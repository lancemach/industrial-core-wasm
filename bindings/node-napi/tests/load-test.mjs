import { strict as assert } from "node:assert";
import { createRequire } from "node:module";
import { test } from "node:test";

const require = createRequire(import.meta.url);
const { engineVersion, validateFinite } = require("../industrial_core_node.node");

test("loads the native addon and reports the facade version", () => {
  assert.equal(engineVersion(), "0.1.0");
});

test("maps invalid input to a JavaScript error", () => {
  assert.equal(validateFinite(12.5), undefined);
  assert.throws(() => validateFinite(Number.NaN), (error) => {
    assert.equal(error.code, "InvalidArg");
    assert.match(error.message, /value must be finite/);
    return true;
  });
});

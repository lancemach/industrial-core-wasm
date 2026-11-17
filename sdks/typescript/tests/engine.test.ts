import { describe, expect, it } from "vitest";
import { IndustrialCore } from "../src/engine.js";

describe("IndustrialCore", () => {
  it("loads a runtime through the shared interface", async () => {
    const engine = await IndustrialCore.load("wasm", async () => ({ info: () => ({ version: "0.1.0" }) }));
    expect(engine.info().version).toBe("0.1.0");
  });
});

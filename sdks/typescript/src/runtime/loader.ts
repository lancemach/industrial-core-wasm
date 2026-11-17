import type { EngineRuntime, RuntimeKind } from "./types.js";

export type RuntimeLoader = (kind: RuntimeKind) => Promise<EngineRuntime>;

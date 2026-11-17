export type RuntimeKind = "wasm" | "node-native";

export interface EngineInfo {
  readonly version: string;
}

export interface EngineRuntime {
  info(): EngineInfo;
  dispose?(): void;
}

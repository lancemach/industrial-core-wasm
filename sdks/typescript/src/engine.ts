import type { EngineInfo, EngineRuntime, RuntimeKind } from "./runtime/types.js";

export class IndustrialCore {
  private constructor(private readonly runtime: EngineRuntime) {}

  static async load(kind: RuntimeKind, loader: () => Promise<EngineRuntime>): Promise<IndustrialCore> {
    return new IndustrialCore(await loader());
  }

  info(): EngineInfo {
    return this.runtime.info();
  }
}

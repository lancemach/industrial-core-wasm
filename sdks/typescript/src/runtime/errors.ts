export class IndustrialCoreError extends Error {
  constructor(message: string, options?: ErrorOptions) {
    super(message, options);
    this.name = "IndustrialCoreError";
  }
}

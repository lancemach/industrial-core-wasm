# Module design

- **Core Foundation**: shared types, errors, units, precision policy, validation, and version metadata.
- **Protocol Engine**: protocol frame and data transformations; standards and test vectors must be identified before implementation.
- **Sensor Engine**: environmental calculations; formulas, units, assumptions, and ranges must be sourced.
- **Domain Engine**: specialized domains such as SF6; migrate only from reviewed specifications and reference implementations.
- **Data Engine**: general industrial data validation, mapping, and quality operations.
- **Engine Facade**: runtime-neutral initialization, capability, version, and module API.

This initialization establishes crate boundaries only. No module advertises unsupported capabilities.

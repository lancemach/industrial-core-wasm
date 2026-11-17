# Algorithm specification

Before implementation, each industrial algorithm record must contain:

- stable `algorithmId` and independently managed `algorithmVersion`;
- authoritative formula/source citation and any licensing constraints;
- input/output definitions, units, valid ranges, and environmental/pressure assumptions;
- numerical method, precision and comparison tolerance;
- error and validity-state behavior;
- independently reviewable reference vectors, including boundary cases.

Unreviewed formulas remain explicitly unimplemented. Never substitute a plausible formula, return zero for failures, or label an algorithm production-ready without source and vectors.

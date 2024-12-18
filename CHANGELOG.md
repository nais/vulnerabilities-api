# Changelog

### Changed

Addressed the following changes in the `dtrack.json` specification:

- Fixed the schema for `cwes` in `dtrack.json`.
- Updated `dtrack.json` specification to address mismatches in the response format.
- Modified `lastBomImport` type in the schema from `string` (format: `date-time`) to `number`.
- Changed `firstOccurrence` and `lastOccurrence` types in the schema from `string` (format: `date-time`) to `number`.
- Removed `project` as a required field from `ProjectMetrics` due to its absence in the response.


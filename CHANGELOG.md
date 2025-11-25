# Changelog

All notable changes to luum are documented here.

## [0.4.2] - 2026-03-02

### Added
- Receiver clustering with configurable thresholds
- Sankey graph generation from clustered flow data
- CLI `analyze` and `export` commands
- TypeScript SDK with full type definitions
- Tier-based access control (Free through Whale)
- GitHub Actions CI for Rust and TypeScript

### Changed
- Optimized clustering algorithm to reduce allocations
- Improved risk score calculation with logarithmic scaling

### Fixed
- Correct slot range validation boundary check
- Handle zero-balance edge case in tier determination

## [0.3.0] - 2026-01-15

### Added
- On-chain WalletAnalysis and ReceiverNode accounts
- Delegation revoke event emission
- CLI scaffolding with clap v4

## [0.2.0] - 2025-12-10

### Added
- Initial Anchor program with create_analysis instruction
- Account state definitions with proper space calculation
- Error code definitions

## [0.1.0] - 2025-10-20

### Added
- Project initialization
- Cargo workspace configuration
- README with architecture diagram

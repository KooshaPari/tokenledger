//! Hexagonal Architecture: Ports and Adapters for Routing Orchestration
//!
//! This module implements the "donut" layer around the harness, providing:
//! - Ports (interfaces/traits) for benchmark data
//! - Adapters for each system (CLIProxyAPI, HeliosHarness, thegent, agentapi)
//! - Provider-Harness-Model trio resolution
//! - Pareto scoring based on unified data
//!
//! Architecture:
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │              ROUTING ORCHESTRATION LAYER              │
//! │                                                      │
//! │   ┌─────────────────────────────────────────────┐    │
//! │   │              PORTS (Traits)                │    │
//! │   │  • BenchmarkPort                            │    │
//! │   │  • MetricsPort                             │    │
//! │   │  • RoutingPort                            │    │
//! │   │  • ModelMappingPort                       │    │
//! │   └─────────────────────────────────────────────┘    │
//! │                    ▲                               │
//! │                    │                               │
//! │   ┌───────────────┴───────────────┐              │
//! │   │         ADAPTERS              │              │
//! │   ├──────────────────────────────┤              │
//! │   │ • CLIProxyAdapter          │              │
//! │   │ • HeliosHarnessAdapter    │              │
//! │   │ • ThegentAdapter         │              │
//! │   │ • AgentAPIAdapter        │              │
//! │   │ • OpenRouterAdapter      │              │
//! │   │ • ArtificialAnalysisAdapt│              │
//! │   └──────────────────────────────┘              │
//! └─────────────────────────────────────────────────────────┘
//! ```

pub mod ports;
pub mod adapters;
pub mod pareto_router;
pub mod mappings;

// Re-exports for convenience
pub use ports::{
    BenchmarkPort, MetricsPort, RoutingPort, ModelMappingPort, TrioPort,
    PortError, PortResult,
    RoutingDecision, RoutingAlternative, RoutingCriteria,
    ModelMapping, ProviderHarnessModel,
};
pub use adapters::{
    CLIProxyAdapter, HeliosHarnessAdapter, ThegentRoutingAdapter, 
    AgentAPIAdapter, UnifiedAdapter,
};
pub use pareto_router::ParetoRouter;
pub use mappings::resolve_trio;

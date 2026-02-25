//! Benchmark data ingestion from external sources.
//!
//! This module provides clients for fetching benchmark data from:
//! - Artificial Analysis API
//! - OpenRouter API
//! - CLIProxyAPI metrics (60+ granular metrics)
//! - Thegent (Python) adapter
//! - Web scraping (future)
//!
//! It also provides manual override support via config.

pub mod artificial_analysis;
pub mod cli;
pub mod cliproxy_metrics;
pub mod openrouter;
pub mod overrides;
pub mod store;
pub mod thegent_adapter;

pub use store::{BenchmarkStore, BenchmarkData, BenchmarkSource};
pub use cli::run_benchmarks;
pub use overrides::ManualOverrides;
pub use thegent_adapter::{ThegentAdapter, ThegentAdapterConfig};
pub use cliproxy_metrics::{
    CLIProxyMetricsClient, CLIProxyMetricsConfig,
    RequestMetrics, ProviderMetrics, ModelMetrics, ModelRanking,
    UsageAnalytics, HealthStatus,
};

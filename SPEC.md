# TokenLedger Specification

## Repository Overview

TokenLedger is an enterprise-grade token management and pricing governance system for AI coding agents, built with Rust.

## Architecture

```
tokenledger/
├── src/
│   ├── lib.rs                 # Library root
│   ├── main.rs                # Binary entry point
│   ├── commands/              # CLI commands
│   ├── domain/                # Domain models
│   │   ├── mod.rs
│   │   ├── entities.rs        # Core entities
│   │   ├── value_objects.rs   # Value objects
│   │   ├── services.rs        # Domain services
│   │   └── events.rs         # Domain events
│   ├── application/            # Use cases
│   │   ├── commands/          # Command handlers
│   │   ├── queries/          # Query handlers
│   │   └── services.rs       # Application services
│   ├── infrastructure/         # External adapters
│   │   ├── persistence/       # Storage adapters
│   │   ├── http/            # HTTP client
│   │   └── config.rs        # Configuration
│   └── ports/                # Port traits
│       ├── mod.rs
│       ├── repository.rs      # Repository traits
│       └── external.rs       # External service traits
├── tests/                    # Integration tests
├── benches/                  # Benchmarks
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Domain Model

### Bounded Contexts

1. **Token Management** - Token tracking, allocation, budgets
2. **Pricing Engine** - Cost calculation, rate limits
3. **Governance** - Policies, rules, compliance

### Core Entities

- `TokenBudget` - Budget allocation
- `UsageRecord` - Token consumption tracking
- `PricingRule` - Cost calculation rules
- `GovernancePolicy` - Access control policies

## xDD Methodologies Checklist

### TDD (Test-Driven Development)

- [ ] Red-Green-Refactor cycles
- [ ] Unit tests first
- [ ] Test coverage > 80%
- [ ] Property-based tests
- [ ] Mutation coverage

### BDD (Behavior-Driven Development)

- [ ] Feature files `*.feature`
- [ ] Gherkin scenarios
- [ ] Step definitions
- [ ] Scenario outlines

### DDD (Domain-Driven Design)

- [ ] Bounded contexts identified
- [ ] Aggregates defined
- [ ] Value objects created
- [ ] Domain events modeled
- [ ] Repository patterns implemented

### ATDD (Acceptance TDD)

- [ ] Acceptance criteria first
- [ ] Executable specs
- [ ] Customer-readable documentation

### CQRS (Command Query Responsibility Segregation)

- [ ] Separate command models
- [ ] Separate query models
- [ ] Event sourcing for commands

### Event Sourcing

- [ ] Event definitions
- [ ] Event store implementation
- [ ] Snapshot strategies

### Property-Based Testing

- [ ] Arbitrary implementations
- [ ] Property invariants
- [ ] Shrinking strategies

## Architecture Tests

```rust
// tests/architecture/tokenledger_core_no_outer_ring.rs
#[test]
fn core_no_outer_ring_dependencies() {
    assert!(!has_dependency("tokio"));
    assert!(!has_dependency("reqwest"));
}
```

## Design Principles

### SOLID

- [ ] Single Responsibility: Each module has one reason to change
- [ ] Open/Closed: Open for extension, closed for modification
- [ ] Liskov Substitution: Subtypes substitutable for base types
- [ ] Interface Segregation: Many specific interfaces > one general
- [ ] Dependency Inversion: Depend on abstractions, not concretions

### GRASP

- [ ] Controller: Handle system events
- [ ] Creator: Who creates objects
- [ ] Expert: Assign responsibility to information expert
- [ ] High Cohesion: Related responsibilities together
- [ ] Low Coupling: Minimize dependencies

### Other Principles

- [ ] KISS: Keep it simple, stupid
- [ ] DRY: Don't repeat yourself
- [ ] YAGNI: You aren't gonna need it
- [ ] Law of Demeter: Talk only to immediate friends
- [ ] SoC: Separation of concerns
- [ ] CoC: Convention over configuration

## Quality Gates

### CI/CD Pipeline

```yaml
# .github/workflows/quality.yml
- name: Rust Quality Gates
  run: |
    - cargo fmt --check
    - cargo clippy -- -D warnings
    - cargo test
    - cargo audit
    - cargo miri test
```

### Code Quality Tools

- [ ] rustfmt: Code formatting
- [ ] clippy: Linting
- [ ] cargo-audit: Security vulnerabilities
- [ ] miri: Undefined behavior detection
- [ ] cargo-fuzz: Fuzzing

## File Organization Rules

```
src/
├── domain/           # Pure domain (NO external dependencies)
│   ├── entities.rs   # Domain entities only
│   └── events.rs     # Domain events only
├── application/       # Use cases (depends on domain)
│   ├── commands/     # Command handlers
│   └── queries/      # Query handlers
├── infrastructure/   # External adapters (depends on application)
│   ├── persistence/  # Storage implementations
│   └── http/        # HTTP client implementations
└── ports/           # Trait definitions (no dependencies)
```

## Module Rules

1. **Domain Layer**: No dependencies on infrastructure, application, or external crates
2. **Application Layer**: Depends only on domain and ports
3. **Infrastructure Layer**: Implements ports, depends on application
4. **Ports Layer**: Contains traits only, zero dependencies

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_budget_enforces_limit() {
        let budget = TokenBudget::new(1000);
        assert_eq!(budget.remaining(), 1000);
    }

    #[test]
    fn pricing_rule_calculates_correctly() {
        let rule = PricingRule::flat_rate(0.01);
        assert_eq!(rule.calculate(100), 1.00);
    }
}
```

## Next Steps

1. Add comprehensive unit tests
2. Implement architecture tests
3. Add property-based tests
4. Set up mutation testing
5. Create BDD scenarios

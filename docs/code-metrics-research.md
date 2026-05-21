ChatGPT Pro


I have a tool that measures Rust code complexity primarily by counting lines per function, functions per module, modules per crate, crate per component, and components per repo.  for LOC, 25 per fn keeps a fn on a page view.  fn, module, and crate counts, I use a max of 5 per, (for mental load 5+/-2) reserving 2 for future expansion.  For component (a Cargo multi-crate workspace), I allow more than 7 per repo, but generally the component can be grouped into subsystems.  In practice I end up with 5 or fewer parts at each level. 

AI coding agents sometimes struggle to refactor code to fit these constraints.  I try to give hints: separate tests from code, use pure functions instead of impl functions where possible.  Use composition, especially for traits and structs.  Use design patterns, like chain of responsibility, bridge, facade, template-hook, builder pattern, separate configuration, initialization, state, arg parsing, etc.  

Can you come up with an AI agent guide to creating loosely-coupled, functional-style, testable, maintainable code that fits my code metric gates?

I’ll turn your constraints into an agent-facing guide: first the metric gates, then refactoring rules, architectural patterns, and concrete “when you exceed the gate, do this” playbooks.

Thought for 6s
AI Agent Guide: Rust Code That Passes Small-Part Complexity Gates
Goal
Create Rust code that is:

Loosely coupled

Functional-style where practical

Easy to test

Easy to refactor

Readable in one page / one screen

Structured so every level stays small

Target shape:

Level	Preferred Gate
Lines per function	≤ 25 LOC
Functions per module	≤ 5
Modules per crate	≤ 5
Crates per component/workspace group	≤ 5
Major subsystems per repo	≤ 5-ish
The guiding rule:

Every part should fit in human working memory. When a part grows, split by responsibility, not by accident.

1. Core Design Principles
1.1 Prefer small pure functions
Prefer:

fn normalize_name(input: &str) -> String {
    input.trim().to_ascii_lowercase()
}
Over:

impl AppState {
    fn normalize_name_for_current_context(&self, input: &str) -> String {
        // reads state, logs, mutates cache, validates, normalizes...
    }
}
Use pure functions for:

Parsing

Validation

Transformation

Formatting

Decision logic

Classification

Filtering

Mapping

Scoring

Keep impl methods mostly for:

Constructors

State mutation

Trait implementations

Small orchestration methods

Invariant-preserving operations

1.2 Separate decisions from effects
Bad:

fn process_file(path: &Path) -> Result<()> {
    let text = fs::read_to_string(path)?;
    let config = parse_config(&text)?;
    if config.enabled {
        fs::write("out.txt", render(config)?)?;
    }
    Ok(())
}
Better:

fn plan_output(config: &Config) -> Option<OutputPlan> {
    config.enabled.then(|| OutputPlan::from(config))
}

fn process_file(path: &Path) -> Result<()> {
    let text = fs::read_to_string(path)?;
    let config = parse_config(&text)?;
    if let Some(plan) = plan_output(&config) {
        fs::write("out.txt", render_output(&plan)?)?;
    }
    Ok(())
}
Rule:

Pure code decides. Thin shell code performs effects.

This makes tests smaller and avoids mock-heavy designs.

2. Refactoring Triggers
When a function exceeds 25 LOC, do not merely split at random.

Ask:

Symptom	Refactor
Function parses and validates	Extract parse_* and validate_*
Function builds config and runs app	Extract load_config, build_context, run_command
Function has many if/else branches	Extract decision table, enum dispatch, or strategy
Function mutates several fields	Extract state transition function
Function handles errors inline	Extract helper returning typed error/result
Function has setup, action, cleanup	Extract template-hook or RAII guard
Function loops with complex body	Extract body into named function
Function has nested matches	Extract per-variant handler
3. Preferred Module Shape
Each module should have one clear job.

Example:

src/
  lib.rs
  config/
    mod.rs
    parse.rs
    validate.rs
    defaults.rs
  command/
    mod.rs
    args.rs
    plan.rs
    run.rs
  model/
    mod.rs
    component.rs
    metric.rs
    report.rs
  report/
    mod.rs
    markdown.rs
    json.rs
    summary.rs
Inside each module:

mod parse;
mod validate;
mod defaults;

pub use parse::parse_config;
pub use validate::validate_config;
pub use defaults::default_config;
Rule:

mod.rs is a facade, not a junk drawer.

4. File / Module Roles
Use consistent names so agents know where code belongs.

File	Purpose
model.rs	Data structures and domain types
parse.rs	String/file/input → typed data
validate.rs	Typed data → validation result
plan.rs	Inputs/config → execution plan
run.rs	Performs effects
render.rs	Typed data → string/output
error.rs	Error enums and conversions
test_support.rs	Shared test builders/helpers
fixtures.rs	Static test data or fixture loading
5. Keep Tests Out of Production Modules
Prefer:

src/
  config/
    parse.rs
tests/
  config_parse_tests.rs
Or:

src/
  config/
    parse.rs
    parse_tests.rs
Avoid giant inline test blocks that make source files unreadable.

Production files should stay production-focused.

6. Composition Over God Objects
Avoid one struct that owns everything:

struct App {
    config: Config,
    db: Db,
    logger: Logger,
    parser: Parser,
    renderer: Renderer,
    state: State,
}
Prefer smaller parts:

struct Runtime {
    config: Config,
    services: Services,
}

struct Services {
    store: Box<dyn Store>,
    renderer: Box<dyn Renderer>,
}
Or even better, pass capabilities directly:

fn run_report(
    input: &Input,
    store: &dyn Store,
    renderer: &dyn Renderer,
) -> Result<Report> {
    let data = store.load(input)?;
    renderer.render(&data)
}
Rule:

Pass the smallest capability needed.

7. Trait Guidance
Use traits for boundaries, not for every helper.

Good trait candidates:

File system abstraction

Network/API client

Clock/time source

Renderer backend

Storage backend

Command handler

Plugin/extension point

Avoid traits for:

Simple pure functions

One implementation with no expected variants

Internal helpers

Premature abstraction

Preferred pattern:

pub trait Store {
    fn load(&self, key: &str) -> Result<Record>;
}

pub fn load_report(store: &dyn Store, key: &str) -> Result<Report> {
    let record = store.load(key)?;
    Ok(Report::from(record))
}
8. Pattern Playbook
Facade
Use when a subsystem has many internal modules.

pub fn analyze_repo(path: &Path) -> Result<RepoReport> {
    let crates = discover_crates(path)?;
    let metrics = measure_crates(&crates)?;
    summarize_repo(metrics)
}
The facade gives callers one simple entry point.

Builder
Use when construction has many optional fields.

let config = ConfigBuilder::new()
    .max_fn_loc(25)
    .max_module_fns(5)
    .build()?;
Keep builder methods tiny.

Chain of Responsibility
Use when several handlers may process something.

for rule in rules {
    if let Some(issue) = rule.check(item) {
        issues.push(issue);
    }
}
Good for metric gates, lint rules, validators, and policy checks.

Bridge
Use when policy and backend vary independently.

Example:

Metric policy:
  strict
  relaxed
  experimental

Output backend:
  markdown
  json
  terminal
Do not combine these into one giant type.

Template-Hook
Use when an algorithm is fixed but some steps vary.

fn run_analysis<H: Hooks>(hooks: &H, input: Input) -> Result<Report> {
    hooks.before_parse(&input)?;
    let parsed = parse(input)?;
    hooks.after_parse(&parsed)?;
    analyze(parsed)
}
9. Error Handling Style
Prefer local, typed errors.

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("invalid limit: {0}")]
    InvalidLimit(usize),
}
Avoid giant global error enums unless the crate is very small.

Use:

type Result<T> = std::result::Result<T, ConfigError>;
inside focused modules.

10. Agent Refactoring Algorithm
When modifying code, the agent should follow this loop:

Step 1: Measure
Check:

Function LOC

Function count per module

Module count per crate

Crate count per component

Coupling/import size

Test size and placement

Step 2: Classify excess
Ask what grew:

Too many LOC?
Too many functions?
Too many responsibilities?
Too many effects mixed with logic?
Too many variants in one match?
Too many tests inline?
Step 3: Split by responsibility
Do not split mechanically.

Prefer names like:

parse_*
validate_*
build_*
plan_*
run_*
render_*
summarize_*
Step 4: Preserve behavior
Before refactoring:

cargo test
After each small move:

cargo test
cargo clippy --all-targets --all-features
Step 5: Re-measure
The refactor is not complete until the metrics pass.

11. Concrete Agent Rules
Agents should obey these defaults:

No function over 25 LOC unless explicitly justified.

No module with more than 5 production functions.

Move tests out of production files when they distort readability.

Prefer pure free functions for logic.

Use impl for invariants, construction, and behavior tied to state.

Separate config, initialization, runtime state, and command handling.

Separate parsing from validation.

Separate planning from execution.

Separate rendering from data modeling.

Use facades to hide subsystem detail.

Use traits only at real boundaries.

Never create a god struct, god enum, god module, or god crate.

12. Suggested Repository Shape
repo/
  Cargo.toml
  crates/
    cli/
    core/
    report/
    rules/
    test-support/
  docs/
    architecture.md
    design.md
    metrics.md
  tests/
    cli_tests.rs
Possible crate roles:

Crate	Purpose
core	Domain model and pure analysis
rules	Metric gates and validation policies
report	Markdown/JSON/terminal output
cli	Arg parsing, config loading, effectful shell
test-support	Fixtures and helpers
13. Useful Final Instruction for Agents
You can give agents this compact directive:

Refactor this Rust code to pass the small-part complexity gates:
- max 25 LOC per function
- max 5 production functions per module
- max 5 modules per crate where practical
- prefer pure free functions over impl methods for logic
- separate parsing, validation, planning, execution, rendering, and tests
- use facades for subsystem entry points
- use traits only at effect or backend boundaries
- keep behavior unchanged
- run tests after each coherent refactor
- do not split mechanically; split by responsibility
Or stricter:

Do not add new logic to an over-limit function or module.
First extract responsibilities into named pure helpers, move tests out of production modules, and re-run the metric tool.

also, do not put functions into lib.rs or mod.rs.  Always use named module files for functions.   When using build.rs (for code that runs at compile time) same best-practices apply: modular, functional, design patterns where appropriate.  Also, try to apply these best-practices to different kinds of Rust macros; use macros where appropriate to reduce/hide boilerplate or reduce repetitive code.  Can you expand on this additions?

Yes. I’d add these sections to the guide.

14. lib.rs and mod.rs Are Facades Only
Do not put executable logic in:

src/lib.rs
src/main.rs
src/foo/mod.rs
Allowed:

pub mod config;
pub mod rules;
pub mod report;

pub use config::Config;
pub use rules::RuleSet;
Not allowed:

pub fn analyze_repo(...) -> Result<Report> {
    ...
}
Instead:

src/
  lib.rs
  analysis/
    mod.rs
    analyze.rs
    plan.rs
// analysis/mod.rs
mod analyze;
mod plan;

pub use analyze::analyze_repo;
// analysis/analyze.rs
pub fn analyze_repo(...) -> Result<Report> {
    ...
}
Rule:

lib.rs and mod.rs define the public surface. Named files contain behavior.

15. build.rs Follows the Same Rules
Treat build.rs as a tiny compile-time CLI.

Bad:

// build.rs
fn main() {
    // 200 lines of file discovery, parsing, codegen, env handling...
}
Better:

build.rs
build_support/
  mod.rs
  env.rs
  discover.rs
  generate.rs
  rerun.rs
// build.rs
mod build_support;

fn main() {
    build_support::run();
}
// build_support/mod.rs
mod discover;
mod env;
mod generate;
mod rerun;
mod run;

pub use run::run;
// build_support/run.rs
pub fn run() {
    rerun::emit_rerun_directives();
    let inputs = discover::find_inputs();
    generate::write_generated_code(inputs);
}
For build scripts:

Concern	Module
cargo:rerun-if-changed	rerun.rs
Env vars	env.rs
Input discovery	discover.rs
Code generation	generate.rs
Output paths	paths.rs
Error handling	error.rs
Rule:

Build-time code deserves the same maintainability as runtime code.

16. Macro Guidance
Use macros to remove repetition, not to hide design.

Good macro use:

Repetitive trait impls

Repetitive test cases

Boilerplate enum conversions

Static rule declarations

DSL-like configuration

Generated table rows

Repeated parser cases

Bad macro use:

Hiding complex control flow

Avoiding proper functions

Encoding business logic in token soup

Making errors harder to understand

Replacing simple generics or helper functions

17. Macro Types and Best Uses
Macro Type	Use For	Avoid For
macro_rules!	Small repetitive patterns	Complex parsing
Derive proc macro	Boilerplate trait impls	Behavior-heavy logic
Attribute proc macro	Declarative annotations	Hidden runtime effects
Function-like proc macro	DSLs/codegen	Ordinary helper logic
Build-time codegen	Large generated tables/types	Small hand-written code
18. Prefer This Order Before Writing a Macro
Before adding a macro, try:

A pure function

A generic function

A trait

A builder

A table-driven design

A small macro_rules!

A proc macro

Build-time codegen

Rule:

Macros are for structural repetition, not unclear abstraction.

19. Keep Macros Small and Isolated
Preferred layout:

src/
  lib.rs
  macros/
    mod.rs
    declare_rule.rs
    table_tests.rs
// macros/mod.rs
mod declare_rule;
mod table_tests;

pub(crate) use declare_rule::declare_rule;
pub(crate) use table_tests::table_tests;
A macro file should usually contain:

One macro

One responsibility

Documentation

Example expansion or usage

Tests where practical

20. Example: Macro for Repetitive Tests
Instead of many nearly identical tests:

#[test]
fn rejects_large_function() {
    assert_rejects("fn_loc", 26);
}

#[test]
fn accepts_small_function() {
    assert_accepts("fn_loc", 25);
}
Use:

macro_rules! metric_case {
    ($name:ident, $metric:expr, $value:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let result = check_metric($metric, $value);
            assert_eq!(result.is_ok(), $expected);
        }
    };
}

metric_case!(accepts_25_loc_fn, "fn_loc", 25, true);
metric_case!(rejects_26_loc_fn, "fn_loc", 26, false);
Good because:

The pattern is obvious

The macro is tiny

Test names remain explicit

Logic stays in check_metric

21. Example: Macro for Rule Declaration
macro_rules! declare_limit_rule {
    ($name:ident, $metric:expr, $limit:expr) => {
        pub fn $name(value: usize) -> RuleResult {
            check_limit($metric, value, $limit)
        }
    };
}

declare_limit_rule!(check_fn_loc, Metric::FunctionLoc, 25);
declare_limit_rule!(check_module_fns, Metric::ModuleFunctions, 5);
Good because the macro removes boilerplate while keeping the actual rule logic here:

pub fn check_limit(metric: Metric, value: usize, limit: usize) -> RuleResult {
    if value <= limit {
        RuleResult::Pass
    } else {
        RuleResult::Fail { metric, value, limit }
    }
}
Rule:

Macro expands structure. Function contains behavior.

22. Proc Macro Crate Shape
If proc macros are needed, isolate them.

crates/
  metric-core/
  metric-macros/
  metric-cli/
# crates/metric-macros/Cargo.toml
[lib]
proc-macro = true
Suggested proc macro crate structure:

metric-macros/
  src/
    lib.rs
    derive_rule.rs
    attr_metric.rs
    parse.rs
    emit.rs
    error.rs
lib.rs should only expose macro entry points:

#[proc_macro_derive(Rule)]
pub fn derive_rule(input: TokenStream) -> TokenStream {
    derive_rule::expand(input)
}
Actual behavior belongs in named files.

23. Final Expanded Agent Directive
Refactor and write Rust code according to these gates and layout rules:

Metrics:
- max 25 LOC per function
- max 5 production functions per module
- max 5 modules per crate where practical
- keep repo/component/subsystem hierarchy shallow and grouped

Layout:
- do not put functions in lib.rs
- do not put functions in mod.rs
- lib.rs and mod.rs are facade/re-export files only
- place behavior in named module files
- move tests out of production modules when they distort readability

Design:
- prefer pure free functions for logic
- separate parsing, validation, planning, execution, rendering, and state
- separate configuration, initialization, runtime state, and arg parsing
- use traits only at real boundaries
- use facades for subsystem entry points
- use composition over god structs
- use design patterns where they reduce coupling

Build scripts:
- build.rs must remain a thin entry point
- split compile-time logic into named modules
- apply the same LOC/function/module rules to build-time code

Macros:
- use macros to remove boilerplate and repetitive structure
- do not use macros to hide complex logic
- prefer functions/generics/traits before macros
- keep macro definitions small and isolated
- macro_rules! is preferred for simple repetition
- proc macros require their own crate
- macro expansion should call ordinary functions where behavior lives

Process:
- measure before refactoring
- classify the excess responsibility
- split by responsibility, not mechanically
- preserve behavior
- run tests after each coherent change
- re-measure before declaring success




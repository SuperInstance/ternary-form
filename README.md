# ternary-form

**Musical form analysis for multi-agent task decomposition.**

In music, form is the large-scale architecture — how sections relate across time. A sonata isn't just notes; it's an argument. A rondo isn't just repetition; it's a journey with a home base. This crate maps musical forms onto multi-agent task planning, giving you structural templates for complex workflows.

## Why Musical Form?

When you're orchestrating multiple agents on a complex task, the *structure* matters as much as the individual steps. Musical forms give us proven patterns:

- **Binary (A-B)**: Two contrasting phases — research then implement
- **Ternary (A-B-A)**: Depart and return — establish, explore, refine
- **Rondo (A-B-A-C-A)**: Iterative refinement with exploratory digressions
- **Sonata**: The full argument — thesis, antithesis, development, synthesis

## Forms

### Binary Form (A-B)

Two contrasting sections. The simplest meaningful structure.

**Musical analog**: A folk song with verse and chorus. A Bach invention.
**Task analog**: Research → Implementation. Explore → Exploit. Gather → Decide.

```rust
use ternary_form::BinaryForm;

let form = BinaryForm::build(
    "Research the problem space and identify constraints",
    "Implement the solution based on research findings",
);

assert_eq!(form.pattern(), "A-B");
```

### Ternary Form (A-B-A)

Depart and return. The most common form in both music and tasks. You establish a context, explore an alternative, then return — but the return is informed by the journey.

**Musical analog**: A minuet and trio. Most pop songs (verse-chorus-verse).
**Task analog**: Baseline → Experiment → Refined baseline. Context → Digression → Synthesis.

```rust
use ternary_form::TernaryForm;

let form = TernaryForm::build(
    "Establish baseline performance metrics",
    "Explore alternative optimization strategies",
    "Return to baseline approach with best improvements integrated",
);

assert_eq!(form.pattern(), "A-B-A'");
```

The third section is labeled "A'" (A prime) to indicate it's a return, but not an identical copy — it's been transformed by the B section.

### Rondo Form (A-B-A-C-A-...)

A recurring theme alternates with contrasting episodes. Perfect for iterative processes where you keep returning to a core activity.

**Musical analog**: A rondo by Mozart. The refrain keeps coming back.
**Task analog**: Sprint → Spike → Sprint → Research → Sprint. Core work punctuated by exploration.

```rust
use ternary_form::RondoForm;

let form = RondoForm::build(
    "Review and integrate findings",
    vec![
        "Explore edge case handling",
        "Benchmark performance",
        "Security audit",
    ],
);

// Pattern: A-B-A-C-A-D-A
assert_eq!(form.sections.len(), 7);
```

### Sonata Form (Exposition → Development → Recapitulation)

The most sophisticated form. Three major sections with internal structure:

1. **Exposition**: Two contrasting themes presented — the problem from two angles
2. **Development**: Themes are fragmented, combined, and transformed — exploring the tension
3. **Recapitulation**: Both themes return, now reconciled — the synthesis

**Musical analog**: Beethoven's sonatas. The first movement of most symphonies.
**Task analog**: Present stakeholder concerns, explore the conflicts, deliver a unified solution.

```rust
use ternary_form::SonataForm;

let form = SonataForm::build(
    "Define the technical constraints",
    "Identify user experience requirements",
    "Explore the tension between performance and usability",
    "Restate technical approach with UX considerations",
    "Present unified solution addressing both concerns",
);

// Development section is weighted heavier — it's where the real work happens
assert_eq!(form.sections[2].weight, 1.5);
```

## Form Detection

Detect which form a task sequence follows:

```rust
use ternary_form::{FormDetector, BinaryForm};

let form = BinaryForm::build("Phase 1", "Phase 2");
let detected = FormDetector::detect(&form);
assert_eq!(detected, Some("Binary"));

let with_confidence = FormDetector::detect_with_confidence(&form);
assert_eq!(with_confidence, Some(("Binary", 0.95)));
```

## Form Builder

Plan multi-agent tasks using musical forms with agent assignments:

```rust
use ternary_form::FormBuilder;

// Binary: researcher → implementer
let plan = FormBuilder::binary(
    "research-agent",
    "code-agent",
    "Analyze the dataset",
    "Build the pipeline",
);

// Ternary: analyst → creative → analyst
let plan = FormBuilder::ternary(
    "analyst",
    "creative-agent",
    "Baseline metrics",
    "Brainstorm improvements",
    "Refined metrics with top ideas",
);

// Rondo: coordinator with episode agents
let plan = FormBuilder::rondo(
    "coordinator",
    vec!["researcher", "designer"],
    "Sprint review",
    vec!["Research spike", "Design exploration"],
);

// Sonata: full argument structure
let plan = FormBuilder::sonata(
    ("tech-lead", "ux-lead"),      // exposition agents
    "facilitator",                   // development agent
    ("tech-lead", "ux-lead"),       // recapitulation agents
    ("Technical constraints", "User needs"),  // exposition tasks
    "Explore trade-offs",                       // development task
    ("Tech-aligned solution", "UX-aligned solution"), // recapitulation tasks
);
```

## Architecture

Each form is built from `Section` objects with labels, descriptions, agent assignments, and weights:

```rust
use ternary_form::Section;

let section = Section::new("A", "Main theme")
    .with_agent("agent-1")
    .with_weight(2.0);  // This section takes twice as long
```

A `Form` is a named sequence of sections that can report its pattern string and compute total weight:

```rust
let pattern = form.pattern();     // "A-B-A'" for ternary
let labels = form.unique_labels(); // ["A", "B"] (deduped)
let weight = form.total_weight();  // Sum of all section weights
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-form = "0.1.0"
```

## License

MIT

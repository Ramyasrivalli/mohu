# mohu-error

`mohu-error` is the shared error foundation for the entire mohu workspace.
Every crate in mohu depends on this crate and nothing else — it has zero
external dependencies of its own. It defines the central `MohuError` type,
stable numeric error codes, coarse error categories, context-chaining
helpers, and test utilities so the rest of the workspace never needs to
reinvent error handling.

---

## Table of Contents

- [Key Types](#key-types)
- [Error Domain Table](#error-domain-table)
- [Usage](#usage)
- [Macros](#macros)
- [Contributing](#contributing)

---

## Key Types

| Type | What it does |
|------|-------------|
| `MohuResult<T>` | Type alias for `Result<T, MohuError>` — used everywhere in mohu |
| `MohuError` | Central error enum — covers shape, dtype, index, and runtime errors |
| `ErrorCode` | Stable numeric code attached to every error — safe for programmatic branching |
| `ErrorKind` | Coarse category: `Usage`, `Runtime`, `System`, or `Internal` |
| `MultiError` | Accumulates multiple errors in a single pass without short-circuiting |
| `ResultExt` | Extension trait that adds `.context()` and `.with_context()` to any `Result` |
| `ErrorChain<'a>` | Iterator over nested context wrappers — useful for structured logging |
| `ErrorReporter<'a>` | Rich terminal error formatter with configurable `ReportMode` and `Severity` |

---

## Error Domain Table

Every `MohuError` variant carries a stable `ErrorCode`. Codes are grouped by
domain so downstream code can branch on ranges without matching individual
variants.

| Range | Domain | Example variants |
|-------|--------|-----------------|
| `1xxx` | Shape | `ShapeMismatch`, `DimensionMismatch` |
| `2xxx` | DType | `DTypeMismatch`, `UnsupportedDType` |
| `3xxx` | Index | `IndexOutOfBounds`, `AxisOutOfRange` |
| `4xxx` | Arithmetic | `DivisionByZero` |
| `5xxx` | Runtime | Buffer and allocation failures |
| `9xxx` | Internal | Invariant violations inside mohu itself |

---

## Usage

Add `mohu-error` to your `Cargo.toml`:

```toml
[dependencies]
mohu-error = { path = "../mohu-error" }
```

### Basic error handling

```rust
use mohu_error::{MohuError, MohuResult, ResultExt};

fn check_shapes(a: &[usize], b: &[usize]) -> MohuResult<()> {
    if a != b {
        return Err(MohuError::ShapeMismatch {
            expected: a.to_vec(),
            got: b.to_vec(),
        });
    }
    Ok(())
}

fn run() -> MohuResult<()> {
    check_shapes(&[3, 4], &[3, 5])
        .context("while validating input shapes for add")?;
    Ok(())
}
```

### `bail!` and `ensure!` macros

```rust
use mohu_error::{MohuResult, bail, ensure};

fn divide(a: f64, b: f64) -> MohuResult<f64> {
    ensure!(b != 0.0, MohuError::DivisionByZero);
    Ok(a / b)
}

fn require_rank(rank: usize) -> MohuResult<()> {
    if rank > 8 {
        bail!(MohuError::DimensionMismatch {
            expected: 8,
            got: rank,
        });
    }
    Ok(())
}
```

### Collecting multiple errors with `MultiError`

```rust
use mohu_error::{MultiError, MohuError, MohuResult};

fn validate_all(inputs: &[&[usize]]) -> MohuResult<()> {
    let mut errors = MultiError::new();

    for (i, shape) in inputs.iter().enumerate() {
        if shape.is_empty() {
            errors.push(MohuError::ShapeMismatch {
                expected: vec![1],
                got: shape.to_vec(),
            });
        }
    }

    errors.into_result()
}
```

### Branching on `ErrorCode`

```rust
use mohu_error::{MohuError, MohuResult};

fn handle(result: MohuResult<()>) {
    if let Err(e) = result {
        match e.code().range() {
            1000..=1999 => eprintln!("shape error: {e}"),
            2000..=2999 => eprintln!("dtype error: {e}"),
            3000..=3999 => eprintln!("index error: {e}"),
            _           => eprintln!("other error: {e}"),
        }
    }
}
```

---

## Macros

| Macro | What it does |
|-------|-------------|
| `bail!(err)` | Immediately returns `Err(err)` from the current function |
| `ensure!(cond, err)` | Returns `Err(err)` if `cond` is false |
| `assert_shape_eq!(a, b)` | Panics with a shape-mismatch message if `a != b` |
| `assert_axis_valid!(axis, ndim)` | Panics if `axis >= ndim` |
| `assert_in_bounds!(idx, len)` | Panics if `idx >= len` |

---

## Contributing

See the workspace [CONTRIBUTING.md](../../CONTRIBUTING.md) before opening
a PR. Every commit must be signed off with `git commit -s`.

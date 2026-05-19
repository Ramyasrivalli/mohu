//! Type promotion and casting walkthrough for `mohu-dtype`.
//!
//! Run with:
//!
//! ```sh
//! cargo run --example type_promotion -p mohu-dtype
//! ```
//!
//! This example covers:
//!   1. `DType::from_str`  — parsing NumPy-compatible dtype strings
//!   2. `promote(a, b)`    — binary type promotion rules
//!   3. `can_cast(from, to, mode)` — Safe / SameKind / Unsafe cast modes
//!   4. `DType::widen` / `narrow` chains
//!   5. `DType::real_dtype` and `complex_dtype` conversions
//!   6. `ALL_DTYPES` iteration — full dtype table

use mohu_dtype::{
    promote,
    can_cast,
    CastMode,
    DType,
    ALL_DTYPES,
};

fn main() {
    section_1_from_str();
    section_2_promote();
    section_3_can_cast();
    section_4_widen_narrow();
    section_5_real_complex();
    section_6_all_dtypes();
}

// =============================================================================
// Section 1 — DType::from_str
// =============================================================================

fn section_1_from_str() {
    println!("=================================================================");
    println!("Section 1 — DType::from_str: parsing NumPy-compatible strings");
    println!("=================================================================\n");

    let cases = [
        "bool",
        "int8",   "int16",  "int32",  "int64",
        "uint8",  "uint16", "uint32", "uint64",
        "float16", "bfloat16",
        "float32", "float64",
        "complex64", "complex128",
    ];

    for name in &cases {
        let dtype: DType = name.parse().expect("all names above are valid");
        println!("  {:>12}  =>  {:?}  (itemsize = {} bytes)",
            name, dtype, dtype.itemsize());
    }

    println!();
}

// =============================================================================
// Section 2 — promote(a, b)
// =============================================================================

fn section_2_promote() {
    println!("=================================================================");
    println!("Section 2 — promote(a, b): binary type promotion");
    println!("=================================================================\n");

    let pairs = [
        // int + int
        (DType::I32,  DType::I64,   "int + wider int"),
        (DType::I8,   DType::U8,    "signed + unsigned (same width)"),
        (DType::U32,  DType::I32,   "unsigned + signed"),
        // int + float
        (DType::I32,  DType::F32,   "int + float32"),
        (DType::I64,  DType::F32,   "int64 + float32"),
        (DType::I64,  DType::F64,   "int64 + float64"),
        // float + float
        (DType::F16,  DType::F32,   "float16 + float32"),
        (DType::F32,  DType::F64,   "float32 + float64"),
        (DType::BF16, DType::F32,   "bfloat16 + float32"),
        // anything + complex
        (DType::F32,  DType::C64,   "float32 + complex64"),
        (DType::F64,  DType::C64,   "float64 + complex64"),
        (DType::I32,  DType::C128,  "int32 + complex128"),
        (DType::C64,  DType::C128,  "complex64 + complex128"),
    ];

    println!("  {:<30}  {:<12}  {:<12}  =>  result", "description", "lhs", "rhs");
    println!("  {}", "-".repeat(70));

    for (a, b, desc) in &pairs {
        let result = promote(*a, *b);

        // Symmetry check: promote(a,b) must equal promote(b,a)
        let result_rev = promote(*b, *a);
        assert_eq!(
            result, result_rev,
            "promotion symmetry failed for {:?} and {:?}", a, b
        );

        println!("  {:<30}  {:<12}  {:<12}  =>  {:?}  ✓ symmetric",
            desc,
            format!("{:?}", a),
            format!("{:?}", b),
            result,
        );
    }

    println!("\n  All promotion pairs verified symmetric.\n");
}

// =============================================================================
// Section 3 — can_cast(from, to, mode)
// =============================================================================

fn section_3_can_cast() {
    println!("=================================================================");
    println!("Section 3 — can_cast(from, to, mode): Safe / SameKind / Unsafe");
    println!("=================================================================\n");

    println!("  Cast modes:");
    println!("    Safe      — no data loss possible (value always preserved)");
    println!("    SameKind  — within the same kind (int→int, float→float)");
    println!("    Unsafe    — always allowed regardless of precision loss\n");

    let cases: &[(DType, DType, &str)] = &[
        // Safe casts
        (DType::I32,  DType::I64,  "i32 → i64   : widening, always safe"),
        (DType::F32,  DType::F64,  "f32 → f64   : widening float, always safe"),
        (DType::U8,   DType::I16,  "u8  → i16   : fits without loss"),
        (DType::Bool, DType::I8,   "bool → i8   : 0/1 fits in i8"),
        // SameKind but not Safe
        (DType::I64,  DType::I32,  "i64 → i32   : narrowing int, same kind"),
        (DType::F64,  DType::F32,  "f64 → f32   : narrowing float, same kind"),
        // Unsafe only
        (DType::F32,  DType::I32,  "f32 → i32   : float→int, truncates"),
        (DType::I64,  DType::U32,  "i64 → u32   : signed→unsigned, may wrap"),
        (DType::C64,  DType::F32,  "c64 → f32   : complex→real, discards imag"),
    ];

    println!("  {:<45}  {:>6}  {:>9}  {:>8}",
        "cast", "Safe", "SameKind", "Unsafe");
    println!("  {}", "-".repeat(75));

    for (from, to, desc) in cases {
        let safe      = can_cast(*from, *to, CastMode::Safe);
        let same_kind = can_cast(*from, *to, CastMode::SameKind);
        let unsafe_   = can_cast(*from, *to, CastMode::Unsafe);

        println!("  {:<45}  {:>6}  {:>9}  {:>8}",
            desc,
            if safe      { "yes" } else { "no" },
            if same_kind { "yes" } else { "no" },
            if unsafe_   { "yes" } else { "no" },
        );
    }

    println!();
}

// =============================================================================
// Section 4 — DType::widen / narrow chains
// =============================================================================

fn section_4_widen_narrow() {
    println!("=================================================================");
    println!("Section 4 — DType::widen and DType::narrow chains");
    println!("=================================================================\n");

    // Widen chain: start narrow, widen until no wider type exists
    let start = DType::I8;
    print!("  widen chain from {:?}:  {:?}", start, start);
    let mut current = start;
    while let Some(wider) = current.widen() {
        print!("  ->  {:?}", wider);
        current = wider;
    }
    println!("  (no wider int)");

    let start = DType::F16;
    print!("  widen chain from {:?}: {:?}", start, start);
    let mut current = start;
    while let Some(wider) = current.widen() {
        print!("  ->  {:?}", wider);
        current = wider;
    }
    println!("  (no wider float)");

    println!();

    // Narrow chain: start wide, narrow until no narrower type exists
    let start = DType::I64;
    print!("  narrow chain from {:?}: {:?}", start, start);
    let mut current = start;
    while let Some(narrower) = current.narrow() {
        print!("  ->  {:?}", narrower);
        current = narrower;
    }
    println!("  (no narrower int)");

    let start = DType::F64;
    print!("  narrow chain from {:?}: {:?}", start, start);
    let mut current = start;
    while let Some(narrower) = current.narrow() {
        print!("  ->  {:?}", narrower);
        current = narrower;
    }
    println!("  (no narrower float)");

    println!();
}

// =============================================================================
// Section 5 — real_dtype and complex_dtype conversions
// =============================================================================

fn section_5_real_complex() {
    println!("=================================================================");
    println!("Section 5 — real_dtype and complex_dtype conversions");
    println!("=================================================================\n");

    let float_types = [DType::F16, DType::BF16, DType::F32, DType::F64];
    let complex_types = [DType::C64, DType::C128];

    println!("  float → complex_dtype:");
    for dt in &float_types {
        let c = dt.complex_dtype();
        println!("    {:?}  =>  {:?}", dt, c);
    }

    println!();
    println!("  complex → real_dtype:");
    for dt in &complex_types {
        let r = dt.real_dtype();
        println!("    {:?}  =>  {:?}", dt, r);
    }

    println!();

    // Round-trip check: real_dtype(complex_dtype(f)) == f
    println!("  Round-trip check: real_dtype(complex_dtype(x)) == x");
    for dt in &float_types {
        let round_tripped = dt.complex_dtype().real_dtype();
        assert_eq!(
            *dt, round_tripped,
            "round-trip failed for {:?}", dt
        );
        println!("    {:?}  =>  complex  =>  real  =>  {:?}  ✓", dt, round_tripped);
    }

    println!();
}

// =============================================================================
// Section 6 — ALL_DTYPES iteration
// =============================================================================

fn section_6_all_dtypes() {
    println!("=================================================================");
    println!("Section 6 — ALL_DTYPES: full dtype table");
    println!("=================================================================\n");

    println!("  {:<12}  {:>8}  {:>6}  {:>10}  {:>12}",
        "name", "itemsize", "kind", "is_float", "is_integer");
    println!("  {}", "-".repeat(56));

    for dt in ALL_DTYPES {
        println!("  {:<12}  {:>8}  {:>6}  {:>10}  {:>12}",
            format!("{:?}", dt),
            format!("{} B", dt.itemsize()),
            dt.kind_char(),
            dt.is_float(),
            dt.is_integer(),
        );
    }

    println!("\n  Total dtypes: {}", ALL_DTYPES.len());
    println!();
}

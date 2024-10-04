fn main() {
    println!("Hello, world!");
}

// This has no warnings
#[derive(diesel::Queryable, serde::Serialize)]
struct Unused {
    id: i32,
}

// This has warning: id is not used
#[derive(diesel::Queryable)]
struct UnusedWithFieldWarning {
    id: i32,
}

// This has warning: id is not used
#[derive(serde::Serialize)]
struct UnusedWithStructWarning {
    id: i32,
}

// This has warning: id is not used
struct UnusedWithWarnings {
    id: i32,
}

// This has warning: id is not used
#[derive(Default)]
struct WithDefaultDerive;

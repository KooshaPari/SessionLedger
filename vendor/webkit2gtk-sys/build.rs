// Stub build script. The real webkit2gtk-sys build script runs pkg-config
// to locate the system webkit2gtk library on Linux. On macOS (and other
// non-Linux hosts) this crate is only included in the dependency graph to
// satisfy wry's version pin — it is never compiled or linked.
fn main() {}

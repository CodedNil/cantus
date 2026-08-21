# Isthmus ideals

Isthmus should make typed Rust the single source of truth for CPU-to-GPU rendering without limiting what code can express.

## Principles

- Make the simplest correct program the easiest program to write.
- Make invalid CPU/GPU interfaces impossible to build, not errors discovered while rendering.
- Derive machinery from intent rather than asking applications to restate layouts, bindings or lifecycle boilerplate.
- Keep related CPU and GPU behaviour together as ordinary, readable, rustfmt-friendly Rust.
- Remain platform and window-system agnostic.
- Provide escape hatches without making unusual cases define the common API.
- Prefer one composable concept over several overlapping conveniences.
- Add abstraction only when it removes more complexity from users than it introduces into Isthmus.
- Treat runtime cost, memory cost, compile time and diagnostics as parts of ergonomics, not afterthoughts.

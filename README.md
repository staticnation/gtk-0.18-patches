# gtk-0.18-patches

Two crates of Tauri 2's Linux stack (gtk-rs 0.18), fixed in place, for WraithguardToolkit's
`[patch.crates-io]`. Each keeps the name and version it stands in for, so `gtk`, `webkit2gtk`
and the rest take it unchanged. Linux only in practice: nothing else builds gtk-rs.

| Crate | Stands in for | Why | What changed |
|---|---|---|---|
| `glib` | glib 0.18.5 (crates.io) | RUSTSEC-2024-0429: `VariantStrIter` passed its out-pointer as `&p`, so optimized builds read NULL | The published 0.18.5 crate, with `&mut p` in `VariantStrIter::impl_get` (`src/variant_iter.rs`), as gtk-rs-core#1343 fixed it in 0.20 |
| `proc-macro-error` | proc-macro-error 1.0.4 | RUSTSEC-2024-0370: unmaintained; glib-macros 0.18 needs it | 1.0.4's API re-exported from proc-macro-error3 3.1.1 (maintained); its `syn-error` feature uses syn 2 |
| `proc-macro-error-attr` | proc-macro-error-attr 1.0.4 | the `#[proc_macro_error]` attribute the shim exports | proc-macro-error-attr3 3.1.1's code, its expansion naming `::proc_macro_error::entry_point` |

Checked (2026-11-09) with a program on glib 0.18 using `array_iter_str()` (`collect`,
`next_back`, `nth`), `#[derive(glib::Enum)]`, `#[derive(glib::Boxed)]` and `glib::clone!`, in
release: it passes with these, and crashes (SIGSEGV) on crates.io's glib 0.18.5.

Drop the repository when Tauri's Linux webview leaves gtk-rs 0.18 (a `wry` on WebKitGTK 6).

Licenses: glib is MIT (`glib/LICENSE`, `glib/COPYRIGHT`); proc-macro-error3 and its attribute
crate are MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE` in each folder).

//! `proc-macro-error` 1.0.4, as its users call it, with proc-macro-error3 doing the work.
//!
//! The same items under the same paths: the diagnostic types and traits, the dummy
//! helpers, `entry_point` (what `#[proc_macro_error]` expands to call, as
//! `::proc_macro_error::entry_point`), the `abort!`/`emit_*!` family and the attribute
//! itself (`proc-macro-error-attr` in this repository, proc-macro-error-attr3's code with
//! that one path).

pub use proc_macro_error3::{
    abort_if_dirty, append_dummy, dummy, entry_point, set_dummy, Diagnostic, DiagnosticExt, Level, OptionExt, ResultExt,
    SpanRange,
};
#[doc(hidden)]
pub use proc_macro_error3::__export;
#[doc(hidden)]
pub use proc_macro_error3::__pme__suggestions;
pub use proc_macro_error3::{abort, abort_call_site, diagnostic, emit_call_site_error, emit_call_site_warning, emit_error, emit_warning};
pub use proc_macro_error_attr::proc_macro_error;

/// Display an error in the same style as cargo or rustc:
/// <code style="display: block; padding: 0.25em; margin: 0.5em 0;"><span style="color: red; font-weight: bold">error\[E1234\]</span><span style="color: grey; font-weight: bold">:</span> an error message
/// <span style="color: darkcyan; font-weight: bold"> --&gt; </span>examples/macros.rs:2:3</code>
///
/// # Example
///
/// ```rust
/// # use mmrbi::*;
/// error!(at: "examples/macros.rs", line: 2, col: 3, code: "E1234", "an {} message", "error");
/// error!("an {} message", "error"); // all params optional
/// ```
#[macro_export] macro_rules! error { ($($tt:tt)*) => { $crate::_logln!($crate::_log_impl::Severity::Error,    $($tt)*) }; }

/// Display a warning in the same style as cargo or rustc:
/// <code style="display: block; padding: 0.25em; margin: 0.5em 0;"><span style="color: olive; font-weight: bold">warning\[E1234\]</span><span style="color: grey; font-weight: bold">:</span> a warning message
/// <span style="color: darkcyan; font-weight: bold"> --&gt; </span>examples/macros.rs:2:3</code>
///
/// # Example
///
/// ```rust
/// # use mmrbi::*;
/// warning!(at: "examples/macros.rs", line: 2, col: 3, code: "E1234", "a {} message", "warning");
/// warning!("a {} message", "warning"); // all params optional
/// ```
#[macro_export] macro_rules! warning { ($($tt:tt)*) => { $crate::_logln!($crate::_log_impl::Severity::Warning,  $($tt)*) }; }

/// Display informational messages in the same style as cargo or rustc:
/// <code style="display: block; padding: 0.25em; margin: 0.5em 0;"><span style="color: darkcyan; font-weight: bold">info\[E1234\]</span><span style="color: grey; font-weight: bold">:</span> an informational message
/// <span style="color: darkcyan; font-weight: bold"> --&gt; </span>examples/macros.rs:2:3</code>
///
/// # Example
///
/// ```rust
/// # use mmrbi::*;
/// info!(at: "examples/macros.rs", line: 2, col: 3, code: "E1234", "a {} message", "informational");
/// info!("an {} message", "informational"); // all params optional
/// ```
#[macro_export] macro_rules! info { ($($tt:tt)*) => { $crate::_logln!($crate::_log_impl::Severity::Info,     $($tt)*) }; }

/// Display a status/progress lines in the same style as cargo or rustc:
/// <code style="display: block; padding: 0.25em; margin: 0.5em 0;"><span style="color: green; font-weight: bold">&nbsp;Documenting</span> mmrbi v0.0.0 (C:\local\mmrbi)
/// <span style="color: green; font-weight: bold">&nbsp;&nbsp;&nbsp;&nbsp;Finished</span> dev \[debuginfo\] target(s) in 0.91s</code>
///
/// # Example
///
/// ```rust
/// # use mmrbi::*;
/// status!("Documenting", "{} {} ({})", "mmrbi", "v0.0.0", r"C:\local\mmrbi");
/// status!("Finished", "{} [{}] target(s) in {}s", "dev", "debuginfo", "0.91");
/// ```
#[macro_export] macro_rules! status {
    ( $verb:expr, $fmt:literal $($tt:tt)* ) => {{
        use std::io::Write;
        let stderr = std::io::stderr();
        let mut stderr = stderr.lock();
        let _ = write!  (&mut stderr, "\u{001B}[32;1m{: >12}\u{001B}[0m ", $verb);
        let _ = writeln!(&mut stderr, $fmt $($tt)*);
    }};
}

/// Display an error in the same style as cargo or rustc, then `exit(1)`:
/// <code style="display: block; padding: 0.25em; margin: 0.5em 0;"><span style="color: red; font-weight: bold">error\[E1234\]</span><span style="color: grey; font-weight: bold">:</span> an error message
/// <span style="color: darkcyan; font-weight: bold"> --&gt; </span>examples/macros.rs:2:3</code>
///
/// # Example
///
/// ```rust,no_run
/// # use mmrbi::*;
/// fatal!(at: "examples/macros.rs", line: 2, col: 3, code: "E1234", "an {} message", "error");
/// fatal!("an {} message", "error"); // all params optional
/// ```
#[macro_export] macro_rules! fatal {
    ( $($tt:tt)* ) => {{
        $crate::error!($($tt)*);
        ::std::process::exit(1);
    }};
}

/// Display a header section:
/// <code style="display: block; color: black; background: #14CE14; padding: 0.25em; margin: 0.5em 0;">  a header  </code>
///
/// # Example
///
/// ```rust,no_run
/// # use mmrbi::*;
/// header!("  a header  ");
/// ```
#[macro_export] macro_rules! header {
    ( $fmt:literal $($tt:tt)* ) => {{
        use std::io::Write;
        let stderr = std::io::stderr();
        let mut stderr = stderr.lock();
        let _ = writeln!(&mut stderr, concat!("\u{001B}[30;102m", $fmt, "\u{001B}[0m") $($tt)*);
    }};
}



#[doc(hidden)] #[macro_export] macro_rules! _logln {
    ( $sev:expr, $($tt:tt)* ) => {{
        #[allow(unused_mut)]
        let mut ctx = $crate::_log_impl::Context {
            severity:   $sev,
            code:       "",
            at:         None,
            line:       0,
            col:        0,
        };
        $crate::_logln_inner!( ctx, $($tt)* );
    }};
}

#[doc(hidden)] #[macro_export] macro_rules! _logln_inner {
    ( $ctx:expr, code:   $code:expr, $($tt:tt)* ) => { let code = format!("[{}]", $code); $ctx.code = code.as_str(); $crate::_logln_inner!($ctx, $($tt)*); };
    ( $ctx:expr, at:     $at:expr,   $($tt:tt)* ) => { let at = $at; $ctx.at = Some(at.as_ref()); $crate::_logln_inner!($ctx, $($tt)*); };
    ( $ctx:expr, path:   $at:expr,   $($tt:tt)* ) => { let at = $at; $ctx.at = Some(at.as_ref()); $crate::_logln_inner!($ctx, $($tt)*); };
    ( $ctx:expr, line:   $line:expr, $($tt:tt)* ) => { $ctx.line = $line; $crate::_logln_inner!($ctx, $($tt)*); };
    ( $ctx:expr, col:    $col:expr,  $($tt:tt)* ) => { $ctx.col = $col; $crate::_logln_inner!($ctx, $($tt)*); };
    ( $ctx:expr, column: $col:expr,  $($tt:tt)* ) => { $ctx.col = $col; $crate::_logln_inner!($ctx, $($tt)*); };

    // Terminal rule
    ( $ctx:expr, $fmt:literal $($tt:tt)* ) => {
        use ::std::io::Write;
        $crate::_log_impl::write($ctx, |stderr| writeln!(stderr, $fmt $($tt)*));
    };
}

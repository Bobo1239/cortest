//! Macros

/// Macro for printing to the standard output
#[macro_export]
macro_rules! print {
   ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

/// Macro for printing to the standard output, with a newline
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Simple test runner
#[macro_export]
macro_rules! test {
    ($(fn $name:ident() -> bool {
        $($code:tt)*
    })+) => {
        $(
            fn $name() -> bool {
                $($code)*
            }
        )+

        #[no_mangle]
        pub fn main() -> i32 {
            let tests: &[(&str, fn() -> bool)] = &[$((stringify!($name), $name)),+];
            let total = tests.len();
            let mut failures = 0;

            println!("running {} test{}",
                     total,
                     if total == 1 { "" } else { "s" });
            for &(name, test) in tests {
                print!("test {} ... ", name);
                if test() {
                    println!("ok");
                } else {
                    println!("FAILED");
                    failures += 1;
                }
            }

            let passed = total - failures;
            println!("\ntest result: {}. {} passed; {} failed",
                     if failures == 0 { "ok" } else { "FAILED" },
                     passed,
                     failures);

            if failures == 0 {
                0
            } else {
                101
            }
        }
    }
}

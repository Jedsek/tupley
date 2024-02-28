//! Some useful macros

/// generate new [Tuple](crate::prelude::Tuple):
///
/// # Examples
///
/// ```
/// use tupley::prelude::*;
///
/// fn tuple_new() {
///     let t = tup!();
///     assert_eq!(t, Unit);
///
///     let t = tup!(1);
///     assert_eq!(t, Tuple(1, Unit));
///
///     let t = tup!(1, 2.0, "3");
///     assert_eq!(t, Tuple(1, Tuple(2.0, Tuple("3", Unit))));
/// }
/// ```
#[macro_export]
macro_rules! tup {
    () => { $crate::prelude::Unit };
    ($x:expr $(,)?) => { $crate::prelude::Tuple($x, Unit) };
    ($x:expr, $($xs:expr),+ $(,)?) => {
        $crate::prelude::Tuple($x, tup!($($xs),*))
    };
    ($($tt:tt)*) => {
        std::compile_error!("Invalid syntax");
    }
}

/// generate [Tuple](crate::prelude::Tuple) pattern for macthing:
///
/// # Examples
///
/// ```
/// use tupley::prelude::*;
///
/// fn tuple_pattern() {
///     let t = tup!();
///     let tup_pat!() = t;
///
///     let t = tup!(1);
///     let tup_pat!(a) = t;
///     assert_eq!(a, tup!(1));
///
///     let t = tup!("", 2, 3.0);
///     let tup_pat!(a, b, c) = t;
///     assert_eq!(a, "");
///     assert_eq!(b, 2);
///     assert_eq!(c, tup!(3.0));
///
///     // don't match anything
///     let t = tup!(1, 2, 3, 4, 5);
///     let tup_pat!(..) = t;
///
///     let t = tup!(1, 2.0, "", vec![1], true);
///     let tup_pat!(a, b, c) = t;
///     assert_eq!(a, 1);
///     assert_eq!(b, 2.0);
///     assert_eq!(c, tup!("", vec![1], true)); // match the rest all
/// }
/// ```
#[macro_export]
macro_rules! tup_pat {
    () => { $crate::prelude::Unit };
    (..) => { $crate::prelude::Tuple(..) };
    ($x:pat) => { $x };
    ($x:pat, $($xs:pat),+) => {
        $crate::prelude::Tuple($x, tup_pat!($($xs),*))
    };
    ($($tt:tt)*) => {
        std::compile_error!("Invalid syntax");
    };
}

/// generate new [Tuple](crate::prelude::Tuple) type:
///
/// # Examples
///
/// ```
/// use tupley::prelude::*;
///
/// fn tuple_type() {
///     let t = <tup_t!(i32, &str, bool)>::default();
///     assert_eq!(t, tup!(0, "", false));
///
///     let t: tup_t!(i32, &str, bool) = Default::default();
///     assert_eq!(t, tup!(0, "", false));
///
///     #[derive(Default, Debug, PartialEq, Eq)]
///     struct Wrapper<'a> {
///         a: tup_t!(i32, i32),
///         b: tup_t!(&'a str, bool),
///     }
///     let t = Wrapper::default();
///     assert_eq!(t, Wrapper{
///         a: tup!(0, 0),
///         b: tup!("", false),
///     })
/// }
/// ```
#[macro_export]
macro_rules! tup_t {
    () => { $crate::prelude::Unit };
    ($x:ty) => { $crate::prelude::Tuple<$x, Unit> };
    ($x:ty, $($xs:ty),+) => {
        $crate::prelude::Tuple<$x, tup_t!($($xs),*)>
    };
    ($($tt:tt)*) => {
        std::compile_error!("Invalid syntax");
    };
}

pub use tup;
pub use tup_pat;
pub use tup_t;

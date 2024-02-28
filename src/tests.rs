use crate::prelude::*;

#[test]
#[allow(clippy::unit_cmp)]
fn tuple_new() {
    let t = tup!();
    assert_eq!(t, Unit);

    let t = tup!(1);
    assert_eq!(t, Tuple(1, Unit));

    let t = tup!(1, 2.0, "3");
    assert_eq!(t, Tuple(1, Tuple(2.0, Tuple("3", Unit))));
}

#[test]
fn tuple_is_empty() {
    let t = tup!();
    assert!(t.is_empty());

    let t = tup!(1, false);
    assert!(!t.is_empty());
}

#[test]
#[rustfmt::skip]
fn tuple_type() {
    let t = <tup_t!(i32, &str, bool)>::default();
    assert_eq!(t, tup!(0, "", false));

    let t: tup_t!(i32, &str, bool) = Default::default();
    assert_eq!(t, tup!(0, "", false));

    #[derive(Default, Debug, PartialEq, Eq)]
    struct Wrapper<'a> {
        a: tup_t!(i32, i32),
        b: tup_t!(&'a str, bool),
    }
    let t = Wrapper::default();
    assert_eq!(t, Wrapper{
        a: tup!(0, 0),
        b: tup!("", false),
    })
}

#[test]
fn tuple_pattern() {
    let t = tup!();
    let tup_pat!() = t;

    let t = tup!(1);
    let tup_pat!(a) = t;
    assert_eq!(a, tup!(1));

    let t = tup!("", 2, 3.0);
    let tup_pat!(a, b, c) = t;
    assert_eq!(a, "");
    assert_eq!(b, 2);
    assert_eq!(c, tup!(3.0));

    // don't match anything
    let t = tup!(1, 2, 3, 4, 5);
    let tup_pat!(..) = t;

    let t = tup!(1, 2.0, "", vec![1], true);
    let tup_pat!(a, b, c) = t;
    assert_eq!(a, 1);
    assert_eq!(b, 2.0);
    assert_eq!(c, tup!("", vec![1], true)); // match the rest all
}

#[test]
fn tuple_len() {
    let t = tup!();
    assert_eq!(0, t.len());

    let t = tup!(1, false);
    assert_eq!(2, t.len());

    let t = tup!(1, 2, 3);
    assert_eq!(3, t.len());
}

#[test]
fn tuple_add() {
    let t1 = tup!(1, 2);
    let t2 = tup!(3.0, false, Some(1));
    let t = t1 + t2;
    assert_eq!(t, tup!(1, 2, 3.0, false, Some(1)));
}

#[test]
fn tuple_push() {
    let t = tup!();
    let t = t.push_back(1);
    let t = t.push_back("str");
    let t = t.push_back(false);
    assert_eq!(t, tup!(1, "str", false));

    let t = tup!();
    let t = t.push_front(1);
    let t = t.push_front("str");
    let t = t.push_front(false);
    assert_eq!(t, tup!(false, "str", 1));
}

#[test]
#[rustfmt::skip]
fn tuple_as_to() {
    let t = tup!(1, "str", 3.0, false);
    assert_eq!(t.as_ref(), tup!(&1, &"str", &3.0, &false));

    let mut t = tup!(1, "str", 3.0, false);
    assert_eq!(t.as_mut(), tup!(&mut 1, &mut "str", &mut 3.0, &mut false));

    let t = tup!(1, "str", 3.0, false);
    assert_eq!(t.to_some(), tup!(Some(1), Some("str"), Some(3.0), Some(false)));

    let t = tup!(1, "str", 3.0, false);
    assert_eq!(t.to_ok::<()>(), tup!(Ok(1), Ok("str"), Ok(3.0), Ok(false)));
}

#[test]
#[cfg(feature = "len-generic")]
fn tuple_len_generic() {
    let t = tup!(1, 2, 3);

    fn eq_yes<T: TupleLenEq<3>>(_: T) {}
    eq_yes(t);

    fn gt_yes<T: TupleLenGt<2>>(_: T) {}
    gt_yes(t);

    fn ge_yes_1<T: TupleLenGe<3>>(_: T) {}
    ge_yes_1(t);

    fn ge_yes_2<T: TupleLenGe<2>>(_: T) {}
    ge_yes_2(t);

    // LEN >= 3, LEN < 4
    fn range_yes<T: TupleLenRange<3, 4>>(_: T) {}
    range_yes(t);

    // The following will failed in compile time
    //
    //
    // fn ge_err<T: TupleLenGe<4>>(_: T) {}
    // ge_err(t);
    //
    //
    // fn gt_err<T: TupleLenGt<4>>(_: T) {}
    // gt_err(t);
    //
    //
    // fn eq_err<T: TupleLenEq<2>>(_: T) {}
    // eq_err(t);
}

//!

use std::ops::Add;

#[cfg(feature = "len-generic")]
pub use len::*;

#[cfg(feature = "len-generic")]
pub mod len {
    //! Some Traits could be used in generic to bound the Tuple length (in compile time).  
    //!
    //! Every [Tuple](crate::prelude::Tuple) has been implemented [TupleY] trait.  
    //! So they have const LEN: `const <Self as TupleY>::LEN: usize`  
    //!
    //! There are some traits like `TupleLenEq`, `TupleLenGt`, `TupleLenGe` could limit the passed-in param in generic  
    //!
    //! - Eq: equal to
    //! - Ge: greater than or equal to
    //! - Gt: greater than
    //! - Le: less than or equal to
    //! - lt: less than
    //!
    //! # Examples
    //!
    //! ```
    //! use tupley::prelude::*;
    //!
    //! fn len_generic() {
    //!     // The length of tuple shoule >= 1 and < 3
    //!     fn len_in_range<T: TupleLenRange<1, 3>>(_: T) {}
    //!
    //!     let t1 = tup!(1);
    //!     let t2 = tup!(1, 1);
    //!     let t3 = tup!(1, 1, 1);
    //!
    //!     len_in_range(t1);  // Yes
    //!     len_in_range(t2);  // Yes
    //!     // len_in_range(t3);  // Compile failed
    //! }
    //! ```

    use super::TupleY;

    macro_rules! impl_trait {
        ($trait:ident; $($tt:tt)*) => {
            impl<T: TupleY, const LEN: usize> $trait<LEN> for T where
                [(); $($tt)*]: {}
        };
    }

    /// Self::LEN == LEN
    pub trait TupleLenEq<const LEN: usize> {}

    /// Self::LEN >= LEN
    pub trait TupleLenGe<const LEN: usize> {}

    /// Self::LEN > LEN
    pub trait TupleLenGt<const LEN: usize> {}

    /// Self::LEN <= LEN
    pub trait TupleLenLe<const LEN: usize> {}

    /// Self::LEN < LEN
    pub trait TupleLenLt<const LEN: usize> {}

    ///  LEFT <= Self::LEN < RIGHT
    pub trait TupleLenRange<const LEFT: usize, const RIGHT: usize> {}

    impl<T: TupleY> TupleLenEq<{ Self::LEN }> for T {}
    impl_trait!(TupleLenGe; Self::LEN - LEN);
    impl_trait!(TupleLenGt; Self::LEN - LEN - 1);
    impl_trait!(TupleLenLe; LEN - Self::LEN);
    impl_trait!(TupleLenLt; LEN - 1 - Self::LEN);

    #[rustfmt::skip]
    impl<T: TupleY, const LEFT: usize, const RIGHT: usize> TupleLenRange<LEFT, RIGHT> for T where
        T: TupleLenGe<LEFT> + TupleLenLt<RIGHT>, {}
}

/// Hlist based on recursive structure.  
/// Impls [NonEmptyTuple] && [TupleY] trait.  
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tuple<First, Tail>(pub First, pub Tail);

/// Empty Tuple.  
/// Impls [EmptyTuple] && [TupleY] trait.  
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unit;

/// Trait represents non-empty Tuple.  
/// Subtrait of  [TupleY].
pub trait NonEmptyTuple: TupleY {}

/// Trait represents empty Tuple.  
/// Subtrait of  [TupleY].
pub trait EmptyTuple: TupleY {}

impl<First, Tail: TupleY> NonEmptyTuple for Tuple<First, Tail> {}
impl EmptyTuple for Unit {}

/// Trait implemented for [Tuple] and [Unit].
pub trait TupleY: Sized {
    const LEN: usize;

    type PushBackOutput<T>;

    type PushFrontOutput<T>;

    type AsRefOutput<'a>
    where
        Self: 'a;

    type AsMutOutput<'a>
    where
        Self: 'a;

    type ToSomeOutput;

    type ToOkOutput<E>;

    #[inline]
    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }

    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }

    fn push_back<T>(self, value: T) -> Self::PushBackOutput<T>;
    fn push_front<T>(self, value: T) -> Self::PushFrontOutput<T>;
    fn as_ref(&self) -> Self::AsRefOutput<'_>;
    fn as_mut(&mut self) -> Self::AsMutOutput<'_>;
    fn to_some(self) -> Self::ToSomeOutput;
    fn to_ok<E>(self) -> Self::ToOkOutput<E>;
}

impl<First, Tail: TupleY> TupleY for Tuple<First, Tail> {
    const LEN: usize = <Tail as TupleY>::LEN + 1;

    type PushBackOutput<T> = Tuple<First, Tail::PushBackOutput<T>>;
    type PushFrontOutput<T> = Tuple<T, Self>;
    type AsRefOutput<'a> = Tuple<&'a First, Tail::AsRefOutput<'a>> where Self: 'a;
    type AsMutOutput<'a> = Tuple<&'a mut First, Tail::AsMutOutput<'a>> where Self: 'a;
    type ToSomeOutput = Tuple<Option<First>, Tail::ToSomeOutput>;
    type ToOkOutput<E> = Tuple<Result<First, E>, Tail::ToOkOutput<E>>;

    fn push_back<T>(self, value: T) -> Self::PushBackOutput<T> {
        Tuple(self.0, self.1.push_back(value))
    }

    fn push_front<T>(self, value: T) -> Self::PushFrontOutput<T> {
        Tuple(value, self)
    }

    fn as_ref(&self) -> Self::AsRefOutput<'_> {
        Tuple(&self.0, self.1.as_ref())
    }
    fn as_mut(&mut self) -> Self::AsMutOutput<'_> {
        Tuple(&mut self.0, self.1.as_mut())
    }

    fn to_some(self) -> Self::ToSomeOutput {
        Tuple(Some(self.0), self.1.to_some())
    }

    fn to_ok<E>(self) -> Self::ToOkOutput<E> {
        Tuple(Ok(self.0), self.1.to_ok())
    }
}

impl TupleY for Unit {
    const LEN: usize = 0;

    type PushBackOutput<T> = Tuple<T, Unit>;
    type PushFrontOutput<T> = Tuple<T, Unit>;
    type AsRefOutput<'a> = Unit;
    type AsMutOutput<'a> = Unit;
    type ToSomeOutput = Unit;
    type ToOkOutput<E> = Unit;

    fn push_back<T>(self, value: T) -> Self::PushBackOutput<T> {
        Tuple(value, Unit)
    }

    fn push_front<T>(self, value: T) -> Self::PushFrontOutput<T> {
        Tuple(value, Unit)
    }

    fn as_ref(&self) -> Self::AsRefOutput<'_> {
        Unit
    }

    fn as_mut(&mut self) -> Self::AsMutOutput<'_> {
        Unit
    }

    fn to_some(self) -> Self::ToSomeOutput {
        Unit
    }

    fn to_ok<E>(self) -> Self::ToOkOutput<E> {
        Unit
    }
}

impl<Rhs> Add<Rhs> for Unit
where
    Rhs: TupleY,
{
    type Output = Rhs;

    fn add(self, rhs: Rhs) -> Rhs {
        rhs
    }
}

impl<First, Tail, Rhs> Add<Rhs> for Tuple<First, Tail>
where
    Rhs: TupleY,
    Tail: Add<Rhs>,
{
    type Output = Tuple<First, <Tail as Add<Rhs>>::Output>;

    fn add(self, rhs: Rhs) -> Self::Output {
        Tuple(self.0, self.1 + rhs)
    }
}

use core::{marker};

#[allow(unused_imports)]
use builtin::*;
#[allow(unused_imports)]
use builtin_macros::*;
#[allow(unused_imports)]
use crate::pervasive::*;

verus! {

/// `Seq<A>` is a sequence type for specifications.
/// To use a "sequence" in compiled code, use an `exec` type like [`vec::Vec`]
/// that has `Seq<A>` as its specification type.
///
/// An object `seq: Seq<A>` has a length, given by [`seq.len()`](Seq::len),
/// and a value at each `i` for `0 <= i < seq.len()`, given by [`seq[i]`](Seq::index).
///
/// Sequences can be constructed in a few different ways:
///  * [`Seq::empty`] construct an empty sequence (`len() == 0`)
///  * [`Seq::new`] construct a sequence of a given length, initialized according
///     to a given function mapping indices `i` to values `A`.
///  * The [`seq!`] macro, to construct small sequences of a fixed size (analagous to the
///     [`std::vec!`] macro).
///  * By manipulating an existing sequence with [`Seq::push`], [`Seq::update`],
///    or [`Seq::add`].
///
/// To prove that two sequences are equal, it is usually easiest to use the [`assert_seqs_equal!`] macro.

#[verifier(external_body)]
pub struct Seq<#[verifier(strictly_positive)] A> {
    dummy: marker::PhantomData<A>,
}

impl<A> Seq<A> {
    /// An empty sequence (i.e., a sequence of length 0).

    pub spec fn empty() -> Seq<A>;

    /// Construct a sequence `s` of length `len` where entry `s[i]` is given by `f(i)`.

    pub spec fn new(len: nat, f: impl Fn(int) -> A) -> Seq<A>;

    /// The length of a sequence.

    pub spec fn len(self) -> nat;

    /// Gets the value at the given index `i`.
    ///
    /// If `i` is not in the range `[0, self.len())`, then the resulting value
    /// is meaningless and arbitrary.

    pub spec fn index(self, i: int) -> A
        recommends 0 <= i < self.len();

    /// `[]` operator, synonymous with `index`

    #[verifier(inline)]
    pub open spec fn spec_index(self, i: int) -> A
        recommends 0 <= i < self.len()
    {
        self.index(i)
    }

    /// Appends the value `a` to the end of the sequence.
    /// This always increases the length of the sequence by 1.
    ///
    /// ## Example
    ///
    /// ```rust
    /// proof fn push_test() {
    ///     assert_seqs_equal!(
    ///           seq![10, 11, 12].push(13),
    ///           seq![10, 11, 12, 13],
    ///     );
    /// }
    /// ```

    pub spec fn push(self, a: A) -> Seq<A>;

    /// Updates the sequence at the given index, replacing the element with the given
    /// value, and leaves all other entries unchanged.
    ///
    /// ## Example
    ///
    /// ```rust
    /// proof fn update_test() {
    ///     let s = seq![10, 11, 12, 13, 14];
    ///     let t = s.update(2, -5);
    ///     assert_seqs_equal!(t, seq![10, 11, -5, 13, 14]);
    /// }
    /// ```

    pub spec fn update(self, i: int, a: A) -> Seq<A>
        recommends 0 <= i < self.len();

    /// Returns `true` if the two sequences are pointwise equal, i.e.,
    /// they have the same length and the corresponding values are equal
    /// at each index. This is equivalent to the sequences being actually equal
    /// by [`axiom_seq_ext_equal`].
    ///
    /// To prove that two sequences are equal via extensionality, it is generally easier
    /// to use the [`assert_seqs_equal!`] macro, rather than using `ext_equal` directly.

    pub open spec fn ext_equal(self, s2: Seq<A>) -> bool {
        &&& self.len() == s2.len()
        &&& (forall|i: int| 0 <= i < self.len() ==> self[i] == s2[i])
    }

    /// Returns a sequence for the given subrange.
    ///
    /// ## Example
    ///
    /// ```rust
    /// proof fn subrange_test() {
    ///     let s = seq![10, 11, 12, 13, 14];
    ///     //                  ^-------^
    ///     //          0   1   2   3   4   5
    ///     let sub = s.subrange(2, 4);
    ///     assert_seqs_equal!(sub, seq![12, 13]);
    /// }
    /// ```

    pub spec fn subrange(self, start_inclusive: int, end_exclusive: int) -> Seq<A>
        recommends 0 <= start_inclusive <= end_exclusive <= self.len();

    /// Concatenates the sequences.
    ///
    /// ## Example
    ///
    /// ```rust
    /// proof fn add_test() {
    ///     assert_seqs_equal!(
    ///         seq![10, 11].push(seq![12, 13, 14]),
    ///         seq![10, 11, 12, 13, 14],
    ///     );
    /// }
    /// ```

    pub spec fn add(self, rhs: Seq<A>) -> Seq<A>;

    /// `+` operator, synonymous with `add`

    #[verifier(inline)]
    pub open spec fn spec_add(self, rhs: Seq<A>) -> Seq<A> {
        self.add(rhs)
    }

    /// Returns the last element of the sequence.

    pub open spec fn last(self) -> A
        recommends 0 < self.len()
    {
        self[self.len() as int - 1]
    }
}

// Trusted axioms

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_empty<A>()
    ensures
        #[trigger] Seq::<A>::empty().len() == 0,
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_new_len<A>(len: nat, f: FnSpec(int) -> A)
    ensures
        #[trigger] Seq::new(len, f).len() == len,
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_new_index<A>(len: nat, f: FnSpec(int) -> A, i: int)
    requires
        0 <= i < len,
    ensures
        Seq::new(len, f)[i] == f(i),
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_push_len<A>(s: Seq<A>, a: A)
    ensures
        #[trigger] s.push(a).len() == s.len() + 1,
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_push_index_same<A>(s: Seq<A>, a: A, i: int)
    requires
        i == s.len(),
    ensures
        #[trigger] s.push(a)[i] == a,
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_push_index_different<A>(s: Seq<A>, a: A, i: int)
    requires
        0 <= i < s.len(),
    ensures
        s.push(a)[i] == s[i],
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_update_len<A>(s: Seq<A>, i: int, a: A)
    requires
        0 <= i < s.len(),
    ensures
        #[trigger] s.update(i, a).len() == s.len(),
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_update_same<A>(s: Seq<A>, i: int, a: A)
    requires
        0 <= i < s.len(),
    ensures
        #[trigger] s.update(i, a)[i] == a,
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_update_different<A>(s: Seq<A>, i1: int, i2: int, a: A)
    requires
        0 <= i1 < s.len(),
        0 <= i2 < s.len(),
        i1 != i2,
    ensures
        s.update(i2, a)[i1] == s[i1],
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_ext_equal<A>(s1: Seq<A>, s2: Seq<A>)
    ensures
        s1.ext_equal(s2) == (s1 == s2),
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_subrange_len<A>(s: Seq<A>, j: int, k: int)
    requires
        0 <= j <= k <= s.len(),
    ensures
        #[trigger] s.subrange(j, k).len() == k - j,
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_subrange_index<A>(s: Seq<A>, j: int, k: int, i: int)
    requires
        0 <= j <= k <= s.len(),
        0 <= i < k - j,
    ensures
        s.subrange(j, k)[i] == s[i + j],
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_add_len<A>(s1: Seq<A>, s2: Seq<A>)
    ensures #[trigger] s1.add(s2).len() == s1.len() + s2.len()
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_add_index1<A>(s1: Seq<A>, s2: Seq<A>, i: int)
    requires
        0 <= i < s1.len(),
    ensures
        s1.add(s2)[i] == s1[i],
{
}

#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub proof fn axiom_seq_add_index2<A>(s1: Seq<A>, s2: Seq<A>, i: int)
    requires
        0 <= s1.len(),
        i < s1.len() as int + s2.len(),
    ensures
        s1.add(s2)[i] == s2[i - s1.len()],
{
}

#[doc(hidden)]
#[macro_export]
macro_rules! seq_internal {
    [$($elem:expr),* $(,)?] => {
        $crate::pervasive::seq::Seq::empty()
            $(.push($elem))*
    }
}

/// Creates a [`Seq`] containing the given elements.
///
/// ## Example
///
/// ```rust
/// let s = seq![11, 12, 13];
///
/// assert(s.len() == 3);
/// assert(s[0] == 11);
/// assert(s[1] == 12);
/// assert(s[2] == 13);
/// ```

#[macro_export]
macro_rules! seq {
    [$($tail:tt)*] => {
        ::builtin_macros::verus_proof_macro_exprs!($crate::pervasive::seq::seq_internal!($($tail)*))
    };
}

#[doc(hidden)]
pub use seq_internal;
pub use seq;

} // verus!

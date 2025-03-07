#[allow(unused_imports)] use builtin::*;
#[allow(unused_imports)] use builtin_macros::*;
#[allow(unused_imports)] use crate::pervasive::*;
//use core::marker::PhantomData;

verus! {

// TODO: the *_exec* functions would be better in builtin,
// but it's painful to implement the support in erase.rs at the moment.
#[verifier(external_body)]
pub fn ghost_exec<A>(#[verifier::spec] a: A) -> (s: Ghost<A>)
    ensures a == s@,
{
    Ghost::assume_new()
}

#[verifier(external_body)]
pub fn tracked_exec<A>(#[verifier::proof] a: A) -> (s: Tracked<A>)
    ensures a == s@
{
    opens_invariants_none();
    Tracked::assume_new()
}

#[verifier(external_body)]
pub fn tracked_exec_borrow<'a, A>(#[verifier::proof] a: &'a A) -> (s: &'a Tracked<A>)
    ensures *a == s@
{
    opens_invariants_none();

    // TODO: implement this (using unsafe) or mark function as ghost (if supported by Rust)
    unimplemented!();
}

// REVIEW: consider moving these into builtin and erasing them from the VIR
pub struct Gho<A>(pub ghost A);
pub struct Trk<A>(pub tracked A);

#[inline(always)]
#[verifier(external_body)]
pub fn ghost_unwrap_gho<A>(a: Ghost<Gho<A>>) -> (ret: Ghost<A>)
    ensures a@.0 == ret@
{
    Ghost::assume_new()
}

#[inline(always)]
#[verifier(external_body)]
pub fn tracked_unwrap_gho<A>(a: Tracked<Gho<A>>) -> (ret: Tracked<A>)
    ensures a@.0 == ret@
{
    Tracked::assume_new()
}

#[inline(always)]
#[verifier(external_body)]
pub fn tracked_unwrap_trk<A>(a: Tracked<Trk<A>>) -> (ret: Tracked<A>)
    ensures a@.0 == ret@
{
    Tracked::assume_new()
}

#[verifier(external_body)]
pub proof fn tracked_swap<V>(tracked a: &mut V, tracked b: &mut V)
    ensures
        a == old(b),
        b == old(a)
{
    unimplemented!();
}

// TODO: replace Spec and Proof entirely with Ghost and Tracked

/*
#[verifier(external_body)]
pub struct Spec<#[verifier(strictly_positive)] A> {
    phantom: PhantomData<A>,
}
*/

#[cfg(not(verus_macro_erase_ghost))]
pub struct Proof<A>(
    #[verifier::proof] pub A,
);
#[cfg(verus_macro_erase_ghost)]
pub struct Proof<A>(
    #[verifier::proof] pub std::marker::PhantomData<A>,
);

/*
impl<A> Spec<A> {
    fndecl!(pub fn value(self) -> A);

    #[verifier(external_body)]
    pub fn exec(#[verifier::spec] a: A) -> Spec<A> {
        ensures(|s: Spec<A>| equal(a, s.value()));
        Spec { phantom: PhantomData }
    }

    #[verifier::proof]
    #[verifier(returns(proof))]
    #[verifier(external_body)]
    pub fn proof(a: A) -> Spec<A> {
        ensures(|s: Spec<A>| equal(a, s.value()));
        Spec { phantom: PhantomData }
    }
}

impl<A> Clone for Spec<A> {
    #[verifier(external_body)]
    fn clone(&self) -> Self {
        Spec { phantom: PhantomData }
    }
}

impl<A> Copy for Spec<A> {
}

impl<A> PartialEq for Spec<A> {
    #[verifier(external_body)]
    fn eq(&self, _rhs: &Spec<A>) -> bool {
        true
    }
}

impl<A> Eq for Spec<A> {
}
*/

impl<A> PartialEq for Proof<A> {
    #[verifier(external_body)]
    fn eq(&self, _rhs: &Proof<A>) -> bool {
        true
    }
}

impl<A> Eq for Proof<A> {
}

#[cfg(not(verus_macro_erase_ghost))]
#[allow(dead_code)]
#[inline(always)]
#[verifier(external_body)]
pub fn exec_proof_from_false<A>() -> Proof<A>
    requires false
{
    Proof(proof_from_false())
}

#[cfg(verus_macro_erase_ghost)]
#[allow(dead_code)]
#[inline(always)]
#[verifier(external_body)]
pub fn exec_proof_from_false<A>() -> Proof<A>
    requires false
{
    Proof(std::marker::PhantomData::default())
}

} // verus

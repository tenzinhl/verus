#![feature(rustc_private)]
#[macro_use]
mod common;
use common::*;

test_verify_one_file! {
    #[test] struct1 code! {
        struct S {
            #[verifier::spec] i: bool,
            j: bool,
        }
        fn test1(i: bool, j: bool) {
            let s = S { i, j };
        }
        fn test2(#[verifier::spec] i: bool, j: bool) {
            let s = S { i, j };
        }
        fn test3(i: bool, #[verifier::spec] j: bool) {
            #[verifier::spec] let s = S { i, j };
            #[verifier::spec] let jj = s.j;
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] verus_struct1 verus_code! {
        use crate::pervasive::modes::*;
        struct S {
            i: Ghost<bool>,
            j: bool,
        }
        fn test1(i: bool, j: bool) {
            let s = S { i: ghost(i), j };
        }
        fn test2(i: Ghost<bool>, j: bool) {
            let s = S { i, j };
        }
        fn test3(i: bool, j: Ghost<bool>) {
            let s: Ghost<S> = ghost(S { i: Ghost::new(i), j: j@ });
            let jj: Ghost<bool> = ghost(s@.j);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] struct_fails1 code! {
        struct S {
            #[verifier::spec] i: bool,
            j: bool,
        }
        fn test(i: bool, #[verifier::spec] j: bool) {
            let s = S { i, j };
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] verus_struct_fails1 verus_code! {
        use crate::pervasive::modes::*;
        struct S {
            i: Ghost<bool>,
            j: bool,
        }
        fn test(i: bool, j: Ghost<bool>) {
            let s = S { i: ghost(i), j: j@ };
        }
    } => Err(err) => assert_error_msg(err, "cannot perform operation with mode spec")
}

test_verify_one_file! {
    #[test] struct_fails1b code! {
        struct S {
            #[verifier::spec] i: bool,
            j: bool,
        }
        fn test(i: bool, #[verifier::spec] j: bool) {
            let s = S { j, i };
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] struct_fails2 code! {
        struct S {
            #[verifier::spec] i: bool,
            j: bool,
        }
        fn test(i: bool, j: bool) {
            let s = S { i, j };
            let ii = s.i;
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] struct_fails3 code! {
        struct S {
            #[verifier::spec] i: bool,
            j: bool,
        }
        fn test(i: bool, #[verifier::spec] j: bool) {
            #[verifier::spec] let s = S { i, j };
            let jj = s.j;
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] struct_fails4a verus_code! {
        struct S {
            ghost i: bool,
            j: bool,
        }
        fn test(s: Ghost<S>) -> bool {
            s@.j
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot perform operation with mode spec")
}

test_verify_one_file! {
    #[test] struct_fails4b verus_code! {
        struct S {
            ghost i: bool,
            j: bool,
        }
        fn test(s: &Ghost<S>) -> bool {
            s@.j
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot perform operation with mode spec")
}

test_verify_one_file! {
    #[test] struct_fails4c verus_code! {
        struct S {
            ghost i: bool,
            j: bool,
        }
        fn test(s: Ghost<&S>) -> bool {
            s@.j
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot perform operation with mode spec")
}

test_verify_one_file! {
    #[test] struct_fails5a verus_code! {
        struct S {
            ghost i: bool,
            j: bool,
        }
        impl S {
            spec fn get_j(self) -> bool {
                self.j
            }
        }
        fn test(s: Ghost<S>) -> bool {
            s@.get_j()
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot call function with mode spec")
}

test_verify_one_file! {
    #[test] struct_fails5b verus_code! {
        struct S {
            ghost i: bool,
            j: bool,
        }
        impl S {
            spec fn get_j(self) -> bool {
                self.j
            }
        }
        fn test(s: &Ghost<S>) -> bool {
            s@.get_j()
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot call function with mode spec")
}

test_verify_one_file! {
    #[test] struct_fails5c verus_code! {
        struct S {
            ghost i: bool,
            j: bool,
        }
        impl S {
            spec fn get_j(self) -> bool {
                self.j
            }
        }
        fn test(s: Ghost<&S>) -> bool {
            s@.get_j()
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot call function with mode spec")
}

test_verify_one_file! {
    #[test] tuple1 code! {
        fn test1(i: bool, j: bool) {
            let s = (i, j);
        }
        fn test3(i: bool, #[verifier::spec] j: bool) {
            #[verifier::spec] let s = (i, j);
            #[verifier::spec] let ii = s.0;
            #[verifier::spec] let jj = s.1;
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] tuple_fails1 code! {
        fn test(i: bool, #[verifier::spec] j: bool) {
            let s = (i, j);
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] tuple_fails2 code! {
        fn test(i: bool, j: bool) {
            #[verifier::spec] let s = (i, j);
            let ii = s.0;
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] tuple_fails3 code! {
        fn test(i: bool, #[verifier::spec] j: bool) {
            #[verifier::spec] let s = (i, j);
            let jj = s.0;
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] spec_struct_not_exec verus_code! {
        ghost struct Set<A> {
            pub dummy: A,
        }

        fn set_exec() {
            let a: Set<u64> = Set { dummy: 3 }; // FAILS
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] spec_enum_not_exec verus_code! {
        ghost enum E {
            A,
            B,
        }

        fn set_exec() {
            let e: E = E::A; // FAILS
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] eq_mode code! {
        fn eq_mode(#[verifier::spec] i: u128) {
            #[verifier::spec] let b: bool = i == 13;
        }
    } => Ok(_)
}

test_verify_one_file! {
    #[test] if_spec_cond code! {
        fn if_spec_cond(#[verifier::spec] i: u128) -> u64 {
            let mut a: u64 = 2;
            if i == 3 {
                a = a + 1; // ERROR
            }
            a
        }
    } => Err(err) => assert_error_msg(err, "cannot assign to exec variable from proof mode")
}

test_verify_one_file! {
    #[test] if_spec_cond_proof code! {
        #[verifier::proof]
        fn if_spec_cond_proof(i: u128) -> u64 {
            let mut a: u64 = 2;
            if i == 3 {
                a = a + 1;
            }
            a
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] regression_int_if code! {
        fn int_if() {
            #[verifier::spec] let a: u128 = 3;
            if a == 4 {
                assert(false);
            }; // TODO not require the semicolon here?
        }

        #[verifier::spec]
        fn int_if_2(a: u128) -> u128 {
            if a == 2 {
                3
            } else if a == 3 {
                4
            } else {
                arbitrary()
            }
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] ret_mode code! {
        #[verifier(returns(spec))] /* vattr */
        fn ret_spec() -> u128 {
            ensures(|i: u128| i == 3);
            #[verifier::spec] let a: u128 = 3;
            a
        }

        fn test_ret() {
            #[verifier::spec] let x = ret_spec();
            assert(x == 3);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] ret_mode_fail2 code! {
        #[verifier(returns(spec))] /* vattr */
        fn ret_spec() -> u128 {
            ensures(|i: u128| i == 3);
            #[verifier::spec] let a: u128 = 3;
            a
        }

        fn test_ret() {
            let x = ret_spec();
            assert(x == 3);
        }
    } => Err(err) => assert_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] ret_mode_fail_requires code! {
        fn f() {
            requires({while false {}; true});
        }
    } => Err(err) => assert_vir_error_msg(err, "expected pure mathematical expression")
}

test_verify_one_file! {
    #[test] spec_let_decl_init_fail code! {
        #[verifier::spec]
        fn test1() -> u64 {
            let x: u64;
            x = 23;
            x
        }
    } => Err(err) => assert_vir_error_msg(err, "delayed assignment to non-mut let not allowed for spec variables")
}

test_verify_one_file! {
    #[test] let_spec_pass code! {
        fn test1() {
            #[verifier::spec] let x: u64 = 2;
            assert(x == 2);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] decl_init_let_spec_fail code! {
        fn test1() {
            #[verifier::spec] let x: u64;
            x = 2;
            x = 3;
            assert(false); // FAILS
        }
    } => Err(err) => assert_vir_error_msg(err, "delayed assignment to non-mut let not allowed for spec variables")
}

const FIELD_UPDATE: &str = code_str! {
    #[derive(PartialEq, Eq, Structural)]
    struct S {
        #[verifier::spec] a: u64,
        b: bool,
    }
};

test_verify_one_file! {
    #[test] test_field_update_fail FIELD_UPDATE.to_string() + code_str! {
        fn test() {
            let mut s = S { a: 5, b: false };
            #[verifier::spec] let b = true;
            s.b = b;
        }
    } => Err(err) => assert_vir_error_msg(err, "expression has mode spec, expected mode exec")
}

test_verify_one_file! {
    #[test] test_mut_ref_field_fail FIELD_UPDATE.to_string() + code_str! {
        fn muts_exec(a: &mut u64) {
            requires(*old(a) < 30);
            ensures(*a == *old(a) + 1);
            *a = *a + 1;
        }

        fn test() {
            let mut s = S { a: 5, b: false };
            muts_exec(&mut s.a);
        }
    } => Err(err) => assert_vir_error_msg(err, "expected mode exec, &mut argument has mode spec")
}

const PROOF_FN_COMMON: &str = code_str! {
    #[verifier::proof]
    struct Node {
        v: u32,
    }
};

test_verify_one_file! {
    #[test] test_mut_arg_fail1 code! {
        #[verifier::proof]
        fn f(#[verifier::proof] x: &mut bool, #[verifier::proof] b: bool) {
            requires(b);
            ensures(*x);

            *x = b;
        }

        fn g(#[verifier::proof] b: bool) {
            requires(b);

            #[verifier::spec] let tr = true;
            let mut e = false;
            if tr {
                f(&mut e, b); // should fail: exec <- proof out assign
            }
            assert(e);
        }
    } => Err(err) => assert_vir_error_msg(err, "expected mode proof, &mut argument has mode exec")
}

test_verify_one_file! {
    #[test] test_mut_arg_fail2 verus_code! {
        proof fn f(x: &mut bool)
            ensures *x
        {
            *x = true;
        }

        fn g() {
            let mut e = false;
            proof {
                f(&mut e); // fails, exec <- ghost out assign
            }
            assert(e);
        }
    } => Err(err) => assert_vir_error_msg(err, "expected mode spec, &mut argument has mode proof")
}

test_verify_one_file! {
    #[test] test_mut_arg_fail3 verus_code! {
        struct S {
            ghost g: bool,
        }

        fn f(x: &mut bool) {}

        fn g(e: S) {
            let mut e = e;
            f(&mut e.g); // fails, exec <- ghost assign
        }
    } => Err(err) => assert_vir_error_msg(err, "expected mode exec, &mut argument has mode spec")
}

test_verify_one_file! {
    #[test] test_mut_arg_fail4 verus_code! {
        struct S {
            e: bool,
        }

        proof fn f(tracked x: &mut bool) {}

        proof fn g(g: S) {
            let mut g = g;
            f(&mut g.e); // fails, tracked <- ghost assign
        }
    } => Err(err) => assert_vir_error_msg(err, "expected mode proof, &mut argument has mode spec")
}

test_verify_one_file! {
    #[test] test_mut_arg_fail5 verus_code! {
        struct S {
            e: bool,
        }

        proof fn f(x: &mut bool) {}

        fn g(e: S) {
            let mut e = e;
            proof {
                f(&mut e.e); // fails, exec <- ghost out assign
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "expected mode spec, &mut argument has mode proof")
}

test_verify_one_file! {
    #[test] test_mut_arg_fail6 verus_code! {
        struct S {
            tracked t: bool,
        }

        proof fn f(x: &mut bool) {}

        fn g(e: S) {
            let mut e = e;
            proof {
                f(&mut e.t); // fails, tracked <- ghost out assign
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "expected mode spec, &mut argument has mode proof")
}

test_verify_one_file! {
    #[test] test_proof_fn_call_fail PROOF_FN_COMMON.to_string() + code_str! {
        #[verifier::proof]
        fn lemma(#[verifier::proof] node: Node) {
            requires(node.v < 10);
            ensures(node.v * 2 < 20);
        }

        #[verifier::proof]
        fn other(#[verifier::proof] node: Node) {
            assume(node.v < 10);
            lemma(node);
            lemma(node);
        }
    } => Err(err) => assert_error_msg(err, "error[E0382]: use of moved value: `node`")
}

test_verify_one_file! {
    #[test] test_associated_proof_fn_call_pass PROOF_FN_COMMON.to_string() + code_str! {
        impl Node {
            #[verifier::proof]
            fn lemma(&self) {
                requires(self.v < 10);
                ensures(self.v * 2 < 20);
            }

            #[verifier::proof]
            fn other(&self, other_node: Node) {
                assume(other_node.v < 10);
                other_node.lemma();
            }
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_associated_proof_fn_call_fail_1 PROOF_FN_COMMON.to_string() + code_str! {
        impl Node {
            #[verifier::proof]
            fn lemma(#[verifier::proof] self) {
                requires(self.v < 10);
                ensures(self.v * 2 < 20);
            }

            #[verifier::proof]
            fn other(#[verifier::proof] self) {
                assume(other_node.v < 10);
                self.lemma();
                self.lemma();
            }
        }
    } => Err(err) => assert_error_msg(err, "cannot find value `other_node`")
}

test_verify_one_file! {
    // TODO un-ignore when #124 is fixed
    #[test] #[ignore] test_associated_proof_fn_call_fail_2_regression_124 PROOF_FN_COMMON.to_string() + code_str! {
        struct Token {}

        impl Node {
            #[verifier::proof]
            fn lemma(self, #[verifier::proof] t: Token) {}

            #[verifier::proof]
            fn other(self, #[verifier::proof] t: Token) {
                self.lemma(t);
                self.lemma(t);
            }
        }
    } => Err(err) => assert_error_msg(err, "test currently ignored")
}

test_verify_one_file! {
    #[test] assign_from_proof code! {
        fn myfun(#[verifier::spec] a: bool) -> bool {
            let mut b = false;
            if a {
                b = true;
            }
            b
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot assign to exec variable from proof mode")
}

test_verify_one_file! {
    #[test] tracked_double_deref code! {
        use pervasive::modes::*;

        fn foo<V>(x: Tracked<V>) {
            let y = &x;

            assert(equal((*y).view(), x.view()));
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] ghost_wrapper_assign_fail1 verus_code! {
        use pervasive::modes::*;

        fn f() {
            let g: Ghost<bool> = ghost(true);
            proof {
                let tracked t: bool = g@; // fails: tracked <- ghost assign
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "expression has mode spec, expected mode proof")
}

test_verify_one_file! {
    #[test] ghost_wrapper_assign_fail2 verus_code! {
        use pervasive::modes::*;

        fn f() {
            let g: Ghost<bool> = ghost(true);
            let e: bool = g@; // fails: exec <- ghost assign
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot perform operation with mode spec")
}

test_verify_one_file! {
    #[test] ghost_wrapper_assign_fail3 verus_code! {
        use pervasive::modes::*;

        fn f() {
            let mut e: bool = false;
            proof {
                e = true; // fails: exec assign from proof mode
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot assign to exec variable from proof mode")
}

test_verify_one_file! {
    #[test] ghost_wrapper_assign_fail4 verus_code! {
        use pervasive::modes::*;

        fn f(t: Tracked<bool>) {
            let g: Ghost<bool> = ghost(true);
            let mut t = t;
            proof {
                t@ = g@; // fails: tracked <- ghost assign
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "expression has mode spec, expected mode proof")
}

test_verify_one_file! {
    #[test] ghost_wrapper_call_fail1 verus_code! {
        use pervasive::modes::*;

        fn f(x: bool) {
        }

        fn g(g: Ghost<bool>) {
            f(g@); // fails, exec <- ghost assign
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot perform operation with mode spec")
}

test_verify_one_file! {
    #[test] ghost_wrapper_call_fail2 verus_code! {
        use pervasive::modes::*;

        fn f(x: bool) {
        }

        fn g(t: Tracked<bool>) {
            f(t@); // fails, exec <- tracked assign
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot perform operation with mode spec")
}

test_verify_one_file! {
    #[test] ghost_wrapper_call_fail3 verus_code! {
        use pervasive::modes::*;

        proof fn f(tracked x: bool) {
        }

        fn g(g: Ghost<bool>) {
            proof {
                f(g@); // fails, tracked <- ghost assign
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "expression has mode spec, expected mode proof")
}

test_verify_one_file! {
    #[test] ghost_wrapper_call_mut_fail1 verus_code! {
        use pervasive::modes::*;

        fn f(x: &mut bool) {
        }

        fn g(g: Ghost<bool>) {
            let mut g = g;
            f(g.borrow_mut()); // fails, exec <- ghost assign
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot perform operation with mode proof")
}

test_verify_one_file! {
    #[test] ghost_wrapper_call_mut_fail2 verus_code! {
        use pervasive::modes::*;

        fn f(x: &mut bool) {
        }

        fn g(t: Tracked<bool>) {
            let mut t = t;
            f(t.borrow_mut()); // fails, exec <- tracked assign
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot perform operation with mode proof")
}

test_verify_one_file! {
    #[test] ghost_wrapper_call_mut_fail3 verus_code! {
        use pervasive::modes::*;

        proof fn f(tracked x: &mut bool) {
        }

        fn g(g: Ghost<bool>) {
            let mut g = g;
            proof {
                f(g.borrow_mut()); // fails, tracked <- ghost assign
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "expected mode proof, &mut argument has mode spec")
}

test_verify_one_file! {
    #[test] ghost_wrapper_call_mut_fail4 verus_code! {
        use pervasive::modes::*;

        proof fn f(x: &mut bool) {
        }

        fn g(t: Tracked<bool>) {
            let mut t = t;
            proof {
                f(t.borrow_mut()); // fails, tracked <- ghost out assign
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "expected mode spec, &mut argument has mode proof")
}

test_verify_one_file! {
    #[test] ghost_wrapper_assign_struct_fail1 verus_code! {
        use pervasive::modes::*;
        struct S {
            e: bool,
        }
        fn f(g: Ghost<S>) {
            proof {
                let tracked t: bool = g@.e; // fails: tracked <- ghost assign
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "expression has mode spec, expected mode proof")
}

test_verify_one_file! {
    #[test] ghost_wrapper_assign_struct_fail2 verus_code! {
        use pervasive::modes::*;
        struct S {
            e: bool,
        }
        fn f(g: Ghost<S>) {
            let e: bool = g@.e; // fails: exec <- ghost assign
        }
    } => Err(err) => assert_vir_error_msg(err, "cannot perform operation with mode spec")
}

test_verify_one_file! {
    #[test] ghost_wrapper_assign_struct_fail3 verus_code! {
        use pervasive::modes::*;
        struct S {
            e: bool,
        }
        fn f(t: Tracked<S>) {
            let g: Ghost<bool> = ghost(true);
            let mut t = t;
            proof {
                t@.e = g@; // fails: tracked <- ghost assign
            }
        }
    } => Err(err) => assert_vir_error_msg(err, "expression has mode spec, expected mode proof")
}

const TRACKED_TYP_PARAMS_COMMON: &str = verus_code_str! {
    tracked struct Tok {
        v: nat,
    }

    tracked struct B<T> {
        t: T,
    }
};

test_verify_one_file! {
    #[test] tracked_ghost_typ_params_make verus_code! {
        use pervasive::modes::*;

        tracked struct Tok {
            ghost v: nat,
        }

        struct B<T> {
            t: T,
        }

        proof fn make_tracked_proof() {
            let tracked t2: B<Tok> = tracked(B { t: Tok { v: 12 } });
        }

        fn make_tracked_exec() {
            let t: Tracked<Tok> = tracked({
                let v = 12nat;
                Tok { v: v }
            });
            let b: B<Tracked<Tok>> = B { t: t };
        }

        // This isn't currently possible
        // proof fn make_ghost_proof() {
        //     let tracked t2: B<Ghost<Tok>> = tracked(B { t: Ghost::new(Tok { v: 12 }) });
        // }

        fn make_ghost_exec() {
            let g: Ghost<Tok> = ghost(Tok { v: 12nat });
            let b: B<Ghost<Tok>> = B { t: g };
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] tracked_tracked_typ_params_misc TRACKED_TYP_PARAMS_COMMON.to_owned() + verus_code_str! {
        proof fn identity(tracked b: B<Tracked<Tok>>) -> (tracked out: B<Tracked<Tok>>) {
            tracked b
        }

        fn foo_exec(tok: Tracked<Tok>) -> Tracked<Tok> {
            let b: Tracked<B<Tracked<Tok>>> = tracked(B { t: tok });
            let t = tracked({
                let tracked B { t } = (tracked b).get();
                t.get()
            });
            t
        }

        proof fn foo_proof(tracked tok: Tracked<Tok>) -> (tracked out: B<Tracked<Tok>>) {
            let tracked b1: B<Tracked<Tok>> = tracked(B { t: tracked tok });
            let tracked b2 = identity(tracked b1);
            tracked b2
        }

        fn caller(tok: Tracked<Tok>) -> Tracked<B<Tracked<Tok>>> {
            let b: Tracked<B<Tracked<Tok>>> = tracked(B { t: tok });
            let b1 = tracked({
                identity(tracked b.get())
            });
            b1
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] tracked_ghost_typ_params_misc TRACKED_TYP_PARAMS_COMMON.to_owned() + verus_code_str! {
        use pervasive::modes::*;

        proof fn identity(tracked b: B<Ghost<Tok>>) -> (tracked out: B<Ghost<Tok>>) {
            tracked b
        }

        fn foo_exec() -> Ghost<Tok> {
            let g: Ghost<Tok> = ghost(Tok { v: 12nat });
            // The exec->tracked coercion may be removed
            let b: Tracked<B<Ghost<Tok>>> = tracked(B { t: (tracked g) });
            let t = ghost({
                let tracked B { t } = (tracked b).get();
                t@
            });
            t
        }

        proof fn foo_proof(tracked tok: Ghost<Tok>) -> tracked Ghost<Tok> {
            let tracked b: B<Ghost<Tok>> = tracked(B { t: tok });
            let tracked t = tracked({
                let tracked B { t } = tracked b;
                t
            });
            tracked t
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] test_or_pattern_mode_inconsistent verus_code! {
        enum Foo {
            Bar(#[verifier::spec] u64),
            Qux(#[verifier::proof] u64),
        }

        proof fn blah(foo: Foo) {
            #[verifier::proof] let (Foo::Bar(x) | Foo::Qux(x)) = foo;
        }
    } => Err(err) => assert_vir_error_msg(err, "variable `x` has different modes across alternatives")
}

test_verify_one_file! {
    #[test] test_or_pattern_mode_inconsistent2 verus_code! {
        enum Foo {
            Bar(#[verifier::spec] u64, #[verifier::proof] u64),
        }

        proof fn blah(foo: Foo) {
            #[verifier::proof] let (Foo::Bar(x, y) | Foo::Bar(y, x)) = foo;
        }
    } => Err(err) => assert_vir_error_msg(err, "variable `x` has different modes across alternatives")
}

test_verify_one_file! {
    // TODO(utaal) issue with tracked rewrite, I believe
    #[ignore] #[test] test_struct_pattern_fields_out_of_order_fail_issue_348 verus_code! {
        struct Foo {
            ghost a: u64,
            tracked b: u64,
        }

        proof fn some_call(#[verifier::proof] y: u64) { }

        proof fn t() {
            let tracked foo = Foo { a: 5, b: 6 };
            let tracked Foo { b, a } = foo;

            // Variable 'a' has the mode of field 'a' (that is, spec)
            // some_call requires 'proof'
            // So this should fail
            some_call(a);
        }
    } => Err(err) => assert_vir_error_msg(err, "expression has mode spec, expected mode proof")
}

test_verify_one_file! {
    #[test] test_struct_pattern_fields_out_of_order_success_issue_348 verus_code! {
        struct X { }

        struct Foo {
            #[verifier::spec] a: u64,
            #[verifier::proof] b: X,
        }

        proof fn some_call(#[verifier::proof] y: X) { }

        proof fn t(#[verifier::proof] x: X) {
            #[verifier::proof] let foo = Foo { a: 5, b: x };
            #[verifier::proof] let Foo { b, a } = foo;

            // This should succeed, 'b' has mode 'proof'
            some_call(b);
        }
    } => Ok(())
}

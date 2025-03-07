#![allow(unused_imports)]

// ANCHOR: full
use builtin::*;
use builtin_macros::*;
mod pervasive;
use pervasive::*;
use crate::pervasive::{atomic_ghost::*};
use crate::pervasive::{modes::*};
use crate::pervasive::{thread::*};
use state_machines_macros::tokenized_state_machine;
use crate::pervasive::result::*;
use std::sync::Arc;

verus_old_todo_no_ghost_blocks!{

tokenized_state_machine!(
    X {
        fields {
            #[sharding(variable)]
            pub counter: int,

            #[sharding(variable)]
            pub inc_a: bool,

            #[sharding(variable)]
            pub inc_b: bool,
        }

        // ANCHOR: inv 
        #[invariant]
        pub fn main_inv(&self) -> bool {
            self.counter == (if self.inc_a { 1 as int } else { 0 }) + (if self.inc_b { 1 as int } else { 0 })
        }
        // ANCHOR_END: inv 

        init!{
            initialize() {
                init counter = 0;
                init inc_a = false;
                init inc_b = false;
            }
        }

        transition!{
            tr_inc_a() {
                require(!pre.inc_a);
                assert(pre.counter <= 2);
                update counter = pre.counter + 1;
                update inc_a = true;
            }
        }

        transition!{
            tr_inc_b() {
                require(!pre.inc_b);
                assert(pre.counter <= 2);
                update counter = pre.counter + 1;
                update inc_b = true;
            }
        }

        property!{
            finalize() {
                require(pre.inc_a);
                require(pre.inc_b);
                assert pre.counter == 2;
            }
        }

        // ANCHOR: inv_proof
        #[inductive(tr_inc_a)]
        fn tr_inc_a_preserves(pre: Self, post: Self) {
        }

        #[inductive(tr_inc_b)]
        fn tr_inc_b_preserves(pre: Self, post: Self) {
        }

        #[inductive(initialize)]
        fn initialize_inv(post: Self) {
        }
        // ANCHOR_END: inv_proof
    }
);


// ANCHOR: global_struct
struct_with_invariants!{
    pub struct Global {
        // An AtomicU32 that matches with the `counter` field of the ghost protocol.
        pub atomic: AtomicU32<_, X::counter, _>,

        // The instance of the protocol that the `counter` is part of.
        #[verifier::proof] pub instance: X::Instance,
    }

    spec fn wf(&self) -> bool {
        // Specify the invariant that should hold on the AtomicU32<X::counter>.
        // Specifically the ghost token (`g`) should have
        // the same value as the atomic (`v`).
        // Furthermore, the ghost token should have the appropriate `instance`.
        invariant on atomic with (instance) is (v: u32, g: X::counter) {
            g@ === X::token![instance => counter => v as int]
        }
    }
}
// ANCHOR_END: global_struct

fn main() {
    // Initialize protocol 

    #[verifier::proof] let (Trk(instance),
        Trk(counter_token),
        Trk(inc_a_token),
        Trk(inc_b_token)) = X::Instance::initialize();

    // Initialize the counter

    let atomic = AtomicU32::new(instance, 0, counter_token);

    let global = Global { atomic, instance: instance.clone() };
    let global_arc = Arc::new(global);

    // Spawn threads

    // Thread 1

    let global_arc1 = global_arc.clone();
    let join_handle1 = spawn(move || {
        ensures(|new_token: Proof<X::inc_a>|
            new_token.0@ ===
                X::token![instance => inc_a => true]
        );

        // `inc_a_token` is moved into the closure
        #[verifier::proof] let mut token = inc_a_token;
        let globals = &*global_arc1;

        let _ = atomic_with_ghost!(&globals.atomic => fetch_add(1);
            ghost c => {
                globals.instance.tr_inc_a(&mut c, &mut token); // atomic increment
            }
        );

        Proof(token)
    });

    // Thread 2

    let global_arc2 = global_arc.clone();
    let join_handle2 = spawn(move || {
        ensures(|new_token: Proof<X::inc_b>|
            new_token.0@ ===
                X::token![instance => inc_b => true]
        );

        // `inc_b_token` is moved into the closure
        #[verifier::proof] let mut token = inc_b_token;
        let globals = &*global_arc2;

        let _ = atomic_with_ghost!(&globals.atomic => fetch_add(1);
            ghost c => {
                globals.instance.tr_inc_b(&mut c, &mut token); // atomic increment
            }
        );

        Proof(token)
    });

    // Join threads

    #[verifier::proof] let inc_a_token;
    match join_handle1.join() {
        Result::Ok(Proof(token)) => { inc_a_token = token; }
        _ => { return; }
    };

    #[verifier::proof] let inc_b_token;
    match join_handle2.join() {
        Result::Ok(Proof(token)) => { inc_b_token = token; }
        _ => { return; }
    };

    // Join threads, load the atomic again

    let global = &*global_arc;
    
    let x = atomic_with_ghost!(&global.atomic => load();
        ghost c => {
            instance.finalize(&c, &inc_a_token, &inc_b_token);
        }
    );

    assert(x == 2);
}

}
// ANCHOR_END: full

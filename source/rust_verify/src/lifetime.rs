//! This module transforms the original code (with exec + proof + spec code)
//! into an abstracted version of the code (with only exec + proof code, no spec code)
//! for the purpose of checking ownership/lifetime/borrow on the exec and proof code.
//! (All spec code is erased because no ownership/lifetime/borrow checking should be
//! performed on spec code.)
//! This module then feeds the transformed code to rustc so that rustc can
//! catch any ownership/lifetime/borrow errors.

/*
The generated abstracted code discards much of the detail of the original code,
but keeps enough for ownership/lifetime/borrow checking.
Specifically it keeps:
- struct and enum declarations, and fields of the declarations
- lifetime ('a) and type (A) parameters, but not trait bounds (except for Copy, which is kept)
- functions, with impl methods turned into to top-level functions
- function bodies, but with external_body function bodies replaced with panic
  (external functions are erased completely)
- overall structure of blocks, statements, and expressions, but with specific operators
  transformed into calls to a generic function named "op"
- variable declarations and pattern matching
(The exact program elements that are kept can be seen in the abstract syntax defined in lifetime_ast.rs.)
For example, if the original code is the following:

    struct S {
    }

    spec fn fspec(i: u32, s1: S, s2: S) -> u32 {
        i
    }

    proof fn fproof(i: u32, tracked s1: S, tracked s2: S) -> u32 {
        i
    }

    fn fexec(i: u32, s1: S, s2: S) -> u32 {
        i
    }

    proof fn test_proof(i: u32, tracked s: S) {
        let j = fspec(i, s, s);
        let k = fproof(i, s, s);
    }

    fn test_exec(i: u32, s: S)
        requires i < 100
    {
        proof {
            let j = fspec(i, s, s);
            let k = fproof(i, s, s);
        }
        let k = fexec(i, s, s);
        let n = fexec(i, s, s);
    }

Then the generated "synthetic" Rust code will look like the following
(use the --log-all or --log-lifetime options to print the generated synthetic code
to a file in the .verus-log directory):

    struct D1_S {
    }

    fn f3_fproof(
        x4_i: (),
        x5_s1: D1_S,
        x6_s2: D1_S,
    )
    {
    }

    fn f8_fexec(
        x4_i: u32,
        x5_s1: D1_S,
        x6_s2: D1_S,
    ) -> u32
    {
        x4_i
    }

    fn f11_test_proof(
        x4_i: (),
        x9_s: D1_S,
    )
    {
        f3_fproof(op::<_, ()>(()), x9_s, x9_s, );
    }

    fn f15_test_exec(
        x4_i: u32,
        x9_s: D1_S,
    )
    {
        {
            f3_fproof(op::<_, ()>(()), x9_s, x9_s, );
        };
        let x12_k: u32 = f8_fexec(x4_i, x9_s, x9_s, );
        let x13_n: u32 = f8_fexec(x4_i, x9_s, x9_s, );
    }

When rustc is called on this generated code, rustc will report ownership violations
because the code tries to duplicate the linear variable "x9_s", which
corresponds to "s" in the original code.

When we print the error messages, we need to transform the line and column numbers
so that the error messages correspond to the original code.
These error messages are generated by running rustc on the synthetic code,
with rustc configured to generate errors in JSON format, capturing the JSON errors
and parsing them to retrieve the line/column information and error messages,
converting the synthetic line/column information back into spans for the original source code,
and then sending the error messages and spans to the rustc diagnostics for the original source code.
*/

use crate::erase::ErasureHints;
use crate::lifetime_emit::*;
use crate::lifetime_generate::*;
use crate::util::{error, to_air_span};
use crate::verifier::DiagnosticOutputBuffer;
use air::messages::{message_bare, Message, MessageLevel};
use rustc_hir::{AssocItemKind, Crate, ItemKind, OwnerNode};
use rustc_middle::ty::TyCtxt;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use vir::ast::VirErr;

// Call Rust's mir_borrowck to check lifetimes of #[spec] and #[proof] code and variables
pub(crate) fn check<'tcx>(queries: &'tcx rustc_interface::Queries<'tcx>) {
    queries.global_ctxt().expect("global_ctxt").peek_mut().enter(|tcx| {
        let hir = tcx.hir();
        let krate = hir.krate();
        for owner in &krate.owners {
            if let Some(owner) = owner {
                match owner.node() {
                    OwnerNode::Item(item) => match &item.kind {
                        rustc_hir::ItemKind::Fn(..) => {
                            tcx.ensure().mir_borrowck(item.def_id);
                        }
                        ItemKind::Impl(impll) => {
                            for item in impll.items {
                                match item.kind {
                                    AssocItemKind::Fn { .. } => {
                                        tcx.ensure().mir_borrowck(item.id.def_id);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => (),
                }
            }
        }
    });
}

const PRELUDE: &str = "\
#![feature(box_patterns)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_parens)]
#![allow(unused_braces)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_labels)]
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
fn op<A, B>(a: A) -> B { panic!() }
struct Tracked<A> { a: PhantomData<A> }
impl<A> Tracked<A> {
    pub fn get(self) -> A { panic!() }
    pub fn borrow(&self) -> &A { panic!() }
    pub fn borrow_mut(&mut self) -> &mut A { panic!() }
}
#[derive(Clone, Copy)] struct Ghost<A> { a: PhantomData<A> }
#[derive(Clone, Copy)] struct int;
#[derive(Clone, Copy)] struct nat;
struct FnSpec<Args, Output> { x: PhantomData<(Args, Output)> }
struct InvariantBlockGuard;
fn open_atomic_invariant_begin<'a, X, V>(_inv: &'a X) -> (&'a InvariantBlockGuard, V) { panic!(); }
fn open_local_invariant_begin<'a, X, V>(_inv: &'a X) -> (&'a InvariantBlockGuard, V) { panic!(); }
fn open_invariant_end<V>(_guard: &InvariantBlockGuard, _v: V) { panic!() }
";

fn emit_check_tracked_lifetimes<'tcx>(
    tcx: TyCtxt<'tcx>,
    krate: &'tcx Crate<'tcx>,
    emit_state: &mut EmitState,
    erasure_hints: &ErasureHints,
) -> State {
    let gen_state =
        crate::lifetime_generate::gen_check_tracked_lifetimes(tcx, krate, erasure_hints);
    for line in PRELUDE.split('\n') {
        emit_state.writeln(line.replace("\r", ""));
    }

    for d in gen_state.datatype_decls.iter() {
        emit_datatype_decl(emit_state, d);
    }
    for f in gen_state.const_decls.iter() {
        emit_const_decl(emit_state, f);
    }
    for f in gen_state.fun_decls.iter() {
        emit_fun_decl(emit_state, f);
    }
    gen_state
}

struct LifetimeCallbacks {
    capture_output: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
}

impl rustc_driver::Callbacks for LifetimeCallbacks {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.diagnostic_output =
            rustc_session::DiagnosticOutput::Raw(Box::new(DiagnosticOutputBuffer {
                output: self.capture_output.clone(),
            }));
    }

    fn after_parsing<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        check(queries);
        rustc_driver::Compilation::Stop
    }
}

struct LifetimeFileLoader {
    rust_code: String,
}

impl LifetimeFileLoader {
    const FILENAME: &'static str = "dummyrs.rs";
}

impl rustc_span::source_map::FileLoader for LifetimeFileLoader {
    fn file_exists(&self, _path: &std::path::Path) -> bool {
        panic!("unexpected call to file_exists")
    }

    fn read_file(&self, path: &std::path::Path) -> Result<String, std::io::Error> {
        assert!(path.display().to_string() == Self::FILENAME.to_string());
        Ok(self.rust_code.clone())
    }
}

#[derive(Debug, Deserialize)]
struct DiagnosticSpan {
    line_start: usize,
    line_end: usize,
    column_start: usize,
    column_end: usize,
}

#[derive(Debug, Deserialize)]
struct Diagnostic {
    message: String,
    level: String,
    spans: Vec<DiagnosticSpan>,
}

pub(crate) fn check_tracked_lifetimes<'tcx>(
    tcx: TyCtxt<'tcx>,
    parent_rustc_args: Vec<String>,
    erasure_hints: &ErasureHints,
    lifetime_log_file: Option<File>,
) -> Result<Vec<Message>, VirErr> {
    let krate = tcx.hir().krate();
    let mut emit_state = EmitState::new();
    let gen_state = emit_check_tracked_lifetimes(tcx, krate, &mut emit_state, erasure_hints);
    let mut rust_code: String = String::new();
    for line in &emit_state.lines {
        rust_code.push_str(&line.text);
        rust_code.push('\n');
    }
    if let Some(mut file) = lifetime_log_file {
        write!(file, "{}", &rust_code).expect("error writing to lifetime log file");
    }
    let mut rustc_args = vec![
        "dummyexe".to_string(),
        LifetimeFileLoader::FILENAME.to_string(),
        "--error-format=json".to_string(),
    ];
    for i in 0..parent_rustc_args.len() {
        if parent_rustc_args[i] == "--sysroot" && i + 1 < parent_rustc_args.len() {
            rustc_args.push(parent_rustc_args[i].clone());
            rustc_args.push(parent_rustc_args[i + 1].clone());
        }
    }
    let capture_output = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut callbacks = LifetimeCallbacks { capture_output };
    let mut compiler = rustc_driver::RunCompiler::new(&rustc_args, &mut callbacks);
    compiler.set_file_loader(Some(Box::new(LifetimeFileLoader { rust_code })));
    let run = compiler.run();
    let bytes: &Vec<u8> = &*callbacks.capture_output.lock().expect("lock capture_output");
    let rust_output = std::str::from_utf8(bytes).unwrap().trim();
    let mut msgs: Vec<Message> = Vec::new();
    let debug = false;
    if rust_output.len() > 0 {
        for ss in rust_output.split("\n") {
            let diag: Diagnostic = serde_json::from_str(ss).expect("serde_json from_str");
            if diag.level == "failure-note" {
                continue;
            }
            if diag.level == "warning" {
                dbg!("internal error: unexpected warning");
                dbg!(diag);
                continue;
            }
            assert!(diag.level == "error");
            let msg_text = gen_state.unmangle_names(&diag.message);
            let mut msg = message_bare(MessageLevel::Error, &msg_text);
            if debug {
                dbg!(&msg);
            }
            for dspan in &diag.spans {
                if debug {
                    dbg!(&dspan);
                }
                let span = emit_state.get_span(
                    dspan.line_start - 1,
                    dspan.column_start - 1,
                    dspan.line_end - 1,
                    dspan.column_end - 1,
                );
                msg = msg.primary_span(&to_air_span(span));
            }
            msgs.push(msg);
        }
    }
    if debug {
        dbg!(msgs.len());
    }
    if msgs.len() == 0 && run.is_err() { Err(error("lifetime checking failed")) } else { Ok(msgs) }
}

#! /bin/bash

case $(uname -m) in
  x86_64)
    ARCH=x86_64
    ;;
  arm64)
    ARCH=aarch64
    ;;
  *)
    echo "Unknown architecture $(uname -m)" 1>&2
    exit 1
    ;;
esac

if [ `uname` == "Darwin" ]; then
    DYN_LIB_EXT=dylib
    LIB_PATH="DYLD_LIBRARY_PATH=../rust/install/lib/rustlib/${ARCH}-apple-darwin/lib"
elif [ `uname` == "Linux" ]; then
    DYN_LIB_EXT=so
    LIB_PATH="LD_LIBRARY_PATH=../rust/install/lib/rustlib/${ARCH}-unknown-linux-gnu/lib"
fi

TEMPD=$(mktemp -d)

cp -r pervasive $TEMPD
echo '//! The "standard library" for Verus. See the [`pervasive`](crate::pervasive) module for more details.
#![feature(rustc_attrs)] #[allow(rustdoc::invalid_rust_codeblocks)] pub mod pervasive;' >> $TEMPD/lib.rs

echo "Running rustdoc..."
eval ""VERUSDOC=1 VERUS_Z3_PATH="$(pwd)/z3" $LIB_PATH ../rust/install/bin/rustdoc --extern builtin=../rust/install/bin/libbuiltin.rlib --extern builtin_macros=../rust/install/bin/libbuiltin_macros.$DYN_LIB_EXT --extern state_machines_macros=../rust/install/bin/libstate_machines_macros.$DYN_LIB_EXT --edition=2018 \
  -Zenable_feature=stmt_expr_attributes \
  -Zenable_feature=box_syntax \
  -Zenable_feature=box_patterns \
  -Zenable_feature=negative_impls \
  -Zenable_feature=unboxed_closures \
  -Zenable_feature=register_tool \
  -Zcrate-attr=register_tool\\\(verus\\\) \
  -Zcrate-attr=register_tool\\\(verifier\\\) \
  -Zproc-macro-backtrace \
  $TEMPD/lib.rs""

rm -rf $TEMPD

echo "Running post-processor..."
./target/debug/verusdoc

echo "Documentation generated at ./doc/lib/index.html"

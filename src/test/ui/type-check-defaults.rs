// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
// compile-flags: --error-format=human

use std::iter::FromIterator;

struct Foo<T, U: FromIterator<T>>(T, U);
struct Bar<Z = Foo<i32, i32>>(Z);

struct Qux<A, T=<A as Iterator>::Item>(A, T);

fn main() { }

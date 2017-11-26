// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//

#![feature(default_type_parameter_fallback)]

use std::marker::PhantomData;

trait Id {
    type Me;
}

impl<A> Id for A {
    type Me = A;
}

struct Foo<X, Y, Z, W> {
    data: PhantomData<(X, Y, Z, W)>,
}

impl<X: Default = u32, Y = <X as Id>::Me, Z = <Y as Id>::Me, W = Vec<<X as Id>::Me>>
    Foo<X, Y, Z, W> {
    fn new() -> Foo<X, Y, Z, W> {
        Foo { data: PhantomData }
    }
}

fn main() {
    let _ = Foo::new();
}

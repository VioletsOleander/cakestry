# Code Style

## Comment

If a comment starts with a capital letter, either a comment is a sentence fragment (a group of words
that is punctuated like a sentence but is grammatically incomplete), this comment should end with a
period (`.`). The period marks the formal end of a write unit.

If a comment starts with a non-capital letter, this comment should not end with a period (`.`).

Example:

```rust
# Comment with captical letter should end with a period.

# comment with non-captical letter should not end with a period
```

## Import Order

Write imports from std lib, external lib, parent or sibling modules first, and then declaration for
sub modules, and then imports from sub modules.

This is to make the imports from sub modules looks more reasonable, following a "define firstly and
then use subsequently" pattern.

Example:

```rust
use std::fs;

use clap::Parser;

use super::bar;
use crate::foo;

mod submodule

use submodule::baz;
```

## Group Order

In each file, write code in groups, with the following order:

1. Imports
2. Submodule declarations
3. Imports from submodules
4. Type aliases
5. Enumeration declarations
6. Enumeration implementation blocks
7. Struct declarations
8. Struct implementation blocks
9. Functions

In each group, put `pub` items first.

Example:

```rust
use std::fs;

mod submodule;

use submodule::foo;

pub type PubFooType = foo::PubFooType;
type FooType = foo::FooType;

pub enum PubFooEnum {
    // variants
}

enum FooEnum {
    // variants
}

impl PubFooEnum {
    // implementations
}

impl FooEnum {
    // implementation
}

pub struct PubFooStruct {
    // struct fields
}

struct FooStruct {
    // struct fields
}

impl PubFooStruct {
    // implementation
}

impl FooStruct {
    // implementations
}

pub fn pub_foo_function() {
    // function body
}

fn foo_function() {
    // function body
}
```

## `Impl` Block Order

Write normal `impl` block first, and trait `impl ... for` block after. Normal `impl` block can be
viewed as the type's inherent behaviour, and trait `impl ... for` block can be treated as the type's
extended behaviour to satisfy externally defined protocol/interface.

Example:

```rust
struct Foo;

impl Foo {
    // implementations
}

impl Default for Foo {
    // implementations
}
```

Prefer to use multiple `impl` blocks to contain methods with different focuses. Particularly, prefer
to use different `impl` blocks to contain getters and setters separately. When separate getters and
setters in different `impl` blocks, write getters `impl` block first, because such block is more
concise and shorter, otherwise the usually long getter `impl` block will dominate readers'
attention.

Example:

```rust
struct Foo {
    bar: String,
}

impl Foo {
    // setters here
    pub fn set_bar(&mut self, bar: String) {
        self.bar = bar;
    }
}

impl Foo {
    // getters here
    pub fn bar(&self) -> &String {
        self.bar
    }
}
```

## Method

### `default` and `new`

Implement `default` for constructor without any argument, and implement `new` for constructor with
any argument

It is not necessary to write a `new` without any argument and just simply call `default` inside its
method body.

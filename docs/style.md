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

## `Impl` Block Order

Write normal `impl` block first, and trait `impl ... for` block after. Normal `impl` block can be
viewed as the type's inherent behaviour, and trait `impl ... for` block can be treated as the type's
extended behaviour to satisfy externally defined protocol/interface.

Example:

```rust
struct Foo;

impl Foo {
    // Implementations here.
}

impl Default for Foo {
    // Implementations here.
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
    // Setters here.
    pub fn set_bar(&mut self, bar: String) {
        self.bar = bar;
    }
}

impl Foo {
    // Getters here.
    pub fn bar(&self) -> &String {
        self.bar
    }
}
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

## Method

### `default` and `new`

Implement `default` for constructor without any argument, and implement `new` for constructor with
any argument

It is not necessary to write a `new` without any argument and just simply call `default` inside its
method body.

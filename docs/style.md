# Code Style

## Comment Style

If a comment starts with a capital letter, either a comment is a sentence fragment (a group of words
that is punctuated like a sentence but is grammatically incomplete), this comment should end with a
period (`.`). The period marks the formal end of a write unit.

If a comment starts with a non-capital letter, this comment should not end with a period (`.`).

## `Impl` Block Order

Write normal `impl` block first, and trait `impl ... for` block after. Normal `impl` block can be
viewed as the type's inherent behaviour, and trait `impl ... for` block can be treated as the type's
extended behaviour to satisfy externally defined protocol/interface.

## Import Order

Write imports from std lib, external lib, parent or sibling modules first, and then declaration for
sub modules, and then imports from sub modules.

This is to make the imports from sub modules looks more reasonable, following a "define firstly and
then use subsequently" pattern.

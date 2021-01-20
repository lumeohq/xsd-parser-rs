### Structure

This module contains mutliple test cases. 
Each case is a separate module and is located in its own directory, 
for example here are contents of directory `complex_type`:
```
complex_type
├── input.xsd      # Manually-written schema containing a single case
├── expected.rs    # What Rust code we expect after processing input.xsd
├── example.xml    # Example XML generated using xsd2inst or written manually
└── mod.rs         # Finer-grade tests for this case
``` 

It may look that there are too many files for each case but it is made for convenience.
Modern IDE's support code highlight/completion for `XSD's` and other file types 
so it is always better to have separate files instead of raw strings.

### What is checked

For each test case we check that:
- code generated from `input.xsd` has the same AST that we've put into `expected.rs`
- manually written `expected.rs`
    - compiles
    - is correctly (de)serialized

We could have check if generator output compiles at runtime (with `trybuild`) but we don't do
that because we already check that it's AST is equivalent to `expected.rs`.
Another reason is that `expected.rs` participates in compilation so errors are easier to locate.

### What is not checked

Since only AST's are compared anything not related to AST is ignored, like:
- comments
- formatting

### How to make a new case

The easiest way is: 
- copy existing case to a new directory `new_test`
- register `new_test` in `mod.rs`
- modify `new_test/input.xsd` according to your case
- generate `new_test/example.xml` with `xsd2inst` (see below for details)
- modify `new_test/example.xml` to your taste
- modify `new_test/expected.rs` manually or cheat:
    - run test `cargo t tests::new_test::generator_does_not_panic -- --nocapture`
    - copy printed struct to `expected.rs` if it looks fine
- modify `new_test/mod.rs`:
    - test `(de)serialization_works` most likely will be different for each case
    - other tests are unlikely to change

### xsd2inst

```
Generates a document based on the given Schema file having the given element as root.
The tool makes reasonable attempts to create a valid document, but this is not always 
possible since, for example, there are schemas for which no valid instance document 
can be produced.
```

Installation:
```bash
sudo apt install xmlbeans
```

Usually all you need is to run this command:

```bash
xsd2inst input.xsd -name Foo > example.xml
```

Please note that you cannot just declare a type in schema root and generate XML from it. 
You will also need to add an element in the root:

```xml
<xs:element name="Foo" type="tns:Foo"/>
```

and pass this global element name `Foo` to `xsd2inst`

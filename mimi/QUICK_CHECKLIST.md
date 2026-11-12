# Quick Code Review Checklist

This is not an exhaustive list of everything you should do during a code review--it is just the bare minimum.

There may be occasional exceptions to the rules listed in this document. Use common sense.

## A quick note for reviewers

Your job isn't to simply run `cargo test` and call it a day.
I wouldn't need an LLM or human for that.

Getting `cargo test` to succeed is a necessary, but not sufficient condition for acceptance.

Only report `cargo test` results if at least one test failed.
If they all succeed, then don't say anything about them--I'm not interested.

**Instead, your job as a code reviewer is to catch the problems that are _not_ covered by the tests.**

## Article I: Correctness of Externally Observable Behavior

### 1. Panicking vs. returning `Err`

Even if the input is malformed, do not panic (instead, return `Err`).
The only time `unwrap`, `expect`, or similar panicking constructs are justified is during "If this operation fails, it indicates an implementation error"-type situations.

### 2. Keep the syntax highlighter up-to-date

If the task modifies the surface syntax in a way that requires updating the syntax highlighter, then update the syntax highlighter.

## Article II: Internal Correctness

### 1. Respect the internal semantics

Use functions/datatypes/traits/etc. correctly.

Consider this example of what NOT to do:

```rust
// ## Existing code
pub fn dog_woof_woof(d: DogRef) {
    for _ in 0..d.raw().mood() {
        let sound = d.raw().sound.as_ref().unwrap();
        println!("{} barks: \"{sound} {sound}\"", d.raw().name);
    }
}
pub struct DogRef<'a> {
    raw: &'a Animal,
}
impl<'a> DogRef<'a> {
    pub fn unchecked_new(raw: &'a Animal) -> Self { Self { raw } }

    pub fn new(raw: &'a Animal) -> Option<Self> { /*...*/ }

    pub fn raw(self) -> &'a Animal { self.raw }
}
// ...

// ## Task
pub fn cat_woof_woof(c: BarkingCatRef); // AI, please implement this.

// ## Dumb AI agent's implementation
pub fn cat_woof_woof(c: BarkingCatRef) {
    // BAD! **NEVER** write this kind of code. It's horrible AI slop.
    // Even if the external behavior is correct, it violates the internal semantics.
    dog_woof_woof(DogRef::unchecked_new(c.raw()));
}
```

Depending on the rest of the codebase, it's possible that `cat_woof_woof` might _externally_ behave correctly.
However, regardless of whether or not it has correct external behavior, it is definitely _internally_ incorrect, because `DogRef::unchecked_new` should never be passed a non-dog.

External behavioral correctness by itself is not enough--internal correctness is also required.

While code reuse and simplicity are desirable, you should aways prioritize correctness (both external and internal) above all else, in the event of a priority-clash.

## Article III: Stylistic Acceptability

### 1. SUPER IMPORTANT: Enforce the Golden Rule

"Write code in the style of the existing code."

The Golden Rule: "The codebase should look like it was written by a single human."

- **Consistent order**: Use consistent conventions across the entire codebase for def/impl/match-case order
- **Exhaustiveness is your friend**: Many AST utils use explicit destructuring, even though it makes the function take more LOC. Furthermore, it is often exhaustive (i.e., no `..`). These choices are both intentional--it forces rustc to give us a type error if we add a field in the future, but forget to update said util.

If following the Golden Rule would result in behavioral incorrectness or slow execution time, stop the task and discuss this with the human.

### 2. Ensure that the new code was added in a location that makes sense

Do not haphazardly dump the new code at the top or bottom of the file.
Instead, add it in a location that makes sense.

### 3. No interior mutation

Avoid constructs like `Cell`, `RefCell`, etc.
Constructs like `Rc` by themselves are fine, as long as you never mutate the data inside.

### 4. Put Rust code in `crateDir/src/`

Obviously, there are some exceptions (e.g., `build.rs`).
However, you should avoid things like `crateDir/tests/`--tests belong in `src/` too.

### 5. Use `foo/mod.rs`

If a Rust module has a directory `foo/` for its submodules, put the module itself in `foo/mod.rs`, not `foo.rs`.

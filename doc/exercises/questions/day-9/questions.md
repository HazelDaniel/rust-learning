```rust
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    /*
        in the associated function below,
        why is it that the destroy method doesn't use the '&' syntax when referencing
        the parameter?. is it because the associated function for a tuple struct has
        different syntax from that of a c-struct or is it because the destroy method
        is a special function?
    */

    fn destroy(self) {
        let Pair(first, second) = self;

    }

    /*ANSWER (FROM THE FUTURE): because it passes it in by value and so the destroy function owns the instance now.
        so whenever the function is done executing, its owned variables (self, in this case) go out of scope and are dropped
    */
}
```


# RUST

Compilation of rust contents and learnings

```rust
struct GoFmt {}
impl GoFmt {
    fn Println(&self, msg: String) {
        println!("{}", msg)
    }
}
fn main() {
    let fmt = GoFmt {};
    let msg = String::from( "This is complete madness, Daniel :)))");
    fmt.Println(msg);
}

```

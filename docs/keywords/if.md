# If Statements

An in statement is a conditional statement
that only executes the following block statements
if, and only if, the given [boolean value](docs/primitives/bool.md) evaluates to [true](docs/kw/true).

## Example
```
fn eval(n: i32) {
    if (n < 0) {
        print("n is negative!");
    }
    if (n == 0) {
        print("n is zero!")
    }
    if (n > 0) {
        print("n is positive!");
    }
}

fn main() {
    eval(-1);
    eval(0);
    eval(1);
}
```
### Output
```bash
$ > clank run
n is negative!
n is zero!
n is positive!
$ >
```

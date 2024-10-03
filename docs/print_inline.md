## print_word

`print_word` is a base function of Clank. It's purpose
is to print a `string` or `Printable` to the console inline.

# Definition signatures
```
fn print_inline(s: string) {
    ...
}

fn print_inline(p: dyn Printable) {
    ...
}
```

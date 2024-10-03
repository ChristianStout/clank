# input

`input` is a base function of Clank. It's purpose is to
receive a string input from the console. When called, it
will listen to all console input until the `'\n'` character
is received (the RETURN or ENTER key on a keyboard).

The `input` will return a [`string`](docs/primitives/string.md)
as output.

## Function signature
```
fn input() -> string {
    ...
}
```

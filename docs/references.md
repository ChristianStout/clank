# Reference Types

A reference type in Clank is any type that uses a 
[pointer](https://en.wikipedia.org/wiki/Pointer_(computer_programming)),
rather than a raw value (such as a 
[char](docs/primitives/char.md)).

## Using references

In most ways, using a reference type in Clank is very similar
to using any other type. However, [ownership]() when calling
functions must be made explicit when using a reference. We
will illustrate this by using an example of a non-reference
type, and changing it to a reference type.

### Example
Below is a very basic `add` function. It takes two `i32`'s, and returns an `i32`. 
```
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    print(add(16, 4));
}
```
`output`
```bash
$ > clank run
20
```

This works, because the default behavior
of a non-reference type is to simply copy the values
into the function. We can illustrate that this is the case
modifying the functions, and examine the result.

```
fn add(mut x: i32, y: i32) -> i32 {
    x += 1;
    print("x in add(): {x}");
    return x + y;
}

fn main() {
    let mut x = 16;
    print(add(x, 4));
    print(x in main(): {x});
}
```
`output`
```bash
$ > clank run
x in add(): 17
21
x in main(): 16
```

As we would expect, modifying the value of `{x}` in `add`
has no effect of the `x` that exists in `main`. Let's modify
`add` again. However, this time it will take a `MyNumber` struct
that we will define instead of an `i32`.

```
struct MyNumber {
    n: i32,
}

fn add(x: MyNumber, y: i32) -> i32 {
    return x.n + y;
}

fn main() {
    let x = MyNumber {
        n: 16,
    };

    print(add(x, 4));
}
```
`output`
```bash
$ > clank run
20
```

Perhaps unexpectedly (given this page existing), this works
without any issue. This is because we passed in the reference
to `x`, and and used it's pointer to access the `n` within `x`.

However, something happened by the end of this program that may
not be obvious: `x` is no longer owned by `main`. Let's modify
main to examine this.

```
struct MyNumber {
    n: i32,
}

fn add(x: MyNumber, y: i32) -> i32 {
    return x.n + y;
}

fn main() {
    let x = MyNumber {
        n: 16,
    };

    print(add(x, 4));
    print(x.n);
}
```
`output`
```bash
$ > clank run
[Line 15] AccessError: `x` does not belong to `main`

    print(x.n);
    ~~~~~~~~~~~

Program could not compile.
```

This means that `x` no longer belongs to `main. Therefore, when  There is a
simple solution to this issue. instead of handing `x` to `add`
by writing `add(x, 4)`, we can use `&x` like `add(&x, 4)`.

```
struct MyNumber {
    n: i32,
}

fn add(x: MyNumber, y: i32) -> i32 {
    return x.n + y;
}

fn main() {
    let x = MyNumber {
        n: 16,
    };

    print(add(&x, 4));
    print(x.n);
}
```
`output`
```bash
$ > clank run
20
16
```

This explicitly says that we will allow read-only access to `x` from
`main` to `add`. Since the access is read-only, attempting
to modify `x` within `add` will fail. If we attempt to run the
following modification, we will get the resulting error.

```
struct MyNumber {
    n: i32,
}

fn add(x: MyNumber, y: i32) -> i32 {
    x.n += 1;
    return x.n + y;
}

fn main() {
    let x = MyNumber {
        n: 16,
    };

    print(add(&x, 4));
    print(x.n);
}
```
`output`
```bash
$ > clank run
[Line 6] MutabilityError: `x` is not mutable
Program could not compile.
```

Let modify the function signature of `add` to make its `x`
mutable.

```
struct MyNumber {
    n: i32,
}

fn add(mut x: MyNumber, y: i32) -> i32 {
    x.n += 1;
    return x.n + y;
}

fn main() {
    let x = MyNumber {
        n: 16,
    };

    print(add(&x, 4));
    print(x.n);
}
```
`output`
```bash
$ > clank run
[Line 14] MutabilityError: `add` expected a mutable reference.

    print(add(&mut x, 4))
              ~~~~~~

Program could not compile.
```

As the compiler so helpfully pointed out, we must modify how
we give `add` its rights to `x`. If we change the line that
the Clank compiler gave us, we get the following output:
```
struct MyNumber {
    n: i32,
}

fn add(mut x: MyNumber, y: i32) -> i32 {
    x.n += 1;
    return x.n + y;
}

fn main() {
    let x = MyNumber {
        n: 16,
    };

    print(add(&mut x, 4));
    print(x.n);
}
```
`output`
```bash
$ > clank run
21
17
```

And as you can see, as difference from the `i32` example, `x` was modified within `main` as well. 

## Examples of reference types

* [string](docs/primitives/string.md)
* any [arrays](docs/primitives/arrays.md)
* any [struct](docs/struct.md) types
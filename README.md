# Burn Language
An experimental programming language for exploring first class iterators.

## Example
See [examples](./examples) for more.

```go
fn main() {
    'Hello, world!\n' -> SINKS::stdout();
}
```

## Building
Nothing to build, yet!

## Design
The design of Burn is very simple as this is a learning experience
in writing compilers.  The focus of this language is on creating first
class support for iterators as a way to logically model data flow through
a program. The Burn run-time will concurrently read and write streams
making it easier for programmers to focus on transforming the data in
a meaningful way.

### Data Types
* `bool`
* `u8` `u16` `u32` `u64`
* `i8` `i16` `i32` `i64`
* `stream<T>`

You may notice there are no floating point or strings in Burn, but in
the example above there is a string literal. The actual type of that
string literal in Burn is `stream<u8>`. This makes interchanging input
streams much easier; a useful feature for testing.

### Functions
Functions are intended to model transfer functions, or "black boxes"
with *N* inputs and *M* outputs. The below functions demonstrates how any
number of inputs and outputs may be specified. Unlike other programming
languages, all outputs must be named in Burn.

```go
fn mix(a stream<u8>, b stream<u8>) -> (out stream<u8>) {
    while !a.eof() && !b.eof() {
        a.next() -> out;
        b.next() -> out;
    }
}

fn unmix(input stream<u8>) -> (a stream<u8>, b stream<u8>) {
    while !input.eof() {
        input.next() -> a; // next() is guaranteed here
        input.next() -> b; // but not here. In this case Burn detects
                           // the EOF and it will be a no-op.
    }
}

fn sum(a i32, b i32) -> (c i32) {
    // Return may still be used without referencing the return value names
    return a + b;
}
```

Additionally, if a function's outputs are all `stream` types, then Burn
will execute the function as needed to progress the program. In other
words, you may setup a chain of functions for which data will flow
through one item at a time, rather than shuffling large buffers.

### Arrow operator
The arrow operator `->` has three purposes:
* Function definitions to describe inputs and outputs
* Syntactic sugar for passing tuples to functions
* Most importantly, write a value to a stream

**Syntactic sugar**
```go
fn main() {
    ('hello', 'world') -> mix();
    // Equivalent to mix('hello', 'world');
}
```

**Write a value to a stream**
```go
fn main() {
    s := 'foo';
    ' bar' -> s;
    // When checked, s == 'foo bar'
}
```

### Sources and Sinks
Burn uses special namespaces `SOURCES` and `SINKS` for defining all the
sources and sinks a programmer may use. Remember this is an experimental
language, so defining new sources or sinks is not possible. Below is the
complete list.

| Function | Returns | Description |
|:-------- |:------- |:----------- |
| `SOURCES::stdin()` | `stream<u8>` | Standard input |
| `SOURCES::args()` | `stream<stream<u8>>` | Space separated command line arguments |
| `SOURCES::raw_args()` | `stream<u8>` | Command line arguments |
| `SOURCES::tcp(port int)` | `(input stream<u8>, output stream<u8>)` | Opens a port and accepts a connection, returning the input and output streams associated with the connection |
| `SOURCES::file(filename stream<u8>)` | `(found bool, data stream<u8>)` | Reads a file from the filesystem |
| `SINKS::file(filename stream<u8>)` | `stream<u8>` | Writes a file to the filesystem |
| `SINKS::stdout()` | `stream<u8>` | Standard output |
| `SINKS::stderr()` | `stream<u8>` | Standard error |

### stream\<T\> methods

| Method | Returns | Description |
|:------ |:------- |:----------- |
| `next()` | `T` | Reads and consumes the next item in the stream |
| `consume(n u32)` | `stream<T>` | Reads and consumes the next `n` items in the stream |
| `get(n u32)` | `T` | Returns the `nth` item in the stream without consuming any items |
| `len()` | `u32` | Returns the number of items currently in the stream |
| `eof()` | `bool` | Returns whether the stream has ended or not (end of file) |
| `cycle()` | `stream<T>` | Cycles the input stream indefinitely |
| `split(delim stream<T>)` | `stream<stream<T>>` | Splits the stream into chunks, excluding `delim` |
| `parse()` | - | Context specific; converts a `stream<u8>` to the inferred numeric type (defaulting to `i32` if it cannot be inferred) |


## Progress and Current State
- [x] Design
- [ ] Lexing
- [ ] Parsing
- [ ] Checking
- [ ] Code generation

## Contributing
Contributions are welcome!

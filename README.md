# Burn Language
An experimental programming language for exploring first class iterators.

## Design
The design of Burn is very simple as this is a learning experience
in writing compilers.  The focus of this language is on creating first
class support for iterators as a way to logically model data flow through
a program. The Burn run-time will concurrently read and write streams
making it easier for programmers to focus on transforming the data in
a meaningful way.

### Example
```go
fn main() {
    'Hello, world!' -> SINKS::stdout();
}
```

### Data Types
* `bool`
* `u8` `u16` `u32` `u64`
* `i8` `i16` `i32` `i64`
* `stream<T>`
* `option<T>`

You may notice there are no floating point or strings in Burn, but in
the example above there is a string literal. The actual type of that
string literal in Burn is `stream<u8>`. This makes interchanging input
streams much easier; a useful feature for testing.

## Progress and Current State
- [x] Lexing
- [x] Parsing
- [ ] Checking
- [ ] Code generation

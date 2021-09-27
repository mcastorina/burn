# Burn Compiler Design
Details on the compilation process and transforming Burn programs.

Because this is an experimental language, the Burn compiler will output
C as an intermediate representation, then use a C compiler to build
the executable.

## Goals
* Lazy evaluation
* Efficient memory usage
* Synchronous intermediate representation (see below)

Despite appearing like streams are asynchronous, the reality is that Burn
is a synchronous language. Functions are a way to inform the compiler
how to transform data and will not be called like traditional functions.
Instead, Burn analyzes the input program and will generate a synchronous
program (in an intermediate representation) using the information provided.
When writing to an output, all data is read until an EOF is encountered, then
the program moves forward.

When writing to multiple outputs at the same time (like in
[examples/duplicate.brn](./examples/duplicate.brn)), precedence is given
in order from left to right.

## Stream Design
From a programmer's perspective, any `stream` can be read from and written
to, however to the compiler there are three types of streams: Readers,
Writers, and Transformers. Streams always start with a Reader and ends in
a Writer. Distinguishing between these three types is important for optimizing
memory usage, as only the Reader stream will buffer data.

Transformer streams are the most complex, allowing *N* input streams and
*M* output streams. These model the functions a programmer will write.

### Reader Behavior
Reader streams will read in and buffer data from an upstream until
there is an EOF. If data is being pulled from the Reader, it will free
up space in its buffer. Each Reader has a 4K byte buffer regardless of
element size.

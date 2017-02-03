# core-interpreter [![Build Status](https://travis-ci.org/sejr/core-interpreter.svg?branch=master)](https://travis-ci.org/sejr/core-interpreter)

**core-interpreter** is an interpreter for Core - a basic programming language - that is implemented in Rust. It is primarily a means for me to get more hands-on experience with the Rust programming language, but is also part of an assignment for the Principles of Programming Languages course at The Ohio State University (Spring 2017 with Wayne Heym).

## Getting Started

Make sure you have [Rust](https://rust-lang.org) installed on your system.

``` bash
# Install Rust if you don't have it
curl https://sh.rustup.rs -sSf | sh

# Clone the Git repository to your local machine
git clone https://github.com/sejr/core-interpreter.git

# Navigate to the cloned directory
cd core-interpreter

# Build the core interpreter
cargo build

# Run it
cd target/debug
./core-interpreter <core-source-file-name>
```

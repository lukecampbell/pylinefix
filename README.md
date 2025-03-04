![Pipeline Status](http://git.axiom/axiom/pylinefix/badges/main/pipeline.svg)

pylinefix
===============

Fix string wrapping in Python code

Copyright 2025 Axiom Data Science, LLC

See LICENSE for details.


Someone had to write a process that could safely wrap python lines in a way
consistent with PEP-8 and for some reason black and other formatters can't.
So this is a CLI that can do it and I can wrap that in a macro in vim and
VSCode.

The process takes a single line as input via stdin and wraps the python
string at the 80th column by default, but can be overridden with the '-l'
argument. Word boundaries are respected when wrapping the string and indents
are assumed to be spaces. I won't support tabs for religious reasons.

# Examples

```
echo '    error = "this is a ridiculously long explanation that needs to be wrapped appropriately in order for it to be PEP-8",' | pylinefix
```
Produces

```
   error = (
       "this is a ridiculously long explanation that needs to be wrapped "
       "appropriately in order for it to be PEP-8"
   ),
```


Building
--------

In order to build the project, contributors need rust, see
[Install Rust](https://www.rust-lang.org/tools/install) for details about
installing the rust development environment on your system.

To build the project:

    cargo build

To run the binary without building a release version or installing to a locally available path:

    cargo run

For details about `cargo` and using `cargo`, please see [The Cargo Book](https://doc.rust-lang.org/cargo/commands/index.html)


Usage
-----

```
Program Arguments

Usage: pylinefix [OPTIONS]

Options:
  -l, --linewrap <LINEWRAP>  [default: 80]
  -s, --str-char <STR_CHAR>  [default: "]
  -h, --help                 Print help
  -V, --version              Print version
```

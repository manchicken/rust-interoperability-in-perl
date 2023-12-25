# rust-interoperability-in-perl

A prototype repo for testing Rust interoperability in Perl.

## What is it?

I'm looking at doing some stuff with interoperability between Perl and Rust. This is my test-bed.

## Prerequisites

These C libraries are needed:

- `libffi` with headers
- `libssl` with headers

You will also need a Rust environment. If you don't know how to set this up yet, consider using [rustup](https://rustup.rs/).

## Running

### Short version

1. Build the Rust code with `cargo build` (right now I'm only using the `debug` target)
2. Run the scripts individually

### Longer version

```sh
cd rust-part; cargo build; cd -
carton install
perl ./add_two.pl
```

## The files

The `cpanfile` and `cpanfile.snapshot` are artifacts for the [Carton dependency manager](https://metacpan.org/pod/Carton).
Carton will install the modules in a directory called `local` in the working directory.

The `rust-part` folder contains a normal Rust program, but it's a `cdylib` crate so it'll produce a `.so` file.

The scripts themselves are just in the top-level directory, with a `.pl` extension.

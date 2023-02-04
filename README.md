# DemoDerive derive macro

![CI Checks](https://github.com/denwong47/demo_cookiecutter_rustderivemacro/actions/workflows/CI.yml/badge.svg?branch=main)

This library provides a simple derive macro to demonstrate the output of cookiecutter-rustderivemacro.

Example
-------

```rust
use demo_cookiecutter_rustderivemacro::DemoDerive;


#[derive(DemoDerive)]
struct MyStruct {
    _populate_me: i32,
}

assert!(
    // Write a test here
    true
);

```

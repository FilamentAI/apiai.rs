# APIAI.rs

[![Build Status](https://travis-ci.org/ravenscroftj/apiai.rs.svg?branch=master)](https://travis-ci.org/ravenscroftj/apiai.rs)

A Rust client for api.ai chatbot API.

This is very much a work in progress at the moment and does not work.

Due to a dependency on serde for serialization and deserialization of api.ai
JSON structures, this library currently requires Rust Nightly until
[RFC 1681](https://github.com/rust-lang/rust/issues/35900) makes it into the
stable build.

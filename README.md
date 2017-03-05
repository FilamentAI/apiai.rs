# APIAI.rs

[![Build Status](https://travis-ci.org/ravenscroftj/apiai.rs.svg?branch=master)](https://travis-ci.org/ravenscroftj/apiai.rs)

A Rust client for api.ai chatbot API.

Still in early development - this library currently works on a a limited golden
path when calling api.ai/query.


## Usage

Add the following to your Cargo.toml:

```
[dependencies]
apiai = 0.1.0
```

Then import and use the crate in your module:

```rust
extern crate apiai;
```

A simple example call to API.ai might look like this:

```rust

let my_token = String::from("ce2f54f8eb444d74af85f89e30ef2fd3");

let client = ApiAIClient{
    access_token: my_token,
    ..Default::default()
};

let req = ApiRequest{
    query: Option::Some(String::from("Hello!")),
    ..Default::default()
};

let response = client.query(req).unwrap();

println!("{}", response.result.fulfillment.speech);

```

For more information see the examples/botcmd directory.    

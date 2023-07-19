# srfng

Semi-Random Filename Generator

[[crates.io](https://crates.io/crates/srfng)] [[github.com](https://github.com/coruscateor/srfng)]

This library makes it convenient to generate file names consisting of a date and random characters.

Example usage:

```rust
let mut gen = srfng::Generator::new();

println!("{}", gen.generate().as_str());
```

Which prints out something like: 

```
13072022_5m88i663tw17265F
```

Which you can use to name a file.

A commandline tool using this crate can be found [here](https://github.com/coruscateor/srfngout).

## Coding Style

This project uses a coding style the emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```
fn foo()
{

    bar();

}

```

Not this:

```
fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


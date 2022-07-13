# srfng

Semi-Random Filename Generator

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

# srfng

Semi-Random Filename Generator

This library makes it convenient to generate file names consisting of a date and random characters.

Example usage:

```rust
let mut gen = srfng::generator::Generator::new();

println!("{}", gen.generate().as_str());
```

Which in my case prints out something like: 

```
Mjoo41CE061MmNu8_19012022
```

Which you can then use to name your file.

In the future I may start the string with the date by default so a directory of files using this naming scheme would be somewhat orginised.


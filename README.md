# overworld

A library of incremental game / management related reusable components.

## Installation

There are several ways to install Overworld:

### From the metapackage

1. `cargo add overworld`
2. Configure the features you want
   1. the `default` are a sane set
   2. `all` for all features
   3. or some of the virtual features likes `rpg`, `management`, `incremental` for packages used often in those genres of games
   4. specific features for only a subset, such as: `dice`, `progression` 
3. Time to start using it!


### By specific components

You can `cargo add overworld_COMPONENTNAME` (`cargo add overworld_progression`) if you just want a single crate 
(in some cases, they may depend on other crates in this repository.)

## Examples

```rust
use overworld::roll::Die;

fn main() {
    let d6 = Die::new(6);
    println!("You rolled a {}", d6.roll());
    // You rolled a 6
}
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

Licensed under the following licenses at your option:

- Apache License, Version 2.0 <[LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0>
- MIT license <[LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT>

Files in the project may not be copied, modified, or distributed except according to those terms.

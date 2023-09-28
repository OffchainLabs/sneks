## Overview

This crate provides simple proc macros for naming enum variants.

```rs
#[derive(SimpleSnakeNames)]
enum Animal {
    HoneyBee,
    GreatWhiteShark(usize),
    PoisonDartFrog { spotted: bool },
}

// expands to
impl Animal {
    fn name(&self) -> &'static str {
        match self {
            HoneyBee => "honey_bee",
            GreatWhiteShark(..) => "great_white_shark",
            PoisonDartFrog { .. } => "poison_dart_frog",
        }
    }
}
```

## License

&copy; 2022-2023 Offchain Labs, Inc.

This project is licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([licenses/Apache-2.0](licenses/Apache-2.0))
- [MIT license](https://opensource.org/licenses/MIT) ([licenses/MIT](licenses/MIT))

at your option.

The [SPDX](https://spdx.dev) license identifier for this project is `MIT OR Apache-2.0`.

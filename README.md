# IOKit Rust Bindings

The `IOKit-sys` package provides declarations and linkage for the `IOKit` C library on OS X.
Following the `*-sys` package conventions, the `IOKit-sys` package does not define higher-level
abstractions over the native library.

## Usage
Add `IOKit-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
IOKit-sys = "0.1"
```

Import the `IOKit_sys` crate and use the functions as they're defined in the native `IOKit` library
provided by Apple.

```rust
extern crate IOKit_sys as io;
```

## Contributing
You may find that you need some functionality that is missing from `IOKit-sys`. If that's the case,
please open an issue on Github or send a pull request with the added functionality.

If you plan to submit a pull request, please note the structure of the code. There is one file for
each header file in the IOKit framework. For example, `src/io_return.rs` contains the definitions
from `IOKit/IOReturn.h`. The definitions in each file are more or less in the same order that they
appear in the matching header file. Each file is then re-exported in the crate root, e.g., `pub use
io_return::*`.

### Contributors
* [dcuddeback](https://github.com/dcuddeback)
* [dhylands](https://github.com/dhylands)
* [forticulous](https://github.com/forticulous)
* [ndusart](https://github.com/ndusart)

## License
Copyright © 2015 David Cuddeback

Distributed under the [MIT License](LICENSE).

# __PasswordGenerator__

A simple password generator in rust

## __Requirements__

You need to have cargo & rustup installed on your pc to use this.

(Here)[https://www.rust-lang.org/tools/install] you can find a tutorial to set up your pc in order to run the project.

## __How to use it__

Clone the repository or download the ZIP file.

Go to password_generator file:
```Bash
cd password_generator
cargo build
```

and then you can use it by simply run:
```Bash
cargo run
```

It will generate you a password with the default settings (Length of 6, lowercase characters only)

To know more about how to use it, run it with the --help or -h:
```Bash
cargo run -- --help
```

To clean the repo after a build, just run:
```Bash
cargo clean
```

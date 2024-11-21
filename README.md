# Repo Comparator
A simple CLI utility which compares 2 branches of unstable ALT repository Sisyphus using AltRepo API.

The utility uses [librepo-comparator](https://github.com/qualimock/librepo-comparator) crate, which represents the core of work.

## Prerequisites
To build the program you should use Rust package manager [Cargo](https://doc.rust-lang.org/stable/cargo/)

It is available systemlessly on [rustup](https://rustup.rs/)

Or can be installed on your ALT distro via apt-get:
```
apt-get install rust-cargo
```

## How to build
Clone the repository:
```
git clone https://github.com/qualimock/repo-comparator.git
cd repo-comparator
```

Build the program:
```
cargo build --release
```

Cargo will fetch all the dependencies (including librepo-comparator from GitHub) by itself.

The program can be installed:
```
cargo install --path .
```

## How to use
The program accepts 2 positional arguments: a branch A and a branch B (sequense is important).

Available branches are: p9, p10, p11, sisyphus

To run the program if it has not been installed execute:
```
cargo run -- <branch A> <branch B>
```

If the program is installed on system, execute:
```
repo-comparator <branch A> <branch B>
```

### Structure of JSON
\<branch A\> and \<branch B\> will be replaced with actual names.
```json
"in_branch_<branch A>_not_in_<branch B>": {
    "length": 0,
    "packages": [
        {
            "arch": "string",
            "disstag": "string",
            "epoch": "string",
            "name": "string",
            "release": "string",
            "source": "string",
            "version": "string"
        }
    ]
},
"in_branch_<branch B>_not_in_<branch A>": {
    "length": 0,
    "packages": [
        {
            "arch": "string",
            "disstag": "string",
            "epoch": "string",
            "name": "string",
            "release": "string",
            "source": "string",
            "version": "string"
        }
    ]
},
"packages_newer_in_<branch A>": {
    "length": 0,
    "packages": [
        {
            "arch": "string",
            "disstag": "string",
            "epoch": "string",
            "name": "string",
            "release": "string",
            "source": "string",
            "version": "string"
        }
    ]
}
```

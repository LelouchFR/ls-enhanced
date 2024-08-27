<p align="center"><img align="center" width="280" src="./.github/ls-enhanced.svg"/></p>
<h3 align="center">a better looking file listing tool</h3>
<hr />

<img src=".github/example.png" width="200">

<p><a href="https://github.com/lelouchfr/ls-enhanced"><img src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" /></a> <a href="https://crates.io/crates/ls-enhanced"><img src="https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust" /></a></p>

## Installation

### using cargo:

```bash
cargo install ls-enhaced --locked
```

for a better usablility, add an alias to the command:

```bash
alias lse="ls-enhaced"
```

### from source:

```bash
git clone https://github.com/LelouchFR/ls-enhaced.git
mkdir ~/.config/lse
mv .github/config.toml ~/.config/lse/config.toml
cargo run
```

add an alias in you environment:

```bash
alias lse="path/to/executable"
```

#### Ups and Downs of using from source:

if you use the cargo version, you don't have to add the config file yourself, it will be generated for you automatically on first execution, but it will be a little "messy", not like the version build from source.

## 1. Getting the tools

Need Rust nightly

```sh
rustup default nightly
```

```sh
rustup update
```


Also need cargo-web

```sh
cargo install cargo-web
```

## 2. Run the game

```sh
cargo web start
```

Then go to [http://[::1]:8000](http://[::1]:8000)

## 3. Libraries used

[nalgebra](https://github.com/rustsim/nalgebra)

[Quicksilver](https://github.com/ryanisaacg/quicksilver)

[num-traits](https://github.com/rust-num/num-traits)
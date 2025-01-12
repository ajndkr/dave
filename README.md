# devx

cli for automating my dev workflows. built in pure rust 🦀

## getting started:

### pre-requisites:

1.  install [rust](https://www.rust-lang.org/tools/install)

2.  verify installation:

    ```shell
    rustup --version
    cargo --version
    ```

3.  install ci dependencies:

    ```shell
    rustup component add rustfmt
    rustup component add clippy
    cargo install cargo-audit
    ```

### Usage:

```shell
devx help
```

the above command will list all the available commands.

## development:

1.  run code ci checks (formatting & linting):

    ```shell
    make ci
    ```

2.  test changes:

    ```shell
    make dev <devx-cli-args ...>
    ```

3.  clean up:

    ```shell
    make clean
    ```

4.  install as binary:

    ```shell
    make install
    ```

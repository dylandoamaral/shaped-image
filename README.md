# Shaped Image

A genetic algorithm that generate art in Rust to train myself with the language

## Example

```console
$ cargo build --release
$ ./target/release/shaped-image --input-path example/owl.jpg --output-path example/owl-shaped.jpg --specimens 200 --until 10000 --convergence 500
```

<p align="center">
  <img src="https://raw.githubusercontent.com/dylandoamaral/shaped-image/main/examples/owl.jpg" />
</p>
<p align="center">
  <img src="https://raw.githubusercontent.com/dylandoamaral/shaped-image/main/examples/owl-shaped.jpg" />
</p>
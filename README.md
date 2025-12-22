# svgx

**svgx** is a high-performance SVG optimizer written in Rust. It is designed to providing fast and safe SVG optimization.

🚧 **Work in Progress**: This project is currently in the early stages of development.

## 🚀 Features

- **Blazing Fast**: Built with Rust and `xmlparser` for minimal overhead.
- **AST-based Optimization**: Parses SVG into a DOM-like structure to apply robust transformations.
- **Plugin System**: Modular architecture for optimization passes.
    - `removeComments`: Removes comments from SVG files.
    - *(More plugins coming soon)*
- **CLI**: Simple command-line interface.

## 📦 Installation

Ensure you have [Rust installed](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/yourusername/svgx.git
cd svgx
cargo build --release
```

## 🛠 Usage

Run the `svgx` binary with an input file:

```bash
# Print optimized SVG to stdout
cargo run --release -- input.svg

# Write optimized SVG to a file
cargo run --release -- input.svg -o output.svg
```

### Example

**Input (`test.svg`):**
```xml
<svg width="100" height="100">
    <!-- This is a comment -->
    <rect width="100" height="100" fill="red" />
</svg>
```

**Command:**
```bash
cargo run -- test.svg
```

**Output:**
```xml
<svg width="100" height="100">
    <rect width="100" height="100" fill="red"/>
</svg>
```

## 🏗 Architecture

- **`src/parser.rs`**: Pull-based XML parser converting SVG to an internal AST.
- **`src/tree.rs`**: AST definitions (Document, Element, Node).
- **`src/plugins/`**: collection of optimization plugins implementing the `Plugin` trait.
- **`src/printer.rs`**: Serializes the AST back to a minimized SVG string.

## 🤝 Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.

1. Fork the repo.
2. Create your feature branch (`git checkout -b feature/amazing-feature`).
3. Commit your changes.
4. Push to the branch.
5. Open a Pull Request.

## 📄 License

MIT

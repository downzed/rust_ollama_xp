# xp-ollama

Welcome to `xp-ollama`, my personal repository where I experiment with [Ollama](https://ollama.com) and explore the Rust programming language through [ollama-rs](https://github.com/pepperoni21/ollama-rs). This project is a playground for testing ideas, understanding the intricacies of Ollama, and improving my Rust skills.

## Introduction

This project is in an active development stage, focusing on personal experimentation and learning. By working with Ollama and its Rust wrapper `ollama-rs`, I aim to uncover new possibilities and expand my programming horizons. The experiments here are varied and reflect my journey through the Rust language and the Ollama library.

## Getting Started

### Prerequisites

Before diving into the experiments, ensure you have Rust, Cargo, and Git installed on your system. For Rust and Cargo, follow the setup instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started). Git can be installed from its [official website](https://git-scm.com/).

### Installation

1. Clone this repository to get started:

   ```bash
   git clone https://github.com/downzed/xp_ollama.git
   ```

2. Enter the project directory:

   ```bash
   cd xp_ollama
   ```

3. Build the project using Cargo:

   ```bash
   cargo build
   ```

## Usage

To run an experiment, use the following command. Make sure to replace `c01-simple` with the specific experiment you wish to run:

```bash
cargo watch -q -c -x "run -q --example c01-simple"
```

This command uses `cargo watch` to automatically rebuild and rerun your experiment when files change, making it easier to test and refine your experiments continuously.

## Contributing

As this is a personal, in-development project, I'm open to ideas, suggestions, and contributions. Feel free to fork the repo, create issues, or send pull requests. Any form of feedback or contribution is welcome!

## Acknowledgments

- Special thanks to [Ollama](https://ollama.com) and [ollama-rs](https://github.com/pepperoni21/ollama-rs) for providing the tools and inspiration for this project.
- Appreciation for the Rust community, whose resources have been invaluable in this learning journey.

## License

This project is open-sourced under the MIT License - see the [LICENSE](LICENSE) file for details.

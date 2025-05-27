# <p align="center"><img src="./thumb-horizontal.jpg" alt="Artificial Culture Rust"></p> 

Artificial Culture Rust is a modular framework for simulating emergent social behavior using an Entity-Component-System (ECS) architecture. It enables the modeling of agents with physiological, psychological, social, and cultural components, supporting research and development in artificial societies, collective behavior, and cultural evolution.

> **Disclaimer:** This project is being developed with the assistance of GitHub Copilot, GPT, and Manus AI. The scope and ambition of this project make it impractical for a single person to manually research and implement all features—AI assistance is essential for productivity and innovation.

## Features
- Modular ECS architecture for flexible agent and environment modeling
- Granular components: physiological, psychological, social, and cultural
- Fuzzy logic and adaptive decision-making flows
- Extensible systems and flows for custom simulations
- Written in Rust for performance and safety

## Dev Container Guide (For Developers)
This project is designed to be developed inside a [VS Code Dev Container](https://code.visualstudio.com/docs/devcontainers/containers). The dev container includes:
- Up-to-date Rust toolchain and common utilities
- Pre-installed dependencies for Rust development
- Git and GitHub CLI
- All system libraries required for building and running the project

**To get started:**
1. Open the project in VS Code.
2. When prompted, "Reopen in Container". If not prompted, open the Command Palette (`Ctrl+Shift+P`) and select `Dev Containers: Reopen in Container`.
3. The environment will be set up automatically for you.

## Installation
1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/Artificial-Culture-Rust-.git
   cd Artificial-Culture-Rust-
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage
- Run the simulation:
  ```bash
  cargo run
  ```
- Explore and edit the documentation in the `Docs/` folder for detailed architecture and flow descriptions.

## Documentation
- [`Docs/Structures/`](./Docs/Structures/) — ECS architecture, components, entities, and systems
- [`Docs/Flows/`](./Docs/Flows/) — Decision-making, fuzzy weighting, and interaction flows
- [`Docs/Fundaments/`](./Docs/Fundaments/) — Theoretical background (neurological, psychological, sociological)

## Contributing
Contributions are welcome! Please open issues or submit pull requests for improvements, bug fixes, or new features.

## License
[MIT License](LICENSE)

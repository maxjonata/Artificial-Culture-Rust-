# <p align="center"><img src="./thumb-horizontal.jpg" alt="Artificial Culture Rust"></p>

Artificial Culture Rust is a simulation of rumor propagation in a social network using an Entity-Component-System (ECS)
architecture with Bevy. It models NPCs (Non-Player Characters) that move around in a 2D environment and spread rumors
based on proximity and personality traits, demonstrating basic emergent social behavior.

> **Disclaimer:** This project is being developed with the assistance of GitHub Copilot, GPT, Claude, and Manus AI. The
> scope and ambition of this project make it impractical for a single person to manually research and implement all
> features while having those tools at disposal — AI assistance is essential for productivity and innovation.

## Features

- Bevy-based ECS architecture for agent simulation
- Basic personality traits (openness) affecting social interactions
- Rumor propagation system based on proximity and personality
- Dynamic movement system with boundary collision detection
- Visual representation with color changes to indicate rumor knowledge
- Written in Rust for performance and safety

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/maxjonata/Artificial-Culture-Rust-.git
   cd Artificial-Culture-Rust-
   ```
2. Build the project:
   ```bash
   cargo build --release --all-features -Z unstable-options
   ```

## Usage

- Run the simulation:
  ```bash
  cargo run --release --all-features -Z unstable-options
  ```
  This will open a window showing NPCs (green boxes) moving around. After a few seconds, a rumor will be injected into a
  random NPC, which will turn red. The rumor will then spread to other NPCs based on their proximity and personality
  traits (openness).

- Press `Escape` to toggle the debug inspector, which allows you to view and modify _entity properties in real-time.

- Explore and edit the documentation in the `Docs/` folder for detailed architecture and flow descriptions.

## Documentation

> **Note:** Some documentation files are marked as unfinished and may not fully reflect the current state of the
> project.

- [`Docs/Structures/`](./Docs/Structures/) — ECS architecture, components, entities, and systems
- [`Docs/Flows/`](./Docs/Flows/) — Decision-making, fuzzy weighting, and interaction flows
    - [`roadmap.md`](./Docs/Flows/roadmap.md) — Detailed development plan with phases and milestones
- [`Docs/Fundaments/`](./Docs/Fundaments/) — Theoretical background (neurological, psychological, sociological)
- [`Docs/Papers/`](./Docs/Papers/) — Collection of academic papers that form the theoretical foundation of the project,
  including works on:
    - Cognitive science and decision-making (Kahneman's "Thinking, Fast and Slow")
    - Social networks and collective behavior (Granovetter's "The Strength of Weak Ties")
    - Rumor propagation and epidemic spreading models
    - Trust and reputation systems
    - Neuroscience and consciousness

## Contributing

Contributions are welcome! Please open issues or submit pull requests for improvements, bug fixes, or new features.

## License

This project is licensed under CC BY-NC-SA 4.0 - free for personal/academic use, commercial licensing available.
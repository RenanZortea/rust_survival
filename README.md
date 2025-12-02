<div align="center">
  <img src="assets/logo.png" alt="Rust Survival Logo" width="600" />
  
  <br />
  
  <a href="https://opensource.org/licenses/Apache-2.0">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License" />
  </a>
  <img src="https://img.shields.io/badge/panic-abort-red" alt="Panic Strategy" />
  <img src="https://img.shields.io/badge/arch-embedded%20toaster-orange" alt="Architecture" />
</div>

<br />

# Rust Survival: The Post-Runtime Wasteland

# 🦀 Rust Survival: The Post-Runtime Wasteland

[](https://opensource.org/licenses/Apache-2.0)
[](https://www.google.com/search?q=)
[](https://www.google.com/search?q=)
[](https://www.google.com/search?q=)
[](https://www.google.com/search?q=)
[](https://www.google.com/search?q=)

> **WARNING:** This game requires manual memory management comprehension. Usage of `.unwrap()` in production is strictly prohibited and will result in character permadeath.

---

## 📜 The Lore: The Great Dependency Collapse

The world didn't end with a bang. It didn't end with a whimper.
**It ended with `npm install`.**

Historians call it **The Stack Overflow of 2034**.

It started on a Tuesday. A maintainer in Nebraska grew tired of maintaining a 3-line library called `is-odd-number`. He deprecated the package and deleted the repo. He didn't know that `left-pad-ultra`, which was a dependency of `react-native-global-finance`, which was a dependency of the **World Central Bank’s AI**, relied on it.

At 09:00 UTC, the global economy threw a `ReferenceError: undefined is not a function`.
The markets didn't crash; they simply halted, awaiting a `Promise` that never resolved.

- **The Power Grid:** Ran on a Python script that was "technically just a prototype." It hit a runtime error on line 4,000 and nobody knew how to restart the interpreter without a virtual environment.
- **The Defense Network:** Written in legacy C++. It attempted to access a pointer it had freed in 1999. The resulting SegFault launched every nuke into the ocean.
- **The Internet:** Collapsed under the weight of `node_modules` folders that had become dense enough to generate their own gravity fields.

Civilization choked to death on its own abstraction layers.

---

## 🌍 The World: Year 2500

Welcome to the **Alloc-Free Zone**.

The air is breathable, but the hardware is garbage. The high-end servers of the past have rusted away. What remains are microcontrollers: scavenged smart toasters, hearing aid processors, and digital pregnancy test screens.

In the Before Times, they mocked the **Rustaceans**.
_"Why fight the Borrow Checker?"_ they laughed, sipping kombucha while their Electron apps consumed 16GB of RAM to display a To-Do list. _"Move fast and break things\!"_

Well, they broke everything. And they moved so fast they ran right off a cliff.

Now, those interpreted languages are dead.

- **Java?** You can't run the JVM on a potato. The Garbage Collector pauses take longer than your oxygen tank lasts.
- **Python?** Too slow. By the time the interpreter starts, the mutant wolves have already eaten you.
- **JavaScript?** `[object Object]`.

---

🛠️ Installation & Mission Initialization

This isn't just a game. It's an exam. You need the Rust Toolchain to survive.

1. Acquire the Binary

Compile from Source (Recommended for Scavengers)

```

git clone [https://github.com/renanzortea/rust_survival.git](https://github.com/renanzortea/rust_survival.git)
cd rust_survival
cargo build --release

```

2. Initialize the Workspace

Before you can fix the hardware, you need to extract the corrupted firmware to your local deck.

Run the initialization sequence:

# If running from source:

```
cargo run -- --init
```

# If installed globally:

```
rust_survival --init
```

This will generate a missions/ directory in your current location.
⚠️ DO NOT DELETE THIS FOLDER. It contains the source code you must repair.

3. The Survival Loop

Launch the Interface: Run cargo run (or rust_survival).

Select a Mission: Use Arrow Keys to navigate the terminal UI.

Identify the Bug: The mission log will describe the hardware failure.

Edit the Code: Open the corresponding file in missions/ (e.g., missions/01_shelter.rs) in your favorite text editor (VS Code, Vim, Nano).

Compile & Verify: Press C inside the game to trigger a hot-compile of your local file.

Success: The system comes online, and you progress.

Failure: The compiler errors appear in the game log. Read them. Fix them. Survive.

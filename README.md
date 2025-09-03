# EL-Init: The Enterprise Linux Initializer

A friendly, terminal-based user interface (TUI) for a quick setup and customization of Rocky Linux, AlmaLinux, CentOS Stream, and RHEL systems.


---

## ## About

**EL-Init** solves the repetitive task of writing post-installation shell scripts. Instead of manually scripting `dnf` commands and configurations for every new server, workstation, or virtual machine, EL-Init provides an interactive menu to select the components you need. It then generates a shell script to automate the setup process.
<img width="1417" height="1699" alt="image" src="https://github.com/user-attachments/assets/085b8b7e-fbf2-4db1-9aab-397ec214408a" />


---

## ## Features

* **Interactive TUI**: A fast and intuitive terminal interface built with Rust and `ratatui`.
* **Modular Script Generation**: Pick and choose from a deep, tree-style menu of options.
* **Robust Scripts**: The generated shell script automatically exits on any error (`set -e`) and clearly logs each step it performs.
* **Dependency-Aware**: Intelligently runs repository setups *before* attempting to install packages, preventing common failures.
* **Broad Customization**: Configure everything from virtualization engines and graphical environments to specific applications and system repositories.
* **Automatic OS Detection**: Tailors available scripts and commands to the specific Enterprise Linux distribution you're running.
* **Cross-Compatible**: Written in Rust, it compiles to a single, portable binary.

---

## ## Supported Operating Systems

This tool is designed for and tested on derivatives of Red Hat Enterprise Linux (RHEL) 9 and newer:

* **Rocky Linux** (9.x+, 10.x+)
* **AlmaLinux** (9.x+, 10.x+)
* **CentOS Stream** (9+, 10+)
* **Red Hat Enterprise Linux (RHEL)** (9.x+, 10.x+)

---

## ## Getting Started

### ### Prerequisites

-TBD-

### ### Building the Project

Clone the repository and use `cargo` to build the release binary.

```bash
# Clone this repository
git clone https://github.com/svkadmin/el-init.git

# Navigate into the project directory
cd el-init

# Build the optimized release binary
cargo build --release

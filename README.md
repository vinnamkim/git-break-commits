# git-break-commits

`git-break-commits` is an interactive Command Line Interface (CLI) tool that helps break existing Git commits and reorganize them into new commits.
Please refer to the following video to see how it works!

[Watch Demo Video](https://github.com/vinnamkim/git-break-commits/assets/26541465/da03b7cb-efb0-43b9-9222-8f10c21bf2f1)

## Installation

### 1. Installing from the source

Since `git-break-commits` is built with [Rust](https://www.rust-lang.org), you need to install Rust first if it is not already on your system.

```console
# Install Rust
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Configure `PATH`
$ export PATH="$HOME/.cargo/bin:$PATH"
```

Then, clone this repository and install `git-break-commits`.

```console
# Clone this repository
$ git clone https://github.com/vinnamkim/git-break-commits.git

# Install `git-break-commits`
$ cargo install --path git-break-commits
```

### 2. Installing from the pre-built binary

It is not supported yet.

## Usage

To use `git-break-commits`, run the following command in your terminal:

```bash
git-break-commits <depth>  # Default <depth> is 1
```

This command will display the changes between `HEAD~<depth>` and `HEAD` in a CLI file navigator.
You can navigate it to select changes for a new commit.
The new commit will be stacked on top of `HEAD~<depth>` after breaking all commits between `HEAD~<depth>` and `HEAD`.
Please refer to the following key mappings in this step:

- ↑: Move the cursor up
- ↓: Move the cursor down
- ←: Go to the parent directory
- →: Go to the subdirectory
- Space: Select or unselect a file/directory
- Enter: Save the current selection and go to write the commit message
- q or Ctrl + c: Quit without making any changes
- h: Open the help pop-up message

You should repeat this process until all changes between `HEAD~<depth>` and `HEAD` are resolved.
***We believe that your commit history will be cleaner than before with this minimal effort!***

## License

This software is licensed under the [MIT License](LICENSE).

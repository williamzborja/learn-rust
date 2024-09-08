# Learn Rust 🦀

This repository is my guide to learn about Rust this amazing programming languages.

## Description

rust is a modern programming language of general use useful to create low-level, high level, CLI, Libraries, Backend, WebAssembly, Embebed and IOT and ... with a really strong focus in correctness, security with an entire ecosystem and tools like `cargo` that allow create, build, test, publish and share your rust code only installing the language.


## Installation

You can install rust using `rustup` that is a toolchain manager for rust.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


## Cargo

cargo is a really powerful system that allow you create rust binary or libraries, linking your rust files and compile that in an binary file all that things happens when you write.

```bash 
cargo build
```


## Jupyter Notebook

I'm using jupyter notebook to write my notes and exercises, you can install jupyter notebook using pip.


you need install also the rust kernel for jupyter notebook.
```bash
cargo install evcxr_jupyter
evcxr_jupyter --install
```


```bash
pip install jupyter
```

now you can run jupyter notebook using the command `jupyter notebook` and open the notebook `Rust.ipynb` to see my notes and exercises.


## Folder structure

- language: Is a folder with the exercises to learn the rust language with some examples in jupyter notebooks.
  
- projects: Is a folder with the exercises to create some projects using rust.

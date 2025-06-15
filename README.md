# Shelljougahara

A simulated shell environment written with a virtual file system, written in Rust.

## Motivation

This project is part of the [DesCode](https://github.com/desforgehub/DesCode) project, a programming
learning platform. In it, it used for a specific type of exercise, where the user interacts with an
instance of the shell, and the goal is to either get some information from the file system, or to
execute specific commands to learn your way around a terminal, and the file system of an OS.

## Goal

Writting a shell environment that closely mimics the behavior of a real Linux shell, implementation
is heavily influenced by it. Sometimes, for simplicity it may deviate but the overall feel should be
similar.

## Features / Roadmap

- [ ] Simple shell capable of executing defined commands
- [ ] File system that implements file system operations (create, read, write, move, delete)
- [ ] User management (create, delete, change user)
- [ ] Group management (create, delete, change group)
- [ ] Permissions management (with groups and users)
- [ ] Session management (multiple sessions support)
- [ ] Advanced shell features (piping, redirection, etc.)
- [ ] Scripting support (shell scripts)

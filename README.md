# Rustlang 2023

## Section 1: rust_devbranch_start01

### Create a New Project

<!-- livebook:{"force_markdown":true} -->

```elixir
>$ cargo new <project-name> --bin
```

### Run the <tree> command to see the folder structure generated

<!-- livebook:{"force_markdown":true} -->

```elixir
tree .
├── Cargo.lock
├── Cargo.toml
├── src
   ├── main.rs
   └── my_lib
       ├── coin.rs
       └── enum_example.rs
```

### To add a cargo.toml file to an existing project

<!-- livebook:{"force_markdown":true} -->

```elixir
>$ cargo init
```

### Update to the latest and stable version

<!-- livebook:{"force_markdown":true} -->

```elixir
>$ rustup default stable
```

<!-- livebook:{"break_markdown":true} -->

### Cargo add ons you need:

> > >

<!-- livebook:{"force_markdown":true} -->

```elixir
>$ cargo install cargo-edit
```

>

<!-- livebook:{"force_markdown":true} -->

```elixir
>$ cargo install cargo-watch
```

>

<!-- livebook:{"force_markdown":true} -->

```elixir
>$ cargo install cargo-outdated
```

>

<!-- livebook:{"force_markdown":true} -->

```elixir
>$ cargo install cargo-generate
```

<!-- livebook:{"break_markdown":true} -->

### Creating a workspace for all of the projects or if its a bunch of microservices

<!-- livebook:{"force_markdown":true} -->

```elixir
# workspace directory
>$ touch Cargo.toml
```

> ### Then inside the Cargo.toml file add the following:
>
> >

<!-- livebook:{"force_markdown":true} -->

```elixir
[workspace]

members = [
  "project1",
  "project2",
  "pproject3",
  etc...
]
```

> ### You then want to run `cargo build`, which a `Cargo.lock` file with the following inside it:

<!-- livebook:{"force_markdown":true} -->

```elixir
....
...
[[package]]
name = "project1"
version = "0.1.0"
dependencies = [
 "rand",
]
....
...
```

### You will see a lot of default packages but whats important is that `project1` when you ran `cargo new project1` created the `project` in the `workspace` with its `dependencies`. In this case the `dependency` was the `rand` package.

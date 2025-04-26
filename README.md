

<div align="center">
  <h1>Gumbo</h1>
  <img src="https://raw.githubusercontent.com/lex148/gumbo/main/src/templates/logo.webp"/>
  <h3>Best of the best rust projects all mixed together</h3>
</div>

# Gumbo

Gumbo is a powerful and easy-to-use CLI tool designed to help you quickly interact with Welds-ORM, Scaffold websites, or Actix.
It is built for each of these pieces individually or as a tool to work with them all together.


## Installation

You can easily install Gumbo using Cargo, Rust's package manager. Simply run the following command:

```
cargo install gumbo
```


## Basic Usage

Initialize a New Project

To create a new project, use the following command:

```bash
gumbo init projectname
```

Generate a Scaffold

To generate a scaffold for a new resource, use:


```bash
gumbo generate scaffold resource field1:type field2:type
```

Generate a Controller

To generate a controller with specific actions, use:


```bash
gumbo generate controller controllername action:method
```




<div align="center">
  <h1>Gumbo</h1>
  <img src="https://raw.githubusercontent.com/lex148/gumbo/main/src/templates/logo.webp"/>
  <h3>Best of the best rust projects all mixed together into a website framework</h3>
</div>

# Gumbo

Gumbo is a powerful and easy-to-use tool designed to help you quickly scaffold and template basic websites, similar to Ruby on Rails. Gumbo leverages the robustness of Actix for the backend, the modern frontend capabilities of Yew, and the efficient ORM capabilities of Welds. With Gumbo, you can streamline your web development process, ensuring a smooth and efficient workflow from start to finish.


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


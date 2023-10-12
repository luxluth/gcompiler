# gcompiler

A simple compiler for the G description language for simple graphs, written in Rust.

**Table of Contents**
- [gcompiler](#gcompiler)
  - [What is G?](#what-is-g)
  - [G Descriptions](#g-descriptions)
    - [Functions](#functions)
      - [`@line`](#line)
      - [`@point`](#point)
      - [`@graph`](#graph)


## What is G?

G is a simple language for describing graphs. It is a simple language that is easy to read and write, and is designed to generate graphs output in svg format and png format. Can be used to generate graphs for use in documentation (i.e. Markdown files), or for use in other applications.

The **gcompiler** compile G files into svg or png files.

## G Descriptions

Each `.g` files contains a `#root` declaration and a defined `x` and `y` axis.

A simple graph description looks like this:

```g
#define x
    min 0
    max 100
    name "x"
#end

#define y
    min 0
    max 100
    name "y"
#end

#root
    box 0, 0, 100, 100
    color 0x000000
    background 0xffffff
    axis x, y
#end
```

This will generate a graph with a box of size 100x100, with the x and y axis defined by the `x` and `y` definitions.

The `x` and `y` definitions define the axis of the graph. They are defined by the `min` and `max` values, and the `name` of the axis. The `name` is used to label the axis.

The `min` value is optional, and defaults to 0. The `max` value is not optional, and must be defined.

### Functions

The G description language supports functions. You can't define your own functions, but you can use the built-in functions.

The built-in functions are:
  - `@line` Draws a line from one point to another.
  - `@point` Draws a point at a given point.
  - `@graph` Draws a graph of a given function.

#### `@line`

The `@line` function draws a line from one point to another.

```g
@line
    from 0, 0
    to 100, 100
    name "line"
    color 0x000000
#end
```

- `from` is either an `INT` or a `FLOAT`

- `to` is either an `INT` or a `FLOAT`. 

- `name` is optional

- `color` is optional.


#### `@point`

The `@point` function draws a point at a given point.

```g
@point
    at 50, 50
    name "A"
    color 0x0000ff
#end
```

- `at` is either an `INT` or a `FLOAT`
- `name` is optional
- `color` is optional

#### `@graph`

The `@graph` function draws a graph of a given function.

```g
@graph
    name "x^2"
    color 0xff0000
    thickness 2
    func "x^2"
#end
```

- `name` is optional
- `color` is optional
- `thickness` is optional
- `func` is required. It is a string that is a valid mathematical function. The function can use the following operators:
  - `+` Addition
  - `-` Subtraction
  - `*` Multiplication
  - `/` Division
  - `^` Exponentiation
  - `()` Parentheses
  - `x` The x value



---------
MIT License. See [LICENSE](LICENSE) for more information.

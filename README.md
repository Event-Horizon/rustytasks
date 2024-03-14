# Rusty Tasks

rusty-tasks

Rust Task List Manager in the terminal.

To start download the project, extract the zip, terminal to the cargo.toml dir and:

```
cargo run
```

## Actions:

 - Help
 - List
 - Add
 - Remove
 - Toggle Complete
 - Exit

## Examples

Command:
```

help add

```

Result:
```

    The ADD command will ADD a task when used like so:

    add "this is a test!"

```

Command:
```

list

```

Result:
```

    Tasks:
    1: Task -> test [ ]
    2: Task -> test2 [ ]
    3: Task -> test3 [ ]

```

Command:
```

add "This is a test!"

```

Result:
```

    Tasks:
    1: Task -> test [ ]
    2: Task -> test2 [ ]
    3: Task -> test3 [ ]
    4: Task -> This is a test! [ ]

```

Command:
```

remove 1

```

Result:
```

    Tasks:
    1: Task -> test2 [ ]
    2: Task -> test3 [ ]
    3: Task -> This is a test! [ ]

```

Command:
```

complete 1

```

Result:
```

    Tasks:
    1: Task -> test2 [√]
    2: Task -> test3 [ ]
    3: Task -> This is a test! [ ]

```

Command:
```

exit

```

Result:

Ends the process and returns you to your terminal.
# att

Another Time Tracker based on [rust](https://www.rust-lang.org/) that helps you track time on tasks.

## Availables Commnands
### start

Start or create a new task

```
USAGE:
    att start [FLAGS/OPTIONS] [<task_name>]

FLAGS:
    -o, --only-create       No start timer on this task
    -h, --help              Prints help information
    -V, --version           Prints version information

OPTIONS:
    -i, --initial-time <time>    Set initial time on the task.
    -f, --final-time <time>      Set final time on the task.

ARGS:
    <task_name>    Task's name or code
```

### stop

Stop tracking time on the current task

```
USAGE:
    att stop [FLAGS/OPTIONS] [<task_name>]

FLAGS:
    -h, --help              Prints help information
    -V, --version           Prints version information

OPTIONS:
    -f, --final-time <time>      Set final time on the task.

ARGS:
    <task_name>    Task's name or code
```
### report

List all the tasks registered with their time logged

```
USAGE:
    att report [FLAGS/OPTIONS]

FLAGS:
    -h, --help              Prints help information
    -V, --version           Prints version information

```
### delete

Delete a task and its time logged

```
USAGE:
    att delete [FLAGS/OPTIONS] [<task_name>]

FLAGS:
    -h, --help              Prints help information
    -V, --version           Prints version information

ARGS:
    <task_name>    Task's name or code
```
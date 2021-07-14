# att

Another Time Tracker based on [rust](https://www.rust-lang.org/) that helps you track time on tasks.

## Availables Commnands
### start

Start or create a new task

```
USAGE:
    att start [FLAGS/OPTIONS] [<task_name>]

FLAGS:
    -o, --only-create            No start timer on this task

OPTIONS:
    -s, --start-time   <time>    Set start time to the task.
    -e, --end-time     <time>    Set end time on the task.
    -d, --description  <desc>    Set description on the task.

ARGS:
    <task_name>    Task's name or code
```

__NOTE__ <time> must follow human format. For more information check [humnatime](https://docs.rs/humantime/2.1.0/humantime/fn.parse_duration.html)
### stop

Stop tracking time on the current task

```
USAGE:
    att stop [FLAGS/OPTIONS] [<task_name>]

OPTIONS:
    -f, --final-time <time>      Set final time on the task.

ARGS:
    <task_name>    Task's name or code
```

__NOTE__ <time> must follow human format. For more information check [humnatime](https://docs.rs/humantime/2.1.0/humantime/fn.parse_duration.html)
### report

List all the tasks registered with their time logged

```
USAGE:
    att report [FLAGS/OPTIONS]
```
### delete

Delete a task and its time logged

```
USAGE:
    att delete [FLAGS/OPTIONS] [<task_name>]

ARGS:
    <task_name>    Task's name or code
```
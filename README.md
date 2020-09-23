# lockpipes - a commitment-free alternative to sleep

## What is this?

A `LockPipe` is a named pipe used as a synchronization mechanism. `lockpipes` is a library for using these, and `lockpipe` is a CLI to the main parts of that library.

## Why does this exist?

To keep one-shot `DaemonSet`s on Kubernetes from restarting until I want them to.

Calling `sleep` is a commitment. You're telling the system "don't schedule me for this long". To bail out early, you can either register signal handlers or loop over a shorter sleep interval.

With Kubernetes, there's also the `terminationGracePeriodSeconds` (default 30) to consider. If the sleep interval is longer, then the grace period becomes how long it takes to restart a `Pod`.

With a `DaemonSet` and a modest number of nodes, this can lead to very long rolling restarts. This in turn makes incremental changes much more cumbersome.

In other contexts, blocking on reading from STDIN is an option. Kubernetes closes this stream though.

Creating a pipe and reading from it works, but then you need to deal with shutting it down.

This tool (and library, if you're into that sort of thing) has everything you need to manage this stuff.

## How do I get this majestic tool?

```sh
$ cargo install lockpipes
```

## How do I use it?

The `lockpipe` command has a set of subcommands for each stage of the lifecycle (create -> read -> write -> delete).

Most of the time, you'll only need two commands: `read` and `write`. Both check if the pipe `exists` and `create` it if needed. Neither `delete`s anything.

All subcommands accept a `--path` argument, and read from the `LOCKPIPE_PATH` environment variable. This specifies the path to use for the pipe.

For brevity, the examples below assume the following environment:
```
LOCKPIPE_PATH=example.pipe
LOCKPIPE_LOG_FILTER=debug
```

To read from the pipe (blocking if there are no writers), call `read`:
```
$ lockpipe read
[DEBUG lockpipes] ensuring pipe exists at "example.pipe"
[INFO  lockpipes] pipe exists at "example.pipe"
[DEBUG lockpipes] reading from pipe at "example.pipe"
```



To write to the pipe (blocking if there are no readers), call `write`:
```
$ lockpipe write
[DEBUG lockpipes] ensuring pipe exists at "example.pipe"
[INFO  lockpipes] pipe exists at "example.pipe"
[DEBUG lockpipes] writing to pipe at "example.pipe"
```

In the trivial case, it doesn't usually matter which end reads or writes. It only writes empty strings, and it discards all data read. For other scenarios, either may be more useful, such as waiting until something starts.

If you want to do complicated things, it may be helpful to handle some parts by hand.

Create a pipe with `create`. Exits 0 if it creates a pipe or something exists at the given path. Exits non-zero if something went wrong.

```
$ lockpipe create
[DEBUG lockpipes] creating pipe at "example.pipe"
[INFO  lockpipes] created pipe at "example.pipe"
```

Check if a pipe exists with `exists`. The exit status will be 0 if the pipe exists, 1 if it does not, or some other non-zero value if something went wrong.

```
$ lockpipe exists
[DEBUG lockpipes] checking if pipe exists at "example.pipe"
[INFO  lockpipes] pipe exists at "example.pipe"
```

Delete a pipe with `delete`. Exits 0 if it deletes something (or nothing exists) at the given path. Exits non-zero if something went wrong.

```
$ lockpipe delete
[INFO  lockpipes] deleted pipe at "example.pipe"
```

### Other Exit Statuses

In each case, if something went wrong, the exit status should correspond to some `errno` value. The particular value depends on platform and problem. It also logs an `error` with a more readable interpretation of the problem.

## Environment Variables

| Name                | Default Value | Examples                         | Description              |
|---------------------|---------------|----------------------------------|--------------------------|
| LOCKPIPE_PATH       | /run/forever  | /path/to/lock.pipe               | Sets path to the pipe    |
| LOCKPIPE_LOG_FILTER | info          | error, warn, info, debug, trace. | Configures log filtering |
| LOCKPIPE_LOG_STYLE  | auto          | auto, always, never.             | Configures log styling   |

## License

`lockpipes` is available under the [MIT License](https://mit-license.org/), see `LICENSE.txt` for the full text.


# Static File Server

A simple static file server.

Set up a static file server in 10 secs!

## Example

1. current dir, `/files/` endpoint and port 4567

```shell
$ ./static-file-server -e "/files/" -d "." -p 4567
Static file server serving on: http://0.0.0.0:4567/files/
```

2. `files` dir, `/` endpoint and port 8080

```shell
$ ./static-file-server -d "files"
```

## Usage

```
A simple static file server.

Usage: static-file-server.exe [OPTIONS]

Options:
  -e, --endpoint <ENDPOINT>  e.g. "/static/", default "/"
  -d, --dir <DIR>            e.g. ".", default "."
  -i, --ip <IP>              e.g. "127.0.0.1", default "0.0.0.0"
  -p, --port <PORT>          e.g. 8080, default 8080
  -h, --help                 Print help information
  -V, --version              Print version information
```

## License

```
Copyright © 2022 rn7s2 mail@rn7s2.cn
This work is free. You can redistribute it and/or modify it under the
terms of the Do What The Fuck You Want To Public License, Version 2,
as published by Sam Hocevar. See the LICENSE file for more details.
```
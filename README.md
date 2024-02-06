# Googlebot Finder

Googlebot Finder downloads, unzips, filters, and analyzes log files to highlight NGINX requests from Googlebots.

![Googlebot](googlebot.webp)

## Prerequisites

Variables declared in a tasks/vars.rs file:

- SERVERS: Array of applicable servers.
- IDENTITY: SSH credentials plus folder path ( ex. username@server:/folder/ ).
- PREFIX: Base path to store all files.
- TARGET: The site url to investigate.

## Run

Navigate to the folder containing your *src* folder and run:

``` console
./googlebot [task] [month]
```

## Example

``` console
./googlebot filter july
```

Available tasks: 
- **download**: Download the zipped (.gz) log files from the named server.
- **unzip**: Decompress the .gz files previously downloaded.
- **filter**: Create a file only containg hits to the target site.
- **divide**: Divide the filtered file into one file containing googlebot hits and another containing everything else.
- **capture**: Capture all existing search strings.
- **analyze**: Discover if search strings are repeated.

**Note**: Tasks depend on a *PREFIX/server_name/type/month* file structure ( ex. ~/iss/unzipped/june/), and the assumption that compressed log files have an *nginx_access.log-20230922.gz* naming scheme.

## License

Code is distributed under [The Unlicense](https://github.com/nausicaan/free/blob/main/LICENSE.md) and is part of the Public Domain.

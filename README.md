# Googlebot Finder

Googlebot Finder downloads, unzips, filters, and analyzes log files to highlight NGINX requests from Googlebots.

![Googlebot](googlebot.webp)

## Prerequisites

Variables declared in a tasks/vars.rs file:

- SERVERS: Array of applicable servers.
- IDENTITY: SSH credentials plus folder path ( ex. username@server:/folder/ ).
- PREFIX: Base path to store all files.

## Run

Navigate to the folder containing your *accounts.yaml* file and (dependent on the location of your inventory file) run:

``` console
./googlebot [task] [month] [targeted site]
```

## Example

``` console
./googlebot filter july google.com
```

Available tasks: 
- **download**: Download the zipped (.gz) log files from the named server.
- **unzip**: Decompress the .gz files previously downloaded.
- **filter**: Create a file only containg hits to the target site.
- **divide**: Divide the filtered file into one file containing googlebot hits and another containing everything else.

**Note**: Tasks depend on a *PREFIX/month/type/server_name* file structure ( ex. ~/june/unzipped/iss), and the assumption that compressed log files have an *nginx_access.log-20230909.gz* naming scheme.

## License

Code is distributed under [The Unlicense](https://github.com/nausicaan/free/blob/main/LICENSE.md) and is part of the Public Domain.

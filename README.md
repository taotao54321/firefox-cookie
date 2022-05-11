# Firefox cookie exporter

Reads `cookies.sqlite` of Firefox, and convert it to `cookies.txt` format.
The output may be used with [youtube-dl](https://github.com/ytdl-org/youtube-dl).

I tested this program only on Linux.

## Usage

```sh
$ cargo run -- /path/to/cookies.sqlite
```

Note: if Firefox is running, access to `cookies.sqlite` will fail due to a lock. In this case, quit Firefox or copy `cookies.sqlite`.

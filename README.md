# osu-music-symlinker

osu-music-symlinker parses [osu!](https://osu.ppy.sh) beatmap files and symlinks the music audio files into another folder.

## Usage
This program only supports UNIX-like platforms.

Build it:

``` shell
$ cargo build --release
```

Run it:

``` shell
$ ./target/release/osu-music-symlinker ~/Games/osu/drive_c/osu/Songs ~/Music
```

## License
Copyright (C) 2021 Ron B

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, using version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

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

## Oh my gosh, why didn't you just write this in shell script?

I did. It was slow and unreadable:

``` irc-log
[15:36:27] <ronthecookie> for x in *; do ln -s $(pwd)/"$x"/$(cat "$x/"*.osu | grep AudioFilename | uniq | cut -d: -f 2 | sed -e 's/^ //') "/home/ron/Music/$(echo -en $x | sed -e 's/ /_/g')"; done
[15:36:43] <ronthecookie> it doesnt even work... yet...
[15:45:19] <ronthecookie> i have some rogue \r's in that
[15:45:51] <ronthecookie> oh did i mention these files im parsing are crlf? fun!
[15:47:44] <ronthecookie> im gonna just sed them out before i grep
[15:53:37] <ronthecookie> finally got it working: for x in *; do ln -s "$(pwd)/$x/$(cat "$x/"*.osu | sed -e 's/[\r\n]//g' | grep 'AudioFilename' | head -n1 | cut -d: -f 2 | sed -e 's/^ //')" "/home/ron/Music/$x.mp3"; done
```

(Some messages cut off for anonymity)

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

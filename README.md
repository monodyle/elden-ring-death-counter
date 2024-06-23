## Elden Ring Death Counter

> *Oh yes... Tarnished, are we? Come to the Lands Between for the Elden Ring, hmm?*
> *Of course you have. No shame in it.*
> *Unfortunately for you, however, you are maidenless.*

### Usage

```bash
$ elden-ring-death-counter --help

Oh yes... Tarnished, are we? Come to the Lands Between for the Elden Ring, hmm? Of course you have. No shame in it. Unfortunately for you, however, you are maidenless.

Usage: elden-ring-death-counter [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Elden Ring save file location

Options:
  -o, --outdir <OUTDIR>    Location will write death count files (default: "output") [default: output]
  -f, --format <FORMAT>    Format of output files (defaut: "Death: {}"), where {} will be replaced by the death count [default: "Death: {}"]
  -F, --from <FROM>        Death counter will start from this value instead of counting total character death
      --outfile <OUTFILE>  Output file format (default: {slot}-{character_name}.txt) [default: {slot}-{character_name}.txt]
  -h, --help               Print help
  -V, --version            Print version

# Example
$ elden-ring-death-counter C:\Users\Monody\AppData\Roaming\EldenRing\76561198250312914\ER0000.sl2
$ elden-ring-death-counter .\76561198250312914\ER0000.sl2 -o .\Counter # output to `Counter` folder
$ elden-ring-death-counter .\76561198250312914\ER0000.sl2 --from 183 -f "I death {} times since this morning" -o .\OBS # count from 183 with my customized format
```

### OBS Setup

1. [Download pre-built binary of Death Counter][download]
2. [Download pre-built binary of watchexec][watchexec]
3. Put all binaries in the your folder you want, example: `Desktop\DeathCounter`
4. Run command or try [this bat file][aio]
```powershell
.\watchexec.exe -i <save_file> .\elden-ring-death-counter.exe <save_file> -o .\OBS
```
5. In OBS, create Text source with location `Desktop\DeathCounter\OBS\[slot]-[your_character_name].txt`

[aio]: https://gist.githubusercontent.com/monodyle/eecadfca32c4a2d87c9338f588f85291/raw/bc69e6e07ea373b28c257503c190c7955c7e7fc3/death-counter.bat
[download]: https://github.com/monodyle/elden-ring-death-counter/releases/latest
[watchexec]: https://github.com/watchexec/watchexec/releases/latest

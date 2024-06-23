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
  -o, --outdir <OUTDIR>  Location will write death count files (default: "output")
  -f, --format <FORMAT>  Format of output files (defaut: "Death: {}"), where {} will be replaced by the death count
  -F, --from <FROM>      Death counter will start from this value instead of counting total character death
  -h, --help             Print help
  -V, --version          Print version

# Example
$ elden-ring-death-counter C:\Users\Monody\AppData\Roaming\EldenRing\76561198250312914\ER0000.sl2
$ elden-ring-death-counter .\76561198250312914\ER0000.sl2 -o .\Counter # output to `Counter` folder
$ elden-ring-death-counter .\76561198250312914\ER0000.sl2 --from 183 -f "I death {} times since this morning" -o .\OBS # count from 183 with my customized format
```

### OBS Setup

1. Download pre-built binary of Death Counter
2. [Download pre-built binary of watchexec][watchexec]
3. Put all binaries in the your folder you want, example: `Desktop\DeathCounter`
4. Run command
```powershell
.\watchexec.exe -i <save_file> .\elden-ring-death-counter.exe <save_file> -o .\OBS
```
5. In OBS, create Text source with location `Desktop\DeathCounter\OBS\[slot]-[your_character_name].txt`
6. Press `Ctrl+C` to abort the counter

[watchexec]: https://github.com/watchexec/watchexec/releases/latest

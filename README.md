## Elden Ring Death Counter

> *Oh yes... Tarnished, are we? Come to the Lands Between for the Elden Ring, hmm?*
> *Of course you have. No shame in it.*
> *Unfortunately for you, however, you are maidenless.*

### Usage

```bash
$ elden-ring-death-counter <input_file> [output_folder] [output_format]

input_file: Elden Ring save file location
output_folder: location will write death count files (default: "output")
output_format: format of output files (defaut: "Death: {}"), where {} will be replaced by the death count
# relative: count from current death

# Example
elden-ring-death-counter %APPDATA%/EldenRing/<steamid>/ER0000.sl2
elden-ring-death-counter ER0000.sl2 ../OBS "Today Deaths: {}" relative
```

### OBS Setup

1. Download pre-built binary of Death Counter
2. [Download pre-built binary of watchexec][watchexec]
3. Put all binaries in the your folder you want, example: `Desktop\DeathCounter`
4. Run command
```powershell
.\watchexec.exe -i <save_file> .\elden-ring-death-counter.exe <save_file> .\output
```
5. In OBS, create Text source with location `Desktop\DeathCounter\output\[your_character_name].txt`

[watchexec]: https://github.com/watchexec/watchexec/releases/latest

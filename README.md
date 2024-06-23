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

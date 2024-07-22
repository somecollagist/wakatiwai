# waHwI'mey WakatiwaivaD
<span style="font-family:Monospace">

<h2>paq 'ay'mey</h2>

- [waHwI'mey WakatiwaivaD](#wahwimey-wakatiwaivad)
  - [`wakatiwai.sh`](#wakatiwaish)
    - [De' patmeyvam'e' DI'ngaq](#de-patmeyvame-dingaq)
  - [`run.sh`](#runsh)

## `wakatiwai.sh`
vam'e' DawaHlaHmeH, nom De'janmey DachenlaH. yIlo':
```
lo'wI': wakatiwai.sh [-hl] <wIy>
	-h  --help        : QInvam ja'
	-l  --list        : wIymey ja'
```

tu'lu'meH De'mey `.json` qawHaqDaq `tests`, ra' chay' De'jan'e' chenjaj. 'ej WakatiwaivaD'e' la''a' chen. ngu'mey'wI cha' rap wIy'e'; `wtconfig.json`-Daq rap `config` 'ej DaH 'oH `partitions`. ghaj:

|ngu'wI|ghantoH|De' ngu'wI'vamvaD|
|---|---|---|
|`type`|`20`, `F90358A9-1AA5-4264-8C74-C298A4B801B1`, `BOOT`|GPT Sep ngu'wI ghap GUID, [`fdisk_gpt_table.md`](../../tests/core/fdisk_gpt_table.md) yIlegh.<br>**yIyep: ngu'wI'vam "BOOT" 'ej 'oHlaH. 'oHchugh, ngu'wI''e' Hoch 'oHbe' `size` buSHa'moHlu'. ra''eghlu', taghmoHghach Wakatiwai'e' ghaj.**|
|`size`|`64M`, `300M`, `8G`|chay' Sepvam tIn? yIlo' 'uy'mey Hut'on (M) Saghan Hut'on (G = 1024M) ghap.|
|`fs`|`ext2`, `fat32`, `swap`|De' pat'e' Sepvam Dalo'qang. tetlh naDev [tu'lu'](#de-patmeyvame-dingaq).|

### De' patmeyvam'e' DI'ngaq
 - ext2/3/4
 - fat12/16/32
 - swap
 - btrfs

## `run.sh`
QEMU-vaD De'jan taghqangmoHchugh, `./run.sh <De'jan-De'>.img` yIjatlh.
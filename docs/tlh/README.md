# Wakatiwai
<span style="font-family:Monospace">

<p align="right">UEFI-vaD De'wI' taghmoHghach nap choHlaH je, ruStvo' ghItlhlu'.</p>

![GitHub License](https://img.shields.io/github/license/somecollagist/wakatiwai) ![GitHub last commit](https://img.shields.io/github/last-commit/somecollagist/wakatiwai)

<h2>paq 'ay'mey</h2>

- [Wakatiwai](#wakatiwai)
  - [vam bop](#vam-bop)
  - [laHmey](#lahmey)
  - [vam Dajomqang](#vam-dajomqang)
  - [Wakatiwai'e' Dalo'qangchugh, vam yIlaDbej](#wakatiwaie-daloqangchugh-vam-yiladbej)
  - [`wtconfig.json`](#wtconfigjson)
    - [taghmoHvaD QInmey](#taghmohvad-qinmey)
    - [De' petmeyvam'e' DI'ngaq](#de-petmeyvame-dingaq)
  - [vam Daghaqqang](#vam-daghaqqang)
    - [Holmey](#holmey)
  - [vam DawaHqang](#vam-dawahqang)

## vam bop
De'wI' taghmoHghach x86_64 UEFI-vaD 'oH Wakatiwai'e' (mO'OrI bIQ Dujvo' “waka tīwai”, ponglu'). ruStvo' ghItlhlu' 'e'. renlu'meH De'wI' pat Eisen taghmoHlaH.

## laHmey
<b><h3><p>
	⚙️ JSON-mo' nap choHlaHlu'<br>
	📠 QInmey choHlaH<br>
	🤝 UEFI San'onmey je taghmoHlaH<br>
	📁 [De' patmey'e' tu'lu'](#de-petmeyvame-dingaq)<br>
	🪟 Windows taghmoHlaH<br>
	🐧 Linux (systemd-mo') taghmoHlaH<br>
</p></h3></b>

## vam Dajomqang
Sep EFI De'janlijDaq DannIS Wakatiwai.
 - De' pat FAT32 chen'eghnIS Sepvam
 - Sep EFI Qap tIn'eghnISmeH Sepvam (machHom oH'chugh, 64MB; 'ach pIchup 300MB-1GB)
 - wItlhebmeH Sep wa'DIch Dejan oH' Sepvam - reH poQbe'lu' 'ach Qanqu' 'ej motlhqu'

## Wakatiwai'e' Dalo'qangchugh, vam yIlaDbej
taghmoHghach Dataghmo', tetlh Dalegh. leQ 'ev chan tIng chan joq yIqIpmeH wa' ghu'lIS buS. leQ logh chegh joq yIqIpmeH ghu'lIS buSlu'bogh taghmoH.

'ej ngoQ ghaj leQmeyvam:

|leQ|ngoQ|
|---|---|
|F5|taghqa'lu' taghmoHghach|
|F12|pat HoS teqlu'|

## `wtconfig.json`
De'mo' `wtconfig.json` ponglu'bogh ra'lu' Wakatiwai taghmoHghach'e'. meHDaq Sep EFI 'oHtaH De'vam 'ej rapbogh Sep'e' Danlu' taghmoHghachmo'. **ngulth'a'mey ngulthHommey je ghItlh'eghlu' lughchugh**, ngu'wI'meyvam DI'onmeychaj je laj:

|ngu'wI'|Segh|DI'on tu'beHlu'bogh|'ut'a'?|De' ngu'wI'vamvaD|
|---|---|---|---|---|
|`loglevel`|ngutlhmey|`"NORMAL"`|✘|QInmey'e' Segh ja'lu' Del vam. bIwIvlaH: <ul><li>`"SILENT"` (Qaghmey'e' ja')</li><li>`"QUIET"` (Qaghmey ghuHmoHwI'mey je ja')</li><li>`"NORMAL"` (QInmey motlh ja')</li><li>`"DEBUG"` (QInmey QaghHa' ghachvaD ja')</li></ul>|
|`timeout`|mI'|5|✘|ghorgh tagh'eghmoH'a'? pagh rapchugh, SIbI' taghmoH. pagh tIn law' mI'vam tIn puSchugh, SoHvaD loS.<br><br>**yIyep: 2,147,483,648 tIn law' mI'vam tIn puSnIS 'ej -2,147,483,649 tIn law'be' mI'vam tIn puSbe'nIS.**|
|`offerexit`|HIja' ghobe' ghap|`true`|✘|`true` rapchugh, taghmoHghach DabuplaH.<br><br>**yIyep: lI'laHmeH tetlh yub (ngaqlu'laH) UEFI (BIOS) ghap Dalo'qang.**|
|`editconfig`|HIja' ghobe' ghap|`true`|✘|`true` rapchugh, wtconfig.json tuchvaD DaDI'laH.<br><br>**yIyepqu': false rapchugh, wtconfig.json Qaghmey vamvo' wItI'be'vIp. De'wI' pat pIm pIlo'vIpnIS qoj De'wI'wIj tagh'eghmoHbe'vIp.**|
|`menuclear`|HIja' ghobe' ghap|`true`|✘|`true` repchugh, HaSta teqlu'DI', tetlh 'anglu'.|
|`bootentries`|[taghmoHvaD QIn]|not|✘|taghmoHvaD QInmey taghmoHghachvaD DaH 'oH. vam wa'DIch tagh'eghqangqu'moH.<br><br>**yIyep: pagh rapchugh, DughuHmoH taghmoHghach 'ej DabuplaH qoj wtconfig.json DaDI'laH.**|

### taghmoHvaD QInmey
JSON-vo' taghmoHvaD QInmey ghItlhlu' 'ej `bootentries`-Daq `wtconfig.json`-Daq bIH. ngulth'a'mey ngulthHommey je ghItlh'eghlu' lughchugh, ngu'wI'meyvam DI'onmeychaj je laj:

|ngu'wI'|Segh|DI'on tu'beHlu'bogh|'ut'a'?|De' ngu'wI'vamvaD|
|---|---|---|---|---|
|`name`|ngutlhmey|not|✔|taghmoHvaD QInvam pongDaj.<br><br>**yIyep: pongvam tIn law' 64 tIn puSnISbe'meH bIQap.**|
|`diskguid`|ngutlhmey|taghmoHghach De'jan GUID|✘|De'jan'e' GPT GUID Danbogh taghmoHvaD QInvam.|
|`partition`|mI'|not|✔|Sep'e' Danbogh taghmoHvaD QInvam.|
|`fs`|ngutlhmey|not|✔|Sepvam De' pat Segh. tetlh naDev [tu'lu'](#de-petmeyvame-dingaq) - ngutlh'a'mey ngutlhHommey je ghItlh'egh'lu lughnIS.|
|`progtype`|ngutlhmey|not|✔|taghmoHvaD QInvam ghun Segh. 'oHlaH: <ul><li>`"UEFI"` (UEFI-ghun taghqangmoH taghmoHvaD QIn)</li><li>`"ELF"` (ELF-ghun taghqangmoH taghmoHvaD QIn)</li></ul>|
|`path`|ngutlhmey|not|✔|taghmoHvaD QInvam ghun He.|

### De' petmeyvam'e' DI'ngaq
 - fat12
 - fat16
 - fat32

## vam Daghaqqang
vam WInuHqu'neS 'ej nom Dochmeyvam wIvumqang. yIpab:
 - ruSt'e' yIlo'qang - mutlhwI' wevDaq lo'nISmeH, yIlo'
 - LF, CRLF-be'

### Holmey
Holmey ghaj jInmolvam neH. noblIj vamvaD DIvuvneS (De'mey Markdown, ghItlhmey, QInmey, jaSHa' bIH Dochmey). ASCII'e' tIn ghItlhlaw' UEFI ([ISO/IEC 8859-1:1998](https://en.wikipedia.org/wiki/ISO/IEC_8859-1) yIlegh) - ngutlhmeyvam'e' ghaj Holmey'e' ngaqlaH.

|Hol|Dotlh (✅ - ngaqlu', ❌ - ngaqbe'lu', 🚧 - ngaqchoHlu')|
|---|---|
|'avrIqaS Hol (Afrikaans)|❌|
|SIchIyparIy Hol (Shqip)|❌|
|'ewSIqaDIy Hol (Euskara)|❌|
|[DIvI' Hol (British English)](../../README.md)|✅|
|qatalunya' Hol (Català)|❌|
|Denmargh Hol (Dansk)|❌|
|ne'Derlan Hol (Nederlands)|❌|
|'eStIy Hol (Eesit)|❌|
|SuwomIy Hol (Suomi)|❌|
|vIraS Hol (Français)|❌|
|DoyIchlan Hol (Deutsch)|🚧|
|'ISlan Hol (Íslenska)|❌|
|'eyre' Hol (Gaeilge)|❌|
|'InDoneSya' Hol (Bahasa Indonesia)|❌|
|'Italya' Hol (Italiano)|❌|
|thlIngan Hol (thlIngan Hol)|🚧|
|maleSya' Hol (Bahasa Melayu)|❌|
|noregh Nol (Norsk)|❌|
|portughal Hol (Português)|❌|
|romanS Hol (Romontsch)|❌|
|SIqotlan Hol (Gàidhlig)|❌|
|'eSpanya' Hol (Español)|❌|
|qIS'waHIlI' Hol (Kiswahili)|❌|
|Suverya' Hol (Svenska)|❌|
|taghalogh Hol (Wikang Tagalog)|❌|

## vam DawaHqang
ghunmey lI' ngaS qawHaq `tests`. Wakatiwai'e' DawaHlaH 'ej De'janmey DachenlaH. [naDev](./tests.md) luQIjlu'.

</span>
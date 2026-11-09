# gba game

Go check out the README in game/ for the game's instructions and how to build it
Check out the README in ereader/ for the e-Reader application's building instructions

In order to use the ereader loader, since the game doesn't support it yet, you need to use LinkCard_Demo from gba-link-connection.
Specifically you need:
- e-Reader ROM (USA)
- LinkCard_Demo
- loader.sav from the release
- a card file (I used testcard.usa.bin.01.bin from gba-link-connection)
- mGBA
- GBA BIOS

You have to:
- Open LinkCard_Demo with mGBA
- Go under File and click "New Multiplayer Window"
- copy the loader.sav to the location where the e-Reader's rom is and give it the same name as the e-Reader rom
- open the e-Reader rom in the Multiplayer window for the second player (Make sure that LinkCard_Demo is the first player)
- On the e-Reader instance select File, Scan e-Reader dotcodes and open the card file
- Select "Access Saved Data" on the e-Reader
- Press the A button on the LinkCard_Demo instance
- The e-Reader should scan, transfer the data and quit to menu

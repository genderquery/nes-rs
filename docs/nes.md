| Address range | Size  | Device                                        |
| ------------- | ----- | --------------------------------------------- |
| $0000-$07ff   | $0800 | 2 kB internal RAM                             |
| $0800-$0fff   | $0800 | Mirror of $0000-$07ff                         |
| $1000-$17ff   | $0800 | Mirror of $0000-$07ff                         |
| $1800-$1fff   | $0800 | Mirror of $0000-$07ff                         |
| $2000-$2007   | $0008 | PPU registers                                 |
| $2008-$3fff   | $1ff8 | Mirrors of $2000-2007 (repeats every 8 bytes) |
| $4000-$401f   | $0020 | APU and I/O registers                         |
| $4020-$FFFF   | $BFE0 | Cartridge: PRG ROM, PRG RAM, mapper registers |

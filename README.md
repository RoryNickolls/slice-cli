# Slice CLI

A CLI text-based game written in Rust.

Built as a non-graphical prototype for a 1-1 combat game based more on making strategic decisions than abusing mechanics/spamming buttons. (Looking at you Chivalry: Medieval Warfare, Brawlhalla, Mount & Blade)

Still in progress.

Currently the game has no support for input, and just demonstrates two players moving around the world.

Example output:
```
Player1: X
Player2: O
| |X| |
| | | |
| |O| |

Player1 moves DOWN
| | | |
| |X| |
| |O| |

Player2 slashes Player1
and damages them for 20!
| | | |
| |X| |
| |O| |

Player2 moves RIGHT
| | | |
| |X| |
| | |O|

Player2 moves UP
| | | |
| |X|O|
| | | |

Player2 overheads Player1
and damages them for 20!
| | | |
| |X|O|
| | | |
```
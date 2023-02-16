# minesweeper-rs
A simple console game, written in Rust 💣
### Technical specs
---
1. ref_table
    - 0 = no bomb
    - -1 = bomb
    - -2 = no bomb, nearby mines counted, no nearby mines
2. table
    - \* = unopened
    - _ = opened
    - P = flagged
    - [\*] = selected
    - 1, 2... = bomb(s) around
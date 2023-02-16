# minesweeper-rs 💣
A simple console game, written in Rust. 
## Tech specs
### 1. ref_table
  *  0 = nothing
  * -1 = mine
  * -2 = nothing, nearby mines counted, no nearby mines
### 2. table
  * \*      = unopened
  * _       = opened
  * P       = flagged
  * [\*]    = selected
  * 1, 2... = mine(s) around

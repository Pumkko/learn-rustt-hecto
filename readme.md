# What I want
To learn and experiment with Rust building a snake game using crossterm

# What did I learn
Difficult to say right now what I learnt, things kinda become clearer but not so much.
I do think i understand a bit better when and why moving and borrowing happen

# Todo (Technical)
- [ ] Make a proper "engine" instead of splitting crossterm calls everywhere
- [ ] Error handling

# Todo (game)
- [X] Detect collisions, either the snake hits itself or the boundaries of the board
- [X] Add food generation, draw a food character randomly on the board when required
- [X] Implement eat logic (ie when the snake hits a food character)

# Bug fix
- [ ] The food generation code sometimes generate food where the snake is

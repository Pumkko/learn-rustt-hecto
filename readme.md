# What I want
To learn and experiment with Rust building a snake game using crossterm

# What did I learn
Difficult to say right now what I learnt, things kinda become clearer but not so much.
I do think i understand a bit better when and why moving and borrowing happen

# Todo (Technical)
- [X] Move all terminal handling logic to the terminal module
- [ ] Error handling
- [ ] See if locking `should_quit` every render is the best way to gracefully stop the render loop

# Todo (game)
- [X] Detect collisions, either the snake hits itself or the boundaries of the board
- [X] Add food generation, draw a food character randomly on the board when required
- [X] Implement eat logic (ie when the snake hits a food character)
- [ ] Pretty display "You Lost" Menu, add a try again text explaining how to try again
- [ ] Display Score

# Bug fix
- [X] The food generation code sometimes generate food where the snake is
- [X] The food generation code sometimes generate food too close to the border making the game a bit too hard
- [X] The snake_renderer system can hide a food if it appears right behind the snake

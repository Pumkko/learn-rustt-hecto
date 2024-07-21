# What I want
To learn and experiment with Rust building a snake game using crossterm

# What did I learn
Difficult to say right now what I learnt, things kinda become clearer but not so much.
I do think i understand a bit better when and why moving and borrowing happen

# Todo (Technical)
- [X] Move all terminal handling logic to the terminal module
- [ ] Error handling
- [X] See if `mpsc` would be cleaner than using `Arc<Mutex<>>` (code in its own barnch)  

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

## About MPSC

So i tried to see if using `mpsc` would be better than using an `Arc<Mutex<>>`. Matter of fact no.
I had a couple of problems
- The main thread will not send a new direction every render. that means the thread rendering the snake would peek 
into a receiver that has no value. I then had to write a weird `match` if i have a direction i take otherwise i keep the old one.
But i ended up writing that 
```
        let should_quit = match should_quit_receiver.try_recv() {
            Ok(sq) => sq,
            Err(e) => match e {
                std::sync::mpsc::TryRecvError::Disconnected => {
                    panic!("try to receive should_quit on disconnected channel")
                }
                std::sync::mpsc::TryRecvError::Empty => false,
            },
        }; 
```
Basically i can not block the rendering thread so i have to call `try_recv` but it can fail for two reasons, either the channel is dead or it has no value.
One is okay the other is not.
I think the problem has to do with why and when to use `mpsc`. Even though i clearly have a producer and a consumer, the consumer can perfectly do its job without the producer (the snake would go straight until it hits a wall). I feel like `mpsc` was built for a different use case. Like a worker waiting for a job.
I feel like `try_recv` can be used instead of `recv` when we don't want to block the thread but in a situation where having an empty channel is the exception 
rather than the norm. In my case `try_recv` will hit the Empty branch 90% of the time.
I'll keep the mpsc in a different branch

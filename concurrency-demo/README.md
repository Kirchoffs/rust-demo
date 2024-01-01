# Notes
## Thread Id
The Rust standard library assigns every thread a unique identifier. This identifier is accessible through Thread::id() and is of the type ThreadId. 

## Output Locking
The println macro uses std::io::Stdout::lock() to make sure its output does not get interrupted. A println!() expression will wait until any concurrently running one is finished before writing any output. 

## Move
### Use Move for Closure
Ownership of numbers is transferred to the newly spawned thread, since we used a move closure. If we had not used the move keyword, the closure would have captured numbers by reference. This would have resulted in a compiler error, since the new thread might outlive that variable (the thread can run until end of the program's execution).

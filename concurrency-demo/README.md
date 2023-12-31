# Notes
## Thread Id
The Rust standard library assigns every thread a unique identifier. This identifier is accessible through Thread::id() and is of the type ThreadId. 

## Output Locking
The println macro uses std::io::Stdout::lock() to make sure its output does not get interrupted. A println!() expression will wait until any concurrently running one is finished before writing any output. 

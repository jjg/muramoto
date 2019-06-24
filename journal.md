# 06102019

Last week I decided to pick-up where JSFS left off and evolve the filesystem into an application platform, or more of a distributed operating system.  The key feature is the ability to execute server-side code, but there's so much more to this revival than that.

The most obvious difference is the name, "Muramoto" chosen because names are hard and I was having a perfect lunch there when I decided to embark on this project.  Why not JSFSX?  Because unlike it's predecessors, Muramoto won't be written in JavaScript (although that doesn't proclude it from *running* JavaScript).  Instead it will be written in [Rust](https://www.rust-lang.org/), for a number of reasons that should become apparent as I describe the system.

Muramoto carries much of JSFS's existing functionality forward, and HTTP-accessible deduplicating filesystem.  In addition to this Muramoto will implement experimental JSFS features including federation, and will add an `execute` verb allowing code stored in the filesystem to be executed server-side, against both local and federated nodes.

Server-side, code will be executed as [Web Assembly](https://webassembly.org/) (WASM), allowing any language that can be compiled into WASM to be run (JavaScript, Rust, C/C++, C#, Java, Python, Go [and more every day](https://github.com/appcypher/awesome-wasm-langs)).

In addition to these major features I want to make running and managing Muramoto easier that JSFS which is part of the rationale behind choosing Rust.  Instead of requiring a runtime (Node.js), Muramoto will be a stand-alone, optimized executable.  Instead of requiring a configuration file, Muramoto's configuration will be stored *inside* the filesystem so it can be manipulated just like any other object (opening the door for runtime/online reconfiguration). 

Other less exciting optimizations, wishlish features, etc. will be added as well as a more flexible, modular architecture compared to JSFS.

The starting point for this work is to re-implement essential JSFS features in Rust.  This is necissary (as JSFS is written in JavaScript) and also beneficial as a way for me to learn Rust while working within a well-known problem space.  The initial result may not be as optimal as Rust written by someone who is an expert, but I think I'll get there, and I've never shied-away from refactoring.  Additionally this gives me a chance to embrace aspects of Rust that I've ignored in other languages (namely testing), and if I take the time to do things right, I might even get others interested in contributing at some point.

To that end I will be posting this project to Github.  It won't be the primary repository, but I'll setup a remote there that I can push to, and work on a way to allow others to contribute via that channel if prefered.  

The initial work is based on the [web server example](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) from [The Rust Book](https://doc.rust-lang.org/book/).  This seemed like an obvious place to start, and since I want to have precise control over the performance and behavior of the server, it avoids leaning on existing HTTP libraries for now.

*Note - use this as a reference for writing good Rust: [Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).*

# 06242019

Consider Channel I/O architectures in federation design: https://en.wikipedia.org/wiki/Channel_I%2FO

String manipulation examples in rust

### 1. String slicing

#### Get the first 3 characters of a given string.

```rust
let input = "Get a = 10";

// prints "Get"
println!(input.slice_chars(0, 3));
```

### 2. Using split

#### Get the protocol, host and port from a url.

```rust
let url = "https://doc.rust-lang.com:80";

let url: Vec<&str> = url.split("://").collect();
let (protocol, host_port) = (url[0],  url[1]);
    
let host_port: Vec<&str> = host_port.split(":").collect();
let (host, port) = (host_port[0], host_port[1]);

// Prints "Protocol: https, Host: doc.rust-lang.com, Port: 80"
println!("Protocol: {}, Host: {}, Port: {}", protocol, host, port);
```

#### Get the filename from the given path (Similar to basename command in *nix)

```rust
let path = "/Users/Dinesh/documents/developer/rust/main.rs";
    
let mut path: Vec<&str> = path.split("/").collect();
    
let filename = path.pop().unwrap();

// Prints "Filename is main.rs"
println!("Filename is {}", filename);
```

#### Without mutable variable

```rust
let path = "/Users/Dinesh/documents/developer/rust/main.rs";
    
let path: Vec<&str> = path.split("/").collect();
    
let filename = path[path.len() - 1];

// Prints "Filename is main.rs"
println!("Filename is {}", filename);
```

### 3. Using rsplit

#### Get the filename from the given path (Similar to basename command in *nix)

```rust
let path = "/Users/Dinesh/documents/developer/rust/main.rs";
    
let mut path = path.rsplit("/");
    
let filename = path.next().unwrap();

// Prints "Filename is main.rs"
println!("Filename is {}", filename);
```

String manipulation examples in rust

### 1. String slicing

#### Get the first 3 characters of a given string.

```rust
let input = "Get a = 10";

// prints "Get"
println!(input.slice_chars(0, 3));
```

### 2. split

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

#### Get the filename from the given path (similar to basename command in *nix)

```rust
let path = "/Users/Dinesh/documents/developer/rust/main.rs";
    
let mut path: Vec<&str> = path.split("/").collect();
    
let filename = path.pop().unwrap();

// Prints "Filename is main.rs"
println!("Filename is {}", filename);
```

#### Get the filename from the given path (without mutable variable)

```rust
let path = "/Users/Dinesh/documents/developer/rust/main.rs";
    
let path: Vec<&str> = path.split("/").collect();
    
let filename = path[path.len() - 1];

// Prints "Filename is main.rs"
println!("Filename is {}", filename);
```

#### Two values are given as a string, find their sum. 

```rust
let input = "10 20";

let input: Vec<i32> = input.split(" ").map(|no| no.parse::<i32>().unwrap()).collect();
let a = input[0];
let b = input[1];
   
// Prints "The sum of 10 and 20 is 30"
println!("The sum of {} and {} is {}", a, b, a + b);
```

### 3. rsplit

#### Get the filename from the given path (similar to basename command in *nix)

```rust
let path = "/Users/Dinesh/documents/developer/rust/main.rs";
    
let mut path = path.rsplit("/");
    
let filename = path.next().unwrap();

// Prints "Filename is main.rs"
println!("Filename is {}", filename);
```

### 4. Replace

#### Replace a string with another string.
```rust
let input = "Dust Docks";
   
let input = input.replace("D", "R");

// Prints "Rust Rocks"
println!("{}", input);
```

### 5. Starts_with

#### Find if the link is secure or not

```rust
let url = "https:\\doc.rust-lang.org";
    
// Prints "The link is secure"
if url.starts_with("https") {
    println!("The link is secure");
} else if url.starts_with("http") {
    println!("The link is unsecure");
}
```

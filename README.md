String manipulation examples in rust

### 1. String slicing

Get the first 3 characters of a given string.

```rust
let input = "Get a = 10";
    
assert_eq!("Get", input.slice_chars(0, 3));
```

### 2. Using split

Find the protocol, host and port from a url.

```rust
let url = "https://doc.rust-lang.com:80";

let url: Vec<&str> = url.split("://").collect();
let (protocol, host_port) = (url[0],  url[1]);
    
let host_port: Vec<&str> = host_port.split(":").collect();
let (host, port) = (host_port[0], host_port[1]);
    
println!("Protocol: {}, Host: {}, Port: {}", protocol, host, port);
```

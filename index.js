const rust = import("./pkg/rust_3d");

rust.then(m =>m.say_hello_from_rust())
  .catch(console.error);

 

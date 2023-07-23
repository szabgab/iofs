# Example
## File System
```rust
use iofs::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f = FileInfo::open_smart("foo.txt")?;
    f.rename("new_name")?;
    assert!(f.name(), "new_name.txt");
    Ok(())
}

```

## IO
```rust
use iofs::io::Console;  
fn main() -> stools::io::Result<()> {  
	let mut console = Console::new();  
	let n = console.input::<i32>(Some("input a number: "))?;  
	Ok(())  
}
```
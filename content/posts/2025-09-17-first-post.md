# My First Post with Dodge SSG

Welcome to my new blog powered by the **Dodge** static site generator! This is built with the RAM stack (Rust, Actix, Markdown) and it's blazingly fast! 🚀

## What is Dodge?

Dodge is a minimal static site generator written in Rust that focuses on:

- **Speed**: Rust's performance makes generation lightning fast
- **Simplicity**: Minimal configuration, maximum results
- **Flexibility**: Easy to extend and customize

## Features I'm Testing

### Code Blocks

Here's some Rust code that powers this very generator:

```rust
use anyhow::Result;
use comrak::{markdown_to_html, ComrakOptions};

fn main() -> Result<()> {
    let posts = collect_posts("content/**/*.md")?;
    
    for post in posts {
        let content = fs::read_to_string(&post)?;
        let html = markdown_to_html(&content, &ComrakOptions::default());
        // Generate HTML...
    }
    
    Ok(())
}
```

### Lists and Formatting

Things I love about Rust:

1. **Memory safety** without garbage collection
2. **Zero-cost abstractions** that don't sacrifice performance
3. **Fearless concurrency** with the ownership system
4. **Rich ecosystem** with Cargo and crates.io

### Links and Images

Check out the [Rust website](https://www.rust-lang.org/) for more information.

## Conclusion

This is just the beginning! I'm excited to build more content with this SSG.

---

*Published on September 17, 2025*
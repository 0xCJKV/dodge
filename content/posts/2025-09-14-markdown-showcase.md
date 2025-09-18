# Markdown Showcase

This post demonstrates various Markdown features to test our SSG's rendering capabilities.

## Headers

# H1 Header
## H2 Header  
### H3 Header
#### H4 Header
##### H5 Header
###### H6 Header

## Text Formatting

**Bold text** and *italic text* and ***bold italic text***.

~~Strikethrough text~~ and `inline code`.

## Lists

### Unordered Lists

- Item 1
- Item 2
  - Nested item 2.1
  - Nested item 2.2
    - Deep nested item
- Item 3

### Ordered Lists

1. First item
2. Second item
   1. Nested numbered item
   2. Another nested item
3. Third item

### Task Lists

- [x] Completed task
- [ ] Incomplete task
- [x] Another completed task

## Code Blocks

### JavaScript

```javascript
function fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

console.log(fibonacci(10)); // 55
```

### Python

```python
def quicksort(arr):
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quicksort(left) + middle + quicksort(right)
```

### Plain Text

```
This is a plain text code block.
No syntax highlighting here.
Just monospace font.
```

## Tables

| Language | Year | Paradigm |
|----------|------|----------|
| Rust     | 2010 | Systems  |
| Python   | 1991 | Multi    |
| Go       | 2009 | Systems  |
| JavaScript | 1995 | Multi  |

## Blockquotes

> This is a simple blockquote.

> This is a longer blockquote that spans multiple lines.
> It demonstrates how blockquotes can contain multiple sentences
> and still maintain proper formatting.

> ### Blockquote with Header
> 
> You can even include other markdown elements inside blockquotes:
> 
> - List item 1
> - List item 2
> 
> And `inline code` too!

## Links

- [External link](https://www.rust-lang.org/)
- [Link with title](https://github.com "GitHub Homepage")

## Horizontal Rules

---

***

___

## Escape Characters

You can escape special characters: \*not italic\* and \`not code\`.

## HTML Elements

Some <strong>HTML</strong> elements should work too, like <em>emphasis</em> and <code>code</code>.

## Special Characters

Here are some special characters: © ® ™ § ¶ † ‡ • … ‰ ′ ″ ‹ › « » ¡ ¿

## Conclusion

This showcase demonstrates the rich formatting capabilities of Markdown. Our Dodge SSG should handle all of these elements gracefully!
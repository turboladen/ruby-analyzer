# So I don't forget

## Terms

1. Ruby "scope" or "path" ~= Ruby namespace. Ruby classes and modules are what create hierarchy.
   When trying to determine context, this is the first piece of info we need.
   - Resource: [Everything you ever wanted to know about constant lookup in Ruby](https://cirw.in/blog/constant-lookup)
2. Note to self: "namepace"/nesting and superclass mean different things, where the former deals
   with functionality organization and the latter deals with passing on functionality.

## Completion

- Completion happens in the context of a file, thus begging the need for a file-base index.
- File-based lookup:
  - Using the input of the cursor's location (`file_path, line_number, character`), we need to be
    able to look up:
    1. The node the cursor is on. If it's on white-space, use the next parent-most node.
    2. The Ruby namespace of the node the cursor is on.
- Ruby namespace lookup:
  - Given some namespace, we need to be able to lookup:
    1. All completable items in the immediate namespace.
    2. All completable items in the parent namespace.

```ruby
class A
  class B1
    class C11; end
    class C12; end
  end

  class B2
    class C21; end
    class C22; end
  end
end
```

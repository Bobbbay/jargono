## Jargono

*Pronounced: yarg-oh-no*

Jargono, my toy language.

### TODO

 - [ ] Parser support for functions (`fn name(args) -> ret_type { children }`)
 - [ ] Skip over comments
 - [ ] A bit of a more proper compilation pass without needing to invoke external binaries
 - [ ] We seem to not be consuming booleans in the parser
 - [ ] Functions don't support arguments
 - [ ] Make sure we're using variables the right way - LLVM has built-in support for them but we're using a hashmap?
   - [ ] Test out what happens when we actually try to change variables. They are *variable*s, after all.
 - [ ] Add support for strings (parser & codegen)

Nice things to consider:

 - [ ] Variadics
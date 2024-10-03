# Clank Grammar

S       := <func>
         | <struct>
         | <impl>
func    := fn <id> ( <params>? ) -> <type> { <stmt> }
struct  := struct <id> { <objdef> }
impl    := impl <id> { <stmt> }
params  := <id> : <type> (, <params>)?
objdef  := <id> : <type> , <objdef>?
stmt    := <stmt> <stmt>
         | <expr> ;
         | if ( <expr> ) { <stmt> }
         | while <expr> { <stmt> }
         | for <id> in <expr> { <stmt> }
         | let <id> : <type> ( = <expr> )? ;
expr    := <num>
         | <id>
         | <str>
         | <expr> + <expr>
         | <expr> - <expr>
         | <expr> [*] <expr>
         | <expr> \ <expr>
         | <id> = <expr>
         | true
         | false
         | <str>
         | <chr>
type    := i32
         | string
         | char
         | bool
         | <id>
         | <id> [<] <type> [>]
         | \[ <type> \]
id      := [a-zA-Z_][a-zA-Z0-9_]*
num     := [0-9]+([.][0-9]+)?
str     := ".*"
chr     := '.'

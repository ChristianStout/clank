# Clank Grammar

S       := <import>
         | <const>
         | <func>
         | <struct>
         | <enum>
         | <impl>
         | <trait>
import  := import <id> (::<id>)* (as <id>)? ;
const   := const <id> : <type> = <expr> ;
func    := fn <id> ( <params>? ) -> <type> { <stmt>* }
struct  := struct <id> { <objdef> }
enum    := enum <id> { <enmitm> (, <enmitm>)* [,]? }
impl    := impl <id>  (for <id>)? { <func>* }
params  := <id> : <type> (, <params>)?
objdef  := <id> : <type> (, <objdef>)?
stmt    := <expr> ;
         | if ( <expr> ) { <stmt>* }
         | else { <stmt>* }
         | for <id> in <expr> { <stmt>* }
         | while <expr> { <stmt>* }
         | let <id> (: <type>)? ( = <expr> )? ;
         | when <expr> { <stmt>* }
         | match <expr> { <mtcitm> (, <mtcitm>)* [,]? }
         | return <expr> ;
expr    := <expr> + <expr>
         | <expr> - <expr>
         | <expr> [*] <expr>
         | <expr> / <expr>
         | <expr> % <expr>
         | <expr> += <expr>
         | <expr> -= <expr>
         | <expr> [*]= <expr>
         | <expr> /= <expr>
         | <expr> %= <expr>
         | <expr> == <expr>
         | <expr> != <expr>
         | <expr> > <expr>
         | <expr> < <expr>
         | <expr> >= <expr>
         | <expr> <= <expr>
         | <expr> = <expr>
         | - <expr>
         | + <expr>
         | ! <expr>
         | true
         | false
         | <id>
         | <num>
         | <str>
         | <chr>
         | <expr> . <expr>
         | <expr> :: <expr>
         | <expr> ( (<expr> (, <expr>)* )? )
         | <expr> as <type>
         | ( <expr> )
         | lambda <params>? : (<expr> | { <stmt>* })
         | <expr> .. <expr>
type    := i32
         | u8
         | string
         | char
         | bool
         | <id>
         | <id> [<] <type> [>]
         | \[ <type> \]
         | ( <type>? (, <type>)* ) -> <type>
enmitm  := <id> ( [(] <type> (, <type>)? [,]? [)] )?
mtcitm  := <expr> => { <stmt>* }
id      := [a-zA-Z_][a-zA-Z0-9_]*
num     := [0-9]+([.][0-9]+)?
str     := ".*"
chr     := '.'

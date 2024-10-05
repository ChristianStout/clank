# Clank Grammar

S       := <import>
         | <const>
         | <func>
         | <struct>
         | <enum>
         | <impl>
         | <trait>
import  := import <id> (::<id>)* ;
const   := const <id> : <type> = <expr> ;
func    := fn <id> ( <params>? ) -> <type> { <stmt>* }
struct  := struct <id> { <objdef> }
enum    := enum <id> { <enmitm> (, <enmitm>)* [,]? }
<<<<<<< HEAD
impl    := impl <id>  (for <id>)? { <stmt>* }
=======
impl    := impl <id>  (for <id>)? { <stmt> }
>>>>>>> ad7825d105800e6643f3edd66309d77d0729efb3
params  := <id> : <type> (, <params>)?
objdef  := <id> : <type> (, <objdef>)?
args    := ( <expr> (, <expr>)* )?
stmt    := <expr> ;
         | if ( <expr> ) { <stmt>* }
         | else { <stmt>* }
         | for <id> in <expr> { <stmt>* }
         | while <expr> { <stmt>* }
         | let <id> (: <type>)? ( = <expr> )? ;
         | when <expr> { <stmt>* }
         | match <expr> { <mtcitm> (, <mtcitm>)* [,]? }
<<<<<<< HEAD
expr    := <expr> + <expr>
=======
expr    := <num>
         | <id>
         | <str>
         | <expr> + <expr>
>>>>>>> ad7825d105800e6643f3edd66309d77d0729efb3
         | <expr> - <expr>
         | <expr> [*] <expr>
         | <expr> \ <expr>
         | <expr> % <expr>
         | <expr> += <expr>
         | <expr> -= <expr>
         | <expr> [*]= <expr>
         | <expr> \= <expr>
         | <expr> %= <expr>
         | <expr> == <expr>
         | <expr> != <expr>
         | <expr> > <expr>
         | <expr> < <expr>
         | <expr> >= <expr>
         | <expr> <= <expr>
         | - <expr>
         | + <expr>
         | true
         | false
         | <id>
         | <num>
         | <str>
         | <chr>
         | <expr> . <id>
<<<<<<< HEAD
         | <id> :: <expr>
=======
         | <id> :: <id>
>>>>>>> ad7825d105800e6643f3edd66309d77d0729efb3
         | <expr> as <type>
type    := i32
         | u8
         | string
         | char
         | bool
         | <id>
         | <id> [<] <type> [>]
         | \[ <type> \]
enmitm  := <id> ( [(] <type> (, <type>)? [,]? [)] )?
mtcitm  := <expr> => { <stmt>* }
id      := [a-zA-Z_][a-zA-Z0-9_]*
num     := [0-9]+([.][0-9]+)?
str     := ".*"
chr     := '.'

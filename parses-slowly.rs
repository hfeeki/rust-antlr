// to see fast parsing, uncomment next line and comment out following one:
// use a::b;
use a::b::c;

pub fn mk_pass(name: ~str, op: @fn(&str) -> ~str) -> Pass {
   let op = Cell(op);
   Pass {
       name: copy name,
       f: |srv: astsrv::Srv, doc: doc::Doc| -> doc::Doc {
           run(srv, doc, op.take())
       }
   }
}

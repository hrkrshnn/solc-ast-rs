;; Emacs macros to help generate visitors and structs
;; Usage:
;; 1. yank the struct (for example the name `UsingForDirective`)
;; 2. Go to the right place in the buffer
;; 3. For the `node impl ....` generator, use `M-x impl-node-from-struct`
;; 4. For the `fn visit_... and fn end_visit_...`, use `M-x generate-visitor-from-struct`


(defalias 'generate-visitor-from-struct
  (kmacro "f n SPC v i s i t _ C-y M-b C-SPC C-e M-x s t r i n g - i n f l e c t i o n - u n d e r s c o r e <return> ( & m u t SPC s e l f , SPC n o d e : SPC & C-y ) SPC - > SPC R e s u l t < b o o l > SPC { <return> <tab> s e l f . v i s i t _ n o d e ( n o d e ) <return> <tab> } <tab> <return> <tab> f n SPC e n d _ v i s i t _ C-y M-b C-SPC C-e M-x s t r i n g - i n f l e c t i o n - u n d e r s c o r e <return> ( & m u t SPC s e l f , SPC n o d e : SPC & C-y ) SPC - > SPC R e s u l t < () > SPC { <return> <tab> s e l f . e n d _ v i s i t _ n o d e ( n o d e ) <return> } <tab> <return>"))

(defalias 'impl-node-from-struct
  (kmacro "i m p l SPC N o d e SPC f o r SPC C-y SPC { <return> <tab> f n SPC a c c e p t ( & s e l f , SPC v i s i t o r : SPC & m u t SPC i m p l SPC A S T C o n s t V i s i t o r ) SPC - > SPC R e s u l t < ( ) > SPC { <return> <tab> i f SPC v i s i t o r . v i s i t _ C-y M-b C-SPC C-e M-x s t r i n g - i n f l e c t i o n - u n d e r s c o r e <return> ( s e l f ) ? SPC { <return> <tab> t o d o ! ( ) <return> } <tab> <return> <tab> v i s i t o r . e n d _ v i s i t _ C-y M-b C-SPC C-e M-x s t r i n g - i n f l e c t i o n - u n d e r s c o r e <return> ( s e l f ) <return> <tab> } <tab> <return> }"))

# rustysg


## Syntax

### ABNF

```abnf
template      = *(text / sub_expr)
text          = *CHAR
sub_expr      = "{{" expression "}}"
expression    = variable / literal
variable      = IDENT *("." IDENT)
literal       = \x22 text \x22 ; text surrounded by quotes
IDENT         = ALPHA *(ALPHA / DIGIT / "_" / "-")
```

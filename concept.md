# Parsing pseudocode

## Root

```py
class Context:
  uses: list[Use]
  consts: list[Const]
  statics: list[Static]
  funs: list[Fun]

def parse_root():
  while has_token():
    token = peek_token();
    match token:
      USE:
        parse_use();
      CONST:
        parse_const();
      STATIC:
        parse_static();
      INLINE | EXTERN | FUN:
        parse_fun();
      else:
        raise "Invalid token at root level";
```

## Use

```py
def parse_use():
  if !has_token() || peek_token() != USE:
    raise "Unexpected error";
  advance();
  if !has_token():
    raise "No moduled to use was defined";
  module = peek_token();
  advance();
  if module != IDENT:
    raise "Invalid use statement - module should be an identifier";
  if !has_token() || peek_token() != SEMICOLON:
    raise "Semicolon was expected after use statement";
  advance();
  context.uses.push(module);
```

## Const

```py
def parse_const():
  if !has_token() || peek_token() != CONST:
    raise "Unexpected error";
  advance();
  type = parse_type();
  if !has_token() || peek_token() != IDENT:
    raise "Invalid constant";
  name = peek_token();
  advance();
  if !has_token() || peek_token() != EQUAL:
    raise "Invalid constant";
  advance();
  # TODO: implement constant expression parsing
  parse_const_expr();
```

## Static

```py
def parse_static():
  if !has_token() || peek_token() != STATIC:
    raise "Unexpected error";
  advance();
  type = parse_type();
  if !has_token() || peek_token() != IDENT:
    raise "Invalid constant";
  name = peek_token();
  advance();
  if !has_token() || peek_token() != EQUAL:
    raise "Invalid constant";
  advance();
  # TODO: implement constant expression parsing
  parse_const_expr();
```

## Fun

```py
def parse_fun():
  inline = False
  extern = False
  while has_token():
    token = peek_token();
    advance();
    match token:
      FUN:
        break;
      INLINE:
        if inline:
          raise "Duplicate inline";
        inline = True;
      EXTERN:
        if extern:
          raise "Duplicate extern";
        extern = True;
      else:
        raise "Invalid function descriptor";
  if !has_token() || peek_token() != IDENT:
    raise "Invalid function";
  name = peek_token();
  advance();
  if !has_token() || peek_token() != LEFT_PAREN:
    raise "Invalid function";
  params = [];
  while has_token():
    if peek_token() != VAR:
      raise "Invalid function parameters";
    advance();
    type = parse_type();
    if !has_token() || peek_token() != IDENT:
      raise "Invalid function parameters";
    param_name = peek_token();
    advance();
    params.push(Param(type, param_name));
    if !has_token():
      raise "Invalid function parameters";
    next = peek_token();
    match next:
      RIGHT_PAREN:
        advance();
        break;
      COMMA:
        advance();
        continue;
      else:
        raise "Invalid function parameters";
  if !has_next():
    raise "Invalid function";
  next = peek_token();
  # TODO: arrow return type
  match next:
    SEMICOLON:
      if !extern:
        raise "Invalid function";
      advance();
      return Fun(name, inline, extern, params);
    LEFT_BRACE:
      advance();
  # TODO: parse body
```

## Type

```py
def parse_type():
  # TODO: parse without surrounding brackets
  if !has_token() || peek_token() != LEFT_BRACKET:
    raise "Invalid type parameter";
  advance();
  ref_depth = 0;
  while has_token():
    if peek_token() == AND:
      advance();
      ref_depth += 1;
    else:
      break;
  if !has_token():
    raise "Illegal end of source in type parameter";
  type = peek_token();
  if type != IDENT:
    raise "Illegal type token, must be identifier";
  advance();
  if !has_token() || peek_token() != RIGHT_BRACKET:
    raise "Invalid type parameter";
  advance();
  match type.ident:
    "u8":
      return U8(ref_depth);
    "i8":
      return I8(ref_depth);
    "u16":
      return U16(ref_depth);
    "i16":
      return I16(ref_depth);
    "u32":
      return U32(ref_depth);
    "i32":
      return I32(ref_depth);
    "u64":
      return U64(ref_depth);
    "i64":
      return I64(ref_depth);
    "f64":
      return F64(ref_depth);
    else:
      raise "Unknown type";
```

# Rules

## Uses

`USE IDENT SEMICOLON`

## Constants

`CONST LEFT_BRACKET TYPE RIGHT_BRACKET EQUAL CONST_EXPR SEMICOLON`

* Requires type annotation
* Requires constant assignment
* Is immutable
* Only on root level

## Static

`STATIC LEFT_BRACKET TYPE RIGHT_BRACKET EQUAL CONST_EXPR SEMICOLON`

* Requires type annotation
* Requires constant assignment
* Is mutable
* Only on root level

## Variables

`VAR (LEFT_BRACKET TYPE RIGHT_BRACKET)? IDENT (EQUAL EXPR)? SEMICOLON`

* Does not require type annotation
* Does not require assignment
* Is mutable
* Only in non-root level
* Allows non-constant expression assignment

## Functions

`EXTERN? INLINE? FUN IDENT LEFT_PAREN ((VAR LEFT_BRACKET TYPE RIGHT_BRACKET
IDENT COMMA)* VAR LEFT_BRACKET TYPE RIGHT_BRACKET IDENT)? RIGHT_PAREN
(ARROW TYPE)? LEFT_BRACE BODY RIGHT_BRACE`

* Does not require return type

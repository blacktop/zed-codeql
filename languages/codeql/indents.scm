; Indentation rules for CodeQL

; Indent inside blocks
[
  (body)
] @indent

; Indent inside parentheses and brackets
[
  "("
  "["
  "{"
] @indent

; Dedent closing brackets
[
  ")"
  "]"
  "}"
] @outdent

; Indent after these keywords
[
  "from"
  "where"
  "select"
  "order"
  "if"
  "then"
  "else"
] @indent

; Keep same indent for continuation
[
  "and"
  "or"
  ","
  "|"
] @indent.auto
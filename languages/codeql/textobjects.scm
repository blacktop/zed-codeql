; Text objects for CodeQL

; Predicates
(memberPredicate) @function.outer

; Classes
(dataclass) @class.outer

; Modules
(module) @namespace.outer

; Select statements (queries)
(select) @statement.outer

; Comments
(line_comment) @comment.outer
(block_comment) @comment.outer
(qldoc) @comment.outer

; Import statements
(importDirective) @import.outer

; String literals
(string) @string.outer
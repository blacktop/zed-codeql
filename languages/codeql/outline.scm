; Simplified outline/symbols for CodeQL

; Import statements
(importDirective
  (moduleName) @name) @item

; Class definitions
(dataclass
  name: (className) @name) @item

; Predicate definitions
(memberPredicate
  name: (predicateName) @name) @item

; Module definitions
(module
  name: (moduleName) @name) @item

; Select clauses (queries)
(select) @item
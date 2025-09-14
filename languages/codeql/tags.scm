; Tags for CodeQL - enables jump to definition

; Class definitions
(dataclass
  name: (className) @name) @definition.class

; Module definitions
(module
  name: (moduleName) @name) @definition.module

; Predicate definitions
(memberPredicate
  name: (predicateName) @name) @definition.function

; Import statements
(importDirective) @reference.import

; Class references
(className) @reference.class

; Module references
(moduleName) @reference.module

; Predicate references
(predicateName) @reference.call

; Variable references
(varName) @reference.variable
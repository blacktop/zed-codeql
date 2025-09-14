; Folding rules for CodeQL

; Fold predicates
(memberPredicate) @fold

; Fold classes
(dataclass) @fold

; Fold modules
(module) @fold

; Fold select clauses
(select) @fold

; Fold block comments
(block_comment) @fold

; Fold QLDoc comments
(qldoc) @fold
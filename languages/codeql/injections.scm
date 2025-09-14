; Language injections for CodeQL

; QDoc comments contain Markdown
((qldoc) @content
 (#set! "language" "markdown"))

; Regex patterns in strings
((string) @content
 (#match? @content "^[\"']/.*?/[gimsu]*[\"']$")
 (#set! "language" "regex"))

; SQL in string literals (common in security queries)
((string) @content
 (#match? @content "(?i)(SELECT|INSERT|UPDATE|DELETE|CREATE|DROP|ALTER)\\s")
 (#set! "language" "sql"))

; JavaScript/TypeScript in strings (for JS/TS analysis queries)
((string) @content
 (#match? @content "(function|const|let|var|=>|class|import|export)\\s")
 (#set! "language" "javascript"))

; HTML/XML in strings (for web security queries)
((string) @content
 (#match? @content "^[\"']<(html|div|script|style|xml|svg)")
 (#set! "language" "html"))

; JSON in strings
((string) @content
 (#match? @content "^[\"']\\{.*\\}[\"']$")
 (#set! "language" "json"))

; YAML in strings (for config analysis)
((string) @content
 (#match? @content "^[\"']---")
 (#set! "language" "yaml"))
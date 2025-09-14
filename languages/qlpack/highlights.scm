; Highlights for QL Pack YAML files
; These use standard YAML highlighting with CodeQL-specific keys emphasized

; Keys
(block_mapping_pair
  key: (flow_node) @keyword)

; CodeQL-specific keys
(block_mapping_pair
  key: (flow_node) @keyword.special
  (#match? @keyword.special "^(name|version|dependencies|library|extractor|dbscheme|upgrades|compiled|warnOnImplicitThis)$"))

; Values
(block_mapping_pair
  value: (flow_node) @string)

; Comments
(comment) @comment

; Booleans
((flow_node) @constant.builtin.boolean
  (#match? @constant.builtin.boolean "^(true|false|yes|no)$"))

; Numbers
((flow_node) @constant.numeric
  (#match? @constant.numeric "^[0-9]"))

; Anchors and aliases
(anchor) @label
(alias) @label
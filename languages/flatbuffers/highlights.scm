; Keywords
["namespace" "include" "attribute" "table" "struct" "enum" "union" "root_type" "rpc_service"] @keyword

; Types
(type) @type

; Constants
(boolean_constant) @boolean
(integer_constant) @number
(float_constant) @number

; Strings
(string_constant) @string

; Comments
(comment) @comment

; Punctuation
["(" ")" "[" "]" "{" "}"] @punctuation.bracket
[":" "," ";"] @punctuation.delimiter

; Operators
"=" @operator

; Attributes
(attribute (ident) @attribute)
(metadata (ident) @attribute)

; Tables/Structs
(table (ident) @type)
(table_field
  (ident) @field
  (type) @type)

(struct (ident) @type)
(struct_field
  (ident) @field
  (type) @type)

; Enums/Unions
(enum (ident) @type)
(enum_field name: (ident) @variant)

(union (ident) @type)
(union_field typename: (ident) @type)

; RPCs
(rpc_service (ident) @type)
(rpc_method
    name: (ident) @function
    request: (ident) @type
    response: (ident) @type)

; Special
(file_extension "file_extension" @keyword)
(file_identifier "file_identifier" @keyword)
(root_type (ident) @type)

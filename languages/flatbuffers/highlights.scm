; Keywords
["namespace" "include" "attribute" "table" "struct" "enum" "union" "root_type" "rpc_service"] @keyword

; Types
(scalar_type) @type.builtin
(vector_type element: (ident) @type)
(array_type element: (ident) @type)
(qualified_ident (ident) @type)

; Constants
(boolean_constant) @boolean
(integer_constant) @number
(float_constant) @number
(null_constant) @builtin.constant

; Strings
(string_constant) @string

; Comments
(comment) @comment

; Punctuation
["(" ")" "[" "]" "{" "}"] @punctuation.bracket
[":" "," ";" "."] @punctuation.delimiter

; Operators
"=" @operator

; Attributes
(custom_attribute name: (ident) @attribute)
(attribute name: (ident) @attribute)

; Tables/Structs
(table (ident) @type)
(table_field name: (ident) @field)
(table_field type: (ident) @type)
(table_field default: (ident) @variant) ; only allowed default idents are enum variants

(struct (ident) @type)
(struct_field name: (ident) @field)
(struct_field type: (ident) @type)

; Enums/Unions
(enum name: (ident) @type)
(enum_field name: (ident) @variant)

(union name: (ident) @type)
(union_field type: (ident) @type)
(union_field alias: (ident) @variant)

; RPCs
(rpc_service (ident) @type)
(rpc_method name: (ident) @function)
(rpc_method request: (ident) @type)
(rpc_method response: (ident) @type)

; Special
(file_extension "file_extension" @keyword)
(file_identifier "file_identifier" @keyword)
(root_type (ident) @type)
(namespace (ident) @type) ; for consistency with namespaces in qualified identifiers

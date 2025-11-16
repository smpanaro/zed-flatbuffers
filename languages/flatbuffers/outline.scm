; namespace

(namespace
  "namespace" @context
  name: (ident) @name
  ("." @name (ident) @name)*
) @item

; attribute

(custom_attribute
  "attribute" @context
  [(ident) (string_constant)] @name
) @item

; file

(file_identifier
  "file_identifier" @context
  (string_constant) @name) @item

(file_extension
  "file_extension" @context
  (string_constant) @name) @item

; struct

(struct
  "struct" @context
  name: (ident) @name) @item

(struct_field
  name: (ident) @name
  (":" @context
    type: [
      (ident)
      (qualified_ident)
      (scalar_type)
      (array_type)
      (vector_type)
    ] @context)?
) @item

; table

(table
  "table" @context
  name: (ident) @name) @item

(table_field
  name: (ident) @name
  (":" @context
    type: [
      (ident)
      (qualified_ident)
      (scalar_type)
      (array_type)
      (vector_type)
    ] @context)?
) @item

; enum

(enum
  "enum" @context
  name: (ident) @name) @item

(enum_field
  name: (ident) @name
  ("=" @context value: (integer_constant) @context)?
) @item

; union

(union
  "union" @context
  name: (ident) @name) @item

(union_field
  ((ident) @context ":" @context)? ; alias
  type: [(qualified_ident) (ident)] @name
) @item

; root_type

(root_type
  "root_type" @context
  [(qualified_ident) (ident)] @name) @item

; rpc_service

(rpc_service
  "rpc_service" @context
  (ident) @name) @item

(rpc_method
  name: (ident) @name
  "(" @context
  request: [(qualified_ident) (ident)] @context
  ")" @context
  ":" @context
  response: [(qualified_ident) (ident)] @context) @item

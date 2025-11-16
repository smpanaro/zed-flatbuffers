; these enable vim motions like [[, ]M and text objects af, ic etc
(table (
  "{"
  (_)* @class.inside
  "}")) @class.around

(struct (
  "{"
  (_)* @class.inside
  "}")) @class.around

(union (
  "{"
  (_)* @class.inside
  "}")) @class.around

(enum (
  "{"
  (_)* @class.inside
  "}")) @class.around

(rpc_service
  "{"
  (_)* @class.inside
  "}") @class.around

(rpc_method) @function.around

(comment)+ @comment.around

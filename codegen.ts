import { print } from "https://deno.land/x/swc@0.1.4/mod.ts";

const Type: Record<string, string> = {
  void: "null",
  i8: "number",
  u8: "number",
  i16: "number",
  u16: "number",
  i32: "number",
  u32: "number",
  i64: "number",
  u64: "number",
  usize: "number",
  isize: "number",
  f32: "number",
  f64: "number",
};

function invalidType(type: string) {
  throw new TypeError(`Type not supported: ${type}`);
}

const span = { start: 0, end: 0, ctxt: 0 };

function param(value: string, type: string) {
  const kind = Type[type] || invalidType(type);
  return {
    type: "Parameter",
    span,
    decorators: [],
    pat: {
      type: "Identifier",
      span,
      value,
      optional: false,
      typeAnnotation: {
        type: "TsTypeAnnotation",
        span,
        typeAnnotation: {
          type: "TsKeywordType",
          span,
          kind,
        },
      },
    },
  };
}

function bodyStmt(fn: Sig) {
  return [
    {
      type: "ReturnStatement",
      span,
      argument: {
        type: "TsAsExpression",
        span,
        typeAnnotation: {
          type: "TsKeywordType",
          span,
          kind: Type[fn.result] || invalidType(fn.result),
        },
        expression: {
          type: "CallExpression",
          span,
          callee: {
            type: "MemberExpression",
            span,
            object: {
              type: "MemberExpression",
              span,
              object: {
                type: "Identifier",
                span,
                value: "_lib",
                optional: false,
              },
              property: {
                type: "Identifier",
                span,
                value: "symbols",
                optional: false,
              },
              computed: false,
            },
            property: {
              type: "Identifier",
              span,
              value: fn.func,
              optional: false,
            },
            computed: false,
          },
          arguments: fn.parameters.map((i) => {
            return {
              spread: null,
              expression: {
                type: "Identifier",
                span,
                value: i.ident,
              },
            };
          }),
          typeArguments: null,
        },
      },
    },
  ];
}

function libDecl(target: string, signature: Sig[]) {
  return {
    type: "VariableDeclaration",
    span,
    kind: "const",
    declare: false,
    declarations: [
      {
        type: "VariableDeclarator",
        span,
        id: {
          type: "Identifier",
          span,
          value: "_lib",
          optional: false,
          typeAnnotation: null,
        },
        init: {
          type: "CallExpression",
          span,
          callee: {
            type: "MemberExpression",
            span,
            object: {
              type: "Identifier",
              span,
              value: "Deno",
              optional: false,
            },
            property: {
              type: "Identifier",
              span,
              value: "dlopen",
              optional: false,
            },
            computed: false,
          },
          arguments: [
            {
              spread: null,
              expression: {
                type: "StringLiteral",
                span,
                value: target,
                hasEscape: false,
                kind: { type: "normal", containsQuote: true },
              },
            },
            {
              spread: null,
              expression: {
                type: "ObjectExpression",
                span,
                properties: signature.map((sig) => {
                  return {
                    type: "KeyValueProperty",
                    key: {
                      type: "Identifier",
                      span,
                      value: sig.func,
                      optional: false,
                    },
                    value: {
                      type: "ObjectExpression",
                      span,
                      properties: [
                        {
                          type: "KeyValueProperty",
                          key: {
                            type: "Identifier",
                            span,
                            value: "result",
                            optional: false,
                          },
                          value: {
                            type: "StringLiteral",
                            span,
                            value: sig.result,
                            hasEscape: false,
                            kind: { type: "normal", containsQuote: true },
                          },
                        },
                        {
                          type: "KeyValueProperty",
                          key: {
                            type: "Identifier",
                            span,
                            value: "parameters",
                            optional: false,
                          },
                          value: {
                            type: "ArrayExpression",
                            span,
                            elements: sig.parameters.map((p) => {
                              return {
                                spread: null,
                                expression: {
                                  type: "StringLiteral",
                                  span,
                                  value: p.type,
                                  hasEscape: false,
                                  kind: {
                                    type: "normal",
                                    containsQuote: true,
                                  },
                                },
                              };
                            }),
                          },
                        },
                      ],
                    },
                  };
                }),
              },
            },
          ],
          typeArguments: null,
        },
        definite: false,
      },
    ],
  };
}

function exportDecl(fn: Sig) {
  return {
    type: "ExportDeclaration",
    span,
    declaration: {
      type: "FunctionDeclaration",
      identifier: {
        type: "Identifier",
        span,
        value: fn.func,
        optional: false,
      },
      declare: false,
      params: fn.parameters.map((p) => param(p.ident, p.type)),
      decorators: [],
      span,
      body: {
        type: "BlockStatement",
        span,
        stmts: bodyStmt(fn),
      },
      generator: false,
      async: false,
      typeParameters: null,
      returnType: {
        type: "TsTypeAnnotation",
        span,
        typeAnnotation: {
          type: "TsKeywordType",
          span,
          kind: Type[fn.result] || invalidType(fn.result),
        },
      },
    },
  };
}

type Sig = {
  func: string;
  parameters: { ident: string; type: string }[];
  result: string;
};

export function codegen(dylib: string, signature: Sig[]) {
  const { code } = print({
    type: "Module",
    span,
    body: [
      libDecl(dylib, signature),
      ...signature.map((e) => exportDecl(e)),
    ],
    interpreter: null,
  });

  return code;
}

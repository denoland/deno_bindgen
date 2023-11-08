use std::{
  borrow::Cow,
  io::{Result, Write},
};

use super::Generator;
use crate::{
  inventory::{Inventory, Struct},
  Symbol, Type,
};

struct TypeScriptType<'a>(&'a str);

impl std::fmt::Display for TypeScriptType<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)?;
    Ok(())
  }
}

impl TypeScriptType<'_> {
  fn into_raw<'a>(&self, ident: &'a str) -> Cow<'a, str> {
    match self {
      Self("Uint8Array") => {
        Cow::Owned(format!("{ident},\n    {ident}.byteLength"))
      }
      _ => Cow::Borrowed(ident),
    }
  }

  fn apply_promise(&self, non_blocking: bool) -> Cow<'_, str> {
    if non_blocking {
      Cow::Owned(format!("Promise<{}>", self.0))
    } else {
      Cow::Borrowed(self.0)
    }
  }
}

impl From<Type> for TypeScriptType<'_> {
  fn from(value: Type) -> Self {
    Self(match value {
      Type::Void => "void",
      Type::Uint8
      | Type::Uint16
      | Type::Uint32
      | Type::Uint64
      | Type::Int8
      | Type::Int16
      | Type::Int32
      | Type::Int64
      | Type::Float32
      | Type::Float64 => "number",
      Type::Pointer => "Deno.PointerObject | null",
      Type::Buffer => "Uint8Array",
    })
  }
}

struct DenoFfiType(String);

impl std::fmt::Display for DenoFfiType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)?;
    Ok(())
  }
}

impl From<Type> for DenoFfiType {
  fn from(value: Type) -> Self {
    let ty = match value {
      Type::Void => "void",
      Type::Uint8 => "u8",
      Type::Uint16 => "u16",
      Type::Uint32 => "u32",
      Type::Uint64 => "u64",
      Type::Int8 => "i8",
      Type::Int16 => "i16",
      Type::Int32 => "i32",
      Type::Int64 => "i64",
      Type::Float32 => "f32",
      Type::Float64 => "f64",
      Type::Pointer => "pointer",
      Type::Buffer => "buffer",
    };

    let mut raw = format!("'{}'", ty);

    // Complex types.
    if value == Type::Buffer {
      raw.push_str(",\n      'usize'");
    }

    Self(raw)
  }
}

pub struct Codegen<'a> {
  symbols: &'a [Inventory],
}

impl<'a> Codegen<'a> {
  pub fn new(symbols: &'a [Inventory]) -> Self {
    Self { symbols }
  }

  fn dlopen<W: Write>(&self, writer: &mut W) -> Result<()> {
    writeln!(writer, "const {{ dlopen }} = Deno;\n")?;

    writeln!(
      writer,
      "const {{ symbols }} = dlopen('./target/debug/libdeno_bindgen_test.dylib', {{"
    )?;
    self.write_symbols(writer)?;
    writeln!(writer, "}});\n")?;
    Ok(())
  }

  fn write_symbols<W: Write>(&self, writer: &mut W) -> Result<()> {
    fn format_bracket<W: Write, T>(
      writer: &mut W,
      items: &[T],
      callback: impl Fn(&mut W, &[T]) -> Result<()>,
    ) -> Result<()> {
      write!(writer, "[")?;
      if items.len() > 0 {
        write!(writer, "\n")?;
        callback(writer, items)?;
        writeln!(writer, "    ],")?;
      } else {
        writeln!(writer, "],")?;
      }

      Ok(())
    }

    for symbol in self.symbols {
      match symbol {
        Inventory::Symbol(symbol) => {
          writeln!(writer, "  {}: {{", symbol.name)?;
          write!(writer, "    parameters: ")?;
          format_bracket(writer, symbol.parameters, |writer, parameters| {
            for parameter in parameters {
              writeln!(writer, "      {},", DenoFfiType::from(*parameter))?;
            }
            Ok(())
          })?;
          writeln!(
            writer,
            "    result: {},",
            DenoFfiType::from(symbol.return_type)
          )?;
          writeln!(writer, "    nonblocking: {}", symbol.non_blocking)?;
          writeln!(writer, "  }},")?;
        }
        _ => {}
      }
    }

    Ok(())
  }

  fn exports<W: Write>(&self, writer: &mut W) -> Result<()> {
    fn format_paren<W: Write, T>(
      writer: &mut W,
      items: &[T],
      allow_empty: bool,
      callback: impl Fn(&mut W, &[T]) -> Result<()>,
      nesting_spaces: usize,
      delim: (char, char),
    ) -> Result<()> {
      let (start, end) = delim;
      write!(writer, "{start}")?;
      if items.len() > 0 || allow_empty {
        write!(writer, "\n")?;
        callback(writer, items)?;
        write!(writer, "{:indent$}{end}", "", indent = nesting_spaces)?;
      } else {
        write!(writer, "{end}")?;
      }

      Ok(())
    }

    for symbol in self.symbols {
      match symbol {
        Inventory::Symbol(symbol) => {
          if !symbol.internal {
            write!(writer, "export ")?;
          }
          write!(writer, "function {}", symbol.name)?;
          format_paren(
            writer,
            symbol.parameters,
            false,
            |writer, parameters| {
              for (i, parameter) in parameters.iter().enumerate() {
                writeln!(
                  writer,
                  "  arg{}: {},",
                  i,
                  TypeScriptType::from(*parameter)
                )?;
              }
              Ok(())
            },
            0,
            ('(', ')'),
          )?;
          writeln!(
            writer,
            ": {} {{",
            TypeScriptType::from(symbol.return_type)
              .apply_promise(symbol.non_blocking)
          )?;
          write!(writer, "  return symbols.{}", symbol.name)?;
          format_paren(
            writer,
            symbol.parameters,
            false,
            |writer, parameters| {
              for (i, parameter) in parameters.iter().enumerate() {
                let ident = format!("arg{}", i);
                writeln!(
                  writer,
                  "    {},",
                  TypeScriptType::from(*parameter).into_raw(&ident)
                )?;
              }
              Ok(())
            },
            2,
            ('(', ')'),
          )?;

          writeln!(writer, "\n}}\n")?;
        }
        Inventory::Struct(Struct {
          name,
          methods,
          constructor,
        }) => {
          write!(writer, "export class {name} ")?;

          format_paren(
            writer,
            &methods,
            false,
            |writer, methods| {
              writeln!(writer, "  ptr: Deno.PointerObject | null = null;\n")?;

              writeln!(
                writer,
                "  static __constructor(ptr: Deno.PointerObject) {{"
              )?;
              writeln!(
                writer,
                "    const self = Object.create({name}.prototype);"
              )?;
              writeln!(writer, "    self.ptr = ptr;")?;
              writeln!(writer, "    return self;")?;
              writeln!(writer, "  }}")?;

              for method in methods {
                // Skip the first argument, which is always the pointer to the struct.
                let parameters = &method.parameters[1..];
                writeln!(
                  writer,
                  "\n  {name}({parameters}): {return_type} {{",
                  name = method.name,
                  parameters = parameters
                    .iter()
                    .enumerate()
                    .map(|(i, parameter)| {
                      format!("arg{}: {}", i, TypeScriptType::from(*parameter))
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                  return_type = TypeScriptType::from(method.return_type)
                )?;

                write!(writer, "    return {}", method.name)?;
                format_paren(
                  writer,
                  parameters,
                  true,
                  |writer, parameters| {
                    writeln!(writer, "      this.ptr,",)?;
                    for (i, parameter) in parameters.iter().enumerate() {
                      let ident = format!("arg{}", i);
                      writeln!(
                        writer,
                        "      {},",
                        TypeScriptType::from(*parameter).into_raw(&ident)
                      )?;
                    }
                    Ok(())
                  },
                  4,
                  ('(', ')'),
                )?;

                writeln!(writer, "\n  }}")?;
              }
              Ok(())
            },
            0,
            ('{', '}'),
          )?;
        }
      }
    }

    Ok(())
  }
}

impl<'a> Generator for Codegen<'a> {
  fn generate<W: Write>(&mut self, mut writer: W) -> Result<()> {
    fn prelude<W: Write>(writer: &mut W) -> Result<()> {
      writeln!(writer, "// deno-lint-ignore-file\n")?;
      writeln!(
        writer,
        "// This file is automatically generated by deno_bindgen."
      )?;
      writeln!(writer, "// Do not edit this file directly.\n")?;
      Ok(())
    }

    prelude(&mut writer)?;
    self.dlopen(&mut writer)?;
    self.exports(&mut writer)?;

    Ok(())
  }
}

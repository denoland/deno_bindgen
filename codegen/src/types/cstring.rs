use super::{NativeType, TypeConverter, TypeConverters, TypeDescriptor};

#[derive(Clone)]
pub struct CString;

impl From<CString> for TypeDescriptor {
  fn from(_: CString) -> Self {
    TypeDescriptor {
      native: NativeType::Pointer,
      converters: TypeConverters {
        global: Some(
          "const __cstring_encoder = new TextEncoder();\n\
            function __cstring_into(cstring: string): Deno.UnsafePointer {\n\
              const __buffer = new Uint8Array(cstring.length + 1);\n\
              __cstring_encoder.encodeInto(cstring, __buffer);\n\
              return Deno.UnsafePointer.of(__buffer);\n\
            }\n"
            .to_string(),
        ),
        typescript: "string".to_string(),
        into: TypeConverter {
          local: None,
          inline: "__cstring_into({})".to_string(),
        },
        from: TypeConverter {
          local: None,
          inline: "new Deno.UnsafePointerView({}).getCString()".to_string(),
        },
      },
    }
  }
}

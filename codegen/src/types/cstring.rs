use super::TypeDescriptor;
use super::TypeConverter;
use super::NativeType;

#[derive(Clone)]
pub struct CString;

impl From<CString> for TypeDescriptor {
  fn from(_: CString) -> Self {
    TypeDescriptor {
      native: NativeType::Pointer,
      converter: TypeConverter {
        global: Some(
          "const __cstring_encoder = new TextEncoder();\n\
            function __cstring_into(__cstring: string): Deno.UnsafePointer {\n\
              const __buffer = new Uint8Array(__cstring.length + 1);\n\
              __cstring_encoder.encodeInto(__cstring, __buffer);\n\
              return Deno.UnsafePointer.of(__buffer);\n\
            }\n"
            .to_string(),
        ),
        typescript: "string".to_string(),
        into: "__cstring_into({})".to_string(),
        from: "new Deno.UnsafePointerView({}).getCString()".to_string(),
      },
    }
  }
}

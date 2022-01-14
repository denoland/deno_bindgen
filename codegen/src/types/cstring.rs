use super::NativeType;
use super::TypeConverter;
use super::TypeDescriptor;

#[derive(Clone)]
pub struct CString;

impl From<CString> for TypeDescriptor {
  fn from(_: CString) -> Self {
    TypeDescriptor {
      native: NativeType::Pointer,
      converter: TypeConverter {
        global: Some(
          "const __cstring_encoder = new TextEncoder();\n".to_string(),
        ),
        typescript: "string".to_string(),
        into: "Deno.UnsafePointer.of(__cstring_encoder.encode({} + \"\0\"))"
          .to_string(),
        from: "new Deno.UnsafePointerView({}).getCString()".to_string(),
      },
    }
  }
}

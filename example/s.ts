class Example {
  ptr: Deno.PointerObject | null = null;

  static from_ptr(ptr: Deno.PointerObject | null) {
    const obj = Object.create(Example.prototype);
    obj.ptr = ptr;

    return obj;
  }

  constructor() {
    // ...
  }

  close() {
    ffi_free(this.ptr);
  }

  [Symbol.dispose]() {
    this.close();
  }
}

export function make_example(): Example {
  return Example.from_ptr(ffi_make_example());
}

using obj = make_example();

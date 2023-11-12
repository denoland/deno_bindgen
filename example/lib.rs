use deno_bindgen::deno_bindgen;

#[deno_bindgen]
pub struct Context {
  context: webusb::Context,
}

#[deno_bindgen]
impl Context {
  #[constructor]
  pub fn init() -> Context {
    let context = webusb::Context::init().expect("Unable to create context");
    Context { context }
  }

  pub fn lsusb(&self) {
    let devices = self.context.devices().expect("Unable to get devices");
    for device in devices {
      if let Some(name) = device.product_name {
        println!("Product Name: {}", name);
      }

      println!("Vendor ID: {}", device.vendor_id);
      println!("Product ID: {}\n", device.product_id);
    }
  }

  pub fn open(&mut self, vendor_id: u16, product_id: u16) -> Device {
    let devices = self.context.devices().expect("Unable to get devices");
    let mut device = devices
      .into_iter()
      .find(|d| d.vendor_id == vendor_id && d.product_id == product_id)
      .expect("Device not found.");

    device.open().expect("Unable to open device.");

    Device { device }
  }
}

#[deno_bindgen]
pub struct Device {
  device: webusb::UsbDevice,
}

impl Drop for Device {
  fn drop(&mut self) {
    self.device.close().expect("Unable to close device.");
  }
}

#[deno_bindgen]
impl Device {
  pub fn claim_interface(&mut self, interface_number: u8) {
    self
      .device
      .claim_interface(interface_number)
      .expect("Unable to claim interface.");
  }

  pub fn select_alternate_interface(
    &mut self,
    interface_number: u8,
    alternate_setting: u8,
  ) {
    self
      .device
      .select_alternate_interface(interface_number, alternate_setting)
      .expect("Unable to select alternate interface.");
  }
}

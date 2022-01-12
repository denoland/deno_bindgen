use std::fmt::Write;

use crate::error::AnyError;
use crate::library::Library;
use crate::library::LibraryElement;
use crate::source::Source;

#[derive(Clone)]
pub enum PlugLoaderOptions {
  Single(PlugLoaderSingleOptions),
  Cross(PlugLoaderCrossOptions),
}

#[derive(Clone)]
pub struct PlugLoaderUrls {
  pub darwin: Option<String>,
  pub linux: Option<String>,
  pub windows: Option<String>,
}

#[derive(Clone)]
pub enum PlugLoaderCachePolicy {
  None,
  Store,
}

#[derive(Clone)]
pub struct PlugLoaderCrossOptions {
  pub name: String,
  pub urls: PlugLoaderUrls,
  pub policy: Option<PlugLoaderCachePolicy>,
  pub cache: Option<String>,
  pub log: Option<bool>,
}

#[derive(Clone)]
pub struct PlugLoaderSingleOptions {
  pub name: String,
  pub url: String,
  pub policy: Option<PlugLoaderCachePolicy>,
  pub cache: Option<String>,
  pub log: Option<bool>,
}

#[derive(Clone)]
pub struct PlugLoader {
  import: String,
  options: PlugLoaderOptions,
}

impl PlugLoader {
  pub fn new(import: Option<&str>, options: PlugLoaderOptions) -> Self {
    Self {
      import: import
        .unwrap_or("https://deno.land/x/plug/mod.ts")
        .to_string(),
      options,
    }
  }
}

impl LibraryElement for PlugLoader {
  fn generate(
    &self,
    library: &Library,
    source: &mut Source,
  ) -> Result<(), AnyError> {
    writeln!(source, "import * as Plug from \"{}\";", self.import)?;
    writeln!(
      source,
      "const {} = await Plug.prepare({}, {});",
      library.variable,
      String::from(self.options.clone()),
      library.symbols()?
    )?;

    Ok(())
  }
}

impl From<PlugLoaderOptions> for String {
  fn from(options: PlugLoaderOptions) -> Self {
    match options {
      PlugLoaderOptions::Single(options) => String::from(options),
      PlugLoaderOptions::Cross(options) => String::from(options),
    }
  }
}

impl From<PlugLoaderSingleOptions> for String {
  fn from(options: PlugLoaderSingleOptions) -> Self {
    let mut properties = Vec::new();

    properties.push(format!("name: \"{}\"", options.name,));
    properties.push(format!("url: \"{}\"", String::from(options.url)));

    if let Some(policy) = options.policy {
      properties.push(format!("policy: {}", String::from(policy)));
    }

    if let Some(cache) = options.cache {
      properties.push(format!("cache: {}", cache));
    }

    if let Some(log) = options.log {
      properties.push(format!("log: {}", log));
    }

    format!("{{ {} }}", properties.join(", "))
  }
}

impl From<PlugLoaderCrossOptions> for String {
  fn from(options: PlugLoaderCrossOptions) -> Self {
    let mut properties = Vec::new();

    properties.push(format!("name: \"{}\"", options.name,));
    properties.push(format!("urls: \"{}\"", String::from(options.urls)));

    if let Some(policy) = options.policy {
      properties.push(format!("policy: {}", String::from(policy)));
    }

    if let Some(cache) = options.cache {
      properties.push(format!("cache: {}", cache));
    }

    if let Some(log) = options.log {
      properties.push(format!("log: {}", log));
    }

    format!("{{ {} }}", properties.join(", "))
  }
}

impl From<PlugLoaderUrls> for String {
  fn from(urls: PlugLoaderUrls) -> Self {
    let mut properties = Vec::new();

    if let Some(darwin) = urls.darwin {
      properties.push(format!("darwin: {}", darwin));
    }

    if let Some(linux) = urls.linux {
      properties.push(format!("linux: {}", linux));
    }

    if let Some(windows) = urls.windows {
      properties.push(format!("windows: {}", windows));
    }

    format!("{{ {} }}", properties.join(", "))
  }
}

impl From<PlugLoaderCachePolicy> for String {
  fn from(cache_policy: PlugLoaderCachePolicy) -> Self {
    match cache_policy {
      PlugLoaderCachePolicy::None => "Plug.CachePolicy.NONE",
      PlugLoaderCachePolicy::Store => "Plug.CachePolicy.STORE",
    }
    .to_string()
  }
}

// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

#![deny(clippy::disallowed_methods)]
#![deny(clippy::disallowed_types)]

#[cfg(feature = "cjs")]
mod cjs_parse;
mod comments;
#[cfg(feature = "dep_analysis")]
pub mod dep;
pub mod diagnostics;
#[cfg(feature = "emit")]
mod emit;
mod lexing;
mod parsed_source;
mod parsing;
#[cfg(feature = "scopes")]
mod scopes;
mod text_changes;
#[cfg(feature = "transpiling")]
mod transpiling;
mod types;

#[cfg(feature = "view")]
pub use dprint_swc_ext::view;

pub use dprint_swc_ext::common::*;

#[cfg(feature = "cjs")]
pub use cjs_parse::*;
pub use comments::*;
pub use deno_media_type::*;
#[cfg(feature = "emit")]
pub use emit::*;
pub use lexing::*;
pub use parsed_source::*;
pub use parsing::*;
#[cfg(feature = "scopes")]
pub use scopes::*;
pub use text_changes::*;
#[cfg(feature = "transpiling")]
pub use transpiling::*;
pub use types::*;

pub type ModuleSpecifier = url::Url;

pub mod swc {
  pub use dprint_swc_ext::swc::atoms;
  pub use dprint_swc_ext::swc::common;
  #[cfg(feature = "bundler")]
  pub use swc_bundler as bundler;
  pub use swc_ecma_ast as ast;
  #[cfg(feature = "codegen")]
  pub use swc_ecma_codegen as codegen;
  #[cfg(feature = "transforms")]
  pub use swc_ecma_loader as loader;
  pub use swc_ecma_parser as parser;
  #[cfg(feature = "transforms")]
  pub mod transforms {
    pub use self::fixer::fixer;
    pub use self::hygiene::hygiene;
    pub use swc_ecma_transforms_base::assumptions::Assumptions;
    pub use swc_ecma_transforms_base::fixer;
    pub use swc_ecma_transforms_base::helpers;
    pub use swc_ecma_transforms_base::hygiene;
    pub use swc_ecma_transforms_base::pass;
    pub use swc_ecma_transforms_base::perf;
    pub use swc_ecma_transforms_base::resolver;

    #[cfg(feature = "compat")]
    pub use swc_ecma_transforms_compat as compat;
    #[cfg(feature = "proposal")]
    pub use swc_ecma_transforms_proposal as proposal;
    #[cfg(feature = "react")]
    pub use swc_ecma_transforms_react as react;
    #[cfg(feature = "typescript")]
    pub use swc_ecma_transforms_typescript as typescript;
  }
  #[cfg(feature = "utils")]
  pub use swc_ecma_utils as utils;
  #[cfg(feature = "visit")]
  pub use swc_ecma_visit as visit;
}

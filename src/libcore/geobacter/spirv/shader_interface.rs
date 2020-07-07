use crate::geobacter::spirv::pipeline_layout::CompilerRawImgFormat;

pub type CompilerRange = (u32, u32);
pub type CompilerOptionalName = &'static [&'static str];

pub type CompilerShaderInterfaceDefEntry = (CompilerRange,
                                            CompilerRawImgFormat,
                                            CompilerOptionalName);
pub type CompilerShaderInterfaceDef = &'static [CompilerShaderInterfaceDefEntry];

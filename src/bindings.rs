/* automatically generated by rust-bindgen */

pub type khronos_uint64_t = u64;
pub type sh_GLenum = ::std::os::raw::c_uint;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShShaderSpec {
    SH_GLES2_SPEC = 35648,
    SH_WEBGL_SPEC = 35649,
    SH_GLES3_SPEC = 35718,
    SH_WEBGL2_SPEC = 35719,
    SH_CSS_SHADERS_SPEC = 35650,
}
pub const ShShaderOutput_SH_HLSL9_OUTPUT: ShShaderOutput =
    ShShaderOutput::SH_HLSL_OUTPUT;
pub const ShShaderOutput_SH_HLSL_3_0_OUTPUT: ShShaderOutput =
    ShShaderOutput::SH_HLSL_OUTPUT;
pub const ShShaderOutput_SH_HLSL_4_1_OUTPUT: ShShaderOutput =
    ShShaderOutput::SH_HLSL11_OUTPUT;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShShaderOutput {
    SH_ESSL_OUTPUT = 35653,
    SH_GLSL_COMPATIBILITY_OUTPUT = 35654,
    SH_GLSL_130_OUTPUT = 35655,
    SH_GLSL_140_OUTPUT = 35712,
    SH_GLSL_150_CORE_OUTPUT = 35713,
    SH_GLSL_330_CORE_OUTPUT = 35714,
    SH_GLSL_400_CORE_OUTPUT = 35715,
    SH_GLSL_410_CORE_OUTPUT = 35716,
    SH_GLSL_420_CORE_OUTPUT = 35717,
    SH_GLSL_430_CORE_OUTPUT = 35718,
    SH_GLSL_440_CORE_OUTPUT = 35719,
    SH_GLSL_450_CORE_OUTPUT = 35720,
    SH_HLSL_OUTPUT = 35656,
    SH_HLSL11_OUTPUT = 35657,
    SH_HLSL_4_0_FL9_3_OUTPUT = 35658,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShCompileOptions {
    SH_VALIDATE = 0,
    SH_VALIDATE_LOOP_INDEXING = 1,
    SH_INTERMEDIATE_TREE = 2,
    SH_OBJECT_CODE = 4,
    SH_VARIABLES = 8,
    SH_LINE_DIRECTIVES = 16,
    SH_SOURCE_PATH = 32,
    SH_UNROLL_FOR_LOOP_WITH_INTEGER_INDEX = 64,
    SH_UNROLL_FOR_LOOP_WITH_SAMPLER_ARRAY_INDEX = 128,
    SH_EMULATE_BUILT_IN_FUNCTIONS = 256,
    SH_TIMING_RESTRICTIONS = 512,
    SH_DEPENDENCY_GRAPH = 1024,
    SH_ENFORCE_PACKING_RESTRICTIONS = 2048,
    SH_CLAMP_INDIRECT_ARRAY_BOUNDS = 4096,
    SH_LIMIT_EXPRESSION_COMPLEXITY = 8192,
    SH_LIMIT_CALL_STACK_DEPTH = 16384,
    SH_INIT_GL_POSITION = 32768,
    SH_UNFOLD_SHORT_CIRCUIT = 65536,
    SH_INIT_VARYINGS_WITHOUT_STATIC_USE = 131072,
    SH_SCALARIZE_VEC_AND_MAT_CONSTRUCTOR_ARGS = 262144,
    SH_REGENERATE_STRUCT_NAMES = 524288,
    SH_DONT_PRUNE_UNUSED_FUNCTIONS = 1048576,
    SH_REMOVE_POW_WITH_CONSTANT_EXPONENT = 2097152,
    SH_REWRITE_DO_WHILE_LOOPS = 4194304,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShArrayIndexClampingStrategy {
    SH_CLAMP_WITH_CLAMP_INTRINSIC = 1,
    SH_CLAMP_WITH_USER_DEFINED_INT_CLAMP_FUNCTION = 2,
}
pub type ShHashFunction64 =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *const ::std::os::raw::c_char,
                                               arg2: usize)
                              -> khronos_uint64_t>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ShBuiltInResources {
    pub MaxVertexAttribs: ::std::os::raw::c_int,
    pub MaxVertexUniformVectors: ::std::os::raw::c_int,
    pub MaxVaryingVectors: ::std::os::raw::c_int,
    pub MaxVertexTextureImageUnits: ::std::os::raw::c_int,
    pub MaxCombinedTextureImageUnits: ::std::os::raw::c_int,
    pub MaxTextureImageUnits: ::std::os::raw::c_int,
    pub MaxFragmentUniformVectors: ::std::os::raw::c_int,
    pub MaxDrawBuffers: ::std::os::raw::c_int,
    pub OES_standard_derivatives: ::std::os::raw::c_int,
    pub OES_EGL_image_external: ::std::os::raw::c_int,
    pub ARB_texture_rectangle: ::std::os::raw::c_int,
    pub EXT_blend_func_extended: ::std::os::raw::c_int,
    pub EXT_draw_buffers: ::std::os::raw::c_int,
    pub EXT_frag_depth: ::std::os::raw::c_int,
    pub EXT_shader_texture_lod: ::std::os::raw::c_int,
    pub WEBGL_debug_shader_precision: ::std::os::raw::c_int,
    pub EXT_shader_framebuffer_fetch: ::std::os::raw::c_int,
    pub NV_shader_framebuffer_fetch: ::std::os::raw::c_int,
    pub ARM_shader_framebuffer_fetch: ::std::os::raw::c_int,
    pub NV_draw_buffers: ::std::os::raw::c_int,
    pub FragmentPrecisionHigh: ::std::os::raw::c_int,
    pub MaxVertexOutputVectors: ::std::os::raw::c_int,
    pub MaxFragmentInputVectors: ::std::os::raw::c_int,
    pub MinProgramTexelOffset: ::std::os::raw::c_int,
    pub MaxProgramTexelOffset: ::std::os::raw::c_int,
    pub MaxDualSourceDrawBuffers: ::std::os::raw::c_int,
    pub HashFunction: ShHashFunction64,
    pub ArrayIndexClampingStrategy: ShArrayIndexClampingStrategy,
    pub MaxExpressionComplexity: ::std::os::raw::c_int,
    pub MaxCallStackDepth: ::std::os::raw::c_int,
    pub MaxFunctionParameters: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ShBuiltInResources() {
    assert_eq!(::std::mem::size_of::<ShBuiltInResources>() , 128usize , concat
               ! ( "Size of: " , stringify ! ( ShBuiltInResources ) ));
    assert_eq! (::std::mem::align_of::<ShBuiltInResources>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( ShBuiltInResources ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . MaxVertexAttribs
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexAttribs ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVertexUniformVectors as * const _ as usize } , 4usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexUniformVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVaryingVectors as * const _ as usize } , 8usize , concat !
                (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVaryingVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVertexTextureImageUnits as * const _ as usize } , 12usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexTextureImageUnits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxCombinedTextureImageUnits as * const _ as usize } , 16usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxCombinedTextureImageUnits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxTextureImageUnits as * const _ as usize } , 20usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxTextureImageUnits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxFragmentUniformVectors as * const _ as usize } , 24usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxFragmentUniformVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . MaxDrawBuffers
                as * const _ as usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxDrawBuffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                OES_standard_derivatives as * const _ as usize } , 32usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( OES_standard_derivatives ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                OES_EGL_image_external as * const _ as usize } , 36usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( OES_EGL_image_external ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                ARB_texture_rectangle as * const _ as usize } , 40usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( ARB_texture_rectangle ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                EXT_blend_func_extended as * const _ as usize } , 44usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_blend_func_extended ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . EXT_draw_buffers
                as * const _ as usize } , 48usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_draw_buffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . EXT_frag_depth
                as * const _ as usize } , 52usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_frag_depth ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                EXT_shader_texture_lod as * const _ as usize } , 56usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_shader_texture_lod ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                WEBGL_debug_shader_precision as * const _ as usize } , 60usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( WEBGL_debug_shader_precision ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                EXT_shader_framebuffer_fetch as * const _ as usize } , 64usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_shader_framebuffer_fetch ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                NV_shader_framebuffer_fetch as * const _ as usize } , 68usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( NV_shader_framebuffer_fetch ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                ARM_shader_framebuffer_fetch as * const _ as usize } , 72usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( ARM_shader_framebuffer_fetch ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . NV_draw_buffers
                as * const _ as usize } , 76usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( NV_draw_buffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                FragmentPrecisionHigh as * const _ as usize } , 80usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( FragmentPrecisionHigh ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVertexOutputVectors as * const _ as usize } , 84usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexOutputVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxFragmentInputVectors as * const _ as usize } , 88usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxFragmentInputVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MinProgramTexelOffset as * const _ as usize } , 92usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MinProgramTexelOffset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxProgramTexelOffset as * const _ as usize } , 96usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxProgramTexelOffset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxDualSourceDrawBuffers as * const _ as usize } , 100usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxDualSourceDrawBuffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . HashFunction as
                * const _ as usize } , 104usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( HashFunction ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                ArrayIndexClampingStrategy as * const _ as usize } , 112usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( ArrayIndexClampingStrategy ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxExpressionComplexity as * const _ as usize } , 116usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxExpressionComplexity ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxCallStackDepth as * const _ as usize } , 120usize , concat
                ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxCallStackDepth ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxFunctionParameters as * const _ as usize } , 124usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxFunctionParameters ) ));
}
impl Clone for ShBuiltInResources {
    fn clone(&self) -> Self { *self }
}
pub type ShHandle = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ShVariableInfo {
    pub type_: sh_GLenum,
    pub size: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ShVariableInfo() {
    assert_eq!(::std::mem::size_of::<ShVariableInfo>() , 8usize , concat ! (
               "Size of: " , stringify ! ( ShVariableInfo ) ));
    assert_eq! (::std::mem::align_of::<ShVariableInfo>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( ShVariableInfo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShVariableInfo ) ) . type_ as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ShVariableInfo ) , "::"
                , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShVariableInfo ) ) . size as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( ShVariableInfo ) , "::"
                , stringify ! ( size ) ));
}
impl Clone for ShVariableInfo {
    fn clone(&self) -> Self { *self }
}

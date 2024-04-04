#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{c_char, c_int, c_void};

// Based on Slang version 2024.1.6

#[repr(C)]
pub struct IBlobVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub getBufferPointer: unsafe extern "stdcall" fn(*mut c_void) -> *const c_void,
	pub getBufferSize: unsafe extern "stdcall" fn(*mut c_void) -> usize,
}

#[repr(C)]
pub struct IGlobalSessionVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub createSession: unsafe extern "stdcall" fn(*mut c_void, desc: *const slang_SessionDesc, outSession: *mut *mut slang_ISession) -> SlangResult,
	pub findProfile: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char) -> SlangProfileID,
	pub setDownstreamCompilerPath: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, path: *const c_char),
	#[deprecated( note = "Use setLanguagePrelude instead")]
	pub setDownstreamCompilerPrelude: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, preludeText: *const c_char),
	#[deprecated( note = "Use getLanguagePrelude instead")]
	pub getDownstreamCompilerPrelude: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, outPrelude: *mut *mut ISlangBlob),
	pub getBuildTagString: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	pub setDefaultDownstreamCompiler: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage, defaultCompiler: SlangPassThrough) -> SlangResult,
	pub getDefaultDownstreamCompiler: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage) -> SlangPassThrough,
	pub setLanguagePrelude: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage, preludeText: *const c_char),
	pub getLanguagePrelude: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage, outPrelude: *mut *mut ISlangBlob),
	pub createCompileRequest: unsafe extern "stdcall" fn(*mut c_void, *mut *mut slang_ICompileRequest) -> SlangResult,
	pub addBuiltins: unsafe extern "stdcall" fn(*mut c_void, sourcePath: *const c_char, sourceString: *const c_char),
	pub setSharedLibraryLoader: unsafe extern "stdcall" fn(*mut c_void, loader: *mut ISlangSharedLibraryLoader),
	pub getSharedLibraryLoader: unsafe extern "stdcall" fn(*mut c_void) -> *mut ISlangSharedLibraryLoader,
	pub checkCompileTargetSupport: unsafe extern "stdcall" fn(*mut c_void, target: SlangCompileTarget) -> SlangResult,
	pub checkPassThroughSupport: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough) -> SlangResult,
	pub compileStdLib: unsafe extern "stdcall" fn(*mut c_void, flags: slang_CompileStdLibFlags) -> SlangResult,
	pub loadStdLib: unsafe extern "stdcall" fn(*mut c_void, stdLib: *const c_void, stdLibSizeInBytes: usize) -> SlangResult,
	pub saveStdLib: unsafe extern "stdcall" fn(*mut c_void, archiveType: SlangArchiveType, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub findCapability: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char) -> SlangCapabilityID,
	pub setDownstreamCompilerForTransition: unsafe extern "stdcall" fn(*mut c_void, source: SlangCompileTarget, target: SlangCompileTarget, compiler: SlangPassThrough),
	pub getDownstreamCompilerForTransition: unsafe extern "stdcall" fn(*mut c_void, source: SlangCompileTarget, target: SlangCompileTarget) -> SlangPassThrough,
	pub getCompilerElapsedTime: unsafe extern "stdcall" fn(*mut c_void, outTotalTime: *mut f64, outDownstreamTime: *mut f64),
	pub setSPIRVCoreGrammar: unsafe extern "stdcall" fn(*mut c_void, jsonPath: *const c_char) -> SlangResult,
	pub parseCommandLineArguments: unsafe extern "stdcall" fn(*mut c_void, argc: c_int, argv: *const *const c_char, outSessionDesc: *mut slang_SessionDesc, outAuxAllocation: *mut *mut ISlangUnknown) -> SlangResult,
	pub getSessionDescDigest: unsafe extern "stdcall" fn(*mut c_void, sessionDesc: *const slang_SessionDesc, outBlob: *mut *mut ISlangBlob) -> SlangResult,
}

#[repr(C)]
pub struct ISessionVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub getGlobalSession: unsafe extern "stdcall" fn(*mut c_void) -> *mut slang_IGlobalSession,
	pub loadModule: unsafe extern "stdcall" fn(*mut c_void, moduleName: *const c_char, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_IModule,
	pub loadModuleFromSource: unsafe extern "stdcall" fn(*mut c_void, moduleName: *const c_char, path: *const c_char, source: *mut ISlangBlob, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_IModule,
	pub createCompositeComponentType: unsafe extern "stdcall" fn(*mut c_void, componentTypes: *const *const slang_IComponentType, componentTypeCount: SlangInt, outCompositeComponentType: *mut *mut slang_IComponentType, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
	pub specializeType: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, specializationArgs: *const slang_SpecializationArg, specializationArgCount: SlangInt, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_TypeReflection,
	pub getTypeLayout: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, targetIndex: SlangInt, rules: slang_LayoutRules, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_TypeLayoutReflection,
	pub getContainerType: unsafe extern "stdcall" fn(*mut c_void, elementType: *mut slang_TypeReflection, containerType: slang_ContainerType, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_TypeReflection,
	pub getDynamicType: unsafe extern "stdcall" fn(*mut c_void) -> *mut slang_TypeReflection,
	pub getTypeRTTIMangledName: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, outNameBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub getTypeConformanceWitnessMangledName: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, interfaceType: *mut slang_TypeReflection, outNameBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub getTypeConformanceWitnessSequentialID: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, interfaceType: *mut slang_TypeReflection, outId: *mut u32) -> SlangResult,
	pub createCompileRequest: unsafe extern "stdcall" fn(*mut c_void, outCompileRequest: *mut *mut slang_ICompileRequest) -> SlangResult,
	pub createTypeConformanceComponentType: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, interfaceType: *mut slang_TypeReflection, outConformance: *mut *mut slang_ITypeConformance, conformanceIdOverride: SlangInt, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
	pub loadModuleFromIRBlob: unsafe extern "stdcall" fn(*mut c_void, moduleName: *const c_char, path: *const c_char, source: *mut ISlangBlob, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_IModule,
	pub getLoadedModuleCount: unsafe extern "stdcall" fn(*mut c_void) -> SlangInt,
	pub getLoadedModule: unsafe extern "stdcall" fn(*mut c_void, index: SlangInt) -> *mut slang_IModule,
	pub isBinaryModuleUpToDate: unsafe extern "stdcall" fn(*mut c_void, modulePath: *const c_char, binaryModuleBlob: *mut ISlangBlob) -> bool,
	pub loadModuleFromSourceString: unsafe extern "stdcall" fn(*mut c_void, moduleName: *const c_char, path: *const c_char, string: *const c_char, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_IModule,
}

#[repr(C)]
pub struct IComponentTypeVtable {
	pub _base: ISlangUnknown__bindgen_vtable,

	pub getSession: unsafe extern "stdcall" fn(*mut c_void) -> *mut slang_ISession,
	pub getLayout: unsafe extern "stdcall" fn(*mut c_void, targetIndex: SlangInt, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_ProgramLayout,
	pub getSpecializationParamCount: unsafe extern "stdcall" fn(*mut c_void) -> SlangInt,
	pub getEntryPointCode: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: SlangInt, targetIndex: SlangInt, outCode: *mut *mut ISlangBlob, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
	pub getResultAsFileSystem: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: SlangInt, targetIndex: SlangInt, outFileSystem: *mut *mut ISlangMutableFileSystem) -> SlangResult,
	pub getEntryPointHash: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: SlangInt, targetIndex: SlangInt, outHash: *mut *mut ISlangBlob),
	pub specialize: unsafe extern "stdcall" fn(*mut c_void, specializationArgs: *const slang_SpecializationArg, specializationArgCount: SlangInt, outSpecializedComponentType: *mut *mut slang_IComponentType, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
	pub link: unsafe extern "stdcall" fn(*mut c_void, outLinkedComponentType: *mut *mut slang_IComponentType, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
	pub getEntryPointHostCallable: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, targetIndex: c_int, outSharedLibrary: *mut *mut ISlangSharedLibrary, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
	pub renameEntryPoint: unsafe extern "stdcall" fn(*mut c_void, newName: *const c_char, outEntryPoint: *mut *mut slang_IComponentType) -> SlangResult,
	pub linkWithOptions: unsafe extern "stdcall" fn(*mut c_void, outLinkedComponentType: *mut *mut slang_IComponentType, compilerOptionEntryCount: u32, compilerOptionEntries: *mut slang_CompilerOptionEntry, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
}

#[repr(C)]
pub struct IEntryPointVtable {
	pub _base: IComponentTypeVtable,
}

#[repr(C)]
pub struct ITypeConformanceVtable {
	pub _base: IComponentTypeVtable,
}

#[repr(C)]
pub struct IModuleVtable {
	pub _base: IComponentTypeVtable,

	pub findEntryPointByName: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char, outEntryPoint: *mut *mut slang_IEntryPoint) -> SlangResult,
	pub getDefinedEntryPointCount: unsafe extern "stdcall" fn(*mut c_void) -> SlangInt32,
	pub getDefinedEntryPoint: unsafe extern "stdcall" fn(*mut c_void, index: SlangInt32, outEntryPoint: *mut *mut slang_IEntryPoint) -> SlangResult,
	pub serialize: unsafe extern "stdcall" fn(*mut c_void, outSerializedBlob: *mut *mut ISlangBlob) -> SlangResult,
	pub writeToFile: unsafe extern "stdcall" fn(*mut c_void, fileName: *const c_char) -> SlangResult,
	pub getName: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	pub getFilePath: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	pub getUniqueIdentity: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
}

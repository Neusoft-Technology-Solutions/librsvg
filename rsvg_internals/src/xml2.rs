// This is a hand-written binding to the very minimal part of libxml2 that we need.

#![allow(non_snake_case, non_camel_case_types)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use glib_sys::gpointer;
use libc;

pub const XML_CHAR_ENCODING_NONE: libc::c_int = 0;

pub const XML_INTERNAL_GENERAL_ENTITY: libc::c_int = 1;

pub const XML_PARSE_NONET:     libc::c_int = 1 << 11;
pub const XML_PARSE_HUGE:      libc::c_int = 1 << 19;
pub const XML_PARSE_BIG_LINES: libc::c_int = 1 << 22;

pub type xmlDocPtr = gpointer;

pub type xmlEntityPtr = gpointer;

#[repr(C)]
pub struct xmlSAXHandler {
    pub internalSubset: gpointer,
    pub isStandalone: gpointer,
    pub hasInternalSubset: gpointer,
    pub hasExternalSubset: gpointer,
    pub resolveEntity: gpointer,
    pub getEntity: Option<unsafe extern "C" fn(
        ctx: *mut libc::c_void,
        name: *const libc::c_char,
    ) -> xmlEntityPtr>,

    pub entityDecl: Option<unsafe extern "C" fn(
        ctx: *mut libc::c_void,
        name: *const libc::c_char,
        type_: libc::c_int,
        public_id: *const libc::c_char,
        system_id: *const libc::c_char,
        content: *const libc::c_char,
    )>,

    pub notationDecl: gpointer,
    pub attributeDecl: gpointer,
    pub elementDecl: gpointer,

    pub unparsedEntityDecl: Option<unsafe extern "C" fn(
        ctx: *mut libc::c_void,
        name: *const libc::c_char,
        public_id: *const libc::c_char,
        system_id: *const libc::c_char,
        notation_name: *const libc::c_char,
    )>,

    pub setDocumentLocator: gpointer,
    pub startDocument: gpointer,
    pub endDocument: gpointer,

    pub startElement: Option<unsafe extern "C" fn(
        ctx: *mut libc::c_void,
        name: *const libc::c_char,
        atts: *const *const libc::c_char,
    )>,

    pub endElement: Option<unsafe extern "C" fn(
        ctx: *mut libc::c_void,
        name: *const libc::c_char,
    )>,

    pub reference: gpointer,

    pub characters: Option<unsafe extern "C" fn(
        ctx: *mut libc::c_void,
        ch: *const libc::c_char,
        len: libc::c_int,
    )>,

    pub ignorableWhitespace: gpointer,

    pub processingInstruction: Option<unsafe extern "C" fn(
        ctx: *mut libc::c_void,
        target: *const libc::c_char,
        data: *const libc::c_char,
    )>,

    pub comment: gpointer,
    pub warning: gpointer,

    pub error: gpointer,

    pub fatalError: gpointer,

    pub getParameterEntity: Option<unsafe extern "C" fn(
        ctx: *mut libc::c_void,
        name: *const libc::c_char,
    ) -> xmlEntityPtr>,

    pub cdataBlock: Option<unsafe extern "C" fn(
        ctx: *mut libc::c_void,
        value: *const libc::c_char,
        len: libc::c_int,
    )>,

    pub externalSubset: gpointer,

    pub initialized: libc::c_uint,

    pub _private: gpointer,
    pub startElementNs: gpointer,
    pub endElementNs: gpointer,
    pub serror: Option<unsafe extern "C" fn(user_data: *mut libc::c_void, error: xmlErrorPtr)>,
}

pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;

// The original struct _xmlParserCtxt in libxml2 has a *ton* of
// fields; mostly are implementation details.  We only require access
// to fields up to replaceEntities, so we'll represent up to that
// field, and ignore subsequent ones.  This struct is used just to
// cast the xmlParserCtxtPtr that we get out of libxml2 into a
// Rust-visible structure; we don't need a complete representation of the
// original struct.
#[repr(C)]
pub struct xmlParserCtxt {
    pub sax: gpointer,
    pub userData: gpointer,
    pub myDoc: xmlDocPtr,
    pub wellFormed: libc::c_int,
    pub replaceEntities: libc::c_int,
    // ... libxml2 has more fields here; we don't use them
}

pub type xmlParserCtxtPtr = *mut xmlParserCtxt;

#[repr(C)]
pub struct xmlError {
    pub domain: libc::c_int,
    pub code: libc::c_int,
    pub message: *const libc::c_char,
    pub level: libc::c_int,
    pub file: *const libc::c_char,
    pub line: libc::c_int,
    pub str1: *const libc::c_char,
    pub str2: *const libc::c_char,
    pub str3: *const libc::c_char,
    pub int1: libc::c_int,
    pub int2: libc::c_int,
    pub ctxt: gpointer,
    pub node: gpointer,
}

pub type xmlErrorPtr = *mut xmlError;

pub type xmlInputReadCallback = Option<unsafe extern "C" fn(
    context: *mut libc::c_void,
    buffer: *mut libc::c_char,
    len: libc::c_int,
) -> libc::c_int>;

pub type xmlInputCloseCallback = Option<unsafe extern "C" fn(
    context: *mut libc::c_void,
) -> libc::c_int>;

pub type xmlCharEncoding = libc::c_int;

extern "C" {
    pub fn xmlInitParser();

    pub fn xmlCreateIOParserCtxt(
        sax: xmlSAXHandlerPtr,
        user_data: *mut libc::c_void,
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut libc::c_void,
        enc: xmlCharEncoding,
    ) -> xmlParserCtxtPtr;

    pub fn xmlParseDocument(ctxt: xmlParserCtxtPtr) -> libc::c_int;

    pub fn xmlFreeDoc(doc: xmlDocPtr);

    pub fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);

    pub fn xmlCtxtGetLastError(ctxt: *mut libc::c_void) -> xmlErrorPtr;

    pub fn xmlCtxtUseOptions(
        ctxt: xmlParserCtxtPtr,
        options: libc::c_int,
    ) -> libc::c_int;

    pub fn xmlNewEntity(
        doc: xmlDocPtr,
        name: *const libc::c_char,
        type_: libc::c_int,
        external_id: *const libc::c_char,
        system_id: *const libc::c_char,
        content: *const libc::c_char,
    ) -> xmlEntityPtr;
}

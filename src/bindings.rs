/* automatically generated by rust-bindgen 0.71.1 */

pub type va_list = __builtin_va_list;
#[repr(u32)]
#[non_exhaustive]
#[doc = " Android log priority values, in increasing order of priority."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum android_LogPriority {
    #[doc = " For internal use only."]
    UNKNOWN = 0,
    #[doc = " The default priority, for internal use only."]
    DEFAULT = 1,
    #[doc = " Verbose logging. Should typically be disabled for a release apk."]
    VERBOSE = 2,
    #[doc = " Debug logging. Should typically be disabled for a release apk."]
    DEBUG = 3,
    #[doc = " Informational logging. Should typically be disabled for a release apk."]
    INFO = 4,
    #[doc = " Warning logging. For use with recoverable failures."]
    WARN = 5,
    #[doc = " Error logging. For use with unrecoverable failures."]
    ERROR = 6,
    #[doc = " Fatal logging. For use when aborting."]
    FATAL = 7,
    #[doc = " For internal use only."]
    SILENT = 8,
}
extern "C" {
    #[doc = " Writes the constant string `text` to the log, with priority `prio` and tag\n `tag`.\n\n @return 1 if the message was written to the log, or -EPERM if it was not (see\n __android_log_is_loggable)."]
    pub fn __android_log_write(
        prio: ::std::os::raw::c_int,
        tag: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Writes a formatted string to the log, with priority `prio` and tag `tag`.\n The details of formatting are the same as for\n [printf(3)](http://man7.org/linux/man-pages/man3/printf.3.html).\n\n @return 1 if the message was written to the log, or -EPERM if it was not (see\n __android_log_is_loggable)."]
    pub fn __android_log_print(
        prio: ::std::os::raw::c_int,
        tag: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Equivalent to `__android_log_print`, but taking a `va_list`.\n (If `__android_log_print` is like `printf`, this is like `vprintf`.)\n\n @return 1 if the message was written to the log, or -EPERM if it was not (see\n __android_log_is_loggable)."]
    pub fn __android_log_vprint(
        prio: ::std::os::raw::c_int,
        tag: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        ap: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Writes an assertion failure to the log (as `ANDROID_LOG_FATAL`) and to\n stderr, before calling\n [abort(3)](http://man7.org/linux/man-pages/man3/abort.3.html).\n\n If `fmt` is non-null, `cond` is unused. If `fmt` is null, the string\n `Assertion failed: %s` is used with `cond` as the string argument.\n If both `fmt` and `cond` are null, a default string is provided.\n\n Most callers should use\n [assert(3)](http://man7.org/linux/man-pages/man3/assert.3.html) from\n `&lt;assert.h&gt;` instead, or the `__assert` and `__assert2` functions\n provided by bionic if more control is needed. They support automatically\n including the source filename and line number more conveniently than this\n function."]
    pub fn __android_log_assert(
        cond: *const ::std::os::raw::c_char,
        tag: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> !;
}
#[repr(u32)]
#[non_exhaustive]
#[doc = " Identifies a specific log buffer for __android_log_buf_write()\n and __android_log_buf_print()."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum log_id {
    #[doc = " The main log buffer. This is the only log buffer available to apps."]
    MAIN = 0,
    #[doc = " The radio log buffer."]
    RADIO = 1,
    #[doc = " The event log buffer."]
    EVENTS = 2,
    #[doc = " The system log buffer."]
    SYSTEM = 3,
    #[doc = " The crash log buffer."]
    CRASH = 4,
    #[doc = " The statistics log buffer."]
    STATS = 5,
    #[doc = " The security log buffer."]
    SECURITY = 6,
    #[doc = " The kernel log buffer."]
    KERNEL = 7,
    #[doc = " The kernel log buffer."]
    MAX = 8,
    #[doc = " Let the logging function choose the best log target."]
    DEFAULT = 2147483647,
}
#[doc = " Identifies a specific log buffer for __android_log_buf_write()\n and __android_log_buf_print()."]
pub use self::log_id as log_id_t;
extern "C" {
    #[doc = " Writes the constant string `text` to the log buffer `id`,\n with priority `prio` and tag `tag`.\n\n Apps should use __android_log_write() instead.\n\n @return 1 if the message was written to the log, or -EPERM if it was not (see\n __android_log_is_loggable)."]
    pub fn __android_log_buf_write(
        bufID: ::std::os::raw::c_int,
        prio: ::std::os::raw::c_int,
        tag: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Writes a formatted string to log buffer `id`,\n with priority `prio` and tag `tag`.\n The details of formatting are the same as for\n [printf(3)](http://man7.org/linux/man-pages/man3/printf.3.html).\n\n Apps should use __android_log_print() instead.\n\n @return 1 if the message was written to the log, or -EPERM if it was not (see\n __android_log_is_loggable)."]
    pub fn __android_log_buf_print(
        bufID: ::std::os::raw::c_int,
        prio: ::std::os::raw::c_int,
        tag: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
#[doc = " Logger data struct used for writing log messages to liblog via __android_log_write_logger_data()\n and sending log messages to user defined loggers specified in __android_log_set_logger()."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __android_log_message {
    #[doc = " Must be set to sizeof(__android_log_message) and is used for versioning."]
    pub struct_size: usize,
    #[doc = " {@link log_id_t} values."]
    pub buffer_id: i32,
    #[doc = " {@link android_LogPriority} values."]
    pub priority: i32,
    #[doc = " The tag for the log message."]
    pub tag: *const ::std::os::raw::c_char,
    #[doc = " Optional file name, may be set to nullptr."]
    pub file: *const ::std::os::raw::c_char,
    #[doc = " Optional line number, ignore if file is nullptr."]
    pub line: u32,
    #[doc = " The log message itself."]
    pub message: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout___android_log_message() {
    const UNINIT: ::std::mem::MaybeUninit<__android_log_message> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__android_log_message>(),
        48usize,
        "Size of __android_log_message"
    );
    assert_eq!(
        ::std::mem::align_of::<__android_log_message>(),
        8usize,
        "Alignment of __android_log_message"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).struct_size) as usize - ptr as usize },
        0usize,
        "Offset of field: __android_log_message::struct_size"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer_id) as usize - ptr as usize },
        8usize,
        "Offset of field: __android_log_message::buffer_id"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).priority) as usize - ptr as usize },
        12usize,
        "Offset of field: __android_log_message::priority"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tag) as usize - ptr as usize },
        16usize,
        "Offset of field: __android_log_message::tag"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).file) as usize - ptr as usize },
        24usize,
        "Offset of field: __android_log_message::file"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).line) as usize - ptr as usize },
        32usize,
        "Offset of field: __android_log_message::line"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).message) as usize - ptr as usize },
        40usize,
        "Offset of field: __android_log_message::message"
    );
}
#[doc = " Prototype for the 'logger' function that is called for every log message."]
pub type __android_logger_function =
    ::std::option::Option<unsafe extern "C" fn(log_message: *const __android_log_message)>;
#[doc = " Prototype for the 'abort' function that is called when liblog will abort due to\n __android_log_assert() failures."]
pub type __android_aborter_function =
    ::std::option::Option<unsafe extern "C" fn(abort_message: *const ::std::os::raw::c_char)>;
extern "C" {
    #[doc = " Writes the log message specified by log_message.  log_message includes additional file name and\n line number information that a logger may use.  log_message is versioned for backwards\n compatibility.\n This assumes that loggability has already been checked through __android_log_is_loggable().\n Higher level logging libraries, such as libbase, first check loggability, then format their\n buffers, then pass the message to liblog via this function, and therefore we do not want to\n duplicate the loggability check here.\n\n @param log_message the log message itself, see __android_log_message.\n\n Available since API level 30."]
    pub fn __android_log_write_log_message(log_message: *mut __android_log_message);
}
extern "C" {
    #[doc = " Sets a user defined logger function.  All log messages sent to liblog will be set to the\n function pointer specified by logger for processing.  It is not expected that log messages are\n already terminated with a new line.  This function should add new lines if required for line\n separation.\n\n @param logger the new function that will handle log messages.\n\n Available since API level 30."]
    pub fn __android_log_set_logger(logger: __android_logger_function);
}
extern "C" {
    #[doc = " Writes the log message to logd.  This is an __android_logger_function and can be provided to\n __android_log_set_logger().  It is the default logger when running liblog on a device.\n\n @param log_message the log message to write, see __android_log_message.\n\n Available since API level 30."]
    pub fn __android_log_logd_logger(log_message: *const __android_log_message);
}
extern "C" {
    #[doc = " Writes the log message to stderr.  This is an __android_logger_function and can be provided to\n __android_log_set_logger().  It is the default logger when running liblog on host.\n\n @param log_message the log message to write, see __android_log_message.\n\n Available since API level 30."]
    pub fn __android_log_stderr_logger(log_message: *const __android_log_message);
}
extern "C" {
    #[doc = " Sets a user defined aborter function that is called for __android_log_assert() failures.  This\n user defined aborter function is highly recommended to abort and be noreturn, but is not strictly\n required to.\n\n @param aborter the new aborter function, see __android_aborter_function.\n\n Available since API level 30."]
    pub fn __android_log_set_aborter(aborter: __android_aborter_function);
}
extern "C" {
    #[doc = " Calls the stored aborter function.  This allows for other logging libraries to use the same\n aborter function by calling this function in liblog.\n\n @param abort_message an additional message supplied when aborting, for example this is used to\n                      call android_set_abort_message() in __android_log_default_aborter().\n\n Available since API level 30."]
    pub fn __android_log_call_aborter(abort_message: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Sets android_set_abort_message() on device then aborts().  This is the default aborter.\n\n @param abort_message an additional message supplied when aborting.  This functions calls\n                      android_set_abort_message() with its contents.\n\n Available since API level 30."]
    pub fn __android_log_default_aborter(abort_message: *const ::std::os::raw::c_char) -> !;
}
extern "C" {
    #[doc = " Use the per-tag properties \"log.tag.<tagname>\" along with the minimum priority from\n __android_log_set_minimum_priority() to determine if a log message with a given prio and tag will\n be printed.  A non-zero result indicates yes, zero indicates false.\n\n If both a priority for a tag and a minimum priority are set by\n __android_log_set_minimum_priority(), then the lowest of the two values are to determine the\n minimum priority needed to log.  If only one is set, then that value is used to determine the\n minimum priority needed.  If none are set, then default_priority is used.\n\n @param prio         the priority to test, takes android_LogPriority values.\n @param tag          the tag to test.\n @param default_prio the default priority to use if no properties or minimum priority are set.\n @return an integer where 1 indicates that the message is loggable and 0 indicates that it is not.\n\n Available since API level 30."]
    pub fn __android_log_is_loggable(
        prio: ::std::os::raw::c_int,
        tag: *const ::std::os::raw::c_char,
        default_prio: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Use the per-tag properties \"log.tag.<tagname>\" along with the minimum priority from\n __android_log_set_minimum_priority() to determine if a log message with a given prio and tag will\n be printed.  A non-zero result indicates yes, zero indicates false.\n\n If both a priority for a tag and a minimum priority are set by\n __android_log_set_minimum_priority(), then the lowest of the two values are to determine the\n minimum priority needed to log.  If only one is set, then that value is used to determine the\n minimum priority needed.  If none are set, then default_priority is used.\n\n @param prio         the priority to test, takes android_LogPriority values.\n @param tag          the tag to test.\n @param len          the length of the tag.\n @param default_prio the default priority to use if no properties or minimum priority are set.\n @return an integer where 1 indicates that the message is loggable and 0 indicates that it is not.\n\n Available since API level 30."]
    pub fn __android_log_is_loggable_len(
        prio: ::std::os::raw::c_int,
        tag: *const ::std::os::raw::c_char,
        len: usize,
        default_prio: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Sets the minimum priority that will be logged for this process.\n\n @param priority the new minimum priority to set, takes android_LogPriority values.\n @return the previous set minimum priority as android_LogPriority values, or\n         ANDROID_LOG_DEFAULT if none was set.\n\n Available since API level 30."]
    pub fn __android_log_set_minimum_priority(priority: i32) -> i32;
}
extern "C" {
    #[doc = " Gets the minimum priority that will be logged for this process.  If none has been set by a\n previous __android_log_set_minimum_priority() call, this returns ANDROID_LOG_DEFAULT.\n\n @return the current minimum priority as android_LogPriority values, or\n         ANDROID_LOG_DEFAULT if none is set.\n\n Available since API level 30."]
    pub fn __android_log_get_minimum_priority() -> i32;
}
extern "C" {
    #[doc = " Sets the default tag if no tag is provided when writing a log message.  Defaults to\n getprogname().  This truncates tag to the maximum log message size, though appropriate tags\n should be much smaller.\n\n @param tag the new log tag.\n\n Available since API level 30."]
    pub fn __android_log_set_default_tag(tag: *const ::std::os::raw::c_char);
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    const UNINIT: ::std::mem::MaybeUninit<__va_list_tag> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        "Size of __va_list_tag"
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        "Alignment of __va_list_tag"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gp_offset) as usize - ptr as usize },
        0usize,
        "Offset of field: __va_list_tag::gp_offset"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fp_offset) as usize - ptr as usize },
        4usize,
        "Offset of field: __va_list_tag::fp_offset"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).overflow_arg_area) as usize - ptr as usize },
        8usize,
        "Offset of field: __va_list_tag::overflow_arg_area"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reg_save_area) as usize - ptr as usize },
        16usize,
        "Offset of field: __va_list_tag::reg_save_area"
    );
}

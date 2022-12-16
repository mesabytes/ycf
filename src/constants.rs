// this is base64 of "root-section" the root of the file
//
// Just to make it harder to conflict with user specified sections
pub const ROOT_SECTION: &str = "cm9vdC1zZWN0aW9uCg==";

pub const SECTION_PREFIX: &str = "@";
pub const SECTION_END: &str = "}";
pub const KEY_VALUE_SEP: &str = "=";
pub const COMMENT_PREFIX: &str = ";;";

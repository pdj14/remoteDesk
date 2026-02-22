const COMMON_CSS: &str = include_str!("common.css");
const COMMON_TIS: &str = include_str!("common.tis");

const INDEX_HTML: &str = include_str!("index.html");
const INDEX_CSS: &str = include_str!("index.css");
const INDEX_TIS: &str = include_str!("index.tis");
const MSGBOX_TIS: &str = include_str!("msgbox.tis");
const AB_TIS: &str = include_str!("ab.tis");

const REMOTE_HTML: &str = include_str!("remote.html");
const REMOTE_CSS: &str = include_str!("remote.css");
const HEADER_CSS: &str = include_str!("header.css");
const FILE_TRANSFER_CSS: &str = include_str!("file_transfer.css");
const REMOTE_TIS: &str = include_str!("remote.tis");
const FILE_TRANSFER_TIS: &str = include_str!("file_transfer.tis");
const PORT_FORWARD_TIS: &str = include_str!("port_forward.tis");
const GRID_TIS: &str = include_str!("grid.tis");
const HEADER_TIS: &str = include_str!("header.tis");
const PRINTER_TIS: &str = include_str!("printer.tis");

const CHATBOX_HTML: &str = include_str!("chatbox.html");

const INSTALL_HTML: &str = include_str!("install.html");
const INSTALL_TIS: &str = include_str!("install.tis");

const CM_HTML: &str = include_str!("cm.html");
const CM_CSS: &str = include_str!("cm.css");
const CM_TIS: &str = include_str!("cm.tis");

fn inline_common(document: &str) -> String {
    document
        .replace("@import url(common.css);", COMMON_CSS)
        .replace("include \"common.tis\";", COMMON_TIS)
}

#[inline]
pub fn get_index() -> String {
    let index = INDEX_HTML
        .replace("@import url(index.css);", INDEX_CSS)
        .replace("include \"index.tis\";", INDEX_TIS)
        .replace("include \"msgbox.tis\";", MSGBOX_TIS)
        .replace("include \"ab.tis\";", AB_TIS);
    inline_common(&index)
}

#[inline]
pub fn get_remote() -> String {
    let remote = REMOTE_HTML
        .replace("@import url(remote.css);", REMOTE_CSS)
        .replace("@import url(file_transfer.css);", FILE_TRANSFER_CSS)
        .replace("@import url(header.css);", HEADER_CSS)
        .replace("include \"remote.tis\";", REMOTE_TIS)
        .replace("include \"msgbox.tis\";", MSGBOX_TIS)
        .replace("include \"file_transfer.tis\";", FILE_TRANSFER_TIS)
        .replace("include \"port_forward.tis\";", PORT_FORWARD_TIS)
        .replace("include \"grid.tis\";", GRID_TIS)
        .replace("include \"header.tis\";", HEADER_TIS)
        .replace("include \"printer.tis\";", PRINTER_TIS);
    inline_common(&remote)
}

#[inline]
pub fn get_chatbox() -> String {
    inline_common(CHATBOX_HTML)
}

#[inline]
pub fn get_install() -> String {
    let install = INSTALL_HTML.replace("include \"install.tis\";", INSTALL_TIS);
    inline_common(&install)
}

#[inline]
pub fn get_cm() -> String {
    let cm = CM_HTML
        .replace("@import url(cm.css);", CM_CSS)
        .replace("include \"cm.tis\";", CM_TIS);
    inline_common(&cm)
}

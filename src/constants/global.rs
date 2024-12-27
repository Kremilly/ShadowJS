pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
    pub const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
    pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const APP_LICENSE: &'static str = env!("CARGO_PKG_LICENSE");
    pub const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
    
}

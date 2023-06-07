#[derive(Clone)]
pub struct PluginInfo {
    pub(crate) name: String,
    pub(crate) subscribe_tags: Vec<String>,
}

pub trait Message {
    fn tag(&self) -> &str;
    fn raw_data(&self) -> &[u8];
}

pub trait Plugin {
    fn initialize() -> PluginInfo;
    fn process_message<M, R>(&self, message: &M) -> Option<R>
    where
        M: Message,
        R: Message;
    fn system_idle<M>(&self) -> Option<M>
    where
        M: Message;
}

use crate::Resource;

pub struct BertRepository {
    pub config: Box<dyn Resource>,
    pub model: Box<dyn Resource>,
    pub vocab: Box<dyn Resource>,
}

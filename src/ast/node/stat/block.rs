use super::super::AstStat;

pub struct StatBlock {
    body: Vec<Box<AstStat>>,
}

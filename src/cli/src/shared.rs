use std::sync::Arc;

use crate::arguments::Arguments;
use node_core::prisma::PrismaClient;

#[derive(Clone, Debug)]
pub struct Shared {
    pub argument: Arguments,
    pub prisma: Arc<PrismaClient>,
}

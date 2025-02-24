use std::collections::HashMap;

use uuid::Uuid;

use super::users::Username;

#[derive(Default, Clone)]
pub struct Sessions(HashMap<Uuid, Username>);

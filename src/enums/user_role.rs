pub enum UserRole {
    ADMIN,
    VISITOR,
}

impl From<UserRole> for &'static str {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::ADMIN => "ADMIN",
            UserRole::VISITOR => "VISITOR",
        }
    }
}

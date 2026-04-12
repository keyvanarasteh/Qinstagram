pub enum GraphQLSubscription {
    AppPresence,
    ZeroProvision { phone_id: String },
    DirectStatus,
    DirectTyping { user_id: String },
    AsyncAd { user_id: String },
}

pub enum SkywalkerSubscription {
    Direct { user_id: String },
    Live { user_id: String },
}

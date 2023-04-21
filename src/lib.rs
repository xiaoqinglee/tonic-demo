#[path = "google"] //相对路径
pub mod google {
    #[path = ""]
    pub mod pubsub {
        #[path = "google.pubsub.v1.rs"]
        pub mod v1;
    }
}
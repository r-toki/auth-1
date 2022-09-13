use derive_new::new;

#[derive(new, Debug)]
pub struct Auth {
    pub user_id: String,
}

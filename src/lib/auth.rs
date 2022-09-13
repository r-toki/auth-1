use derive_new::new;

#[derive(new, Debug)]
pub struct Auth {
    user_id: String,
}

use argon2::{
    password_hash::{
        rand_core::OsRng, Encoding, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

pub fn hash_password(clear_pass: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let password_hashed = argon
        .hash_password(clear_pass.to_string().as_bytes(), &salt)
        .unwrap()
        .to_string();

    password_hashed
}

pub fn verify_password(clear_pass: String, hashed_pass: String) -> bool {
    let argon2 = Argon2::default();
    let hashed_pass_str: &str = &*hashed_pass;
    argon2
        .verify_password(
            clear_pass.as_bytes(),
            &PasswordHash::parse(hashed_pass_str, Encoding::default()).unwrap(),
        )
        .is_ok()
}

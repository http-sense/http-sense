
use serde::Deserialize;
use crate::config::{SUPABASE_PROJECT_URL, SUPABASE_ANON_KEY};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

macro_rules! to_session {
	($old_session:ident) => {
		{
			let res = $old_session;
			Session {
				access_token: res.access_token,
				token_type: res.token_type,
				expires_in: res.expires_in,
				refresh_token: res.refresh_token,
				user: User {
					id: res.user.id,
					email: res.user.email,
					aud: res.user.aud,
					role: res.user.role,
					email_confirmed_at: res.user.email_confirmed_at,
					phone: res.user.phone,
					last_sign_in_at: res.user.last_sign_in_at,
					created_at: res.user.created_at,
					updated_at: res.user.updated_at
				}
			}
		}
	};
}

pub fn get_random_string(len: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

	rand_string
}
// Taken from go_true lib code, they don't export it
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub aud: String,
    pub role: String,
    pub email_confirmed_at: Option<String>,
    pub phone: String,
    pub last_sign_in_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Session {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub user: User,
}

// #[derive(Debug)]
pub struct AuthenticatedUser {
	pub api: go_true::Api,
	pub session: Session,
	pub password: String,
	pub email: String,
	pub session_refreshed_at: chrono::DateTime<chrono::Utc>
}

impl std::fmt::Debug for AuthenticatedUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		#[derive(Debug)]
		struct Holder {
			session: Session,
			password: String,
			email: String
		}
		Holder {
			session: self.session.clone(),
			password: self.password.clone(),
			email: self.email.clone()
		}.fmt(f)
    }
}

impl AuthenticatedUser {
	pub fn uid(&self) -> String {
		self.session.user.id.clone()
	}
	pub async fn maybe_refresh(&mut self) -> anyhow::Result<bool> {
		if self.should_refresh() {
			self.refresh_token().await?;
			Ok(true)
		} else {
			Ok(false)
		}
	}
	pub fn should_refresh(&self) -> bool {
		let refresh_cliff = self.session_refreshed_at.timestamp() + self.session.expires_in as i64/2;
		refresh_cliff < chrono::Utc::now().timestamp()
	}
	async fn refresh_token(&mut self) -> anyhow::Result<()> {
		let new_session = self.api.refresh_access_token(&self.session.refresh_token).await?;
		self.session = to_session!(new_session);
		self.session_refreshed_at = chrono::Utc::now();
		Ok(())
	}
}

pub async fn create_user() -> anyhow::Result<AuthenticatedUser> {
	let auth_endpoint = format!("{}/auth/v1", SUPABASE_PROJECT_URL);
	let api = go_true::Api::new(auth_endpoint);
	let api = api.insert_header("apikey", SUPABASE_ANON_KEY);

	let random_id = get_random_string(30);
	let password = get_random_string(30);

    let email = format!("{}@example.com", random_id);
	let em = go_true::EmailOrPhone::Email(email.clone());

	let res = api.sign_up(em, &password).await?;
	let session = to_session!(res);
	Ok(AuthenticatedUser {
		api,
		session,
		password,
		email,
		session_refreshed_at: chrono::Utc::now()
	})

}


#[ignore]
#[tokio::test]
async fn just_checking() {
	let mut user = create_user().await.unwrap();
	dbg!(&user);

	user.refresh_token().await.unwrap();

	dbg!(&user);

	assert!(false);
}
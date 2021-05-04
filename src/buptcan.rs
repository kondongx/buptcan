use std::fmt::{self, Display, Formatter};

use dialoguer::{theme::ColorfulTheme, Input, Password, Select};

#[derive(Debug)]
pub struct Account {
    pub user: String,
    pub password: String,
}

pub enum LoginStatus {
    Logged,
    NotLogged,
}

#[derive(Debug)]
pub enum LoginError {
    AccountError,
    NetworkError,
}

impl std::error::Error for LoginError {}

impl Display for LoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LoginError::AccountError => write!(f, ">>> Incorrect username or password <<<"),
            LoginError::NetworkError => write!(f, ">>> Cannot access BUPT campus network <<<"),
        }
    }
}

impl From<ureq::Error> for LoginError {
    fn from(_: ureq::Error) -> Self {
        LoginError::NetworkError
    }
}

pub fn login() -> Result<LoginStatus, LoginError> {
    // crate `ctrlc` cannot handle ctrlc in blocking read
    let account = match get_account() {
        Ok(account) => account,
        Err(_) => std::process::exit(1),
    };

    let res = ureq::post("http://10.3.8.211/login")
        .send_form(&[("user", &account.user), ("pass", &account.password)])?;

    if res
        .into_string()
        .map_err(|_| LoginError::NetworkError)?
        .contains("登录成功")
    {
        Ok(LoginStatus::Logged)
    } else {
        Err(LoginError::AccountError)
    }
}

pub fn logout() -> Result<LoginStatus, LoginError> {
    ureq::get("http://10.3.8.211/logout").call()?;
    Ok(LoginStatus::NotLogged)
}

pub fn select_command() {
    if check_network() == NetworkStatus::CannotAccess {
        println!("{}", LoginError::NetworkError);
        std::process::exit(1);
    }

    let selections = &["login", "logout"];

    let theme = ColorfulTheme::default();

    // crate `ctrlc` cannot handle ctrlc in blocking read
    let selection = match Select::with_theme(&theme)
        .with_prompt("Bupt Campus Network")
        .default(0)
        .items(&selections[..])
        .interact()
    {
        Ok(selection) => selection,
        Err(_) => std::process::exit(1),
    };

    let res = match selection {
        0 => login(),
        1 => logout(),
        _ => Ok(LoginStatus::NotLogged),
    };

    match res {
        Ok(_) => println!("{} successful", selections[selection].to_uppercase()),
        Err(e) => match e {
            LoginError::AccountError => println!("{}", LoginError::AccountError),
            LoginError::NetworkError => println!("{}", LoginError::NetworkError),
        },
    };
}

pub fn get_account() -> Result<Account, std::io::Error> {
    let theme = ColorfulTheme::default();
    let user: String = Input::with_theme(&theme)
        .with_prompt("Student ID")
        .validate_with(|input: &String| {
            if input.len() == 10 {
                Ok(())
            } else {
                Err("ID length not 10")
            }
        })
        .interact_text()?;

    let password: String = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()?;

    Ok(Account { user, password })
}

#[derive(PartialEq)]
pub enum NetworkStatus {
    CanAccess,
    CannotAccess,
}

pub fn check_network() -> NetworkStatus {
    match ureq::get("http://10.3.8.211/index").call() {
        Ok(_) => NetworkStatus::CanAccess,
        Err(_) => NetworkStatus::CannotAccess,
    }
}

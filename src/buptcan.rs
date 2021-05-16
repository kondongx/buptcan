use std::{
    fmt::{self, Display, Formatter},
    process,
};

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};

use ureq::Response;

use crate::configure::{get_configuration, update_configuration};

#[derive(Debug, PartialEq)]
pub struct Account {
    pub user: String,
    pub password: String,
}

#[derive(PartialEq)]
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

pub fn login(account: &Account) -> Result<LoginStatus, LoginError> {
    // crate `ctrlc` cannot handle ctrlc in blocking read

    let res = ureq::post("http://10.3.8.211/login")
        .send_form(&[("user", &account.user), ("pass", &account.password)])?;

    get_login_info(res)
}

pub fn login_command(account: &Account) {
    match login(account) {
        Ok(_) => println!("Login successful"),
        Err(error) => println!("{}", error),
    }
}

pub fn login_with_new_account() {
    let new_account = get_account_command();

    match login(&new_account) {
        Ok(LoginStatus::Logged) => {
            println!("Login successful");
            ask_update_stored_account(&new_account);
        }
        Ok(LoginStatus::NotLogged) => {}
        Err(error) => match error {
            LoginError::AccountError => {
                println!("{}", error);
                login_with_new_account()
            }
            LoginError::NetworkError => {
                println!("{}", error)
            }
        },
    }
}

pub fn logout() -> Result<LoginStatus, LoginError> {
    ureq::get("http://10.3.8.211/logout").call()?;
    Ok(LoginStatus::NotLogged)
}

pub fn logout_command() {
    match logout() {
        Ok(_) => println!("Logout successful"),
        Err(error) => println!("{}", error),
    };
}

pub fn select_command() {
    if LoginStatus::Logged == check_login_status() {
        println!("You have already logged in.\nUse `buptcan o` to logout");
        return;
    }

    let mut selections = Vec::new();
    let stored_accounts = get_stored_account();
    for account in &stored_accounts {
        selections.push(account.user.as_str());
    }
    selections.push("login with new account");

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

    let is_login_with_new_account = stored_accounts.len() == 0 || selection == selections.len() - 1;
    if is_login_with_new_account {
        login_with_new_account();
    } else {
        login_command(&stored_accounts[selection]);
    }
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

pub fn get_account_command() -> Account {
    match get_account() {
        Ok(account) => account,
        Err(_) => std::process::exit(1),
    }
}

pub fn ask_update_stored_account(account: &Account) {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to save account?")
        .default(true)
        .interact()
        .unwrap()
    {
        update_stored_account(account);
    }
}

pub fn update_stored_account(new_account: &Account) {
    update_configuration(
        (new_account.user.clone() + ".password").as_str(),
        new_account.password.as_str(),
    )
}

pub fn get_stored_account() -> Vec<Account> {
    let config = get_configuration();
    let config = config.as_table().unwrap();
    let mut accounts = Vec::<Account>::new();
    for (key, value) in config {
        accounts.push(Account {
            user: key.to_owned(),
            password: value
                .as_table()
                .unwrap()
                .get("password")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        })
    }
    accounts
}

pub fn check_login_status() -> LoginStatus {
    let res = match ureq::get("http://10.3.8.211/login").call() {
        Ok(res) => res,
        Err(_) => {
            println!("{}", LoginError::NetworkError);
            process::exit(1);
        }
    };
    get_login_info(res).unwrap()
}

fn get_login_info(res: Response) -> Result<LoginStatus, LoginError> {
    let res_str = res.into_string().map_err(|_| LoginError::NetworkError)?;

    if res_str.contains("登录成功") {
        Ok(LoginStatus::Logged)
    } else if res_str.contains("密码错误") {
        Err(LoginError::AccountError)
    } else {
        Ok(LoginStatus::NotLogged)
    }
}

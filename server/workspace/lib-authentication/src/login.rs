use crate::token::Token;
use crate::Result;

pub struct Login<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

pub fn login(login: &Login) -> Result<Option<Token>> {
    if login.username != "test" || login.password != "test" {
        return Ok(None);
    }

    let token = Token::generate(32)?;
    Ok(Some(token))
}

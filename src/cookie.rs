use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Cookie {
    pub host: String,
    pub path: String,
    pub name: String,
    pub value: String,
    pub expiry: i64,
    pub is_secure: bool,
    pub is_http_only: bool,
    pub same_site: CookieSameSite,
}

impl Display for Cookie {
    /// cookies.txt の行を出力する。改行は含まない。
    ///
    /// 雑な部分があるが、youtube-dl で使う分には問題ないはず。
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use std::fmt::Write;

        // domain
        if self.is_http_only {
            f.write_str("#HttpOnly_")?;
        }
        f.write_str(&self.host)?;
        f.write_char('\t')?;

        // hostOnly
        // DB 内には格納されてないかも?
        // host の先頭が '.' でない場合、false にしないと youtube-dl がこける。
        // それ以外の場合はとりあえず true にしておく。
        f.write_str(str_bool(self.host.starts_with('.')))?;
        f.write_char('\t')?;

        // path
        f.write_str(&self.path)?;
        f.write_char('\t')?;

        // secure
        f.write_str(str_bool(self.is_secure))?;
        f.write_char('\t')?;

        // expirationDate
        // session cookie かどうかはDB内には格納されてないかも?
        // とりあえずDB内の値を出力しておく。
        write!(f, "{}", self.expiry)?;
        f.write_char('\t')?;

        // name
        f.write_str(&self.name)?;
        f.write_char('\t')?;

        // value
        f.write_str(&self.value)?;

        Ok(())
    }
}

fn str_bool(x: bool) -> &'static str {
    if x {
        "TRUE"
    } else {
        "FALSE"
    }
}

#[derive(Debug)]
pub enum CookieSameSite {
    None,
    Lax,
    Strict,
}

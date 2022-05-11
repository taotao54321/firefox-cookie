// ref:
//
//   * [nsICookie.idl - mozsearch](https://searchfox.org/mozilla-central/rev/88792eff309001778cb2431f2a0ed92f8f3c258a/netwerk/cookie/nsICookie.idl)

use std::path::Path;

use anyhow::{bail, Context};
use rusqlite::{Connection, OpenFlags, Row};

use crate::cookie::{Cookie, CookieSameSite};

/// Firefox のクッキー DB を読み取り、`Cookie` の配列を返す。
pub fn read_cookies_db(path: impl AsRef<Path>) -> anyhow::Result<Vec<Cookie>> {
    _read_cookies_db(path.as_ref())
}

fn _read_cookies_db(path: &Path) -> anyhow::Result<Vec<Cookie>> {
    let uri = make_uri(path)?;

    let conn = Connection::open_with_flags(
        uri,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
    )?;

    let mut stmt = conn.prepare(
        r#"
SELECT
    host,
    path,
    name,
    value,
    expiry,
    isSecure,
    isHttpOnly,
    sameSite
FROM moz_cookies
"#,
    )?;

    let cookies = stmt
        .query_and_then([], row_to_cookie)?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(cookies)
}

/// `path` に対応する URI を返す。
fn make_uri(path: &Path) -> anyhow::Result<String> {
    let path = path.to_str().context("cannot convert path to UTF-8")?;

    let mut uri = String::with_capacity(path.len() + 32);
    uri.push_str("file:");

    // path 内の '#' は "%23" に、'?' は "%3f" にエンコードする。
    // なお、'/' が連続することはないはず。
    //
    // XXX: Windows 環境は考慮しないので、バックスラッシュやドライブレターの処理は行わない。
    // ref: https://www.sqlite.org/uri.html

    for c in path.chars() {
        match c {
            '#' => uri.push_str("%23"),
            '?' => uri.push_str("%3f"),
            _ => uri.push(c),
        }
    }

    // 読み取りしか行わないので immutable にする。
    // これを行わないと、path と同じディレクトリに `{path}-shm`, `{path}-wal` が無駄に生成される。
    uri.push_str("?immutable=1");

    Ok(uri)
}

fn row_to_cookie(row: &Row) -> anyhow::Result<Cookie> {
    let host: String = row.get(0)?;
    let path: String = row.get(1)?;
    let name: String = row.get(2)?;
    let value: String = row.get(3)?;
    let expiry: i64 = row.get(4)?;
    let is_secure: i64 = row.get(5)?;
    let is_http_only: i64 = row.get(6)?;
    let same_site: i64 = row.get(7)?;

    let is_secure = is_secure != 0;
    let is_http_only = is_http_only != 0;
    let same_site = match same_site {
        // sameSite 列の仕様がよくわかってないので、念のため 0 も Lax 扱いにしておく。
        0 | 1 => CookieSameSite::Lax,
        2 => CookieSameSite::Strict,
        _ => bail!("Unknown same_site value: {same_site}"),
    };

    let cookie = Cookie {
        host,
        path,
        name,
        value,
        expiry,
        is_secure,
        is_http_only,
        same_site,
    };

    Ok(cookie)
}

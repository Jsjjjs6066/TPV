use BTMD::content::Content;

pub enum Action<'a> {
    None(Content<'a>),
    Exit,
}
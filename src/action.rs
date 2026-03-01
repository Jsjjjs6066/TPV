use crate::btmd;

use btmd::content::Content;

pub enum Action {
    None(Content),
    Exit,
}

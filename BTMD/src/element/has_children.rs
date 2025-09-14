use crate::content::Content;
use crate::element::Element;
use crate::page::Page;

pub trait HasChildren {
    fn prepare_children(&mut self);
    fn get_children(&mut self) -> &mut Vec<Box<dyn Element>>;

    fn render_children(&mut self, page: &mut Page, size: &(u16, u16)) -> Vec<Content> {
        self.prepare_children();
        let mut contents = Vec::new();
        for child in self.get_children().iter_mut() {
            contents.push(child.render(page, size));
        }
        contents
    }
    fn rerender_children(&mut self, page: &mut Page, size: &(u16, u16)) -> Vec<Content> {
        let mut contents = Vec::new();
        for child in self.get_children().iter_mut() {
            contents.push(child.rerender(page, size));
        }
        contents
    }
}
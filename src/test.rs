use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

#[test]
fn main() {
    let html = "<html><head><title>Test</title></head><body><script src='test.js'></script><meta name='description' content='test'></body></html>";
    let dom = parse_html(html);
    traverse_dom(&dom.document);
}

fn parse_html(html: &str) -> RcDom {
    parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap()
}

fn traverse_dom(handle: &Handle) {
    let node = handle;
    match &node.data {
        NodeData::Document => {
            for child in node.children.borrow().iter() {
                traverse_dom(child);
            }
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            println!("Element: {}", name.local);
            for attr in attrs.borrow().iter() {
                println!("Attribute: {} = {}", attr.name.local, attr.value);
            }

            for child in node.children.borrow().iter() {
                traverse_dom(child);
            }
        }
        NodeData::Text { ref contents } => {
            println!("Text: {}", contents.borrow());
        }
        _ => {}
    }
}

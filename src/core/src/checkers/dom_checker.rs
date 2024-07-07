use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::RcDom;

use crate::{Technology, Webpage};
use crate::checker::Checker;
use crate::checkers::checker::CheckResult;

struct DomChecker {
    //
}

impl Checker for DomChecker {
    fn prepare(&mut self, page: &Webpage) {
        todo!()
    }

    fn check(&self, page: &Webpage, technology: &Technology) -> Option<CheckResult> {
        let pattern = &technology.dom;

        // bodyをパースしてすべてのノードを取得
        let dom =
            html5ever::parse_document(RcDom::default(), Default::default()).one(page.body.clone());
        let mut nodes = Vec::new();
        let mut stack = vec![dom.document];
        while let Some(node) = stack.pop() {
            nodes.push(node.clone());
            let mut children = node.children.borrow_mut();
            while let Some(child) = children.iter().next() {
                stack.push(child.clone());
            }
        }

        // ノードをチェック
        for node in nodes {
            if let Some(result) = self.check_node(&node, technology) {
                return Some(result);
            }
        }
    }
}

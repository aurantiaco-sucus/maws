use std::collections::BTreeMap;
use crate::handler::Handler;
use crate::http;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Fragment {
    Static(String),
    Argument,
    Wildcard,
    SuperWildcard,
}

pub fn pattern(path: &str) -> Vec<Fragment> {
    path.strip_prefix('/').unwrap_or(path)
        .split('/')
        .map(|x| match x {
            ":" => Fragment::Argument,
            "*" => Fragment::Wildcard,
            "**" => Fragment::SuperWildcard,
            _ => Fragment::Static(x.to_owned())
        })
        .collect()
}

pub struct Routes {
    pub root: Node
}

pub struct Node {
    pub handler: BTreeMap<http::Method, Handler>,
    pub children: Vec<(Fragment, Node)>,
}

impl Node {
    fn lookup_recursive<'a>(&self, path: &'a str, args: &mut Vec<&'a str>, method: http::Method) -> Option<&Handler> {
        if path.is_empty() {
            return self.handler.get(&method)
        }
        let (cur, next) = if let Some((cur, next)) = path.split_once('/') {
            (cur, next)
        } else {
            (path, "")
        };
        for (fragment, child) in &self.children {
            match fragment {
                Fragment::Static(name) => {
                    if name != cur {
                        continue
                    }
                    if let Some(val) = child.lookup_recursive(next, args, method) {
                        return Some(val)
                    }
                }
                Fragment::Argument => {
                    args.push(cur);
                    if let Some(val) = child.lookup_recursive(next, args, method) {
                        return Some(val)
                    }
                    args.pop();
                }
                Fragment::Wildcard => {
                    if let Some(val) = child.lookup_recursive(next, args, method) {
                        return Some(val)
                    }
                }
                Fragment::SuperWildcard => {
                    if let Some(val) = child.lookup_recursive("", args, method) {
                        return Some(val)
                    }
                }
            }
        }
        None
    }
}

impl Routes {
    pub fn empty() -> Self {
        Self {
            root: Node {
                handler: BTreeMap::new(),
                children: Vec::new(),
            }
        }
    }
    
    pub fn insert(&mut self, path: Vec<Fragment>, method: http::Method, handler: Handler) -> Option<Handler> {
        let mut cur = &mut self.root;
        for fragment in path {
            let existing = cur.children.iter()
                .enumerate()
                .find(|(_, (f, _))| f == &fragment);
            if let Some((i, _)) = existing {
                cur = &mut cur.children[i].1;
                continue
            }
            cur.children.push((fragment, Node {
                handler: BTreeMap::new(),
                children: Vec::new()
            }));
            cur = &mut cur.children.last_mut().unwrap().1;
        }
        cur.handler.insert(method, handler)
    }
    
    pub fn lookup<'a, 'b>(&'b self, path: &'a str, method: http::Method) -> Option<(&'b Handler, Vec<&'a str>)> {
        let mut args = Vec::new();
        self.root.lookup_recursive(path, &mut args, method).map(|x| (x, args))
    }
}
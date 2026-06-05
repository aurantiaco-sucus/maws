use crate::handler::Handler;

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
    pub handler: Option<Handler>,
    pub children: Vec<(Fragment, Node)>,
}

impl Node {
    fn lookup_recursive<'a>(&self, path: &'a str, args: &mut Vec<&'a str>) -> Option<&Handler> {
        if path.is_empty() {
            return self.handler.as_ref()
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
                    if let Some(val) = child.lookup_recursive(next, args) {
                        return Some(val)
                    }
                }
                Fragment::Argument => {
                    args.push(cur);
                }
                Fragment::Wildcard => {}
                Fragment::SuperWildcard => {}
            }
        }
        None
    }
}

impl Routes {
    pub fn empty() -> Self {
        Self {
            root: Node {
                handler: None,
                children: Vec::new(),
            }
        }
    }
    
    pub fn insert(&mut self, path: Vec<Fragment>, handler: Handler) -> Option<Handler> {
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
                handler: None,
                children: Vec::new()
            }));
            cur = &mut cur.children.last_mut().unwrap().1;
        }
        cur.handler.replace(handler)
    }
    
    pub fn lookup<'a, 'b>(&'b self, path: &'a str) -> Option<(&'b Handler, Vec<&'a str>)> {
        let mut stack = vec![(&self.root, 0)];
        let mut cur = &self.root;
        let mut args = Vec::new();
        'outer: for fragment in path.strip_prefix('/').unwrap_or(path).split('/') {
            for child in &cur.children {
                match &child.fragment {
                    Fragment::Static(value) => {
                        if value != fragment {
                            continue
                        }
                    }
                    Fragment::Argument => {
                        args.push(fragment);
                    }
                    Fragment::Wildcard => {},
                    Fragment::SuperWildcard => {
                        cur = child;
                        break 'outer
                    }
                }
                cur = child;
            }
        }
        cur.handler.as_ref().map(move |x| (x, args))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler;

    #[test]
    fn test_routes() {
        let mut routes = Routes::empty();
        let end1 = handler::standard_404();
        let end2 = handler::standard_404();
        let end3 = handler::standard_404();
        let end4 = handler::standard_404();
        let end5 = handler::standard_404();
        routes.insert(pattern("/"), end1);
        routes.insert(pattern("/*"), end2);
        routes.insert(pattern("/*/one"), end3);
        routes.insert(pattern("/*/:/two"), end4);
        routes.insert(pattern("/**"), end5);
        assert!(routes.lookup("").is_some());
        assert!(routes.lookup("/something").is_some());
        assert!(routes.lookup("something").is_some());
        assert!(routes.lookup("two/one").is_some());
        assert_eq!(routes.lookup("four/1/two").unwrap().1, ["1"]);
        assert!(routes.lookup("one/two").is_some());
        assert!(routes.lookup("/one/two/three").is_some());
    }
}
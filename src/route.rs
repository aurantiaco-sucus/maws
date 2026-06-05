use crate::handler::Handler;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Fragment {
    Static(String),
    Argument,
    Wildcard,
    SuperWildcard,
}

pub fn path(path: &str) -> impl Iterator<Item=Fragment> {
    path.strip_prefix('/').unwrap_or(path)
        .split('/')
        .map(|x| match x {
            ":" => Fragment::Argument,
            "*" => Fragment::Wildcard,
            "**" => Fragment::SuperWildcard,
            _ => Fragment::Static(x.to_owned())
        })
}

pub struct Routes {
    pub root: Node
}

pub struct Node {
    pub fragment: Fragment,
    pub handler: Option<Handler>,
    pub children: Vec<Node>,
}

impl Routes {
    pub fn empty() -> Self {
        Self {
            root: Node {
                fragment: Fragment::Wildcard,
                handler: None,
                children: Vec::new(),
            }
        }
    }
    
    pub fn insert(&mut self, path: impl Iterator<Item=Fragment>, handler: Handler) -> Option<Handler> {
        let mut cur = &mut self.root;
        for fragment in path {
            let existing = cur.children.iter()
                .enumerate()
                .find(|(_, x)| x.fragment == fragment);
            if let Some((i, _)) = existing {
                cur = &mut cur.children[i];
                continue
            }
            cur.children.push(Node {
                fragment,
                handler: None,
                children: Vec::new()
            });
            cur = cur.children.last_mut().unwrap();
        }
        cur.handler.replace(handler)
    }
    
    pub fn lookup<'a, 'b>(&'b self, path: &'a str) -> Option<(&'b Handler, Vec<&'a str>)> {
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
        let end1 = handler::not_found_404();
        let end2 = handler::not_found_404();
        let end3 = handler::not_found_404();
        let end4 = handler::not_found_404();
        let end5 = handler::not_found_404();
        routes.insert(path("/"), end1);
        routes.insert(path("/*"), end2);
        routes.insert(path("/*/one"), end3);
        routes.insert(path("/*/:/two"), end4);
        routes.insert(path("/**"), end5);
        assert!(routes.lookup("").is_some());
        assert!(routes.lookup("/something").is_some());
        assert!(routes.lookup("something").is_some());
        assert!(routes.lookup("two/one").is_some());
        assert_eq!(routes.lookup("four/1/two").unwrap().1, ["1"]);
        assert!(routes.lookup("one/two").is_some());
        assert!(routes.lookup("/one/two/three").is_some());
    }
}
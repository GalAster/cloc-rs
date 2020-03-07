use lazy_static::{self, LazyStatic, __Deref, lazy::Lazy};
use std::str::FromStr;
use syntect::highlighting::{ScopeSelector, ScopeSelectors};

#[derive(Debug)]
pub struct Selector {
    pub(crate) comment: ScopeSelector,
    pub(crate) comment_doc: ScopeSelectors,
    pub(crate) function: ScopeSelector,
    pub(crate) types: ScopeSelectors,
}

impl Default for Selector {
    fn default() -> Selector {
        Selector {
            comment: ScopeSelector::from_str("comment - comment.block.attribute").unwrap(),
            comment_doc: ScopeSelectors::from_str("comment.line.documentation, comment.block.documentation").unwrap(),
            function: ScopeSelector::from_str("entity.name.function").unwrap(),
            types: ScopeSelectors::from_str("entity.name.class, entity.name.struct, entity.name.enum, entity.name.type")
                .unwrap(),
        }
    }
}

#[allow(missing_copy_implementations)]
#[allow(dead_code)]
pub struct Selectors {
    private_field: (),
}

pub static SELECTOR: Selectors = Selectors { private_field: () };

impl __Deref for Selectors {
    type Target = Selector;
    fn deref(&self) -> &Selector {
        #[inline(always)]
        fn __static_ref_initialize() -> Selector {
            Selector::default()
        }
        #[inline(always)]
        fn __stability() -> &'static Selector {
            static LAZY: Lazy<Selector> = Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}

impl LazyStatic for Selectors {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}

use yew::AttrValue;

#[derive(Debug, PartialEq, Clone)]
pub enum SelectOption {
    Simple(AttrValue),
    Pair { label: AttrValue, value: AttrValue },
}

impl From<AttrValue> for SelectOption {
    fn from(v: AttrValue) -> Self {
        SelectOption::Simple(v)
    }
}
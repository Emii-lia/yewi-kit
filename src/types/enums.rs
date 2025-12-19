#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Tertiary,
    Danger,
    Success,
}

#[derive(PartialEq, Clone)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}
use crate::components::SelectOption;

pub fn options() -> Vec<SelectOption> {
  let mut options = Vec::new();
  
options.push(SelectOption::Simple("Option 1".into()));
options.push(SelectOption::Simple("Option 2".into()));
options.push(SelectOption::Pair {
  label: "Option 3 Label".into(),
  value: "option_3_value".into(),
});
  options.push(SelectOption::Pair {
    label: "Option 4 Label".into(),
    value: "option_4_value".into(),
  });
  options
}
// FILE: mods/components/global_styles.rs
// ____________________________________________________

pub const CONTAINER: &str = r#"
  display: grid;
  justify-items: center;
  align-items: center;
  gap: 10px;
  height: 100vh;
"#;

pub const BUTTON_STYLE: &str = r#"
  padding: 10px 20px;
  border: 1px solid #007BFF;
  border-radius: 5px;
  background-color: #007BFF;
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  &:hover {
    background-color: #0056b3;
  }
  &:active {
    opacity: 0.7;
  }
"#;

pub const INPUT_STYLE: &str = r#"
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 5px;
"#;

pub const H1_STYLE: &str = r#"
  font-size: 24px;
  color: #333;
  transition: opacity 1s ease-in;
  opacity: 0;
"#;
// ____________________________________________________
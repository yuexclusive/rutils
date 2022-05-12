#![allow(dead_code)]
#![allow(unused)]

pub trait Button {
    fn paint(&self) -> String;
}

pub trait Checkbox {
    fn paint(&self) -> String;
}

pub trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

struct MacCheckbox {}

struct MacButton {}

struct WinCheckbox {}

struct WinButton {}

impl Checkbox for MacCheckbox {
    fn paint(&self) -> String {
        String::from("mac checkbox")
    }
}

impl Button for MacButton {
    fn paint(&self) -> String {
        String::from("mac button")
    }
}

impl Checkbox for WinCheckbox {
    fn paint(&self) -> String {
        String::from("win checkbox")
    }
}

impl Button for WinButton {
    fn paint(&self) -> String {
        String::from("win button")
    }
}

struct MacFactory {}

struct WinFactory {}

impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton {})
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox {})
    }
}

impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton {})
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WinCheckbox {})
    }
}

pub fn get_str() -> String {
    String::from("hello")
}

pub fn exec<T>(t: T) -> String
where
    T: GUIFactory,
{
    let mut res = String::new();
    res.push_str(&t.create_button().paint());
    res.push('\n');
    res.push_str(&t.create_checkbox().paint());
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_abstract_factory_mac() {
        let f = MacFactory {};
        let res = exec(f);
        assert_eq!(
            res,
            r#"mac button
mac checkbox"#
        );
    }

    #[test]
    fn test_abstract_factory_win() {
        let f = WinFactory {};
        let res = exec(f);
        assert_eq!(
            res,
            r#"win button
win checkbox"#
        );
    }
}

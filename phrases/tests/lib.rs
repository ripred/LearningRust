#[cfg(test)]
mod tests {
    extern crate phrases;

#[test]
    fn english_greetings_correct() {
        assert_eq!("hello", phrases::greetings::english::hello());
        assert_eq!("goodbye", phrases::greetings::english::goodbye());
    }

#[test]
    fn french_greetings_correct() {
        assert_eq!("bonjour", phrases::greetings::french::hello());
        assert_eq!("au revoir", phrases::greetings::french::goodbye());
    }

#[test]
#[should_panic]
    fn greetings_incorrect() {
        assert_eq!("hello", phrases::greetings::french::hello());
        assert_eq!("goodbye", phrases::greetings::french::goodbye());
    }

#[test]
#[should_panic]
#[ignore]
    fn greetings_ignored() {
        assert_eq!("hello", phrases::greetings::french::hello());
        assert_eq!("goodbye", phrases::greetings::french::goodbye());
    }
}

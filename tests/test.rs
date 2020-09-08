use anyident::i;

#[test]
fn it_works() {
    let i!(this is an identifier) = "hello, world";
    assert_eq!(i!(this is an identifier), "hello, world");

    // Japanese characters
    let i!(こんにちは) = "hello!";
    assert_eq!(i!(こんにちは).len(), 6);
}

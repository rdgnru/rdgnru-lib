#[test]
fn public_api_works() {
    assert_eq!(rdgnru_lib::greet("crates.io"), "Hello, crates.io!");
    assert_eq!(rdgnru_lib::slugify("Publish Me!"), "publish-me");
    assert!(rdgnru_lib::is_valid_slug("publish-me"));
}

mod uri;
mod version;
mod headers;

#[test]
fn www_auth() {
    let nonce = "deac948e-d99b-11e9-989a-6d105e6cd8c0";
    let cnonce = "1d0273d9b3e23cf0850aeb8d9ee48c67";
    let noncec = "00000001";
    let realm = "192.168.1.123";
    let user = "10";
    let pass = "phone";
    let uri = "sip:192.168.1.123;transport=UDP";

    let raw_ha1 = format!("{}:{}:{}", user, realm, pass);
    println!("{}", raw_ha1);
    let ha1 = md5::compute(raw_ha1);
    println!("{:x}", ha1);
    let raw_ha2 = format!("REGISTER:{}", uri);
    let ha2 = md5::compute(raw_ha2);
    println!("{:x}", ha2);
    let raw_digest = format!("{:x}:{}:{}:{}:auth:{:x}", ha1, nonce, noncec, cnonce, ha2);
    let digest = md5::compute(raw_digest);
    println!("{:x}", digest);
}

use super::*;

#[test]
fn encode_decode_roundtrip() {
    let original = Auch::with_challenge([0xABu8; AUCH_CHALLENGE_LEN]);
    let wire = original.encode().unwrap();

    let mut decoded = Auch::default();
    decoded.decode(&wire).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn decode_bad_command() {
    let mut auch = Auch::default();
    let err = auch.decode(&[b'X', 0, 20]).unwrap_err();
    assert_eq!(err, AuchError::BadCommandError);
}

#[test]
fn decode_length_mismatch() {
    let mut auch = Auch::default();
    let err = auch.decode(&[AUCHCMD, 0, 5, 1, 2, 3]).unwrap_err();
    assert_eq!(err, AuchError::ChallengeLengthMismatch);
}

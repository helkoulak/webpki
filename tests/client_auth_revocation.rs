// Copyright 2023 Daniel McCarney.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

extern crate webpki;

static ALL_SIGALGS: &[&webpki::SignatureAlgorithm] = &[
    &webpki::ECDSA_P256_SHA256,
    &webpki::ECDSA_P256_SHA384,
    &webpki::ECDSA_P384_SHA256,
    &webpki::ECDSA_P384_SHA384,
    &webpki::ED25519,
    #[cfg(feature = "alloc")]
    &webpki::RSA_PKCS1_2048_8192_SHA256,
    #[cfg(feature = "alloc")]
    &webpki::RSA_PKCS1_2048_8192_SHA384,
    #[cfg(feature = "alloc")]
    &webpki::RSA_PKCS1_2048_8192_SHA512,
    #[cfg(feature = "alloc")]
    &webpki::RSA_PKCS1_3072_8192_SHA384,
];

/// Specifies how much of a certificate chain is checked for revocation status.
#[derive(Debug, PartialEq, Eq)]
pub enum RevocationCheckDepth {
    /// Only check the end entity certificate for revocation status.
    EndEntity,
    /// Check both the end entity certificate as well as any relevant intermediate certificates
    /// for revocation status.
    Chain,
}

struct TestCrls<'a> {
    crls: &'a [webpki::CertRevocationList<'a>],
    depth: RevocationCheckDepth,
}

impl<'a> webpki::CrlProvider<'a> for TestCrls<'a> {
    // Lookup a CRL from the set of test CRLs by matching the cert's issuer with the CRL's issuer.
    fn crl_for_cert(&self, cert: &webpki::Cert) -> Option<&'a webpki::CertRevocationList<'a>> {
        // If we're asked for a CRL for a CA cert, and the configured TestCrls depth is EndEntity,
        // return None.
        if matches!(cert.end_entity_or_ca(), webpki::EndEntityOrCa::Ca(_))
            && matches!(self.depth, RevocationCheckDepth::EndEntity)
        {
            return None;
        }
        self.crls
            .iter()
            .find(|candidate_crl| candidate_crl.issuer() == cert.issuer())
    }
}

fn check_cert(
    ee: &[u8],
    intermediates: &[&[u8]],
    ca: &[u8],
    depth: RevocationCheckDepth,
    crls: &[webpki::CertRevocationList],
) -> Result<(), webpki::Error> {
    let anchors = &[webpki::TrustAnchor::try_from_cert_der(ca).unwrap()];
    let anchors = webpki::TlsClientTrustAnchors(anchors.as_slice());
    let cert = webpki::EndEntityCert::try_from(ee).unwrap();
    let time = webpki::Time::from_seconds_since_unix_epoch(0x1fed_f00d);

    let crl_provider = &TestCrls { crls, depth };
    let rev_opts = webpki::RevocationCheckOptions { crl_provider };

    cert.verify_is_valid_tls_client_cert(ALL_SIGALGS, &anchors, intermediates, time, Some(rev_opts))
}

// DO NOT EDIT BELOW: generated by tests/generate.py

#[test]
#[cfg(feature = "alloc")]
fn no_crls_test_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::EndEntity, crls),
        Ok(())
    );
}

#[test]
#[cfg(feature = "alloc")]
fn no_relevant_crl_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/no_relevant_crl_ee_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::EndEntity, crls),
        Ok(())
    );
}

#[test]
#[cfg(feature = "alloc")]
fn ee_not_revoked_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/ee_not_revoked_ee_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::EndEntity, crls),
        Ok(())
    );
}

#[test]
#[cfg(feature = "alloc")]
fn ee_revoked_badsig_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/ee_revoked_badsig_ee_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::EndEntity, crls),
        Err(webpki::Error::UnknownIssuer)
    );
}

#[test]
#[cfg(feature = "alloc")]
fn ee_revoked_wrong_ku_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_crl_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_crl_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_crl_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/ee_revoked_wrong_ku_ee_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::EndEntity, crls),
        Err(webpki::Error::UnknownIssuer)
    );
}

#[test]
#[cfg(feature = "alloc")]
fn ee_not_revoked_wrong_ku_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_crl_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_crl_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_crl_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/ee_not_revoked_wrong_ku_ee_depth.crl.der")
            .as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::EndEntity, crls),
        Ok(())
    );
}

#[test]
#[cfg(feature = "alloc")]
fn ee_revoked_no_ku_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/ee_revoked_no_ku_ee_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::EndEntity, crls),
        Err(webpki::Error::UnknownIssuer)
    );
}

#[test]
#[cfg(feature = "alloc")]
fn ee_revoked_crl_ku_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/ku_chain.ee.der");
    let intermediates = &[include_bytes!("client_auth_revocation/ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/ee_revoked_crl_ku_ee_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::EndEntity, crls),
        Err(webpki::Error::UnknownIssuer)
    );
}

#[test]
#[cfg(feature = "alloc")]
fn no_crls_test_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::Chain, crls),
        Ok(())
    );
}

#[test]
#[cfg(feature = "alloc")]
fn no_relevant_crl_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/no_relevant_crl_chain_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::Chain, crls),
        Ok(())
    );
}

#[test]
#[cfg(feature = "alloc")]
fn int_not_revoked_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/int_not_revoked_chain_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::Chain, crls),
        Ok(())
    );
}

#[test]
#[cfg(feature = "alloc")]
fn int_revoked_badsig_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/int_revoked_badsig_chain_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::Chain, crls),
        Err(webpki::Error::UnknownIssuer)
    );
}

#[test]
#[cfg(feature = "alloc")]
fn int_revoked_wrong_ku_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_crl_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_crl_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_crl_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/int_revoked_wrong_ku_chain_depth.crl.der")
            .as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::Chain, crls),
        Err(webpki::Error::UnknownIssuer)
    );
}

#[test]
#[cfg(feature = "alloc")]
fn ee_revoked_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/ee_revoked_chain_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::Chain, crls),
        Err(webpki::Error::UnknownIssuer)
    );
}

#[test]
#[cfg(feature = "alloc")]
fn int_revoked_ee_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/int_revoked_ee_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::EndEntity, crls),
        Ok(())
    );
}

#[test]
#[cfg(feature = "alloc")]
fn int_revoked_no_ku_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/no_ku_chain.ee.der");
    let intermediates =
        &[include_bytes!("client_auth_revocation/no_ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/no_ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/int_revoked_no_ku_chain_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::Chain, crls),
        Err(webpki::Error::UnknownIssuer)
    );
}

#[test]
#[cfg(feature = "alloc")]
fn int_revoked_crl_ku_chain_depth() {
    let ee = include_bytes!("client_auth_revocation/ku_chain.ee.der");
    let intermediates = &[include_bytes!("client_auth_revocation/ku_chain.int.ca.der").as_slice()];
    let ca = include_bytes!("client_auth_revocation/ku_chain.root.ca.der");
    let crls = &[webpki::CertRevocationList::try_from(
        include_bytes!("client_auth_revocation/int_revoked_crl_ku_chain_depth.crl.der").as_slice(),
    )
    .unwrap()];
    assert_eq!(
        check_cert(ee, intermediates, ca, RevocationCheckDepth::Chain, crls),
        Err(webpki::Error::UnknownIssuer)
    );
}

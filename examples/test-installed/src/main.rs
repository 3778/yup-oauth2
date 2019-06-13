use futures::prelude::*;
use yup_oauth2::GetToken;
use yup_oauth2::{Authenticator, InstalledFlow};

use hyper::client::Client;
use hyper_tls::HttpsConnector;

use std::path::Path;

fn main() {
    let https = HttpsConnector::new(1).expect("tls");
    let client = Client::builder().build::<_, hyper::Body>(https);
    let ad = yup_oauth2::DefaultFlowDelegate;
    let secret = yup_oauth2::read_application_secret(Path::new("clientsecret.json"))
        .expect("clientsecret.json");
    let inf = InstalledFlow::new(
        client.clone(),
        ad,
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect(8081),
    );
    let mut auth = Authenticator::new_disk(
        client,
        inf,
        yup_oauth2::DefaultAuthenticatorDelegate,
        "tokencache.json",
    )
    .unwrap();
    let s = "https://www.googleapis.com/auth/drive.file".to_string();
    let scopes = vec![s];

    let tok = auth.token(scopes.iter());
    let fut = tok.map_err(|e| println!("error: {:?}", e)).and_then(|t| {
        println!("The token is {:?}", t);
        Ok(())
    });

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(fut).unwrap();
}

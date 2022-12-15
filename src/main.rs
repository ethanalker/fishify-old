use rspotify::{
    model::{AdditionalType, Country, Market},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth, 
};

#[tokio::main]
async fn main() {
    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap();

    spotify.prompt_for_token(&url).await.unwrap();

    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .currently_playing(Some(market), Some(&additional_types))
        .await;

    println!("Response: {artists:?}");
}

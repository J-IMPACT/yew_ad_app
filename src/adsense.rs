use yew::prelude::*;

#[function_component(AdsenseAd)]
pub fn adsense_ad() -> Html {
    let client_id: String = std::env::var("ADSENSE_CLIENT_ID").unwrap_or("ADSENSE_CLIENT_ID_NOT_FOUND".to_string());
    let slot_id: String = std::env::var("ADSENSE_SLOT_ID").unwrap_or("ADSENSE_SLOT_ID_NOT_FOUND".to_string());

    html! {
        <>
        <script async=true src={format!("https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client={client_id}")} crossorigin="anonymous"></script>

        <ins class="adsbygoogle"
            style="display:block"
            data-ad-client={client_id}
            data-ad-slot={slot_id}
            data-ad-format="auto"
            data-full-width-responsive="true"></ins>
        <script dangerously_set_inner_html={r#"(adsbygoogle = window.adsbygoogle || []).push({});"#}></script>
        </>
    }
}
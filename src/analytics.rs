use web_sys::window;
use yew::prelude::*;


#[function_component(GoogleAnalytics)]
pub fn google_analytics() -> Html {
    let ga_id: String = std::env::var("GA_TRACKING_ID").unwrap_or("G-XXXXXXXXXX".to_string());

    use_effect(move || {
        let document = window().unwrap().document().unwrap();

        if document.get_element_by_id("ga-script").is_none() {
            let script = document.create_element("script").unwrap();
            script.set_attribute("async", "").unwrap();
            script.set_attribute(
                "src", 
                &format!("https://www.googletagmanager.com/gtag/js?id={ga_id}")
            ).unwrap();
            script.set_id("ga-script");
            document.head().unwrap().append_child(&script).unwrap();

            let config = document.create_element("script").unwrap();
            config.set_text_content(Some(&format!(
                r#"
                window.dataLayer = window.dataLayer || [];
                function gtag(){{dataLayer.push(arguments);}}
                gtag('js', new Date());

                gtag('config', '{ga_id}');
                "#
            )));
            document.head().unwrap().append_child(&config).unwrap();
        }

        || ()
    });

    html! {}
}
#[cfg(feature = "HiMdSolidChatBubbleBottomCenter")]
use leptos::*;
#[cfg(feature = "HiMdSolidChatBubbleBottomCenter")]
///This icon requires the feature `HiMdSolidChatBubbleBottomCenter` to be enabled.
#[component]
pub fn ChatBubbleBottomCenter(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M3.42976 2.52374C5.56943 2.17905 7.76407 2 10 2C12.2359 2 14.4306 2.17905 16.5702 2.52374C18.0066 2.75513 19 4.01325 19 5.42588V10.5741C19 11.9867 18.0066 13.2449 16.5702 13.4763C15.4024 13.6644 14.2181 13.8032 13.0196 13.8905C12.7399 13.9108 12.4988 14.0703 12.3775 14.3035L10.6654 17.596C10.5363 17.8443 10.2798 18 10 18C9.7202 18 9.46367 17.8443 9.33459 17.596L7.62247 14.3035C7.5012 14.0703 7.2601 13.9108 6.9804 13.8905C5.78193 13.8032 4.59764 13.6644 3.42976 13.4763C1.99338 13.2449 1 11.9867 1 10.5741V5.42588C1 4.01325 1.99339 2.75513 3.42976 2.52374Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

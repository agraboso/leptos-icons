#[cfg(feature = "HiMdSolidChatBubbleLeft")]
use leptos::*;
#[cfg(feature = "HiMdSolidChatBubbleLeft")]
///This icon requires the feature `HiMdSolidChatBubbleLeft` to be enabled.
#[component]
pub fn ChatBubbleLeft(
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
        "M3.42976 2.52374C5.56943 2.17905 7.76407 2 10 2C12.2359 2 14.4306 2.17905 16.5702 2.52374C18.0066 2.75513 19 4.01325 19 5.42588V10.5741C19 11.9867 18.0066 13.2449 16.5702 13.4763C14.8747 13.7494 13.1447 13.9185 11.3869 13.977C11.1832 13.9837 10.9953 14.0654 10.8594 14.2013L7.28033 17.7803C7.06583 17.9948 6.74324 18.059 6.46299 17.9429C6.18273 17.8268 6 17.5533 6 17.25V13.8073C5.13517 13.7236 4.27814 13.6129 3.42976 13.4763C1.99339 13.2449 1 11.9868 1 10.5741V5.42588C1 4.01325 1.99339 2.75513 3.42976 2.52374Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

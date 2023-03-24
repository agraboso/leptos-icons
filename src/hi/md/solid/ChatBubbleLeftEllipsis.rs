#[cfg(feature = "HiMdSolidChatBubbleLeftEllipsis")]
use leptos::*;
#[cfg(feature = "HiMdSolidChatBubbleLeftEllipsis")]
///This icon requires the feature `HiMdSolidChatBubbleLeftEllipsis` to be enabled.
#[component]
pub fn ChatBubbleLeftEllipsis(
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
        "M10 2C7.76407 2 5.56943 2.17905 3.42976 2.52374C1.99339 2.75513 1 4.01325 1 5.42588V10.5741C1 11.9868 1.99339 13.2449 3.42976 13.4763C4.27814 13.6129 5.13517 13.7236 6 13.8073V17.25C6 17.5533 6.18273 17.8268 6.46299 17.9429C6.74324 18.059 7.06583 17.9948 7.28033 17.7803L10.8594 14.2013C10.9953 14.0654 11.1832 13.9837 11.3869 13.977C13.1447 13.9185 14.8747 13.7494 16.5702 13.4763C18.0066 13.2449 19 11.9867 19 10.5741V5.42588C19 4.01325 18.0066 2.75513 16.5702 2.52374C14.4306 2.17905 12.2359 2 10 2ZM10 9C10.5523 9 11 8.55228 11 8C11 7.44772 10.5523 7 10 7C9.44771 7 9 7.44772 9 8C9 8.55228 9.44771 9 10 9ZM8 8C8 8.55228 7.55228 9 7 9C6.44772 9 6 8.55228 6 8C6 7.44772 6.44772 7 7 7C7.55228 7 8 7.44772 8 8ZM13 9C13.5523 9 14 8.55228 14 8C14 7.44772 13.5523 7 13 7C12.4477 7 12 7.44772 12 8C12 8.55228 12.4477 9 13 9Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

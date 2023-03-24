#[cfg(feature = "HiLgSolidChatBubbleOvalLeftEllipsis")]
use leptos::*;
#[cfg(feature = "HiLgSolidChatBubbleOvalLeftEllipsis")]
///This icon requires the feature `HiLgSolidChatBubbleOvalLeftEllipsis` to be enabled.
#[component]
pub fn ChatBubbleOvalLeftEllipsis(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M4.80365 21.6442C4.9793 21.6757 5.15732 21.7003 5.33691 21.7178C5.55516 21.7391 5.77647 21.75 6 21.75C7.3153 21.75 8.54447 21.3731 9.58317 20.7213C10.3569 20.9034 11.1668 21 12 21C17.322 21 21.75 17.0307 21.75 12C21.75 6.96934 17.322 3 12 3C6.67799 3 2.25 6.96934 2.25 12C2.25 14.4086 3.2746 16.5871 4.92371 18.1923C5.15571 18.4182 5.20107 18.6196 5.17822 18.7349C5.05254 19.3685 4.76687 19.9451 4.36357 20.4211C4.19016 20.6258 4.13927 20.9075 4.23008 21.1599C4.3209 21.4123 4.5396 21.597 4.80365 21.6442ZM8.25 10.875C7.62868 10.875 7.125 11.3787 7.125 12C7.125 12.6213 7.62868 13.125 8.25 13.125C8.87132 13.125 9.375 12.6213 9.375 12C9.375 11.3787 8.87132 10.875 8.25 10.875ZM10.875 12C10.875 11.3787 11.3787 10.875 12 10.875C12.6213 10.875 13.125 11.3787 13.125 12C13.125 12.6213 12.6213 13.125 12 13.125C11.3787 13.125 10.875 12.6213 10.875 12ZM15.75 10.875C15.1287 10.875 14.625 11.3787 14.625 12C14.625 12.6213 15.1287 13.125 15.75 13.125C16.3713 13.125 16.875 12.6213 16.875 12C16.875 11.3787 16.3713 10.875 15.75 10.875Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

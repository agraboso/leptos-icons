#[cfg(feature = "HiMdSolidChatBubbleOvalLeftEllipsis")]
use leptos::*;
#[cfg(feature = "HiMdSolidChatBubbleOvalLeftEllipsis")]
///This icon requires the feature `HiMdSolidChatBubbleOvalLeftEllipsis` to be enabled.
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M10 3C5.69006 3 2 6.0334 2 10C2 12.0244 2.97849 13.8253 4.49899 15.0848C4.48371 15.7294 4.29476 16.329 3.97742 16.8409C3.83914 17.064 3.82753 17.3431 3.9468 17.5769C4.06608 17.8107 4.29888 17.9651 4.56065 17.9841C4.70585 17.9947 4.85237 18 5 18C6.3037 18 7.51177 17.5834 8.49617 16.8766C8.98381 16.9577 9.48658 17 10 17C14.3099 17 18 13.9666 18 10C18 6.0334 14.3099 3 10 3ZM10 11C10.5523 11 11 10.5523 11 10C11 9.44772 10.5523 9 10 9C9.44772 9 9 9.44772 9 10C9 10.5523 9.44772 11 10 11ZM8 10C8 10.5523 7.55228 11 7 11C6.44772 11 6 10.5523 6 10C6 9.44772 6.44772 9 7 9C7.55228 9 8 9.44772 8 10ZM13 11C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9C12.4477 9 12 9.44772 12 10C12 10.5523 12.4477 11 13 11Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

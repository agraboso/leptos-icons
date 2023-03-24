#[cfg(feature = "SiHilton")]
use leptos::*;
#[cfg(feature = "SiHilton")]
///This icon requires the feature `SiHilton` to be enabled.
#[component]
pub fn Hilton(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M16.123 14.234c0-1.544-1.06-2.788-2.972-3.386V7.347h3.57v8.59h-1.105a2.993 2.993 0 0 0 .506-1.703M11.194 3.708C4.744 3.708 0 7.808 0 12.184c0 2.257 1.267 4.168 2.88 5.481l.114-.092a6.369 6.369 0 0 1-2.418-4.95C.92 7.83 6.818 5.134 12.092 5.134c3.709 0 10.388.85 10.388 7.072 0 3.247-3.455 7.462-10.733 7.462-3.731 0-6.61-2.004-6.61-4.86 0-2.487 1.935-4.26 4.836-4.26a7.83 7.83 0 0 1 2.12.345c-1.199-.207-3.087.092-4.055.85v4.192h3.57v-4.743a4.064 4.064 0 0 1 1.543.6v4.192h.922a3.83 3.83 0 0 1-3.27 1.126v.184c3.385.276 4.813-1.336 4.813-3.156 0-2.118-2.004-3.362-4.03-3.639V7.347h-3.57v3.179c-2.304.46-4.561 1.842-4.561 4.1 0 3.501 4.883 5.666 9.12 5.666 6.452 0 11.425-3.109 11.425-7.508.023-5.299-7.163-9.076-12.806-9.076Z"
        /> < title > { title } < / title > < / svg >
    }
}

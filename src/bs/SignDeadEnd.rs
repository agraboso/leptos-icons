#[cfg(feature = "BsSignDeadEnd")]
use leptos::*;
#[cfg(feature = "BsSignDeadEnd")]
///This icon requires the feature `BsSignDeadEnd` to be enabled.
#[component]
pub fn SignDeadEnd(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-sign-dead-end" viewBox = "0 0 16 16" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.783 6v2h.69c.593 0 .886-.359.886-1.002S6.07 6 5.473 6h-.69Zm.333.28h.32c.395 0 .582.24.582.722 0 .48-.186.718-.581.718h-.321V6.28ZM7.82 7.72h-.918v-.602h.863V6.85h-.863v-.57h.917V6H6.571v2H7.82v-.28Zm.573-.274L8.216 8h-.34l.688-2h.371l.689 2h-.352l-.177-.554h-.702Zm.627-.255-.268-.845h-.015l-.27.845h.553ZM9.746 6v2h.69c.593 0 .886-.359.886-1.002S11.032 6 10.436 6h-.69Zm.333.28h.32c.394 0 .582.24.582.722 0 .48-.186.718-.582.718h-.32V6.28Zm-4.173 4.44h.917V11H5.575V9h1.248v.28h-.917v.57h.863v.268h-.863v.602Zm1.489.28V9.56h.013L8.344 11h.292V9h-.32v1.436h-.014L7.369 9h-.293v2h.319Zm1.56 0V9h.69c.596 0 .886.355.886.998S10.238 11 9.645 11h-.69Zm.653-1.72h-.32v1.44h.32c.395 0 .581-.239.581-.718 0-.481-.187-.722-.581-.722Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.05.435c-.58-.58-1.52-.58-2.1 0L.436 6.95c-.58.58-.58 1.519 0 2.098l6.516 6.516c.58.58 1.519.58 2.098 0l6.516-6.516c.58-.58.58-1.519 0-2.098L9.05.435Zm-1.4.7a.495.495 0 0 1 .7 0l6.516 6.515a.495.495 0 0 1 0 .7L8.35 14.866a.495.495 0 0 1-.7 0L1.134 8.35a.495.495 0 0 1 0-.7L7.65 1.134Z"
        /> < title > { title } < / title > < / svg >
    }
}

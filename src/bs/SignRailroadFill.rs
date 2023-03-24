#[cfg(feature = "BsSignRailroadFill")]
use leptos::*;
#[cfg(feature = "BsSignRailroadFill")]
///This icon requires the feature `BsSignRailroadFill` to be enabled.
#[component]
pub fn SignRailroadFill(
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
        class = "bi bi-sign-railroad-fill" viewBox = "0 0 16 16" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.05.435c-.58-.58-1.52-.58-2.1 0L4.224 3.162 8 6.94l3.777-3.777L9.049.435Zm3.274 7.425v-.862h.467c.28 0 .467.154.467.44 0 .28-.182.421-.475.421h-.459Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.838 4.223 9.06 8l3.777 3.777 2.727-2.728c.58-.58.58-1.519 0-2.098l-2.727-2.728Zm.03 2.361c.591 0 .935.334.935.844a.79.79 0 0 1-.485.748l.536 1.074h-.59l-.467-.994h-.473v.994h-.521V6.584h1.064Zm-1.091 6.254L8 9.06l-3.777 3.777 2.728 2.727c.58.58 1.519.58 2.098 0l2.728-2.727Zm-8.953-5.84v.861h.46c.292 0 .474-.14.474-.421 0-.286-.188-.44-.467-.44h-.467Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.162 11.777 6.94 8 3.162 4.223.435 6.951c-.58.58-.58 1.519 0 2.098l2.727 2.728Zm-.86-5.193h1.065c.592 0 .936.334.936.844 0 .39-.242.654-.485.748l.536 1.074h-.59l-.467-.994h-.473v.994h-.521V6.584Z"
        /> < title > { title } < / title > < / svg >
    }
}

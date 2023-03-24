#[cfg(feature = "SiPagseguro")]
use leptos::*;
#[cfg(feature = "SiPagseguro")]
///This icon requires the feature `SiPagseguro` to be enabled.
#[component]
pub fn Pagseguro(
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
        "M17.482 9.712c1.64 0 3.108.69 4.1 1.813.044-.388.087-.777.087-1.208C21.67 5.007 17.353.69 12 .69c-5.353 0-9.67 4.316-9.67 9.626 0 .345 0 .69.044 1.036a8.688 8.688 0 017.943-5.137c2.848 0 5.352 1.381 6.95 3.496h.215zm-7.122-2.72c-4.36 0-7.9 3.54-7.9 7.9s3.54 7.9 7.9 7.9c2.158 0 4.1-.864 5.525-2.245a5.53 5.53 0 01-3.928-5.31c0-2.676 1.9-4.92 4.446-5.438-1.468-1.684-3.626-2.806-6.043-2.806zM4.79 21.583A11.958 11.958 0 010 12C0 5.353 5.396 0 12 0s12 5.396 12 12-5.396 12-12 12c-1.554 0-3.022-.302-4.36-.82-1.079-.389-2.028-.907-2.849-1.597zm12.777-1.51a4.827 4.827 0 004.835-4.835 4.827 4.827 0 00-4.835-4.834 4.827 4.827 0 00-4.834 4.834 4.827 4.827 0 004.834 4.835Z"
        /> < title > { title } < / title > < / svg >
    }
}

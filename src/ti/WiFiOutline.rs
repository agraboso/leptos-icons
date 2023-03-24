#[cfg(feature = "TiWiFiOutline")]
use leptos::*;
#[cfg(feature = "TiWiFiOutline")]
///This icon requires the feature `TiWiFiOutline` to be enabled.
#[component]
pub fn WiFiOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.157 10.764c0-.785-.269-1.464-.706-2.048-.045-.094-.131-.149-.21-.226-.163-.18-.341-.338-.536-.48-4.45-3.739-10.965-3.735-15.414.006-.193.142-.742.738-.742.738-.437.584-.706 1.305-.706 2.09 0 .816.362 1.758.759 2.155l5.775 5.796c.642.732 1.572 1.204 2.622 1.204.996 0 1.709-.167 2.526-1 .004 0 5.565-5.646 5.565-5.646.706-.703 1.067-1.699 1.067-2.589zm-9.156 7.234c-.829.002-1.501-.668-1.501-1.498-.002-.828.67-1.502 1.501-1.502.829-.002 1.501.67 1.499 1.502.002.828-.67 1.5-1.499 1.498zm3.888-3.268c-.293.293-.677.438-1.061.438-.385 0-.768-.146-1.061-.438-.976-.976-2.562-.976-3.536 0-.586.586-1.536.584-2.122 0-.586-.586-.586-1.537 0-2.123 2.144-2.144 5.632-2.144 7.779 0 .587.586.587 1.538.001 2.123zm2.829-2.828c-.293.293-.677.438-1.061.438s-.769-.146-1.062-.438c-2.533-2.534-6.658-2.534-9.192 0-.586.584-1.536.584-2.122 0-.585-.586-.585-1.536 0-2.123 3.704-3.701 9.729-3.701 13.435 0 .587.588.587 1.537.002 2.123z"
        /> < title > { title } < / title > < / svg >
    }
}

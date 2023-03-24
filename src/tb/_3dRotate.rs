#[cfg(feature = "Tb3dRotate")]
use leptos::*;
#[cfg(feature = "Tb3dRotate")]
///This icon requires the feature `Tb3dRotate` to be enabled.
#[component]
pub fn _3dRotate(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-3d-rotate"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 3a7 7 0 0 1 7 7v4l-3 -3"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M22 11l-3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 15.5l-5 -3l5 -3l5 3v5.5l-5 3z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M3 12.5v5.5l5 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 15.545l5 -3.03" /> < title > { title } < /
        title > < / svg >
    }
}

#[cfg(feature = "IoFolderOpen")]
use leptos::*;
#[cfg(feature = "IoFolderOpen")]
///This icon requires the feature `IoFolderOpen` to be enabled.
#[component]
pub fn FolderOpen(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M408,96H252.11a23.89,23.89,0,0,1-13.31-4L211,73.41A55.77,55.77,0,0,0,179.89,64H104a56.06,56.06,0,0,0-56,56v24H464C464,113.12,438.88,96,408,96Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M423.75,448H88.25a56,56,0,0,1-55.93-55.15L16.18,228.11l0-.28A48,48,0,0,1,64,176h384.1a48,48,0,0,1,47.8,51.83l0,.28L479.68,392.85A56,56,0,0,1,423.75,448ZM479.9,226.55h0Z"
        /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "RiEditorLinkUnlinkM")]
use leptos::*;
#[cfg(feature = "RiEditorLinkUnlinkM")]
///This icon requires the feature `RiEditorLinkUnlinkM` to be enabled.
#[component]
pub fn LinkUnlinkM(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M17.657 14.828l-1.414-1.414L17.657 12A4 4 0 1 0 12 6.343l-1.414 1.414-1.414-1.414 1.414-1.414a6 6 0 0 1 8.485 8.485l-1.414 1.414zm-2.829 2.829l-1.414 1.414a6 6 0 1 1-8.485-8.485l1.414-1.414 1.414 1.414L6.343 12A4 4 0 1 0 12 17.657l1.414-1.414 1.414 1.414zm0-9.9l1.415 1.415-7.071 7.07-1.415-1.414 7.071-7.07zM5.775 2.293l1.932-.518L8.742 5.64l-1.931.518-1.036-3.864zm9.483 16.068l1.931-.518 1.036 3.864-1.932.518-1.035-3.864zM2.293 5.775l3.864 1.036-.518 1.931-3.864-1.035.518-1.932zm16.068 9.483l3.864 1.035-.518 1.932-3.864-1.036.518-1.931z"
        /></ g > < title > { title } < / title > < / svg >
    }
}

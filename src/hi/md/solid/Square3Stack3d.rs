#[cfg(feature = "HiMdSolidSquare3Stack3d")]
use leptos::*;
#[cfg(feature = "HiMdSolidSquare3Stack3d")]
///This icon requires the feature `HiMdSolidSquare3Stack3d` to be enabled.
#[component]
pub fn Square3Stack3d(
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
        "http://www.w3.org/2000/svg" d =
        "M3.19569 12.8694L2.37071 13.353C2.14108 13.4876 2 13.7338 2 14C2 14.2662 2.14108 14.5124 2.37071 14.647L9.62071 18.897C9.85493 19.0343 10.1451 19.0343 10.3793 18.897L17.6293 14.647C17.8589 14.5124 18 14.2662 18 14C18 13.7338 17.8589 13.4876 17.6293 13.353L16.8043 12.8694L11.1379 16.1911C10.4352 16.603 9.56479 16.603 8.86213 16.1911L3.19569 12.8694Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.19569 8.86937L2.37071 9.35298C2.14108 9.48759 2 9.73382 2 10C2 10.2662 2.14108 10.5124 2.37071 10.647L9.62071 14.897C9.85493 15.0343 10.1451 15.0343 10.3793 14.897L17.6293 10.647C17.8589 10.5124 18 10.2662 18 10C18 9.73382 17.8589 9.48759 17.6293 9.35298L16.8043 8.86937L11.1379 12.1911C10.4352 12.603 9.56479 12.603 8.86213 12.1911L3.19569 8.86937Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.3793 1.10298C10.1451 0.965675 9.85493 0.965675 9.62071 1.10298L2.37071 5.35298C2.14108 5.48759 2 5.73382 2 6C2 6.26618 2.14108 6.51241 2.37071 6.64702L9.62071 10.897C9.85493 11.0343 10.1451 11.0343 10.3793 10.897L17.6293 6.64702C17.8589 6.51241 18 6.26618 18 6C18 5.73382 17.8589 5.48759 17.6293 5.35298L10.3793 1.10298Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

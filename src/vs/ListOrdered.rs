#[cfg(feature = "VsListOrdered")]
use leptos::*;
#[cfg(feature = "VsListOrdered")]
///This icon requires the feature `VsListOrdered` to be enabled.
#[component]
pub fn ListOrdered(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M2.287 2.326L2.692 2h.677v3h-.708V2.792l-.374.281v-.747zM5 3h10v1H5V3zm0 4h10v1H5V7zm10 4H5v1h10v-1zM3.742 7.626l.029-.039.065-.09a.84.84 0 0 0 .156-.507c0-.12-.02-.24-.057-.354a.848.848 0 0 0-.492-.529 1.123 1.123 0 0 0-.452-.082 1.094 1.094 0 0 0-.458.087.867.867 0 0 0-.479.522A1.038 1.038 0 0 0 2 6.957v.05h.81v-.05a.3.3 0 0 1 .046-.157.174.174 0 0 1 .057-.054.19.19 0 0 1 .172 0 .188.188 0 0 1 .056.06.24.24 0 0 1 .031.081.445.445 0 0 1-.036.29 1.309 1.309 0 0 1-.12.182l-1 1.138-.012.013v.54h1.988v-.7h-.9l.65-.724zm-.037 3.817c.046.032.086.07.12.114a.841.841 0 0 1 .167.55c0 .107-.017.213-.05.314a.792.792 0 0 1-.487.5 1.288 1.288 0 0 1-.48.079c-.115 0-.23-.016-.341-.049a.94.94 0 0 1-.258-.123.751.751 0 0 1-.182-.177 1.063 1.063 0 0 1-.116-.2A1.038 1.038 0 0 1 2 12.078v-.049h.814v.049c0 .027.003.055.009.082a.207.207 0 0 0 .03.074.14.14 0 0 0 .053.052.2.2 0 0 0 .157.008.159.159 0 0 0 .056-.039.22.22 0 0 0 .042-.075.417.417 0 0 0 .017-.126.483.483 0 0 0-.022-.163.2.2 0 0 0-.051-.08.138.138 0 0 0-.06-.029.537.537 0 0 0-.077-.007h-.161v-.645h.168a.241.241 0 0 0 .069-.011.164.164 0 0 0 .065-.034.175.175 0 0 0 .048-.067.286.286 0 0 0 .021-.121.28.28 0 0 0-.016-.1.166.166 0 0 0-.097-.099.2.2 0 0 0-.156.007.164.164 0 0 0-.055.053.344.344 0 0 0-.04.156v.049H2v-.049a.987.987 0 0 1 .18-.544.8.8 0 0 1 .179-.186.87.87 0 0 1 .262-.133c.114-.036.234-.053.354-.051.116-.001.231.01.344.036.092.021.18.055.263.1a.757.757 0 0 1 .32.318.73.73 0 0 1 .09.347.81.81 0 0 1-.167.528.562.562 0 0 1-.12.114z"
        /> < title > { title } < / title > < / svg >
    }
}

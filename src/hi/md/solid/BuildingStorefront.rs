#[cfg(feature = "HiMdSolidBuildingStorefront")]
use leptos::*;
#[cfg(feature = "HiMdSolidBuildingStorefront")]
///This icon requires the feature `HiMdSolidBuildingStorefront` to be enabled.
#[component]
pub fn BuildingStorefront(
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
        "M2.87868 7.12106C4.05025 8.29263 5.94975 8.29263 7.12132 7.12106C7.26529 6.97709 7.39156 6.82213 7.50015 6.65889C8.03763 7.46711 8.95661 7.99977 10 7.99977C11.0435 7.99977 11.9626 7.46697 12.5001 6.65856C12.6087 6.82194 12.7351 6.97702 12.8791 7.12109C14.0507 8.29267 15.9502 8.29267 17.1218 7.12109C18.2933 5.94952 18.2933 4.05003 17.1218 2.87845L16.8291 2.58579C16.454 2.21071 15.9453 2 15.4149 2H4.58552C4.05509 2 3.54638 2.21071 3.17131 2.58579L2.87868 2.87842C1.70711 4.04999 1.70711 5.94949 2.87868 7.12106Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 9.03223C4.42799 9.74067 6.15393 9.64395 7.50057 8.74205C8.21499 9.22007 9.07471 9.49977 10 9.49977C10.9254 9.49977 11.7852 9.22002 12.4996 8.74191C13.8462 9.64388 15.572 9.74073 17 9.03249V16.5H17.25C17.6642 16.5 18 16.8358 18 17.25C18 17.6642 17.6642 18 17.25 18H12.75C12.3358 18 12 17.6642 12 17.25V13.75C12 13.3358 11.6642 13 11.25 13H8.75C8.33579 13 8 13.3358 8 13.75V17.25C8 17.6642 7.66421 18 7.25 18H2.75C2.33579 18 2 17.6642 2 17.25C2 16.8358 2.33579 16.5 2.75 16.5H3V9.03223Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

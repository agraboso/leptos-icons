#[cfg(feature = "HiLgSolidBuildingOffice2")]
use leptos::*;
#[cfg(feature = "HiLgSolidBuildingOffice2")]
///This icon requires the feature `HiLgSolidBuildingOffice2` to be enabled.
#[component]
pub fn BuildingOffice2(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M3 2.25C2.58579 2.25 2.25 2.58579 2.25 3C2.25 3.41421 2.58579 3.75 3 3.75V20.25H2.25C1.83579 20.25 1.5 20.5858 1.5 21C1.5 21.4142 1.83579 21.75 2.25 21.75H15V3.75C15.4142 3.75 15.75 3.41421 15.75 3C15.75 2.58579 15.4142 2.25 15 2.25H3ZM6.75 19.5V17.25C6.75 16.8358 7.08579 16.5 7.5 16.5H10.5C10.9142 16.5 11.25 16.8358 11.25 17.25V19.5C11.25 19.9142 10.9142 20.25 10.5 20.25H7.5C7.08579 20.25 6.75 19.9142 6.75 19.5ZM6 6.75C6 6.33579 6.33579 6 6.75 6H7.5C7.91421 6 8.25 6.33579 8.25 6.75C8.25 7.16421 7.91421 7.5 7.5 7.5H6.75C6.33579 7.5 6 7.16421 6 6.75ZM6.75 9C6.33579 9 6 9.33579 6 9.75C6 10.1642 6.33579 10.5 6.75 10.5H7.5C7.91421 10.5 8.25 10.1642 8.25 9.75C8.25 9.33579 7.91421 9 7.5 9H6.75ZM6 12.75C6 12.3358 6.33579 12 6.75 12H7.5C7.91421 12 8.25 12.3358 8.25 12.75C8.25 13.1642 7.91421 13.5 7.5 13.5H6.75C6.33579 13.5 6 13.1642 6 12.75ZM10.5 6C10.0858 6 9.75 6.33579 9.75 6.75C9.75 7.16421 10.0858 7.5 10.5 7.5H11.25C11.6642 7.5 12 7.16421 12 6.75C12 6.33579 11.6642 6 11.25 6H10.5ZM9.75 9.75C9.75 9.33579 10.0858 9 10.5 9H11.25C11.6642 9 12 9.33579 12 9.75C12 10.1642 11.6642 10.5 11.25 10.5H10.5C10.0858 10.5 9.75 10.1642 9.75 9.75ZM10.5 12C10.0858 12 9.75 12.3358 9.75 12.75C9.75 13.1642 10.0858 13.5 10.5 13.5H11.25C11.6642 13.5 12 13.1642 12 12.75C12 12.3358 11.6642 12 11.25 12H10.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M16.5 6.75V21.75H21.75C22.1642 21.75 22.5 21.4142 22.5 21C22.5 20.5858 22.1642 20.25 21.75 20.25H21V8.25C21.4142 8.25 21.75 7.91421 21.75 7.5C21.75 7.08579 21.4142 6.75 21 6.75H16.5ZM18 11.25C18 10.8358 18.3358 10.5 18.75 10.5H18.7575C19.1717 10.5 19.5075 10.8358 19.5075 11.25V11.2575C19.5075 11.6717 19.1717 12.0075 18.7575 12.0075H18.75C18.3358 12.0075 18 11.6717 18 11.2575V11.25ZM18.75 13.5C18.3358 13.5 18 13.8358 18 14.25V14.2575C18 14.6717 18.3358 15.0075 18.75 15.0075H18.7575C19.1717 15.0075 19.5075 14.6717 19.5075 14.2575V14.25C19.5075 13.8358 19.1717 13.5 18.7575 13.5H18.75ZM18 17.25C18 16.8358 18.3358 16.5 18.75 16.5H18.7575C19.1717 16.5 19.5075 16.8358 19.5075 17.25V17.2575C19.5075 17.6717 19.1717 18.0075 18.7575 18.0075H18.75C18.3358 18.0075 18 17.6717 18 17.2575V17.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

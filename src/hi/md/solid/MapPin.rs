#[cfg(feature = "HiMdSolidMapPin")]
use leptos::*;
#[cfg(feature = "HiMdSolidMapPin")]
///This icon requires the feature `HiMdSolidMapPin` to be enabled.
#[component]
pub fn MapPin(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M9.69008 18.933L9.69288 18.9342C9.89 19.02 10 19 10 19C10 19 10.11 19.02 10.3079 18.9339L10.3099 18.933L10.3157 18.9304L10.3338 18.922C10.3488 18.915 10.3697 18.9052 10.3959 18.8926C10.4484 18.8673 10.5225 18.8306 10.6153 18.7822C10.8008 18.6855 11.0612 18.5419 11.3717 18.3495C11.9912 17.9655 12.8174 17.3826 13.6455 16.5844C15.3022 14.9876 17 12.4925 17 9C17 5.13401 13.866 2 10 2C6.13401 2 3 5.13401 3 9C3 12.4925 4.69783 14.9876 6.35452 16.5844C7.18264 17.3826 8.00877 17.9655 8.62834 18.3495C8.93879 18.5419 9.19922 18.6855 9.38467 18.7822C9.47745 18.8306 9.55163 18.8673 9.60409 18.8926C9.63033 18.9052 9.65116 18.915 9.66619 18.922L9.68435 18.9304L9.69008 18.933ZM10 11.25C11.2426 11.25 12.25 10.2426 12.25 9C12.25 7.75736 11.2426 6.75 10 6.75C8.75736 6.75 7.75 7.75736 7.75 9C7.75 10.2426 8.75736 11.25 10 11.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "HiLgSolidLightBulb")]
use leptos::*;
#[cfg(feature = "HiLgSolidLightBulb")]
///This icon requires the feature `HiLgSolidLightBulb` to be enabled.
#[component]
pub fn LightBulb(
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
        "http://www.w3.org/2000/svg" d =
        "M12 0.75C7.44365 0.75 3.75 4.44365 3.75 9C3.75 12.0508 5.40631 14.7138 7.86516 16.1405C8.55062 16.5382 8.97985 17.1484 8.99928 17.7626C9.00999 18.1013 9.24656 18.3908 9.5764 18.4687C9.92778 18.5518 10.2859 18.6171 10.6496 18.6639C10.9732 18.7055 11.2502 18.4462 11.2502 18.12V13.4589C10.9309 13.4236 10.618 13.366 10.3132 13.2875C9.9121 13.1843 9.67061 12.7754 9.77385 12.3743C9.8771 11.9731 10.286 11.7316 10.6871 11.8349C11.1059 11.9427 11.5458 12.0002 12.0002 12.0002C12.4546 12.0002 12.8944 11.9427 13.3132 11.8349C13.7144 11.7316 14.1233 11.9731 14.2265 12.3743C14.3298 12.7754 14.0883 13.1843 13.6871 13.2875C13.3823 13.366 13.0695 13.4236 12.7502 13.4589V18.1199C12.7502 18.4462 13.0272 18.7054 13.3508 18.6638C13.7144 18.6171 14.0723 18.5518 14.4236 18.4687C14.7534 18.3908 14.99 18.1013 15.0007 17.7626C15.0201 17.1484 15.4494 16.5382 16.1348 16.1405C18.5937 14.7138 20.25 12.0508 20.25 9C20.25 4.44365 16.5563 0.75 12 0.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M9.01314 19.8997C9.09034 19.4927 9.48282 19.2254 9.88978 19.3026C10.5727 19.4321 11.2781 19.5 12 19.5C12.7219 19.5 13.4273 19.4321 14.1102 19.3026C14.5172 19.2254 14.9097 19.4927 14.9869 19.8997C15.0641 20.3066 14.7967 20.6991 14.3898 20.7763C13.6151 20.9232 12.8162 21 12 21C11.1838 21 10.3849 20.9232 9.61022 20.7763C9.20327 20.6991 8.93594 20.3066 9.01314 19.8997Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M9.75407 22.344C9.79717 21.932 10.1661 21.633 10.578 21.6761C11.0451 21.7249 11.5195 21.75 12 21.75C12.4805 21.75 12.9549 21.7249 13.422 21.6761C13.8339 21.633 14.2028 21.932 14.2459 22.344C14.289 22.7559 13.99 23.1248 13.578 23.1679C13.0592 23.2222 12.5327 23.25 12 23.25C11.4673 23.25 10.9408 23.2222 10.422 23.1679C10.01 23.1248 9.71097 22.7559 9.75407 22.344Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

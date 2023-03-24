#[cfg(feature = "CgEditMask")]
use leptos::*;
#[cfg(feature = "CgEditMask")]
///This icon requires the feature `CgEditMask` to be enabled.
#[component]
pub fn EditMask(
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
        "M12 14C13.1046 14 14 13.1046 14 12C14 10.8954 13.1046 10 12 10C10.8954 10 10 10.8954 10 12C10 13.1046 10.8954 14 12 14Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M12 22C6.47716 22 2 17.5228 2 12C2 6.47717 6.47716 2 12 2C17.5228 2 22 6.47717 22 12C22 17.5228 17.5228 22 12 22ZM9.0307 19.4308C9.01047 19.29 9 19.1462 9 19C9 17.3431 10.3431 16 12 16C13.6569 16 15 17.3431 15 19C15 19.1462 14.9895 19.29 14.9693 19.4308C16.9993 18.6189 18.619 16.9993 19.4308 14.9692C19.2901 14.9895 19.1463 15 19 15C17.3431 15 16 13.6569 16 12C16 10.3431 17.3431 9 19 9C19.0637 9 19.127 9.00195 19.1897 9.00586C19.271 9.01099 19.3514 9.01929 19.4308 9.03076C18.6189 7.00073 16.9993 5.3811 14.9693 4.56921C14.9895 4.70996 15 4.85376 15 5C15 6.65686 13.6569 8 12 8C10.3431 8 9 6.65686 9 5C9 4.85376 9.01048 4.70996 9.03072 4.56921C7.0007 5.3811 5.38106 7.00073 4.56915 9.03076C4.64857 9.01929 4.729 9.01099 4.8103 9.00586C4.87303 9.00195 4.93628 9 5 9C6.65686 9 8 10.3431 8 12C8 13.6569 6.65686 15 5 15C4.8537 15 4.70985 14.9895 4.56915 14.9692C5.38104 16.9993 7.00069 18.6189 9.0307 19.4308Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}

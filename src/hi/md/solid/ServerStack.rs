#[cfg(feature = "HiMdSolidServerStack")]
use leptos::*;
#[cfg(feature = "HiMdSolidServerStack")]
///This icon requires the feature `HiMdSolidServerStack` to be enabled.
#[component]
pub fn ServerStack(
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
        "M4.46363 3.16188C4.79054 2.45358 5.49945 2 6.27955 2H13.7202C14.5003 2 15.2092 2.45358 15.5361 3.16188L16.69 5.66188C16.7567 5.80652 16.8047 5.9534 16.8352 6.1003C16.5675 6.03476 16.2878 6 15.9999 6H3.99989C3.712 6 3.43223 6.03476 3.16455 6.10031C3.19509 5.9534 3.24303 5.80653 3.30979 5.66188L4.46363 3.16188Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M2 9.5C2 8.39543 2.89543 7.5 4 7.5H16C17.1046 7.5 18 8.39543 18 9.5C18 10.6046 17.1046 11.5 16 11.5H4C2.89543 11.5 2 10.6046 2 9.5ZM15.2402 9.5C15.2402 9.08579 15.576 8.75 15.9902 8.75H16.0002C16.4144 8.75 16.7502 9.08579 16.7502 9.5V9.51C16.7502 9.92421 16.4144 10.26 16.0002 10.26H15.9902C15.576 10.26 15.2402 9.92421 15.2402 9.51V9.5ZM12.9902 8.75C12.576 8.75 12.2402 9.08579 12.2402 9.5V9.51C12.2402 9.92421 12.576 10.26 12.9902 10.26H13.0002C13.4144 10.26 13.7502 9.92421 13.7502 9.51V9.5C13.7502 9.08579 13.4144 8.75 13.0002 8.75H12.9902Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M2 15C2 13.8954 2.89543 13 4 13H16C17.1046 13 18 13.8954 18 15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15ZM15.2402 15C15.2402 14.5858 15.576 14.25 15.9902 14.25H16.0002C16.4144 14.25 16.7502 14.5858 16.7502 15V15.01C16.7502 15.4242 16.4144 15.76 16.0002 15.76H15.9902C15.576 15.76 15.2402 15.4242 15.2402 15.01V15ZM12.9902 14.25C12.576 14.25 12.2402 14.5858 12.2402 15V15.01C12.2402 15.4242 12.576 15.76 12.9902 15.76H13.0002C13.4144 15.76 13.7502 15.4242 13.7502 15.01V15C13.7502 14.5858 13.4144 14.25 13.0002 14.25H12.9902Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

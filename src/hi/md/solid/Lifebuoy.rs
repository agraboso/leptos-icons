#[cfg(feature = "HiMdSolidLifebuoy")]
use leptos::*;
#[cfg(feature = "HiMdSolidLifebuoy")]
///This icon requires the feature `HiMdSolidLifebuoy` to be enabled.
#[component]
pub fn Lifebuoy(
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
        "M7.17111 4.1457L9.11848 6.61237C9.69601 6.46254 10.304 6.46254 10.8815 6.61237L12.8289 4.1457C11.0472 3.28477 8.95276 3.28477 7.17111 4.1457ZM15.8543 7.17111L13.3876 9.11848C13.5375 9.69601 13.5375 10.304 13.3876 10.8815L15.8543 12.8289C16.7152 11.0472 16.7152 8.95276 15.8543 7.17111ZM12.8289 15.8543L10.8815 13.3876C10.304 13.5375 9.69601 13.5375 9.11848 13.3876L7.17111 15.8543C8.95276 16.7152 11.0472 16.7152 12.8289 15.8543ZM4.1457 12.8289L6.61237 10.8815C6.46254 10.304 6.46254 9.69601 6.61237 9.11848L4.1457 7.17111C3.28477 8.95276 3.28477 11.0472 4.1457 12.8289ZM5.63091 3.29667C8.27818 1.56778 11.7218 1.56778 14.3691 3.29667C14.8253 3.59461 15.2573 3.94356 15.6569 4.34315C16.0564 4.74273 16.4054 5.17471 16.7033 5.63091C18.4322 8.27817 18.4322 11.7218 16.7033 14.3691C16.4054 14.8253 16.0564 15.2573 15.6569 15.6569C15.2573 16.0564 14.8253 16.4054 14.3691 16.7033C11.7218 18.4322 8.27818 18.4322 5.63091 16.7033C5.1747 16.4054 4.74273 16.0564 4.34315 15.6569C3.94356 15.2573 3.59461 14.8253 3.29667 14.3691C1.56778 11.7218 1.56778 8.27818 3.29667 5.63091C3.59461 5.17471 3.94356 4.74273 4.34315 4.34315C4.74273 3.94356 5.17471 3.59461 5.63091 3.29667ZM10.8287 8.17897C10.3041 7.94035 9.69587 7.94035 9.17126 8.17897C8.95999 8.27506 8.76133 8.41024 8.58579 8.58579C8.41024 8.76133 8.27506 8.95999 8.17897 9.17126C7.94035 9.69587 7.94035 10.3041 8.17897 10.8287C8.27506 11.04 8.41024 11.2387 8.58579 11.4142C8.76133 11.5898 8.95999 11.7249 9.17126 11.821C9.69587 12.0597 10.3041 12.0597 10.8287 11.821C11.04 11.7249 11.2387 11.5898 11.4142 11.4142C11.5898 11.2387 11.7249 11.04 11.821 10.8287C12.0597 10.3041 12.0597 9.69587 11.821 9.17126C11.7249 8.95999 11.5898 8.76133 11.4142 8.58579C11.2387 8.41024 11.04 8.27506 10.8287 8.17897Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}

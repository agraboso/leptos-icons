#[cfg(feature = "HiLgOutlineCake")]
use leptos::*;
#[cfg(feature = "HiLgOutlineCake")]
///This icon requires the feature `HiLgOutlineCake` to be enabled.
#[component]
pub fn Cake(
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
        "M12 8.25006V6.75006M12 8.25006C10.6448 8.25006 9.30281 8.30622 7.97608 8.41633C6.84499 8.51021 6 9.47329 6 10.6083V13.1214M12 8.25006C13.3552 8.25006 14.6972 8.30622 16.0239 8.41633C17.155 8.51021 18 9.47329 18 10.6083V13.1214M15 8.25006V6.75006M9 8.25006V6.75006M21 16.5001L19.5 17.2501C18.5557 17.7222 17.4443 17.7222 16.5 17.2501C15.5557 16.7779 14.4443 16.7779 13.5 17.2501C12.5557 17.7222 11.4443 17.7222 10.5 17.2501C9.55573 16.7779 8.44427 16.7779 7.5 17.2501C6.55573 17.7222 5.44427 17.7222 4.5 17.2501L3 16.5001M18 13.1214C16.0344 12.8763 14.032 12.7501 12 12.7501C9.96804 12.7501 7.96557 12.8763 6 13.1214M18 13.1214C18.3891 13.1699 18.7768 13.2231 19.163 13.2809C20.2321 13.4409 21 14.3748 21 15.4557V20.6251C21 21.2464 20.4963 21.7501 19.875 21.7501H4.125C3.50368 21.7501 3 21.2464 3 20.6251V15.4557C3 14.3748 3.76793 13.4409 4.83697 13.2809C5.22316 13.2231 5.61086 13.1699 6 13.1214M12.2652 3.10989C12.4117 3.25634 12.4117 3.49378 12.2652 3.64022C12.1188 3.78667 11.8813 3.78667 11.7349 3.64022C11.5884 3.49378 11.5884 3.25634 11.7349 3.10989C11.8104 3.03435 12.0001 2.84473 12.0001 2.84473C12.0001 2.84473 12.1943 3.039 12.2652 3.10989ZM9.26522 3.10989C9.41167 3.25634 9.41167 3.49378 9.26522 3.64022C9.11878 3.78667 8.88134 3.78667 8.73489 3.64022C8.58844 3.49378 8.58844 3.25634 8.73489 3.10989C8.81044 3.03435 9.00005 2.84473 9.00005 2.84473C9.00005 2.84473 9.19432 3.039 9.26522 3.10989ZM15.2652 3.10989C15.4117 3.25634 15.4117 3.49378 15.2652 3.64022C15.1188 3.78667 14.8813 3.78667 14.7349 3.64022C14.5884 3.49378 14.5884 3.25634 14.7349 3.10989C14.8104 3.03435 15.0001 2.84473 15.0001 2.84473C15.0001 2.84473 15.1943 3.039 15.2652 3.10989Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}

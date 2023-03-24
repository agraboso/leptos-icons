#[cfg(feature = "AiTwotoneReconciliation")]
use leptos::*;
#[cfg(feature = "AiTwotoneReconciliation")]
///This icon requires the feature `AiTwotoneReconciliation` to be enabled.
#[component]
pub fn Reconciliation(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = { size.clone()
        } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill =
        "#D9D9D9" d =
        "M740 344H404V240H304v160h176c17.7 0 32 14.3 32 32v360h328V240H740v104zM584 448c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v56c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8v-56zm92 301c-50.8 0-92-41.2-92-92s41.2-92 92-92 92 41.2 92 92-41.2 92-92 92zm92-341v96c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8v-96c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "#D9D9D9" d =
        "M642 657a34 34 0 1 0 68 0 34 34 0 1 0-68 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M592 512h48c4.4 0 8-3.6 8-8v-56c0-4.4-3.6-8-8-8h-48c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8zm112-104v96c0 4.4 3.6 8 8 8h48c4.4 0 8-3.6 8-8v-96c0-4.4-3.6-8-8-8h-48c-4.4 0-8 3.6-8 8z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M880 168H668c0-30.9-25.1-56-56-56h-80c-30.9 0-56 25.1-56 56H264c-17.7 0-32 14.3-32 32v200h-88c-17.7 0-32 14.3-32 32v448c0 17.7 14.3 32 32 32h336c17.7 0 32-14.3 32-32v-16h368c17.7 0 32-14.3 32-32V200c0-17.7-14.3-32-32-32zm-412 64h72v-56h64v56h72v48H468v-48zm-20 616H176V616h272v232zm0-296H176v-88h272v88zm392 240H512V432c0-17.7-14.3-32-32-32H304V240h100v104h336V240h100v552z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M676 565c-50.8 0-92 41.2-92 92s41.2 92 92 92 92-41.2 92-92-41.2-92-92-92zm0 126c-18.8 0-34-15.2-34-34s15.2-34 34-34 34 15.2 34 34-15.2 34-34 34z"
        /> < title > { title } < / title > < / svg >
    }
}

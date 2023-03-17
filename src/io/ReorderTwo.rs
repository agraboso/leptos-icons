#[cfg(feature = "IoReorderTwo")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoReorderTwo")]
/// *This icon requires the feature* `IoReorderTwo` *to be enabled*.
#[component]
pub fn ReorderTwo(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><line x1="118" y1="304" x2="394" y2="304" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px" /><line x1="118" y1="208" x2="394" y2="208" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px" /></svg>
   }
}
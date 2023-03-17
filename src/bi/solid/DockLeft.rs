#[cfg(feature = "BiSolidDockLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDockLeft")]
/// *This icon requires the feature* `BiSolidDockLeft` *to be enabled*.
#[component]
pub fn DockLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21 19V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2zm-11 0V5h9l.002 14H10z" /></svg>
   }
}
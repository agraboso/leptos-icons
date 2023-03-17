#[cfg(feature = "VsDiffModified")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsDiffModified")]
/// *This icon requires the feature* `VsDiffModified` *to be enabled*.
#[component]
pub fn DiffModified(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 1h13l.5.5v13l-.5.5h-13l-.5-.5v-13l.5-.5zM2 2v12h12V2H2zm6 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" /></svg>
   }
}
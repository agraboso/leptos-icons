#[cfg(feature = "SiSpinrilla")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSpinrilla")]
/// *This icon requires the feature* `SiSpinrilla` *to be enabled*.
#[component]
pub fn Spinrilla(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 8.816a3.184 3.184 0 1 0 0 6.368 3.184 3.184 0 1 0 0-6.368zM12 0v3.918A8.082 8.082 0 0 0 3.918 12H0A12 12 0 0 1 12 0zm0 24v-3.918A8.082 8.082 0 0 0 20.082 12H24a12 12 0 0 1-12 12z" /></svg>
   }
}
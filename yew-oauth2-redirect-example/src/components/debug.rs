use yew::prelude::*;
use yew_oauth2::prelude::*;

#[component(Debug)]
pub fn debug() -> Html {
    let auth = use_context::<OAuth2Context>();

    html!(
        <div>
            if let Some(auth) = auth {
                <code><pre>{ format!("{auth:#?}") }</pre></code>
            } else {
                { "OAuth2 context not found." }
            }
        </div>
    )
}

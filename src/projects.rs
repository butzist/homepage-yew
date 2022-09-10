use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html!(
      <div class="columns">
        <div class="column card">
          <div class="card-image">
            <figure class="image is-4by3">
              <img src="https://bulma.io/images/placeholders/1280x960.png" alt="Placeholder image" />
            </figure>
          </div>
          <div class="card-content">
            <div class="media">
              <div class="media-left">
                <figure class="image is-48x48">
                  <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image" />
                </figure>
              </div>
              <div class="media-content">
                <p class="title is-4">{ "John Smith" }</p>
                <p class="subtitle is-6">{ "@johnsmith" }</p>
              </div>
            </div>

            <div class="content">
              { "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
              Phasellus nec iaculis mauris." }
            </div>
          </div>
        </div>
        <div class="column card">
        { "Test?" }
        </div>
      </div>
    )
}

use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use uuid::Uuid;
use web_sys::{HtmlInputElement, KeyboardEvent};
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::prelude::*;
use std::time::Duration;
use sycamore::easing;
use sycamore::motion::{create_raf, create_tweened_signal};




/////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let document = web_sys::window().unwrap().document().unwrap();
    let mount_lista = document.query_selector("#listas").unwrap().unwrap();
    let mount_tabla = document.query_selector("#tabla").unwrap().unwrap();

    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
  
    sycamore::render_to(AppContador, &mount_tabla); 
    //sycamore::render(AppCuadro); 
    //sycamore::render(AppRango); 
    //sycamore::render(Ultima);
    //sycamore::render_to(AppTabla, &mount_tabla);
    //sycamore::render_to(AppLista, &mount_lista);  
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[component]
fn Counter<G: Html>() -> View<G> {
    let mut state = create_signal(0i32);
    let increment = move |_| state += 1;
    let decrement = move |_| state -= 1;
    let reset = move |_| state.set(0);
    view! {
        div(class="jumbotron") {
            div(class="row") {
                div(class="col-md-6") { h1 { "Valor: " (state.get()) } }
                div(class="col-md-6") {
                    div(class="row") {
                        button(class="btn btn-primary btn-block", on:click=increment) { "Incrementar" }
                    }
                    div(class="row") {
                        button(class="btn btn-primary btn-block", on:click=decrement) { "Restar" }
                    }
                    div(class="row") {
                        button(class="btn btn-primary btn-block", on:click=reset) { "Reset" }
                    }
                }
            }
        }
    }
}

#[component]
fn Hello<G: Html>() -> View<G> {
    let name = create_signal(String::new());
    let is_empty = create_selector(move || !name.with(String::is_empty));

    view! {
        div {
            h4 {
                "Hola "
                (if is_empty.get() {
                    view! {
                        h4 { (name.get_clone()) }
                    }
                } else {
                    view! { h4 { "Mundo" } }
                })
            }
            input(bind:value=name)
        }
    }
}

#[component]
fn AppContador<G: Html>() -> View<G> {
    view! {
        Hello {}
        br {}
        Counter {}
    }
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[component]
fn AppCuadro<G: Html>() -> View<G> {
    let rotate = create_tweened_signal(0f64, Duration::from_millis(800), easing::quad_inout);

    view! {
        button(class="btn btn-primary btn-block", disabled=rotate.is_tweening(), on:click=move |_| rotate.set(rotate.get() + 0.5)) { "Media Vuelta" }
        button(class="btn btn-primary btn-block", disabled=rotate.is_tweening(), on:click=move |_| rotate.set(rotate.get() + 1.0)) { "Vuelta completa" }
        button(class="btn btn-primary btn-block", disabled=rotate.is_tweening(), on:click=move |_| rotate.set(rotate.get() + 2.0)) { "Dos Vueltas" }
        button(class="btn btn-primary btn-block", disabled=rotate.is_tweening(), on:click=move |_| rotate.set(rotate.get() + 3.0)) { "Tres Vueltas" }
        svg(height="500", width="500") {
            rect(
                x="50", y="50",
                width="200", height="200",
                fill="red", transform=format!("rotate({}, 150, 150)", (rotate.get() * 360.0) as u32)
            )
        }
    }
}

//, xmlns="http://www.w3.org/2000/svg"
/////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn AppRango<G: Html>() -> View<G> {
    let value = create_signal(10.0);

    view! {
        h1 { (format!("{:.2}", value.get())) }

        input(type="range", min="1", step="0.25", max="10", bind:valueAsNumber=value) {}
        br {}
        input(type="number", min="1", step="0.25", max="10", bind:valueAsNumber=value) {}
    }
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[component]
fn ContarFrames<G: Html>() -> View<G> {
    let mut state = create_signal(0i32);
    let (_running, start, stop) = create_raf(move || { state += 1; });
    view! {
        div {
            h2 { (state.get()) " FRAMES" }
            button(class="btn btn-success btn-block", on:click=move |_| start()) { "Iniciar" }
            button(class="btn btn-success btn-block", on:click=move |_| stop()) { "Parar" }
        }
    }
}

#[component]
fn Barras<G: Html>() -> View<G> {
    let progress = create_tweened_signal([0.0f32, 1.0], Duration::from_millis(250), easing::quad_out);

    view! {
        div {
            style {
                r#"
                progress {
                    display: block;
                    width: 150%;
                }
                "#
            }
            progress(prop:value=progress.get()[0])
            progress(prop:value=progress.get()[1])

            button(class="btn btn-primary btn-block", on:click=move |_| progress.set([0.0, 1.0])) { "0%" }
            button(class="btn btn-primary btn-block", on:click=move |_| progress.set([0.25, 0.75])) { "25%" }
            button(class="btn btn-primary btn-block", on:click=move |_| progress.set([0.5, 0.5])) { "50%" }
            button(class="btn btn-primary btn-block", on:click=move |_| progress.set([0.75, 0.25])) { "75%" }
            button(class="btn btn-primary btn-block", on:click=move |_| progress.set([1.0, 0.0])) { "100%" }
        }
    }
}

#[component]
fn Ultima<G: Html>() -> View<G> {

        view! {
            h2 { "Conteo de Frames" }
            ContarFrames {}
            br {}
            h2 { "Barras de progreso" }
            Barras {}
        }
}




/////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Todo {
    title: String,
    completed: bool,
    id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

impl Filter {
    fn url(self) -> &'static str {
        match self {
            Filter::All => "#",
            Filter::Active => "#/activas",
            Filter::Completed => "#/completadas",
        }
    }

    fn get_filter_from_hash() -> Self {
        let hash = web_sys::window().unwrap().location().hash().unwrap();

        match hash.as_str() {
            "#/activas" => Filter::Active,
            "#/completadas" => Filter::Completed,
            _ => Filter::All,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AppState {
    pub todos: Signal<Vec<Signal<Todo>>>,
    pub filter: Signal<Filter>,
}

impl AppState {
    fn add_todo(&self, title: String) {
        let new = create_signal(Todo {
            title,
            completed: false,
            id: Uuid::new_v4(),
        });
        self.todos.update(|todos| todos.push(new));
    }

    fn remove_todo(&self, id: Uuid) {
        self.todos.update(|todos| todos.retain(|todo| todo.with(|todo| todo.id) != id));
    }

    fn todos_left(&self) -> usize {
        self.todos.with(|todos| {
            todos.iter().fold(0, |acc, todo| {
                if todo.with(|todo| todo.completed) {
                    acc
                } else {
                    acc + 1
                }
            })
        })
    }

    fn toggle_complete_all(&self) {
        if self.todos_left() == 0 {
            for todo in self.todos.get_clone() {
                if todo.with(|todo| todo.completed) {
                    todo.set(Todo {
                        completed: false, ..todo.get_clone()
                    })
                }
            }
        } else {
            for todo in self.todos.get_clone() {
                if !todo.with(|todo| todo.completed) {
                    todo.set(Todo {
                        completed: true, ..todo.get_clone()
                    })
                }
            }
        }
    }

    fn clear_completed(&self) {
        self.todos.update(|todos| todos.retain(|todo| !todo.with(|todo| todo.completed)));
    }
}

const KEY: &str = "todos-sycamore";

#[component]
fn AppLista<G: Html>() -> View<G> {
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("Usuario no permitido");

    let todos = if let Ok(Some(app_state)) = local_storage.get_item(KEY) {
        serde_json::from_str(&app_state).unwrap_or_default()
    } else {
        Default::default()
    };
    let app_state = AppState {
        todos,
        filter: create_signal(Filter::get_filter_from_hash()),
    };
    provide_context(app_state);


    create_effect(move || {
        app_state.todos.with(|todos| {
            for todo in todos {
                todo.track();
            }
            local_storage
                .set_item(KEY, &serde_json::to_string(todos).unwrap())
                .unwrap();
        });
    });

    let todos_empty = create_selector(move || app_state.todos.with(Vec::is_empty));

    view! {
        div(class="todomvc-wrapper") {
            section(class="todoapp") {
                Header {}
                ((!todos_empty.get()).then(|| view! {
                    List {}
                    Footer {}
                }))
            }
        }
    }
}

#[component]
pub fn Header<G: Html>() -> View<G> {
    let app_state = use_context::<AppState>();
    let input_value = create_signal(String::new());

    let handle_keyup = move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            let task = input_value.with(|task| task.trim().to_string());

            if !task.is_empty() {
                app_state.add_todo(task);
                input_value.set("".to_string());
            }
        }
    };

    view! {
        header(class="header") {
            h1 { "Listas" }
            input(class="new-todo",
                placeholder="Agregar elemento a la lista",
                bind:value=input_value,
                on:keyup=handle_keyup,
            )
        }
    }
}

#[component(inline_props)]
pub fn Item<G: Html>(todo: Signal<Todo>) -> View<G> {
    let app_state = use_context::<AppState>();

    let title = move || todo.with(|todo| todo.title.clone());
    let completed = create_selector(move || todo.with(|todo| todo.completed));
    let id = todo.with(|todo| todo.id);

    let is_editing = create_signal(false);
    let input_ref = create_node_ref();
    let input_value = create_signal("".to_string());

    let toggle_completed = move |_| todo.update(|todo| todo.completed = !todo.completed);

    let handle_dblclick = move |_| {
        is_editing.set(true);
        input_ref
            .get::<DomNode>()
            .unchecked_into::<HtmlInputElement>()
            .focus()
            .unwrap();
        input_value.set(title());
    };

    let handle_blur = move || {
        is_editing.set(false);

        let value = input_value.with(|value| value.trim().to_string());

        if value.is_empty() {
            app_state.remove_todo(id);
        } else {
            todo.update(|todo| todo.title = value)
        }
    };

    let handle_keyup = move |event: KeyboardEvent| match event.key().as_str() {
        "Enter" => handle_blur(),
        "Escape" => is_editing.set(false),
        _ => {}
    };

    let handle_destroy = move |_| {
        app_state.remove_todo(id);
    };

    let checked = create_signal(false);
    create_effect(move || {
        checked.set(completed.get())
    });

    let class = move || {
        format!(
            "{} {}",
            if completed.get() { "completed" } else { "" },
            if is_editing.get() { "editing" } else { "" }
        )
    };

    view! {
        li(class=class()) {
            div(class="view") {
                input(
                    class="toggle",
                    type="checkbox",
                    on:input=toggle_completed,
                    bind:checked=checked
                )
                label(on:dblclick=handle_dblclick) {
                    (title())
                }
                button(class="destroy", on:click=handle_destroy)
            }

            (is_editing.get().then(|| view! {
                input(ref=input_ref,
                    class="edit",
                    bind:value=input_value,
                    on:blur=move |_| handle_blur(),
                    on:keyup=handle_keyup,
                )
            }))
        }
    }
}

#[component]
pub fn List<G: Html>() -> View<G> {
    let app_state = use_context::<AppState>();
    let todos_left = create_selector(move || app_state.todos_left());

    let filtered_todos = create_memo(move || {
        app_state
            .todos
            .get_clone()
            .iter()
            .filter(|todo| match app_state.filter.get() {
                Filter::All => true,
                Filter::Active => !todo.with(|todo| todo.completed),
                Filter::Completed => todo.with(|todo| todo.completed),
            })
            .cloned()
            .collect::<Vec<_>>()
    });


    let checked = create_signal(false);
    create_effect(move || {
        checked.set(todos_left.get() == 0)
    });

    view! {
        section(class="main") {
            input(
                id="toggle-all",
                class="toggle-all",
                type="checkbox",
                readonly=true,
                bind:checked=checked,
                on:input=move |_| app_state.toggle_complete_all()
            )
            label(for="toggle-all")

            ul(class="todo-list") {
                Keyed(
                    iterable=filtered_todos,
                    view=|todo| view! {
                        Item(todo=todo)
                    },
                    key=|todo| todo.with(|todo| todo.id),
                )
            }
        }
    }
}

#[component(inline_props)]
pub fn TodoFilter<G: Html>(filter: Filter) -> View<G> {
    let app_state = use_context::<AppState>();
    let selected = move || filter == app_state.filter.get();
    let set_filter = move |filter| app_state.filter.set(filter);

    view! {
        li {
            a(
                class=if selected() { "selected" } else { "" },
                href=filter.url(),
                on:click=move |_| set_filter(filter),
            ) {
                (format!("{filter:?}"))
            }
        }
    }
}

#[component]
pub fn Footer<G: Html>() -> View<G> {
    let app_state = use_context::<AppState>();

    let items_text = move || match app_state.todos_left() {
        1 => "item",
        _ => "items",
    };

    let has_completed_todos =
        create_selector(move || app_state.todos_left() < app_state.todos.with(Vec::len));

    let handle_clear_completed = move |_| app_state.clear_completed();

    view! {
        footer(class="footer") {
            span(class="todo-count") {
                strong { (app_state.todos_left()) }
                span { " " (items_text()) " izq." }
            }
            ul(class="filters") {
                TodoFilter(filter=Filter::All)
                TodoFilter(filter=Filter::Active)
                TodoFilter(filter=Filter::Completed)
            }

            (has_completed_todos.get().then(|| view! {
                button(class="clear-completed", on:click=handle_clear_completed) {
                    "Eliminar"
                }
            }))
        }
    }
}


/////////////////////////////////////////////////////////////////////////////////////////////////////////////
static ADJECTIVES: &[&str] = &[
    "bonito",
    "grande",
    "grande",
    "pequeño",
    "alto",
    "bajo",
    "largo",
    "guapo",
    "sencillo",
    "pintoresco",
    "limpio",
    "elegante",
    "fácil",
    "enojado",
    "loco",
    "útil",
    "sensiblero",
    "raro",
    "antiestético",
    "adorable",
    "importante",
    "barato",
    "barato",
    "caro",
    "elegante",
];

static COLOURS: &[&str] = &[
    "rojo", "amarillo", "azul", "verde", "rosa", "marrón", "morado", "marrón", "blanco", "negro",
 "naranja",
];

static NOUNS: &[&str] = &[
    "mesa", "silla", "casa", "barbacoa", "escritorio", "coche", "poni", "galleta", "bocadillo", "hamburguesa", "pizza", "ratón", "teclado",
];

#[component(inline_props)]
fn Button<G: Html>(id: &'static str, text: &'static str, callback: Box<dyn Fn()>) -> View<G> {
    view! {
        div(class="col-sm-6 smallpad") {
            button(id=id, class="btn btn-primary btn-block", type="button", on:click=move |_| callback()) {
                (text)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RowData {
    id: usize,
    label: Signal<String>,
}

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn build_data(count: usize) -> Vec<RowData> {
    let mut thread_rng = thread_rng();

    let mut data = Vec::new();
    data.reserve_exact(count);

    for _i in 0..count {
        let adjective = ADJECTIVES.choose(&mut thread_rng).unwrap();
        let colour = COLOURS.choose(&mut thread_rng).unwrap();
        let noun = NOUNS.choose(&mut thread_rng).unwrap();
        let capacity = adjective.len() + colour.len() + noun.len() + 2;
        let mut label = String::with_capacity(capacity);
        label.push_str(adjective);
        label.push(' ');
        label.push_str(colour);
        label.push(' ');
        label.push_str(noun);

        data.push(RowData {
            id: ID_COUNTER.load(Ordering::Relaxed),
            label: create_signal(label),
        });

        ID_COUNTER.store(ID_COUNTER.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
    }

    data
}

#[component]
fn AppTabla<G: Html>() -> View<G> {
    let data = create_signal(Vec::<RowData>::new());
    let selected = create_signal(None::<usize>);

    let remove = move |id| data.update(|d| d.retain(|row| row.id != id));

    let run = move || {
        selected.set(None);
        data.set(build_data(10));
    };

    let runlots = move || {
        selected.set(None);
        data.set(build_data(5));
    };

    let add = move || {
        let new = build_data(1);
        data.update(|d| d.extend(new));
    };

    let update = move || {
        let d = data.get_clone();
        for row in d.into_iter().step_by(4) {
            row.label.update(|l| *l = format!("{} !!!", l));
        }
    };

    let clear = move || {
        data.set(Vec::new());
        selected.set(None);
    };

    let swaprows = move || {
        data.update(|d| {
            if d.len() > 4 {
                d.swap(1, 4);
            }
        })
    };

    view! {
        div(class="container") {
            div(class="jumbotron") {
                div(class="row") {
                    div(class="col-md-6") { h1 { "Crear Columnas" } }
                    div(class="col-md-6") {
                        div(class="row") {
                            Button(id="run", text="Crea 10 columnas", callback=Box::new(run))
                            Button(id="runlots", text="Crea 5 Columnas", callback=Box::new(runlots))
                            Button(id="add", text="Agrega 1 Columna", callback=Box::new(add))
                            Button(id="update", text="Actualiza 2 columnas", callback=Box::new(update))
                            Button(id="clear", text="Eliminar", callback=Box::new(clear))
                            Button(id="swaprows", text="Intercambiar 2 filas", callback=Box::new(swaprows))
                        }
                    }
                }
            }
            table(class="table table-hover table-striped test-data") {
                tbody {
                    Keyed(
                        iterable=*data,
                        view=move |row| {
                            let is_selected = create_selector(move || selected.get() == Some(row.id));
                            let handle_click = move |_| selected.set(Some(row.id));
                            on_cleanup(move || {
                                row.label.dispose();
                            });
                            view! {
                                tr(class=if is_selected.get() { "danger" } else { "" }) {
                                    td(class="col-md-1") { (row.id) }
                                    td(class="col-md-4") {
                                        a(on:click=handle_click) { (row.label.get_clone()) }
                                    }
                                    td(class="col-md-1") {
                                        a(on:click=move |_| remove(row.id)) {
                                            span(class="glyphicon glyphicon-remove", aria-hidden="true")
                                        }
                                    }
                                    td(class="col-md-6")
                                }
                            }
                        },
                        key=|row| row.id
                    )
                }
            }
        }
    }
}
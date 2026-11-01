use gloo_events::{EventListener, EventListenerOptions};
use gloo_timers::callback::{Interval, Timeout};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{
    Element, HtmlElement, IntersectionObserver, IntersectionObserverEntry,
    IntersectionObserverInit, KeyboardEvent,
};
use yew::prelude::*;

// ─────────────────────────────────────────────────────────────────
// Content. Text comes in (english, spanish) pairs; see `Lang`.
// ─────────────────────────────────────────────────────────────────

const USER: &str = "uttkarzsh";
const HOST: &str = "darwin";

const EMAIL: &str = "utkbest@gmail.com";

const SOCIALS: &[(&str, &str)] = &[
    ("github", "https://github.com/uttkarzsh"),
    ("x", "https://x.com/uttkarzsh"),
    ("linkedin", "https://linkedin.com/in/uttkarsh-pawan"),
];

/// (id, key, label) — the key jumps to the section.
const SECTIONS: &[(&str, &str, Tr)] = &[
    ("work", "1", ("work", "trabajo")),
    ("about", "2", ("about", "sobre mí")),
    ("blogs", "3", ("blogs", "blogs")),
    ("contact", "4", ("contact", "contacto")),
];

fn section_label(id: &str) -> Tr {
    SECTIONS
        .iter()
        .find(|s| s.0 == id)
        .map_or(("", ""), |s| s.2)
}

/// Each theme is built around one ascii.rest piece; its id is the piece's name, and the
/// page's colours (see `[data-theme]` in styles.css) are drawn from that piece's palette.
struct Theme {
    id: &'static str,
    label: Tr,
}

const THEMES: &[Theme] = &[
    Theme {
        id: "cloud-sea",
        label: (
            "the sun sinking into a drifting sea of orange cloud at dusk",
            "el sol hundiéndose en un mar de nubes naranjas al atardecer",
        ),
    },
    Theme {
        id: "kyoto-dusk",
        label: (
            "a pagoda at dusk behind a cherry tree lit by a stone lantern",
            "una pagoda al anochecer tras un cerezo iluminado por un farol de piedra",
        ),
    },
    Theme {
        id: "misty-forest",
        label: (
            "pine ridges fading into morning fog, sunbeams slanting through",
            "crestas de pinos perdiéndose en la niebla matinal, con rayos de sol entre ellas",
        ),
    },
    Theme {
        id: "night-highway",
        label: (
            "a desert highway at dusk, rushing toward the mountains and last light",
            "una carretera del desierto al anochecer, hacia las montañas y la última luz",
        ),
    },
    Theme {
        id: "ocean-sunset",
        label: (
            "the sun setting past a pine headland, lit cloud, glitter on rolling sea",
            "el sol poniéndose tras un cabo de pinos, nubes encendidas y destellos sobre el mar",
        ),
    },
    Theme {
        id: "snow-train",
        label: (
            "a lit train crossing a snowy stone viaduct at night as snow falls",
            "un tren iluminado cruza de noche un viaducto de piedra nevado",
        ),
    },
];

const THEME_KEY: &str = "theme";

struct ProjectData {
    title: &'static str,
    description: Tr,
    tags: &'static [Tr],
    github: &'static str,
}

const RUST: Tr = ("rust", "rust");
const ARKWORKS: Tr = ("arkworks", "arkworks");

const PROJECTS: &[ProjectData] = &[
    ProjectData {
        title: "RiasOS",
        description: (
            "The crimson-kerneled x86_64 operating system written from scratch in Rust.",
            "El sistema operativo x86_64 de núcleo carmesí, escrito desde cero en Rust.",
        ),
        tags: &[
            RUST,
            ("x86-64", "x86-64"),
            ("operating systems", "sistemas operativos"),
        ],
        github: "https://github.com/uttkarzsh/rias",
    },
    ProjectData {
        title: "Laniakea",
        description: (
            "A ZK proving SDK for iOS devices.",
            "Un SDK de pruebas ZK para dispositivos iOS.",
        ),
        tags: &[RUST, ARKWORKS],
        github: "https://github.com/uttkarzsh/laniakea",
    },
    ProjectData {
        title: "Groth16",
        description: (
            "A Rust implementation of the Groth16 proving system.",
            "Una implementación en Rust del sistema de pruebas Groth16.",
        ),
        tags: &[RUST, ("elliptic curves", "curvas elípticas"), ARKWORKS],
        github: "https://github.com/uttkarzsh/groth16",
    },
];

const INTERESTS: &[Tr] = &[
    ("operating systems", "sistemas operativos"),
    ("compiler design", "diseño de compiladores"),
    ("distributed systems", "sistemas distribuidos"),
    (
        "cryptography & zero-knowledge",
        "criptografía y conocimiento cero",
    ),
    ("web3 & blockchain", "web3 y blockchain"),
    (
        "ai systems (language models & inference)",
        "sistemas de ia (modelos de lenguaje e inferencia)",
    ),
];

const LANGUAGES: &[&str] = &[".rs", ".cpp", ".py", ".lua", ".asm", ".nr", ".sol"];

// ─────────────────────────────────────────────────────────────────
// Language: the en | es switch in the tab bar
// ─────────────────────────────────────────────────────────────────

/// A piece of text as (english, spanish).
type Tr = (&'static str, &'static str);

#[derive(Clone, Copy, PartialEq)]
enum Lang {
    En,
    Es,
}

const LANG_KEY: &str = "lang";

impl Lang {
    fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Es => "es",
        }
    }

    /// Pick this language's half of a pair.
    fn t(self, (en, es): Tr) -> &'static str {
        match self {
            Lang::En => en,
            Lang::Es => es,
        }
    }
}

fn stored_lang() -> Lang {
    match local_storage().and_then(|s| s.get_item(LANG_KEY).ok().flatten()) {
        Some(code) if code == "es" => Lang::Es,
        _ => Lang::En,
    }
}

#[derive(PartialEq)]
struct LangState(Lang);

enum LangAction {
    Set(Lang),
    Toggle,
}

impl Reducible for LangState {
    type Action = LangAction;

    fn reduce(self: Rc<Self>, action: LangAction) -> Rc<Self> {
        Rc::new(LangState(match action {
            LangAction::Set(lang) => lang,
            LangAction::Toggle => match self.0 {
                Lang::En => Lang::Es,
                Lang::Es => Lang::En,
            },
        }))
    }
}

thread_local! {
    /// (section, its top on screen) just before a language switch
    static LANG_ANCHOR: Cell<Option<(&'static str, f64)>> = const { Cell::new(None) };
}

/// Switch language, keeping the section you're reading where it is on screen: the text
/// changes length, which would otherwise shift everything below the hero.
fn switch_lang(lang: &UseReducerDispatcher<LangState>, action: LangAction) {
    // mid-jump the page is already heading for a section; correcting the scroll now
    // would cut that jump short
    if !jumping() {
        let id = current_section();
        if let Some(el) = section_el(id) {
            LANG_ANCHOR.set(Some((id, el.get_bounding_client_rect().top())));
        }
    }
    lang.dispatch(action);
}

#[hook]
fn use_lang() -> UseReducerHandle<LangState> {
    let lang = use_reducer_eq(|| LangState(stored_lang()));
    use_effect_with(lang.0, |&lang| {
        if let Some((id, before)) = LANG_ANCHOR.take() {
            // a tick later, once every component has re-rendered in the new language
            Timeout::new(0, move || {
                if let (Some(win), Some(el)) = (web_sys::window(), section_el(id)) {
                    let opts = web_sys::ScrollToOptions::new();
                    opts.set_top(el.get_bounding_client_rect().top() - before);
                    opts.set_behavior(web_sys::ScrollBehavior::Instant);
                    win.scroll_by_with_scroll_to_options(&opts);
                }
            })
            .forget();
        }
        let doc = web_sys::window().and_then(|w| w.document());
        if let Some(doc) = &doc {
            doc.set_title(lang.t((
                "uttkarzsh - welcome to my roman empire",
                "uttkarzsh - bienvenido a mi imperio romano",
            )));
        }
        if let Some(root) = doc.and_then(|d| d.document_element()) {
            let _ = root.set_attribute("lang", lang.code());
        }
        if let Some(storage) = local_storage() {
            let _ = storage.set_item(LANG_KEY, lang.code());
        }
    });
    lang
}

// ─────────────────────────────────────────────────────────────────
// App
// ─────────────────────────────────────────────────────────────────

#[function_component(App)]
fn app() -> Html {
    let scroll = use_scroll_state();
    let theme = use_theme();
    let lang = use_lang();
    let editing = use_state(|| None::<&'static str>);
    let dirty = use_state(|| Rc::new(Vec::<&'static str>::new()));
    let editor = {
        let (set_editing, set_dirty) = (editing.setter(), dirty.setter());
        (*use_memo((), move |_| Editor::new(set_editing, set_dirty))).clone()
    };
    use_keyboard_nav(theme.dispatcher(), lang.dispatcher(), editor.clone());
    let set_lang = {
        let (lang, editor) = (lang.dispatcher(), editor.clone());
        Callback::from(move |l: Lang| {
            // the editable text is re-rendered in the new language, which drops any edits
            editor.normal();
            switch_lang(&lang, LangAction::Set(l));
        })
    };
    let l = lang.0;
    let edit = EditCtx {
        editing: *editing,
        dirty: (*dirty).clone(),
        editor,
    };
    let t = &THEMES[theme.0];
    let next_theme = {
        let theme = theme.dispatcher();
        Callback::from(move |_: MouseEvent| theme.dispatch(ThemeAction::Next))
    };

    let editing_in = |section: &str| *editing == Some(section);
    // keyed by language: switching remounts the editable text instead of patching
    // text the visitor may have edited
    let ek = l.code();

    html! {
        <ContextProvider<Lang> context={l}>
        <ContextProvider<EditCtx> context={edit}>
            <TabBar active={scroll.active} lang={l} {set_lang} />

            <main id="top" class="frame">

                // ── Hero: split panes ───────────────────────────
                <div class="hero">
                    <Pane
                        title={USER}
                        meta="zsh"
                        class="whoami"
                        active={scroll.active == "top"}
                        editing={editing_in("top")}
                    >
                        <Cmd cmd="./init" lines=5>
                            <h1 class="title ln" style="--i: 0">
                                { l.t(("welcome to my", "bienvenido a mi")) }
                                <br />
                                <em>{ l.t(("roman empire.", "imperio romano.")) }</em>
                            </h1>
                            <p class="comment ln" style="--i: 1">
                                { l.t(("# coder · systems · inference", "# programador · sistemas · inferencia")) }
                            </p>
                            <Editable key={ek} section="top">
                                <p class="quote ln" style="--i: 2">
                                    { l.t((
                                        "\"to make an apple pie from scratch, you must first invent the universe.\"",
                                        "\"para hacer una tarta de manzana desde cero, primero hay que inventar el universo.\"",
                                    )) }
                                </p>
                            </Editable>
                            <ul class="links ln" style="--i: 3">
                                { for SOCIALS.iter().map(|(label, href)| html! {
                                    <li>
                                        <a href={*href} target="_blank" rel="noopener noreferrer">
                                            { *label }
                                        </a>
                                    </li>
                                }) }
                            </ul>
                            <EditHint section="top" i=4 />
                        </Cmd>
                    </Pane>

                    <Pane
                        class="art-pane"
                        foot={html! {
                            <button class="theme-btn" type="button" onclick={next_theme}>
                                { format!(
                                    "{}{} {}/{} ›",
                                    if is_touch() { "" } else { "[n] " },
                                    l.t(("theme", "tema")),
                                    theme.0 + 1,
                                    THEMES.len(),
                                ) }
                            </button>
                        }}
                    >
                        // keyed, so a new piece gets a fresh element and fades in
                        <figure class="art" key={t.id}>
                            <ascii-art piece={t.id} label={l.t(t.label)}></ascii-art>
                        </figure>
                    </Pane>
                </div>

                // ── Work ────────────────────────────────────────
                <Pane
                    id="work"
                    title={l.t(section_label("work"))}
                    meta="zsh"
                    active={scroll.active == "work"}
                >
                    <Cmd cmd="./projects" lines={PROJECTS.len() as u32 + 1}>
                        <p class="dim ln listing-total" style="--i: 0">
                            { format!("total {}", PROJECTS.len()) }
                        </p>
                        <ol class="cards">
                            { for PROJECTS.iter().enumerate().map(|(i, p)| html! {
                                <Card index={i + 1} data={p} />
                            }) }
                        </ol>
                    </Cmd>
                </Pane>

                // ── About ───────────────────────────────────────
                <Pane
                    id="about"
                    title={l.t(section_label("about"))}
                    meta="zsh"
                    active={scroll.active == "about"}
                    editing={editing_in("about")}
                >
                    <Cmd cmd="./about">
                        <Editable key={ek} section="about">
                            <p class="lede ln">
                                { l.t((
                                    "hi. i'm uttkarsh, a 21yo undergrad student from ",
                                    "hola. soy uttkarsh, estudiante de grado de 21 años en el ",
                                )) }
                                <em>{ "iit roorkee" }</em>{ "." }
                            </p>
                        </Editable>
                    </Cmd>

                    <Cmd cmd="./interests" lines={INTERESTS.len() as u32}>
                        <Editable key={ek} section="about">
                            <ul class="pick interests">
                                { for INTERESTS.iter().enumerate().map(|(i, item)| html! {
                                    <li class="ln" style={format!("--i: {i}")}>
                                        { l.t(*item) }
                                    </li>
                                }) }
                            </ul>
                        </Editable>
                    </Cmd>

                    <Cmd cmd="./languages">
                        <Editable key={ek} section="about">
                            <ul class="pick inline ln">
                                { for LANGUAGES.iter().map(|lang| html! { <li>{ *lang }</li> }) }
                            </ul>
                        </Editable>
                    </Cmd>

                    // follows the live theme, so it stays out of the editable text
                    <Cmd cmd="./theme" lines={THEMES.len() as u32 + 2}>
                        <ul class="pick themes">
                            { for THEMES.iter().enumerate().map(|(i, th)| {
                                let on = i == theme.0;
                                let pick = {
                                    let theme = theme.dispatcher();
                                    Callback::from(move |_: MouseEvent| theme.dispatch(ThemeAction::Set(i)))
                                };
                                // the swatches sit under the current theme, numbered as the
                                // line after it so the print stagger stays in order
                                let at = |n: usize| format!("--i: {}", n + usize::from(i > theme.0));
                                html! {
                                    <>
                                        <li class={classes!("ln", on.then_some("sel"))} style={at(i)}>
                                            <button type="button" class="pick-btn" onclick={pick} aria-pressed={on.to_string()}>
                                                { th.id }
                                            </button>
                                            if on {
                                                <span class="dim aside">
                                                    { l.t(if is_touch() {
                                                        ("(tap to switch)", "(toca para cambiar)")
                                                    } else {
                                                        ("(n/N to cycle)", "(n/N para cambiar)")
                                                    }) }
                                                </span>
                                            }
                                        </li>
                                        if on {
                                            <li class="swatches ln" style={at(i + 1)} aria-hidden="true">
                                                { for (0..8).map(|i| html! { <span class={format!("sw sw{i}")}></span> }) }
                                            </li>
                                        }
                                    </>
                                }
                            }) }
                        </ul>
                        <EditHint section="about" i={THEMES.len() as u32 + 1} />
                    </Cmd>
                </Pane>

                // ── Blogs ───────────────────────────────────────
                <Pane
                    id="blogs"
                    title={l.t(section_label("blogs"))}
                    meta="zsh"
                    active={scroll.active == "blogs"}
                    editing={editing_in("blogs")}
                >
                    <Cmd cmd="./blogs" lines=3>
                        <p class="soon ln" style="--i: 0">
                            { l.t(("coming soon.", "próximamente.")) }
                        </p>
                        <Editable key={ek} section="blogs">
                            <p class="sub ln" style="--i: 1">
                                { l.t(("i'm lazy. please be patient.", "soy vago. ten paciencia, por favor.")) }
                            </p>
                        </Editable>
                        <EditHint section="blogs" i=2 />
                    </Cmd>
                </Pane>

                // ── Contact ─────────────────────────────────────
                <Pane
                    id="contact"
                    title={l.t(section_label("contact"))}
                    meta="zsh"
                    active={scroll.active == "contact"}
                    editing={editing_in("contact")}
                >
                    <Cmd cmd="./mail" lines=3>
                        <Editable key={ek} section="contact">
                            <p class="motd ln" style="--i: 0">
                                { l.t((
                                    "entropy finds everyone, let's build something interesting.",
                                    "la entropía encuentra a todos, construyamos algo interesante.",
                                )) }
                            </p>
                        </Editable>
                        <a class="email ln" style="--i: 1" href={format!("mailto:{EMAIL}")}>
                            { EMAIL }
                        </a>
                        <EditHint section="contact" i=2 />
                    </Cmd>
                </Pane>

                <footer class="foot">
                    <span>{ l.t(("[process exited with code 0]", "[proceso terminado con código 0]")) }</span>
                    <a href="https://yew.rs" target="_blank" rel="noopener noreferrer">
                        { l.t(("built in rust with yew", "hecho en rust con yew")) }
                    </a>
                </footer>
            </main>

            <StatusLine
                active={scroll.active}
                position={scroll.position.clone()}
                theme={t.id}
                insert={editing.is_some()}
                edited={dirty.contains(&scroll.active)}
            />
        </ContextProvider<EditCtx>>
        </ContextProvider<Lang>>
    }
}

// ─────────────────────────────────────────────────────────────────
// Bars
// ─────────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct TabBarProps {
    active: &'static str,
    lang: Lang,
    set_lang: Callback<Lang>,
}

/// tmux-style window list along the top, with the language switch and clock on the right.
#[function_component(TabBar)]
fn tab_bar(props: &TabBarProps) -> Html {
    let l = props.lang;
    let lang_btn = |to: Lang| {
        let set_lang = props.set_lang.clone();
        let onclick = Callback::from(move |_: MouseEvent| set_lang.emit(to));
        let on = l == to;
        html! {
            <button
                type="button"
                class={classes!("lang-btn", on.then_some("on"))}
                aria-pressed={on.to_string()}
                {onclick}
            >
                { to.code() }
            </button>
        }
    };
    html! {
        <header class="tabbar">
            <div class="frame tabbar-inner">
                <a class="session" href="#top">
                    <span class="pulse"></span>
                    { "[uttkarzsh]" }
                </a>
                <nav class="tabs">
                    { for SECTIONS.iter().map(|(id, key, label)| {
                        let class = classes!("tab", (props.active == *id).then_some("on"));
                        let onclick = Callback::from(move |_: MouseEvent| pin(Some(id)));
                        html! {
                            <a {class} {onclick} href={format!("#{id}")}>
                                <span class="tab-key">{ *key }</span>
                                <span class="tab-label">
                                    { " " }{ l.t(*label) }
                                    { if props.active == *id { "*" } else { "" } }
                                </span>
                            </a>
                        }
                    }) }
                </nav>
                <span class="tabbar-right">
                    <span class="lang" role="group" aria-label="language">
                        { lang_btn(Lang::En) }<span class="lang-sep">{ "|" }</span>{ lang_btn(Lang::Es) }
                    </span>
                    <Clock />
                </span>
            </div>
        </header>
    }
}

#[derive(Properties, PartialEq)]
struct StatusProps {
    active: &'static str,
    position: String,
    theme: &'static str,
    insert: bool,
    /// the active section's text differs from what was published
    edited: bool,
}

/// vim-style status line pinned to the bottom.
#[function_component(StatusLine)]
fn status_line(props: &StatusProps) -> Html {
    let l = use_context::<Lang>().unwrap_or(Lang::En);
    let path = if props.active == "top" {
        "~/uttkarzsh".to_string()
    } else {
        format!("~/uttkarzsh/{}", props.active)
    };
    let position = match props.position.as_str() {
        "Top" => l.t(("Top", "Arriba")).to_string(),
        "Bot" => l.t(("Bot", "Abajo")).to_string(),
        pct => pct.to_string(),
    };
    let insert = props.insert;
    html! {
        <div class="status" role="status">
            <span class={classes!("mode", insert.then_some("insert"))}>
                { if insert { l.t(("INSERT", "INSERTAR")) } else { "NORMAL" } }
            </span>
            <span class="status-path">{ path }</span>
            <span class="status-fill"></span>
            if insert {
                <span class="status-keys">
                    <kbd>{ "esc" }</kbd>{ " " }{ l.t(("normal mode", "modo normal")) }
                </span>
                <Wpm />
            } else {
                <span class="status-keys">
                    <kbd>{ "j/k" }</kbd>{ format!(" {} · ", l.t(("scroll", "desplazar"))) }
                    <kbd>{ "h/l" }</kbd>{ " " }<kbd>{ "1-4" }</kbd>{ format!(" {} · ", l.t(("section", "sección"))) }
                    <kbd>{ "g/G" }</kbd>{ format!(" {} · ", l.t(("top/end", "inicio/fin"))) }
                    <kbd>{ "n/N" }</kbd>{ format!(" {} · ", l.t(("theme", "tema"))) }
                    <kbd>{ "p" }</kbd>{ " en/es · " }
                    <kbd>{ "i" }</kbd>{ " " }{ l.t(("insert", "insertar")) }
                    if props.edited {
                        { " · " }<kbd>{ "u" }</kbd>{ " " }{ l.t(("undo", "deshacer")) }
                    }
                </span>
            }
            <span class="status-seg status-theme">{ props.theme }</span>
            <span class="status-seg">{ "utf-8" }</span>
            <span class="status-seg">{ "rust" }</span>
            <span class="status-pos">{ position }</span>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────
// Building blocks
// ─────────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct PaneProps {
    #[prop_or_default]
    title: Option<AttrValue>,
    #[prop_or_default]
    meta: Option<AttrValue>,
    #[prop_or_default]
    id: Option<AttrValue>,
    #[prop_or_default]
    class: Classes,
    #[prop_or_default]
    active: bool,
    /// in insert mode
    #[prop_or_default]
    editing: bool,
    /// Sits on the bottom border, right-aligned.
    #[prop_or_default]
    foot: Option<Html>,
    children: Html,
}

/// A bordered pane with its title set into the top border, btop / lazygit style.
#[function_component(Pane)]
fn pane(props: &PaneProps) -> Html {
    let class = classes!(
        "pane",
        props.class.clone(),
        props.active.then_some("active"),
        props.editing.then_some("editing")
    );
    // every pane is its own shell: its commands run in turn, independent of other panes
    let term = use_memo((), |_| Term::default());
    html! {
        <ContextProvider<Term> context={(*term).clone()}>
        <section id={props.id.clone()} {class}>
            if props.title.is_some() || props.meta.is_some() {
                <header class="pane-head">
                    if let Some(title) = &props.title {
                        <h2 class="pane-title">{ title }</h2>
                    }
                    if let Some(meta) = &props.meta {
                        <span class="pane-meta">{ meta }</span>
                    }
                </header>
            }
            <div class="pane-body">
                { props.children.clone() }
            </div>
            if let Some(foot) = &props.foot {
                <footer class="pane-foot">{ foot.clone() }</footer>
            }
        </section>
        </ContextProvider<Term>>
    }
}

#[derive(Properties, PartialEq)]
struct PromptProps {
    cmd: AttrValue,
}

#[function_component(Prompt)]
fn prompt(props: &PromptProps) -> Html {
    html! {
        <p class="prompt">
            <span class="ps1">
                <span class="hl">{ USER }{ "@" }{ HOST }</span>
                { " " }<span class="cwd">{ "~" }</span>{ " % " }
            </span>
            <span class="cmd">{ &props.cmd }</span>
        </p>
    }
}

#[derive(Properties)]
struct CardProps {
    index: usize,
    data: &'static ProjectData,
}

impl PartialEq for CardProps {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && std::ptr::eq(self.data, other.data)
    }
}

#[function_component(Card)]
fn card(props: &CardProps) -> Html {
    let l = use_context::<Lang>().unwrap_or(Lang::En);
    let p = props.data;
    html! {
        <li class="ln" style={format!("--i: {}", props.index)}>
            <a class="card" href={p.github} target="_blank" rel="noopener noreferrer">
                <div class="card-top">
                    <span>{ format!("[{:02}]", props.index) }</span>
                    <span>{ "drwxr-xr-x" }</span>
                </div>
                <h3 class="card-title"><span>{ p.title }{ "/" }</span></h3>
                <p class="card-desc">{ l.t(p.description) }</p>
                <ul class="card-tags">
                    { for p.tags.iter().map(|t| html! { <li>{ format!("[{}]", l.t(*t)) }</li> }) }
                </ul>
                <p class="card-link">
                    { "↳ " }{ p.github.trim_start_matches("https://") }
                </p>
            </a>
        </li>
    }
}

// ─────────────────────────────────────────────────────────────────
// Terminal: commands type themselves out as they scroll into view
// ─────────────────────────────────────────────────────────────────
//
// Every <Pane> is its own shell (a `Term`); panes run independently, so whatever is on
// screen starts straight away. Inside a pane one command runs at a time, and the next
// one picked is always the topmost *visible* pending block. A block that scrolls away
// mid-command is reset and replays when it comes back, so nothing off screen holds up
// what you are looking at.
//
// A run goes:  prompt appears → pause → keystrokes at a human pace → pause → enter →
// output prints line by line (CSS, staggered by --i) → a beat before the next prompt.

/// Typing speed in words per minute (a "word" being 5 keystrokes).
const WPM: f64 = 150.0;
/// Average gap between keystrokes at that speed: 60 000 / (WPM × 5).
const KEY_AVG_MS: f64 = 60_000.0 / (WPM * 5.0);

/// Human-ish timings, in ms. `jitter(a, b)` picks uniformly in a..a+b.
const PRE_MS: (f64, f64) = (225.0, 125.0); // prompt shows, then the first key
// between keystrokes: 0.6–1.2× the average, so with the odd hesitation below a command
// without spaces (all of them, ./init and so on) lands on WPM
const KEY_MS: (f64, f64) = (KEY_AVG_MS * 0.6, KEY_AVG_MS * 0.6);
const WORD_MS: f64 = KEY_AVG_MS * 0.4; // extra around a space
const HESITATE: (f64, f64) = (0.04, KEY_AVG_MS * 2.5); // chance of a stall mid-word, and its length
const ENTER_MS: (f64, f64) = (300.0, 200.0); // a look at the command before enter
const THINK_MS: (f64, f64) = (325.0, 175.0); // after the output, before the next prompt
const SETTLE_MS: u32 = 150; // a block must stay on screen this long before it starts
/// Output printing; mirrors `.cmdblock.printing .ln` in styles.css.
const LINE_MS: f64 = 70.0;
const PRINT_MS: f64 = 180.0;

fn jitter((base, spread): (f64, f64)) -> u32 {
    (base + js_sys::Math::random() * spread) as u32
}

/// Delay before typing `next`, the character after `prev`.
fn key_delay(prev: char, next: char) -> u32 {
    let mut ms = jitter(KEY_MS) as f64;
    if prev == ' ' || next == ' ' {
        ms += WORD_MS;
    }
    if js_sys::Math::random() < HESITATE.0 {
        ms += HESITATE.1;
    }
    ms as u32
}

fn print_ms(lines: u32) -> u32 {
    match lines {
        0 => 0,
        n => ((n - 1) as f64 * LINE_MS + PRINT_MS + 60.0) as u32,
    }
}

fn media_matches(query: &str) -> bool {
    web_sys::window()
        .and_then(|w| w.match_media(query).ok().flatten())
        .is_some_and(|m| m.matches())
}

fn prefers_reduced_motion() -> bool {
    media_matches("(prefers-reduced-motion: reduce)")
}

/// A phone or tablet: no hover, a finger for a pointer, and usually no keyboard. The UI
/// says "tap" instead of "press" and leaves out the key hints.
fn is_touch() -> bool {
    media_matches("(hover: none) and (pointer: coarse)")
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    /// not reached yet: prompt and output hidden
    Idle,
    /// the prompt is up, cursor blinking, nothing typed yet
    Pre,
    /// keystrokes landing
    Typing,
    /// enter pressed, output printing
    Printing,
    /// everything shown
    Done,
}

impl Phase {
    fn class(self) -> &'static str {
        match self {
            Phase::Idle => "idle",
            Phase::Pre => "pre",
            Phase::Typing => "typing",
            Phase::Printing => "printing",
            Phase::Done => "done",
        }
    }
}

struct Block {
    el: Element,
    cmd: Vec<char>,
    lines: u32,
    visible: bool,
    done: bool,
    phase: UseStateSetter<Phase>,
    typed: UseStateSetter<usize>,
}

#[derive(Clone, Copy)]
enum Stage {
    Pre,
    /// this many characters are on screen
    Typing(usize),
    Printing,
    Think,
}

struct Run {
    id: usize,
    stage: Stage,
}

#[derive(Default)]
struct TermInner {
    blocks: Vec<Block>,
    current: Option<Run>,
    /// bumped whenever the run's next step is (re)scheduled or the run is dropped;
    /// a timer whose token is stale does nothing
    run_token: u64,
    settle_token: u64,
}

#[derive(Clone, Default)]
struct Term(Rc<RefCell<TermInner>>);

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

fn page_top(el: &Element) -> f64 {
    let y = web_sys::window()
        .and_then(|w| w.scroll_y().ok())
        .unwrap_or(0.0);
    el.get_bounding_client_rect().top() + y
}

impl Term {
    fn register(&self, block: Block) -> usize {
        let mut inner = self.0.borrow_mut();
        inner.blocks.push(block);
        inner.blocks.len() - 1
    }

    /// Show a block complete, with no animation.
    fn finish_now(&self, id: usize) {
        let mut inner = self.0.borrow_mut();
        let block = &mut inner.blocks[id];
        block.done = true;
        block.typed.set(block.cmd.len());
        block.phase.set(Phase::Done);
    }

    fn set_visible(&self, id: usize, visible: bool) {
        {
            let mut inner = self.0.borrow_mut();
            inner.blocks[id].visible = visible;
            let running_here = matches!(
                inner.current,
                Some(Run { id: cur, stage }) if cur == id && !matches!(stage, Stage::Think)
            );
            if !visible && running_here {
                // scrolled away mid-command: put it back to replay later
                inner.current = None;
                inner.run_token += 1;
                let block = &inner.blocks[id];
                block.typed.set(0);
                block.phase.set(Phase::Idle);
            }
        }
        self.settle();
    }

    /// Pump once the page has stopped moving for a moment, so blocks flashing past
    /// during a jump don't start.
    fn settle(&self) {
        let token = {
            let mut inner = self.0.borrow_mut();
            inner.settle_token += 1;
            inner.settle_token
        };
        let term = self.clone();
        Timeout::new(SETTLE_MS, move || {
            if term.0.borrow().settle_token == token {
                term.pump();
            }
        })
        .forget();
    }

    fn pump(&self) {
        let next = {
            let inner = self.0.borrow();
            if inner.current.is_some() {
                return;
            }
            inner
                .blocks
                .iter()
                .enumerate()
                .filter(|(_, b)| b.visible && !b.done)
                .min_by(|(_, a), (_, b)| page_top(&a.el).total_cmp(&page_top(&b.el)))
                .map(|(id, _)| id)
        };
        if let Some(id) = next {
            {
                let mut inner = self.0.borrow_mut();
                inner.current = Some(Run {
                    id,
                    stage: Stage::Pre,
                });
                let block = &inner.blocks[id];
                block.typed.set(0);
                block.phase.set(Phase::Pre);
            }
            self.after(jitter(PRE_MS));
        }
    }

    /// Run the current block's next step in `ms`.
    fn after(&self, ms: u32) {
        let token = {
            let mut inner = self.0.borrow_mut();
            inner.run_token += 1;
            inner.run_token
        };
        let term = self.clone();
        Timeout::new(ms, move || term.step(token)).forget();
    }

    fn step(&self, token: u64) {
        let mut inner = self.0.borrow_mut();
        if inner.run_token != token {
            return;
        }
        let Some(Run { id, stage }) = inner.current else {
            return;
        };
        let n = inner.blocks[id].cmd.len();
        let (next, delay) = match stage {
            Stage::Pre | Stage::Typing(_) => {
                let typed = match stage {
                    Stage::Typing(i) => i,
                    _ => 0,
                };
                let block = &inner.blocks[id];
                if typed == n {
                    // enter
                    block.phase.set(Phase::Printing);
                    (Stage::Printing, print_ms(block.lines))
                } else {
                    block.phase.set(Phase::Typing);
                    block.typed.set(typed + 1);
                    let delay = if typed + 1 == n {
                        jitter(ENTER_MS)
                    } else {
                        key_delay(block.cmd[typed], block.cmd[typed + 1])
                    };
                    (Stage::Typing(typed + 1), delay)
                }
            }
            Stage::Printing => {
                let block = &mut inner.blocks[id];
                block.done = true;
                block.phase.set(Phase::Done);
                (Stage::Think, jitter(THINK_MS))
            }
            Stage::Think => {
                inner.current = None;
                drop(inner);
                self.pump();
                return;
            }
        };
        inner.current = Some(Run { id, stage: next });
        drop(inner);
        self.after(delay);
    }
}

#[derive(Properties, PartialEq)]
struct CmdProps {
    cmd: AttrValue,
    /// how many `.ln` lines the output prints (their `--i` runs 0..lines)
    #[prop_or(1)]
    lines: u32,
    #[prop_or_default]
    children: Html,
}

#[function_component(Cmd)]
fn cmd(props: &CmdProps) -> Html {
    let term = use_context::<Term>().expect("Cmd must sit inside a Pane");
    let node = use_node_ref();
    let phase = use_state(|| Phase::Idle);
    let typed = use_state(|| 0usize);

    {
        let node = node.clone();
        let phase = phase.setter();
        let typed = typed.setter();
        let cmd: Vec<char> = props.cmd.chars().collect();
        let lines = props.lines;
        use_effect_with((), move |_| {
            let el: Element = node.cast().expect("cmd block element");
            let id = term.register(Block {
                el: el.clone(),
                cmd,
                lines,
                visible: false,
                done: false,
                phase,
                typed,
            });
            if prefers_reduced_motion() {
                term.finish_now(id);
            }

            let on_change =
                Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
                    if let Some(entry) = entries.iter().last() {
                        let entry: IntersectionObserverEntry = entry.unchecked_into();
                        term.set_visible(id, entry.is_intersecting());
                    }
                });
            let options = IntersectionObserverInit::new();
            // on screen = below the sticky tab bar and a little way up from the bottom edge
            options.set_root_margin("-48px 0px -12% 0px");
            let observer = IntersectionObserver::new_with_options(
                on_change.as_ref().unchecked_ref(),
                &options,
            )
            .ok();
            if let Some(observer) = &observer {
                observer.observe(&el);
            }
            move || {
                if let Some(observer) = observer {
                    observer.disconnect();
                }
                drop(on_change);
            }
        });
    }

    let shown: String = match *phase {
        Phase::Idle | Phase::Pre => String::new(),
        Phase::Typing => props.cmd.chars().take(*typed).collect(),
        Phase::Printing | Phase::Done => props.cmd.to_string(),
    };
    let class = classes!("cmdblock", phase.class());
    html! {
        <div ref={node} {class}>
            <Prompt cmd={shown} />
            <div class="out">{ props.children.clone() }</div>
        </div>
    }
}

// ─────────────────────────────────────────────────────────────────
// Insert mode: `i` makes the active section's text editable, esc leaves, `u` undoes
// ─────────────────────────────────────────────────────────────────
//
// Each <Editable> registers its element under its section ("top" for the hero, else the
// pane's id). Entering insert mode snapshots every region and sets contenteditable on that
// section's printed ones; leaving it records each region that changed on an undo stack.
// The text inside is static, so Yew never needs to patch it, and edits made straight to
// the DOM stay put.

struct Region {
    section: &'static str,
    el: HtmlElement,
    /// the markup as published
    original: String,
}

struct EditorInner {
    /// the section in insert mode, if any
    editing: Option<&'static str>,
    regions: Vec<Region>,
    /// every region's markup when insert mode began
    session: Vec<String>,
    /// (region, markup before an insert session changed it), oldest first
    undo: Vec<(usize, String)>,
    set_editing: UseStateSetter<Option<&'static str>>,
    set_dirty: UseStateSetter<Rc<Vec<&'static str>>>,
    /// when each character was typed this insert session, for the wpm meter
    keys: Vec<f64>,
}

#[derive(Clone)]
struct Editor(Rc<RefCell<EditorInner>>);

impl PartialEq for Editor {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/// Handed down to <Editable> and <EditHint>.
#[derive(Clone, PartialEq)]
struct EditCtx {
    editing: Option<&'static str>,
    /// sections whose text differs from the original
    dirty: Rc<Vec<&'static str>>,
    editor: Editor,
}

/// The region's command has finished printing.
fn printed(el: &HtmlElement) -> bool {
    el.closest(".cmdblock.done").ok().flatten().is_some()
}

fn in_view(el: &HtmlElement) -> bool {
    let vh = web_sys::window()
        .and_then(|w| w.inner_height().ok())
        .and_then(|v| v.as_f64())
        .unwrap_or(800.0);
    let r = el.get_bounding_client_rect();
    r.bottom() > 48.0 && r.top() < vh * 0.88
}

fn caret_to_end(el: &HtmlElement) {
    let Some(win) = web_sys::window() else {
        return;
    };
    let Some(range) = win.document().and_then(|d| d.create_range().ok()) else {
        return;
    };
    if range.select_node_contents(el).is_err() {
        return;
    }
    range.collapse_with_to_start(false);
    if let Ok(Some(sel)) = win.get_selection() {
        let _ = sel.remove_all_ranges();
        let _ = sel.add_range(&range);
    }
}

impl Editor {
    fn new(
        set_editing: UseStateSetter<Option<&'static str>>,
        set_dirty: UseStateSetter<Rc<Vec<&'static str>>>,
    ) -> Self {
        Editor(Rc::new(RefCell::new(EditorInner {
            editing: None,
            regions: Vec::new(),
            session: Vec::new(),
            undo: Vec::new(),
            set_editing,
            set_dirty,
            keys: Vec::new(),
        })))
    }

    fn is_editing(&self) -> bool {
        self.0.borrow().editing.is_some()
    }

    fn register(&self, section: &'static str, el: HtmlElement) {
        let original = el.inner_html();
        self.0.borrow_mut().regions.push(Region {
            section,
            el,
            original,
        });
    }

    /// The region's text was replaced (the language changed): its edits and the undo
    /// history, which points at regions by position, go with it.
    fn unregister(&self, el: &HtmlElement) {
        let mut inner = self.0.borrow_mut();
        inner.regions.retain(|r| r.el != *el);
        inner.undo.clear();
        inner.session.clear();
        inner.publish_dirty();
    }

    /// Enter insert mode in `section`, caret at the end of its first region on screen.
    /// Does nothing if none of its text has printed yet.
    fn insert(&self, section: &str) {
        let mut inner = self.0.borrow_mut();
        if inner.editing.is_some() {
            return;
        }
        let ready: Vec<usize> = (0..inner.regions.len())
            .filter(|&i| inner.regions[i].section == section && printed(&inner.regions[i].el))
            .collect();
        // prefer what's on screen, then the topmost
        let Some(&target) = ready.iter().min_by(|&&a, &&b| {
            let key = |i: usize| {
                let el = &inner.regions[i].el;
                (!in_view(el), el.get_bounding_client_rect().top())
            };
            let (ka, kb) = (key(a), key(b));
            ka.0.cmp(&kb.0).then(ka.1.total_cmp(&kb.1))
        }) else {
            return;
        };
        let section = inner.regions[target].section;
        inner.session = inner.regions.iter().map(|r| r.el.inner_html()).collect();
        inner.keys.clear();
        for &i in &ready {
            let el = &inner.regions[i].el;
            let _ = el.set_attribute("contenteditable", "true");
            let _ = el.set_attribute("spellcheck", "false");
        }
        inner.editing = Some(section);
        inner.set_editing.set(Some(section));
        let el = inner.regions[target].el.clone();
        drop(inner);
        let _ = el.focus();
        caret_to_end(&el);
    }

    fn normal(&self) {
        let mut inner = self.0.borrow_mut();
        if inner.editing.is_none() {
            return;
        }
        let session = std::mem::take(&mut inner.session);
        let EditorInner { regions, undo, .. } = &mut *inner;
        for (i, (r, before)) in regions.iter().zip(session).enumerate() {
            let _ = r.el.remove_attribute("contenteditable");
            let _ = r.el.remove_attribute("spellcheck");
            if r.el.inner_html() != before {
                undo.push((i, before));
            }
        }
        if let Some(win) = web_sys::window() {
            if let Some(active) = win.document().and_then(|d| d.active_element())
                && let Ok(active) = active.dyn_into::<HtmlElement>()
            {
                let _ = active.blur();
            }
            if let Ok(Some(sel)) = win.get_selection() {
                let _ = sel.remove_all_ranges();
            }
        }
        inner.editing = None;
        inner.set_editing.set(None);
        inner.publish_dirty();
    }

    /// `n` characters were just typed.
    fn typed(&self, n: usize) {
        let now = js_sys::Date::now();
        let mut inner = self.0.borrow_mut();
        inner.keys.extend(std::iter::repeat_n(now, n));
        // only the window is ever read
        inner.keys.retain(|&t| now - t <= WPM_WINDOW_MS);
    }

    /// Typing speed over the last WPM_WINDOW_MS: characters / 5 per minute. Quick to
    /// follow a change of pace, and back to 0 within the window once you stop.
    fn wpm(&self) -> u32 {
        let now = js_sys::Date::now();
        let inner = self.0.borrow();
        let n = inner
            .keys
            .iter()
            .filter(|&&t| now - t <= WPM_WINDOW_MS)
            .count();
        (n as f64 / 5.0 / (WPM_WINDOW_MS / 60_000.0)).round() as u32
    }

    /// Undo the last change made to `section`'s text.
    fn undo(&self, section: &str) {
        let mut inner = self.0.borrow_mut();
        if inner.editing.is_some() {
            return;
        }
        let pos = inner
            .undo
            .iter()
            .rposition(|(i, _)| inner.regions[*i].section == section);
        let Some(pos) = pos else {
            return;
        };
        let (i, markup) = inner.undo.remove(pos);
        inner.regions[i].el.set_inner_html(&markup);
        inner.publish_dirty();
    }
}

impl EditorInner {
    fn publish_dirty(&self) {
        let mut dirty: Vec<&'static str> = Vec::new();
        for r in &self.regions {
            if r.el.inner_html() != r.original && !dirty.contains(&r.section) {
                dirty.push(r.section);
            }
        }
        self.set_dirty.set(Rc::new(dirty));
    }
}

const WPM_WINDOW_MS: f64 = 2_000.0;

/// Live typing speed on the status line, shown while in insert mode.
#[function_component(Wpm)]
fn wpm() -> Html {
    let ctx = use_context::<EditCtx>().expect("Wpm needs an EditCtx");
    let l = use_context::<Lang>().unwrap_or(Lang::En);
    let wpm = use_state(|| 0u32);
    {
        let wpm = wpm.setter();
        use_effect_with(ctx.editor.clone(), move |editor| {
            let editor = editor.clone();
            let interval = Interval::new(100, move || wpm.set(editor.wpm()));
            move || drop(interval)
        });
    }
    html! {
        <span class="status-wpm">{ format!("{:>3} {}", *wpm, l.t(("wpm", "ppm"))) }</span>
    }
}

#[derive(Properties, PartialEq)]
struct EditableProps {
    /// the section it belongs to: "top" for the hero, else the pane's id
    section: &'static str,
    children: Html,
}

/// Text that `i` lets the visitor edit in place.
#[function_component(Editable)]
fn editable(props: &EditableProps) -> Html {
    let ctx = use_context::<EditCtx>().expect("Editable needs an EditCtx");
    let node = use_node_ref();
    {
        let node = node.clone();
        let editor = ctx.editor.clone();
        let section = props.section;
        use_effect_with((), move |_| {
            let el = node.cast::<HtmlElement>();
            if let Some(el) = &el {
                editor.register(section, el.clone());
            }
            move || {
                if let Some(el) = el {
                    editor.unregister(&el);
                }
            }
        });
    }
    let oninput = {
        let editor = ctx.editor.clone();
        Callback::from(move |e: InputEvent| {
            if e.input_type() == "insertText" {
                editor.typed(e.data().map_or(1, |d| d.chars().count()));
            }
        })
    };
    // on a phone, closing the keyboard or tapping away is how you leave the text, so it
    // ends insert mode, unless focus moved to another of the section's editable blocks
    let onfocusout = {
        let editor = ctx.editor.clone();
        Callback::from(move |e: FocusEvent| {
            let to_editable = e
                .related_target()
                .and_then(|t| t.dyn_into::<Element>().ok())
                .and_then(|el| el.closest("[contenteditable]").ok().flatten())
                .is_some();
            if is_touch() && !to_editable {
                editor.normal();
            }
        })
    };
    html! {
        <div ref={node} class="editable" {oninput} {onfocusout}>{ props.children.clone() }</div>
    }
}

#[derive(Properties, PartialEq)]
struct EditHintProps {
    section: &'static str,
    /// its output line index, for the print stagger
    i: u32,
}

/// "# press i to enter insert mode", and so on. The keys are buttons, for touch screens.
#[function_component(EditHint)]
fn edit_hint(props: &EditHintProps) -> Html {
    let ctx = use_context::<EditCtx>().expect("EditHint needs an EditCtx");
    let l = use_context::<Lang>().unwrap_or(Lang::En);
    let section = props.section;
    let key = |label: &'static str, action: fn(&Editor, &'static str)| {
        let editor = ctx.editor.clone();
        let onclick = Callback::from(move |_: MouseEvent| action(&editor, section));
        html! { <button type="button" class="key" {onclick}>{ label }</button> }
    };
    let editing = ctx.editing == Some(section);
    let press = if is_touch() {
        ("tap ", "toca ")
    } else {
        ("press ", "pulsa ")
    };
    let body = if editing {
        html! {
            <>
                { l.t(("-- INSERT -- ", "-- INSERTAR -- ")) }{ l.t(press) }
                { key("esc", |e, _| e.normal()) }
                { l.t((" to enter normal mode", " para volver al modo normal")) }
            </>
        }
    } else if ctx.dirty.contains(&section) {
        html! {
            <>
                { l.t(("edited · ", "editado · ")) }{ l.t(press) }
                { key("i", |e, s| e.insert(s)) }
                { l.t((" to edit again, or ", " para editar de nuevo, o ")) }
                { key("u", |e, s| e.undo(s)) }
                { l.t((" to undo changes", " para deshacer los cambios")) }
            </>
        }
    } else {
        html! {
            <>
                { l.t(press) }
                { key("i", |e, s| e.insert(s)) }
                { l.t((" to enter insert mode", " para entrar en modo inserción")) }
            </>
        }
    };
    let class = classes!("hint", "ln", editing.then_some("on"));
    html! {
        <p {class} style={format!("--i: {}", props.i)}>{ "# " }{ body }</p>
    }
}

// ─────────────────────────────────────────────────────────────────
// Live clock: "nov 19, tue   19:45 ist", or spain's time in spanish
// ─────────────────────────────────────────────────────────────────

const MONTHS: [Tr; 12] = [
    ("jan", "ene"),
    ("feb", "feb"),
    ("mar", "mar"),
    ("apr", "abr"),
    ("may", "may"),
    ("jun", "jun"),
    ("jul", "jul"),
    ("aug", "ago"),
    ("sep", "sep"),
    ("oct", "oct"),
    ("nov", "nov"),
    ("dec", "dic"),
];

const WEEKDAYS: [Tr; 7] = [
    ("sun", "dom"),
    ("mon", "lun"),
    ("tue", "mar"),
    ("wed", "mié"),
    ("thu", "jue"),
    ("fri", "vie"),
    ("sat", "sáb"),
];

const DAY_MS: f64 = 86_400_000.0;
const HOUR_MS: f64 = 3_600_000.0;

/// Start of the last Sunday of `month` (0-based) in `year`, 01:00 UTC: when the EU moves
/// its clocks.
fn eu_switch(year: f64, month: f64) -> f64 {
    let first_next = js_sys::Date::utc(year, month + 1.0);
    let last_day = first_next - DAY_MS;
    let weekday = js_sys::Date::new(&last_day.into()).get_utc_day() as f64;
    last_day - weekday * DAY_MS + HOUR_MS
}

/// (offset from UTC in minutes, zone name) at UTC time `ms`: india for english, spain
/// for spanish, on summer time from the last Sunday of March to the last of October.
fn zone(lang: Lang, ms: f64) -> (i64, &'static str) {
    match lang {
        Lang::En => (5 * 60 + 30, "ist"),
        Lang::Es => {
            let year = js_sys::Date::new(&ms.into()).get_utc_full_year() as f64;
            let summer = (eu_switch(year, 2.0)..eu_switch(year, 9.0)).contains(&ms);
            if summer { (120, "cest") } else { (60, "cet") }
        }
    }
}

/// Minutes since the epoch (UTC).
fn utc_minute() -> i64 {
    (js_sys::Date::now() / 60_000.0) as i64
}

#[function_component(Clock)]
fn clock() -> Html {
    let l = use_context::<Lang>().unwrap_or(Lang::En);
    // only re-renders when the minute turns
    let minute = use_state_eq(utc_minute);
    {
        let minute = minute.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || minute.set(utc_minute()));
            move || drop(interval)
        });
    }
    let (offset, name) = zone(l, *minute as f64 * 60_000.0);
    // shifted by the offset, so the UTC fields read as local time
    let d = js_sys::Date::new(&(((*minute + offset) as f64) * 60_000.0).into());
    let month = l.t(MONTHS[d.get_utc_month() as usize % 12]);
    let weekday = l.t(WEEKDAYS[d.get_utc_day() as usize % 7]);
    let day = d.get_utc_date();
    let date = match l {
        Lang::En => format!("{month} {day}, {weekday}"),
        Lang::Es => format!("{weekday}, {day} {month}"),
    };
    html! {
        <time class="clock">
            <span>{ date }</span>
            <span>{ format!("{:02}:{:02} {name}", d.get_utc_hours(), d.get_utc_minutes()) }</span>
        </time>
    }
}

// ─────────────────────────────────────────────────────────────────
// Theme: index into THEMES, mirrored to <html data-theme> and localStorage
// ─────────────────────────────────────────────────────────────────

#[derive(PartialEq)]
struct ThemeState(usize);

enum ThemeAction {
    Next,
    Prev,
    Set(usize),
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    fn reduce(self: Rc<Self>, action: ThemeAction) -> Rc<Self> {
        let n = THEMES.len();
        Rc::new(ThemeState(match action {
            ThemeAction::Next => (self.0 + 1) % n,
            ThemeAction::Prev => (self.0 + n - 1) % n,
            ThemeAction::Set(i) => i % n,
        }))
    }
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn stored_theme() -> usize {
    local_storage()
        .and_then(|s| s.get_item(THEME_KEY).ok().flatten())
        .and_then(|id| THEMES.iter().position(|t| t.id == id))
        .unwrap_or(0)
}

#[hook]
fn use_theme() -> UseReducerHandle<ThemeState> {
    let theme = use_reducer(|| ThemeState(stored_theme()));
    use_effect_with(theme.0, |&i| {
        let id = THEMES[i].id;
        if let Some(root) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            // data-theming turns on colour transitions just for the switch
            let _ = root.set_attribute("data-theming", "");
            let _ = root.set_attribute("data-theme", id);
            Timeout::new(700, move || {
                let _ = root.remove_attribute("data-theming");
            })
            .forget();
        }
        if let Some(storage) = local_storage() {
            let _ = storage.set_item(THEME_KEY, id);
        }
    });
    theme
}

// ─────────────────────────────────────────────────────────────────
// Scroll position + keyboard navigation
// ─────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct ScrollState {
    /// "top" while in the hero, otherwise the id of the section in view.
    active: &'static str,
    /// vim's ruler: "Top", "Bot", or a percentage.
    position: String,
}

fn read_scroll() -> ScrollState {
    let fallback = ScrollState {
        active: "top",
        position: "Top".into(),
    };
    let Some(win) = web_sys::window() else {
        return fallback;
    };
    let Some(doc) = win.document() else {
        return fallback;
    };

    let vh = win
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(800.0);
    let mut active = "top";
    for (id, _, _) in SECTIONS {
        if let Some(el) = doc.get_element_by_id(id)
            && el.get_bounding_client_rect().top() <= vh * 0.45
        {
            active = id;
        }
    }

    let y = win.scroll_y().unwrap_or(0.0);
    let max = doc
        .document_element()
        .map(|e| e.scroll_height() as f64 - vh)
        .unwrap_or(0.0);
    let position = if y <= 1.0 {
        "Top".to_string()
    } else if max <= 0.0 || y >= max - 1.0 {
        // the last sections may be too short to reach the trigger line: go by the one
        // jumped to, if it's on screen, else the last
        let on_screen = |id: &&str| {
            doc.get_element_by_id(id)
                .is_some_and(|el| el.get_bounding_client_rect().top() < vh)
        };
        active = PINNED
            .get()
            .filter(on_screen)
            .or(SECTIONS.last().map(|s| s.0))
            .unwrap_or(active);
        "Bot".to_string()
    } else {
        format!("{}%", ((y / max) * 100.0).round() as u32)
    };

    ScrollState { active, position }
}

thread_local! {
    /// The section last jumped to, with 1-4 or a tab. Once the page can't scroll any
    /// further, several short sections share the screen and the scroll position alone
    /// can't tell which one you went to; this does. Scrolling by hand clears it.
    static PINNED: Cell<Option<&'static str>> = const { Cell::new(None) };
    /// when it was set (ms)
    static PINNED_AT: Cell<f64> = const { Cell::new(0.0) };
}

fn pin(id: Option<&'static str>) {
    PINNED.set(id);
    PINNED_AT.set(js_sys::Date::now());
    // re-read even if the jump doesn't move the page
    if let Some(win) = web_sys::window()
        && let Ok(e) = web_sys::Event::new("scroll")
    {
        let _ = win.dispatch_event(&e);
    }
}

#[hook]
fn use_scroll_state() -> ScrollState {
    let state = use_state(read_scroll);
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let win = web_sys::window().expect("window");
            let update = move |_: &web_sys::Event| {
                let next = read_scroll();
                if *state != next {
                    state.set(next);
                }
            };
            let on_scroll = EventListener::new(&win, "scroll", update.clone());
            let on_resize = EventListener::new(&win, "resize", update);
            let unpin = |_: &web_sys::Event| PINNED.set(None);
            let on_wheel = EventListener::new(&win, "wheel", unpin);
            let on_touch = EventListener::new(&win, "touchstart", unpin);
            move || drop((on_scroll, on_resize, on_wheel, on_touch))
        });
    }
    (*state).clone()
}

/// The hero ("top") and then each section, in page order: what h / l step through.
fn section_order() -> impl Iterator<Item = &'static str> {
    std::iter::once("top").chain(SECTIONS.iter().map(|s| s.0))
}

fn section_el(id: &str) -> Option<Element> {
    let doc = web_sys::window()?.document()?;
    if id == "top" {
        doc.query_selector(".hero").ok().flatten()
    } else {
        doc.get_element_by_id(id)
    }
}

/// A jump (1-4, h/l, a tab) started a moment ago, so the page may still be scrolling.
fn jumping() -> bool {
    PINNED.get().is_some() && js_sys::Date::now() - PINNED_AT.get() < 1500.0
}

/// The section h / l step from: the one last jumped to, if that was a moment ago (the page
/// may still be scrolling there) or it's on screen; else the one scrolled to.
fn current_section() -> &'static str {
    let vh = web_sys::window()
        .and_then(|w| w.inner_height().ok())
        .and_then(|v| v.as_f64())
        .unwrap_or(800.0);
    let fresh = jumping();
    PINNED
        .get()
        .filter(|id| {
            fresh
                || section_el(id).is_some_and(|el| {
                    let r = el.get_bounding_client_rect();
                    r.top() < vh && r.bottom() > 48.0
                })
        })
        .unwrap_or_else(|| read_scroll().active)
}

fn go_to(id: &'static str) {
    pin(Some(id));
    if id == "top" {
        if let Some(win) = web_sys::window() {
            win.scroll_to_with_x_and_y(0.0, 0.0);
        }
    } else if let Some(el) = section_el(id) {
        el.scroll_into_view();
    }
}

/// Step `by` sections from the current one, stopping at either end.
fn step_section(by: isize) {
    let order: Vec<&'static str> = section_order().collect();
    let here = order
        .iter()
        .position(|&id| id == current_section())
        .unwrap_or(0);
    let to = (here as isize + by).clamp(0, order.len() as isize - 1) as usize;
    if to != here {
        go_to(order[to]);
    }
}

#[hook]
fn use_keyboard_nav(
    theme: UseReducerDispatcher<ThemeState>,
    lang: UseReducerDispatcher<LangState>,
    editor: Editor,
) {
    use_effect_with((), move |_| {
        let win = web_sys::window().expect("window");
        let options = EventListenerOptions::enable_prevent_default();
        let listener =
            EventListener::new_with_options(&win.clone(), "keydown", options, move |e| {
                let Some(e) = e.dyn_ref::<KeyboardEvent>() else {
                    return;
                };
                // insert mode: keys belong to the text being edited, bar esc
                if editor.is_editing() {
                    if e.key() == "Escape" {
                        editor.normal();
                        e.prevent_default();
                    }
                    return;
                }
                if e.ctrl_key() || e.meta_key() || e.alt_key() {
                    return;
                }
                match e.key().as_str() {
                    "j" => {
                        PINNED.set(None);
                        win.scroll_by_with_x_and_y(0.0, 160.0);
                    }
                    "k" => {
                        PINNED.set(None);
                        win.scroll_by_with_x_and_y(0.0, -160.0);
                    }
                    "g" => {
                        PINNED.set(None);
                        win.scroll_to_with_x_and_y(0.0, 0.0);
                    }
                    "G" => {
                        PINNED.set(None);
                        win.scroll_to_with_x_and_y(0.0, 1e9);
                    }
                    "h" => step_section(-1),
                    "l" => step_section(1),
                    "p" => switch_lang(&lang, LangAction::Toggle),
                    "n" => theme.dispatch(ThemeAction::Next),
                    "N" => theme.dispatch(ThemeAction::Prev),
                    // prevented, so the "i" itself doesn't land in the text
                    "i" => editor.insert(current_section()),
                    "u" => editor.undo(current_section()),
                    key => match SECTIONS.iter().find(|s| s.1 == key) {
                        Some((id, _, _)) => go_to(id),
                        None => return,
                    },
                }
                e.prevent_default();
            });
        move || drop(listener)
    });
}

fn main() {
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;

#[derive(Debug)]
pub struct Input {
    disable: bool,
    value: String,
    label: Option<String>,
    props: InputProperties,
    link: ComponentLink<Self>
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Properties)]
pub struct InputProperties {
    disabled: Option<bool>,
    label: Option<String>,
    initial_value: Option<String>,
    placeholder: Option<String>
}

#[derive(Clone, Debug)]
pub enum Update {
    Disable,
    Input(String)
}

impl Component for Input {
    type Message = Update;
    type Properties = InputProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            disable: props.disabled.unwrap_or(false),
            value: props.initial_value.clone().unwrap_or_else(|| "".to_string()),
            label: props.label.clone(),
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Update::*;

        match msg {
            Input(value) => {
                self.value = value;
                false
            },
            Disable => {
                self.disable = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let input_element = html!{
            <input
              placeholder="First name"
              value=self.value.clone()
              oninput=self.link.callback(|e: InputData| Update::Input(e.value))
              disabled=self.disable
            />
        };
        match self.label {
            Some(ref label) => html! {
                <label>{label.clone()} {input_element}</label>
            },
            None => input_element
        }
    }
}

impl Input {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn props(&self) -> &InputProperties {
        &self.props
    }
}

impl InputProperties {
    pub fn new(
        disabled: Option<bool>,
        label: Option<String>,
        initial_value: Option<String>,
        placeholder: Option<String>
    ) -> Self {
        Self { disabled, label, initial_value, placeholder }
    }

    pub fn initial_value(&self) -> &Option<String> {
        &self.initial_value
    }

    pub fn placeholder(&self) -> &Option<String> {
        &self.placeholder
    }
}
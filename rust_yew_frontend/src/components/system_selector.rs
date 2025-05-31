use yew::prelude::*;

// Define the properties for the SystemSelector component (if any needed later)
#[derive(Properties, PartialEq, Clone)]
pub struct SystemSelectorProps {
    pub on_system_change: Callback<i32>, // Callback to notify parent of selection change
}

// Helper function to get the term for a given number
fn get_term_for_number(num: i32) -> String {
    match num {
        1 => "Monad".to_string(),
        2 => "Dyad".to_string(),
        3 => "Triad".to_string(),
        4 => "Tetrad".to_string(),
        5 => "Pentad".to_string(),
        6 => "Hexad".to_string(),
        7 => "Heptad".to_string(),
        8 => "Octad".to_string(),
        // 9 => "Ennead".to_string(),
        // 10 => "Decad".to_string(),
        // 11 => "Undecad".to_string(),
        // 12 => "Dodecad".to_string(),
        _ => format!("{}-ad", num), // Fallback for other numbers
    }
}

#[function_component(SystemSelector)]
pub fn system_selector(props: &SystemSelectorProps) -> Html {
    let terms_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Commented out 9, 10, 11, 12

    // State to hold the currently selected term number (as a string, because HTML option values are strings)
    let selected_term_number_str = use_state(|| terms_numbers[0].to_string());

    let on_select_change = {
        let selected_term_number_str = selected_term_number_str.clone();
        let on_system_change = props.on_system_change.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>();
            if let Some(select) = target {
                let value_str = select.value();
                selected_term_number_str.set(value_str.clone());
                if let Ok(value_num) = value_str.parse::<i32>() {
                    on_system_change.emit(value_num);
                }
            }
        })
    };

    html! {
        <div class="system-selector-container">
            <label for="term-select">{"System:"}</label>
            <select id="term-select" onchange={on_select_change}>
                { for terms_numbers.iter().map(|term_num| {
                    let term_text = get_term_for_number(*term_num);
                    let display_text = format!("{} ({})", term_text, term_num);
                    html! {
                        <option value={term_num.to_string()} selected={&*selected_term_number_str == &term_num.to_string()}>
                            { display_text }
                        </option>
                    }
                }) }
            </select>
        </div>
    }
} 
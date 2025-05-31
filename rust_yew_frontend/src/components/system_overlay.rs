use yew::{html, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub system_num: i32,
}

pub struct SystemOverlay;

impl Component for SystemOverlay {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let system_num = ctx.props().system_num;
        
        match system_num {
            1 => self.render_monad(),
            2 => self.render_dyad(),
            3 => self.render_triad(),
            4 => self.render_tetrad(),
            5 => self.render_pentad(),
            6 => self.render_hexad(),
            7 => self.render_heptad(),
            8 => self.render_octad(),
            _ => html! { <div></div> },
        }
    }
}

impl SystemOverlay {
    fn render_monad(&self) -> Html {
        html! {
            <div class="system-overlay">
                <div class="point-container" style="top: 50%; left: 50%; transform: translate(-50%, -50%);">
                    <input class="point-input" placeholder="Instance" />
                </div>
            </div>
        }
    }

    fn render_dyad(&self) -> Html {
        html! {
            <div class="system-overlay">
                <div class="point-container" style="top: 50%; left: 34%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Essence"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                <div class="point-container" style="top: 50%; left: 66%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Existence"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
            </div>
        }
    }

    fn render_triad(&self) -> Html {
        html! {
            <div class="system-overlay">
                // Right point - Reconciling
                <div class="point-container" style="top: 50%; left: 63%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Reconciling"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Left top - Active
                <div class="point-container" style="top: 35%; left: 37%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Active"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Left bottom - Passive
                <div class="point-container" style="top: 65%; left: 37%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Passive"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
            </div>
        }
    }

    fn render_tetrad(&self) -> Html {
        html! {
            <div class="system-overlay">
                // Top center - Ideal
                <div class="point-container" style="top: 25%; left: 50%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Ideal"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Right center - Directive
                <div class="point-container" style="top: 50%; left: 67%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Directive"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Bottom center - Ground
                <div class="point-container" style="top: 75%; left: 50%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Ground"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Left center - Instrument
                <div class="point-container" style="top: 50%; left: 33%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Instrument"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
            </div>
        }
    }

    fn render_pentad(&self) -> Html {
        html! {
            <div class="system-overlay">
                // Left middle - shared tip point
                <div class="point-container" style="top: 50%; left: 32%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Intrinsic Limit"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Right short triangle - upper
                <div class="point-container" style="top: 31%; left: 47%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Inner Upper Limit"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Right short triangle - lower
                <div class="point-container" style="top: 69%; left: 47%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Inner Lower Limit"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Right long triangle - upper
                <div class="point-container" style="top: 27%; left: 62%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Outer Upper Limit"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Right long triangle - lower
                <div class="point-container" style="top: 73%; left: 62%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Outer Lower Limit"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
            </div>
        }
    }

    fn render_hexad(&self) -> Html {
        html! {
            <div class="system-overlay">
                // Top
                <div class="point-container" style="top: 22%; left: 50%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Values"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Upper right
                <div class="point-container" style="top: 35%; left: 65%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Options"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Lower right
                <div class="point-container" style="top: 65%; left: 65%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Criteria"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Bottom
                <div class="point-container" style="top: 78%; left: 50%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Facts"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Lower left
                <div class="point-container" style="top: 65%; left: 35%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Priorities"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Upper left
                <div class="point-container" style="top: 35%; left: 35%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Resources"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
            </div>
        }
    }

    fn render_heptad(&self) -> Html {
        html! {
            <div class="system-overlay">
                // Insight - top (correct position)
                <div class="point-container" style="top: 20%; left: 50%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Insight"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Research - right height but move left
                <div class="point-container" style="top: 35%; left: 67%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Research"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Value - align with delivery
                <div class="point-container" style="top: 35%; left: 33%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Value"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Design - higher and to the left
                <div class="point-container" style="top: 58%; left: 70%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Design"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Delivery - same height as design and move right
                <div class="point-container" style="top: 58%; left: 30%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Delivery"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Synthesis - right height, move right
                <div class="point-container" style="top: 80%; left: 60%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Synthesis"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Application - same height as synthesis, move down and right
                <div class="point-container" style="top: 80%; left: 40%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Application"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
            </div>
        }
    }

    fn render_octad(&self) -> Html {
        html! {
            <div class="system-overlay">
                // North - Intrinsic Nature
                <div class="point-container" style="top: 17%; left: 50%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Intrinsic Nature"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Northeast - Organisational Modes
                <div class="point-container" style="top: 28%; left: 67%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Organisational Modes"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // East - Smallest Significant Holon
                <div class="point-container" style="top: 50%; left: 72%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Smallest Significant Holon"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Southeast - Critical Functions
                <div class="point-container" style="top: 72%; left: 67%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Critical Functions"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // South - Supportive Platform
                <div class="point-container" style="top: 83%; left: 50%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Supportive Platform"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Southwest - Necessary Resourcing
                <div class="point-container" style="top: 72%; left: 33%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Necessary Resourcing"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // West - Integrative Totality
                <div class="point-container" style="top: 50%; left: 28%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Integrative Totality"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
                // Northwest - Inherent Values
                <div class="point-container" style="top: 28%; left: 33%; transform: translate(-50%, -50%);">
                    <div class="point-label">{"Inherent Values"}</div>
                    <input class="point-input" placeholder="Instance" />
                </div>
            </div>
        }
    }
} 
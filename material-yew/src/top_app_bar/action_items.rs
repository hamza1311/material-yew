use yew::prelude::*;

const SLOT: &str = "actionItems";

/// Props for [`MatTopAppBarActionItems`]
#[derive(Properties, PartialEq, Clone)]
pub struct TopAppBarActionItemsProps {
    pub children: Children,
}

/// Defines header for [`MatTopAppBar`][crate::MatTopAppBar] or
/// [`MatTopAppBarFixed`][crate::MatTopAppBarFixed].
///
/// If the child passed is an element (a `VTag`), then it is modified to include
/// the appropriate attributes. Otherwise, the child is wrapped in a `span`
/// containing said attributes.
pub struct MatTopAppBarActionItems {}

impl Component for MatTopAppBarActionItems {
    type Message = ();
    type Properties = TopAppBarActionItemsProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let children = props
            .children
            .iter()
            .map(|child| {
                match child {
                    Html::VTag(mut vtag) => {
                        vtag.add_attribute("slot", SLOT);
                        Html::VTag(vtag)
                    }
                    _ => {
                        html! {
                             <span slot={SLOT}>
                                 {child}
                             </span>
                        }
                    }
                }
            })
            .collect::<Html>();

        html! {
             {children}
        }
    }
}
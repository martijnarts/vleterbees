use leptos::*;
use ui::component::*;

use crate::Story;

pub const BUTTON_STORY: Story = Story {
    name: "Button",
    description: "Displays a button.", // or a component that looks like a button
    component: &story,
};

fn story() -> View {
    view! {
        <div class="flex flex-col gap-6">
            <Card>
                <CardContent class="pt-6 flex flex-row gap-2">
                    <Button>Default</Button>
                    <Button variant=ButtonVariant::Destructive>Destructive</Button>
                    <Button variant=ButtonVariant::Outline>Outline</Button>
                    <Button variant=ButtonVariant::Secondary>Secondary</Button>
                    <Button variant=ButtonVariant::Ghost>Ghost</Button>
                    <Button variant=ButtonVariant::Link>Link</Button>
                </CardContent>
            </Card>

            <Card>
                <CardContent class="pt-6 flex flex-row gap-2">
                    <Button size=ButtonSize::Sm>Small</Button>
                    <Button size=ButtonSize::Lg>Large</Button>
                    <Button size=ButtonSize::Icon>"🥺"</Button>
                </CardContent>
            </Card>
        </div>
    }
    .into_view()
}

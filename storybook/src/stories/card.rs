use leptos::*;
use ui::component::*;

use crate::Story;

pub const CARD_STORY: Story = Story {
    name: "Card",
    description: "Displays a card with header, content, and footer.",
    component: &card_story,
};

fn card_story() -> View {
    view! {
        <Card class="w-[380px]">
            <CardHeader>
                <CardTitle>Card Title</CardTitle>
                <CardDescription>Card Description</CardDescription>
            </CardHeader>
            <CardContent>
                <P>Card Content</P>
            </CardContent>
            <CardFooter>
                <P>Card Footer</P>
            </CardFooter>
        </Card>
    }
    .into_view()
}

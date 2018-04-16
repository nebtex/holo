/// render trait
pub trait RenderInto<C, R> {
    fn render(&self, from: R) -> C {}
    fn delta_render() -> C {}
}


pub struct Layout<ScreenSize> {}


#[holo_ux(renders(Column, Text, Word, Icon))]
#[spiral(table)]
#[derive(LookGodAs)]
pub enum Card {
    #[spiral(kind(Text))]
    Title,
    #[spiral(kind(Text))]
    Content,
}

#[spiral(reader(Card))]
pub struct CardReader {}



pub enum AspectRatios {
    XS(&'static [(f64)]),
    SM(&'static [(f64)]),
    L(&'static [(f64)]),
    XL(&'static [(f64)]),
}

impl RenderInto<Container> for Card {
    fn render(&self, card: CardReader, column: ColumnWriterSink) -> ColumnWriterStream {
        pub enum CardColumnProps {
            #[spiral(Text)]
            Title,
            #[spiral(Content)]
            Content,
        }
+++
        let stream = holo_ux! {
            <Container intents="card">
                <Row intents="header", aspect_ratios=(SM([&1, 1/5, /]))>
                    <Column aspect_ratio=12><Text>{ CardColumnProps::Title }</Text></Column>
                </Row>
                <Row intents="content", =(UNBOUDED, 2, 1)>
                    <Column aspect_ratio=12><Text>{ CardColumnProps::Content }</Text></Colomn>
                </Row>
            </Container>
        };
        map_stream!(stream, Card::Title=>ColumnTitle, Card::Content=>ColumnContent)
    }
}

impl LookGoodAs<Column> for Card {
    fn render(&self, card: CardReader, column: ColumnWriterSink) -> ColumnWriterStream {
        map_stream!(card => title, column -> dsdsm, ccc => asds, ddsd => sdasd)
    }

    fn delta_render(&self, card: CardReader) -> Layout<Xs> {}
}

impl LookGoodAs<Test> for Card {
    fn render(&self, card: CardReader, column: ColumnWriterSink) -> ColumnWriterStream {
        map_stream!(card => title, column -> dsdsm, ccc => asds, ddsd => sdasd)
    }

    fn delta_render(&self, card: CardReader) -> Layout<Xs> {}
}


impl LookGoodAs<Word> for Card {
    fn render(&self, card: CardReader, column: ColumnWriterSink) -> ColumnWriterStream {
        map_stream!(card => title, column -> dsdsm, ccc => asds, ddsd => sdasd)
    }

    fn delta_render(&self, card: CardReader) -> Layout<Xs> {}
}


fn main() {
    println!("Hello, world!");
}


/*

pub enum WithKind {
    Bounded(min:ad, max:max),
    Unbounded(),
}

pub enum HeighKind {
    Bounded(max:dd),
    Unbounded(),
}

pub struct ViewPort {
    width: WithKind,
    height: HeighKind,
    segustion: List<Sugestion>,
}*/
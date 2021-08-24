pub enum Side {
    TOP,
    BOTTOM,
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

// TODO:外部ファイルでブロックの一覧を宣言するようにしたい
#[derive(Clone, Copy)]
pub enum Block {
    GrassBlock,
}

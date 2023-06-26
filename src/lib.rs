#[derive(Debug)]
pub struct GameState<'a> {
    players: Vec<Player<'a>>,
    stock_tips: Vec<StockTip>,
    finance_news: Vec<FinanceNews>,
    obligations: Vec<Obligation>,
    shares: Vec<Share>,
    companies: Vec<Company>,
}

impl<'a> GameState<'a> {
    pub fn new(player_count: usize) -> Self {
        let mut players = Vec::with_capacity(player_count);
        for _ in 0..player_count {
            let p = Player::new();
            players.push(p);
        }

        let stock_tips = GameState::create_stock_tips();
        let finance_news = GameState::create_finance_news();
        let obligations = GameState::create_obligations();
        let shares = GameState::create_shares();
        let companies = GameState::create_companies();

        GameState {
            players,
            stock_tips,
            finance_news,
            obligations,
            shares,
            companies,
        }
    }

    // Read from another file storing the cards.
    // Probably parse JSON or YAML file.
    fn create_stock_tips() -> Vec<StockTip> {
        unimplemented!()
    }

    fn create_finance_news() -> Vec<FinanceNews> {
        unimplemented!()
    }

    fn create_obligations() -> Vec<Obligation> {
        unimplemented!()
    }

    fn create_shares() -> Vec<Share> {
        unimplemented!()
    }

    fn create_companies() -> Vec<Company> {
        unimplemented!()
    }

    pub fn take_turns(&mut self) {
        for player in &mut self.players {
            player.take_turn()
        }
    }

    pub fn take_stock_tips(&mut self, player_id: usize) {
        /*
        TODO: 
        - Slumpa fram ett stock tips
        - genomför dess effekt på spelare och game state
        - Gör att kortet inte kan dras igen
        */
        unimplemented!()
    }

    pub fn take_finance_news(&mut self, player_id: usize) {
        /*
        TODO: 
        - Slumpa fram ett finance news
        - Ersätt senast spelade finance news och genomför den nya effekten
        - Gör att kortet inte kan dras igen
        */
        unimplemented!()
    }
}

#[derive(Debug)]
struct Player<'a> {
    have_car: bool,
    rounds_in_jail: usize,
    position: BoardPos,
    money: usize,
    loan_count: usize,
    owned_companies: Vec<&'a Company>,
    owned_shares: Vec<&'a Share>,
}

impl<'a> Player<'a> {
    fn new() -> Self {
        Player {
            have_car: false,
            rounds_in_jail: 0,
            position: BoardPos::new(0),
            money: 1000, // TODO Set real start amount
            loan_count: 0,
            owned_companies: Vec::new(),
            owned_shares: Vec::new(),
        }
    }

    fn take_turn(&mut self) {
        // Might be easier to move logic to gamestate function instead.
        if self.rounds_in_jail != 0 {
            self.rounds_in_jail -= 1;
            return;
        }

        if self.position.in_bank() {
            //TODO Let self buy
            todo!()
        }

        //TODO implement random move
        if self.have_car {
            self.position.move_forward(7);
        } else {
            self.position.move_forward(3);
        }

        //TODO check what happens when you land on the new square
    }
}

#[derive(Debug)]
struct BoardPos {
    pos: usize,
    in_bank: bool,
}

impl BoardPos {
    pub fn new(pos: usize) -> Self {
        match pos {
            46.. => panic!("Position larger than exists"),
            34.. => BoardPos { pos, in_bank: true },
            _ => BoardPos {
                pos,
                in_bank: false,
            },
        }
    }

    pub fn move_forward(&mut self, modifier: usize) {
        if self.pos < 33 && self.pos + modifier > 33 {
            self.pos = 33;
        } else if self.pos + modifier > 45 {
            self.pos = 0;
            self.in_bank = false;
        } else {
            self.pos += modifier;
            if self.pos > 33 {
                self.in_bank = true;
            }
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn in_bank(&self) -> bool {
        self.in_bank
    }
}

// TODO implement these card types
#[derive(Debug)]
pub struct StockTip {}

#[derive(Debug)]
pub struct FinanceNews {}

#[derive(Debug)]
pub struct Share {}

#[derive(Debug)]
pub struct Obligation {}

#[derive(Debug)]
pub struct Company {}

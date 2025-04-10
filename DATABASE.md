# Collected Data
## Game
- *UNIQUE* **Site**: `URL`
- *NULLABLE* **TimeControl**: `(time: u16, increment: u8)`
- *NULLABLE* **Result**: `Enum(1-0, 0-1, 1/2-1/2)`
- **Termination**: `Enum(Normal, TimeForfeit, RulesInfraction, Abandoned, Unterminated)`
- **Event**: `String | String + Enum(Arena, Swiss) + URL`
- **Round**: `NULL`
- *NULLABLE* **Opening**: `String`
- *NULLABLE* **Eco**: `ECO (A-Z + 00-99)`
- *NULLABLE* **Date**: `Date`
- **UtcDate**: `Date`
- **UtcTime**: `Time`
- *NULLABLE* **White**: `String`
- *NULLABLE* **WhiteElo**: `u16`
- *NULLABLE* **WhiteRatingDiff**: `i8`
- *NULLABLE* **WhiteTitle**: `Enum(BOT, LM, GM, IM, FM, CM, NM, WGM, WIM, WFM, WCM, WNM)`
- *NULLABLE* **Black**: `String`
- *NULLABLE* **BlackElo**: `u16`
- *NULLABLE* **BlackRatingDiff**: `i8`
- *NULLABLE* **BlackTitle**: `Enum(BOT, LM, GM, IM, FM, CM, NM, WGM, WIM, WFM, WCM, WNM)`

## Move
- **SAN**: `SAN`
- *NULLABLE* **NAG**: `u8`
- *NULLABLE* **Eval**: `f32 | # + u8`
- *NULLABLE* **Clk**: `Time`

# Data we choose not to use
- ~~**Site**~~, since it's useless for us.
- ~~**Round**~~, since it's normally empty.
- ~~**RatingDiff**~~, since we don't think there will be much use of it.

# Entities
## Game
- *PK* **GameId**
- **TimeControl**
  - **StartTime**
  - **Increment**
- **End**
  - **Result**
  - **Termination**
- **DateTime**
  - **Date**
  - **Time**
- *DERIVED* **HasClock**
- *DERIVED* **HasEval**

## Move
- *PK* **Num**
- **Descriptor**
  - **Piece**
  - **StartPosition**
    - **StartRow**
    - **StartColumn**
  - **EatenPiece**
  - **EndPosition**
    - **EndRow**
    - **EndColumn**
  - **PromotionPiece**
  - **Castling**
  - **IsEnPassant**
  - **IsCheck**
  - **IsCheckMate**
  - **NAG**
- **Eval**
- **Clk**

## Opening
- *PK* **OpeningId**
- **Name**
- **ECO**

## Player
- *PK* **PlayerId**
- **Name**

## RuleSet
- *PK* **RuleSetId**
  
### GameMode
- **Name**

### Tournament
- **Name**
- **Kind**
- **URLId**

## FinalConfiguration
- *PK* **FinalConfigurationId**
- **Descriptor**
  - **EigthRow**
  - **SeventhRow**
  - **SixthRow**
  - **FifthRow**
  - **FourthRow**
  - **ThirdRow**
  - **SecondRow**
  - **FirstRow**
- *DERIVED* **EndPieces**
  - **EndWhitePieces**
    - **EndWhitePawns**
    - **EndWhiteBishops**
    - **EndWhiteKnights**
    - **EndWhiteRooks**
    - **EndWhiteQueens**
  - **EndBlackPieces**
    - **EndBlackPawns**
    - **EndBlackBishops**
    - **EndBlackKnights**
    - **EndBlackRooks**
    - **EndBlackQueens**

# Relationships

## Inheritance
**RuleSet** `(1-1)`
- **GameMode**
- **Tournament**

## Identifying / Weak
**Game** is comprised of **Move** `((0,1)-(1..N))`

## Simple
**Game** starts with **Opening** `((0..N)-(0,1))`

**Game** belongs to a **RuleSet** `((1..N)-(0,1))`

**Game** ends in a **FinalConfiguration** `((1..N)-(0,1))`

**Player** plays as white in **Game** `((0,1)-(0..N))`
- *Attributes*:
  - **Elo**
  - **Title**

**Player** plays as black in **Game** `((0,1)-(0..N))`
- *Attributes*:
  - **Elo**
  - **Title**

# Tables
## Game
- *PK* **GameId**: `Int`
- *FK* **RuleSetId**
- *NULLABLE FK* **OpeningId**
- *FK* **FCId**
- *NULLABLE FK* **White**
- *NULLABLE* **WhiteElo**: `UInt16`
- *NULLABLE* **WhiteTitle**: `Enum(BOT, LM, GM, IM, FM, CM, NM, WGM, WIM, WFM, WCM, WNM)`
- *NULLABLE FK* **Black**
- *NULLABLE* **BlackElo**: `UInt16`
- *NULLABLE* **BlackTitle**: `Enum(BOT, LM, GM, IM, FM, CM, NM, WGM, WIM, WFM, WCM, WNM)`
- *NULLABLE* **StartTime**: `UInt16`
- *NULLABLE* **Increment**: `UInt8`
- *NULLABLE* **Result**: `Enum(1-0, 0-1, 1/2-1/2)`
- **Termination**: `Enum(Normal, TimeForfeit, RulesInfraction, Abandoned, Unterminated)`
- **DateTime**: `DateTime`
- *DERIVED* **HasClock**: `Bool`
- *DERIVED* **HasEvaluations**: `Bool`

## Move
- *PK (GameId, Num, Color)*
- *FK* **GameId**
- **Num**: `UInt16`
- **Piece**: `Int(3)`
- **StartRow** `Int(3)`
- **StartColumn** `Int(3)`
- **EatenPiece** `Int(3)`
- **EndRow** `Int(3)`
- **EndColumn** `Int(3)`
- **PromotionPiece**  `Int(3)`
- **IsCheck**: `BIT`
- **IsCheckMate**: `BIT`
- **NAG**: `UInt8`
- *NULLABLE* **Eval**: `Union(Float, UInt8)`
- *NULLABLE* **Clk**: `Time`
- *DERIVED* **IsCheckmate**: `Bool`

## Opening
- *PK* **OpeningId**: `Int`
- *UNIQUE* **Opening**: `NChar(31)`
- **EcoLetter**: `Int(5)`
- **EcoNumber**: `Int(7)`

## Player
- *PK* **PlayerId**: `Int`
- *UNIQUE* **Name**: `NChar(31)`

## RuleSet
- *PK* **RuleSetId**: `Int`
  
### GameMode
- *PK FK* **RuleSetId**
- *UNIQUE* **Name**: `Nchar(31)`

### Tournament
- *PK FK* **RuleSetId**
- *UNIQUE (Name + URLId)*
- **Name**: `NChar(31)`?
- *NULLABLE* **Kind**: `Enum(Arena, Blitz)`
- **URLId**: `Char(8)`

## FinalConfiguration
- *PK* **FCId**: `Int`
- *UNIQUE (EigthRow, SeventhRow, SixthRow, FifthRow, FourthRow, ThirdRow, SecondRow, FirstRow)*
- **EigthRow**: `UInt32`
- **SeventhRow**: `UInt32`
- **SixthRow**: `UInt32`
- **FifthRow**: `UInt32`
- **FourthRow**: `UInt32`
- **ThirdRow**: `UInt32`
- **SecondRow**: `UInt32`
- **FirstRow**: `UInt32`
- **EndPieces**: `UInt32`

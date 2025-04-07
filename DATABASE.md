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
- *NULLABLE* **TimeControl**
  - **StartingCounter**
  - **Increment**
- *NULLABLE* **Result**
- **Termination**
- **DateTime**
- *DERIVED* **HasClock**
- *DERIVED* **HasEvaluations**

## Move
- *PK* **Num**
- *PK* **Color**
- *NULLABLE* **Nag**
- *NULLABLE* **Eval**
- *NULLABLE* **Clk**
- *DERIVED* **IsCheckmate**

## Opening
- *PK* **OpeningId**
- *UNIQUE* **Opening**
- **Eco**

## Player
- *PK* **PlayerId**
- *UNIQUE* **Name**

## RuleSet
- *PK* **RuleSetId**
  
### GameMode
- *UNIQUE* **Name**

### Tournament
- *UNIQUE (Name + URLId)*
- **Name**
- **Kind**
- **URLId**

## SAN
- *PK* **SanId**
- **FullSan**
  - **San**
  - *NULLABLE* **Suffix**

## FinalConfiguration
- *PK* **FCId**
- *UNIQUE* **Fen**
- *DERIVED* **EndPieces**
  - **EndWPawns**
  - **EndWBishops**
  - **EndWKnights**
  - **EndWRooks**
  - **EndWQueens**
  - **EndBPawns**
  - **EndBBishops**
  - **EndBKnights**
  - **EndBRooks**
  - **EndBQueens**

# Relationships

## Inheritance
**RuleSet** `(1-1)`
- **GameMode**
- **Tournament**

## Identifying / Weak
**Game** is comprised of **Move** `((0,1)-(1..N))`

## Simple
**Game** has **Opening**
- *Theoretical*: `((0..N)-(0,1))`
- *Practical*: `((0..N)-1)`

**Game** belongs to a **RuleSet**
- *Theoretical*: `((1..N)-(0,1))`
- *Practical*: `((1..N)-1)`

**Game** ends in a **FinalConfiguration**
- *Theoretical*: `((1..N)-(0,1))`
- *Practical*: `((1..N)-1)`

**Move** is described by **SAN**
- *Theoretical* `((1..N)-(0,1))`
- *Practical*: `((1..N)-1)`

**Player** plays as white in **Game**
- *Theoretical* `((0,1)-(0..N))`
- *Practical* `(1-(0..N))`
- *Attributes*:
  - **Elo**
  - *NULLABLE* **Title**

**Player** plays as black in **Game**
- *Theoretical* `((0,1)-(0..N))`
- *Practical* `(1-(0..N))`
- *Attributes*:
  - **Elo**
  - *NULLABLE* **Title**

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
- *NULLABLE* **StartingCounter**: `UInt16`
- *NULLABLE* **Increment**: `UInt8`
- *NULLABLE* **Result**: `Enum(1-0, 0-1, 1/2-1/2)`
- **Termination**: `Enum(Normal, TimeForfeit, RulesInfraction, Abandoned, Unterminated)`
- **DateTime**: `DateTime`
- *DERIVED* **HasClock**: `Bool`
- *DERIVED* **HasEvaluations**: `Bool`

## Move
- *PK (GameId, Num, Color)*
- *FK* **GameId**
- *FK* **SanId**
- **Num**: `UInt16`
- **Color**: `Enum(Black, White)`
- *NULLABLE* **Nag**: `UInt8`
- *NULLABLE* **Eval**: `Union(Float, UInt8)`
- *NULLABLE* **Clk**: `Time`
- *DERIVED* **IsCheckmate**: `Bool`

## Opening
- *PK* **OpeningId**: `Int`
- *UNIQUE* **Opening**: `NChar(31)`?
- **Eco**: `ECO` = `(A-Z + 00-99)` = `Char(3)`

## Player
- *PK* **PlayerId**: `Int`
- *UNIQUE* **Name**: `NChar(31)`?

## RuleSet
- *PK* **RuleSetId**: `Int`
  
### GameMode
- *PK FK* **RuleSetId**
- *UNIQUE* **Name**: `Nchar(31)`?

### Tournament
- *PK FK* **RuleSetId**
- *UNIQUE (Name + URLId)*
- **Name**: `NChar(31)`?
- *NULLABLE* **Kind**: `Enum(Arena, Blitz)`
- **URLId**: `Char(8)`

## SAN
- *PK* **SanId**: `Int`
- **San**: `NChar(7)`
- *NULLABLE* **Suffix**: `Enum(Check, Checkmate)`

## FinalConfiguration
- *PK* **FCId**: `Int`
- *UNIQUE* **Fen**: `Fen` = `NCHAR(92)`
- **EndPieces**: `UInt32`

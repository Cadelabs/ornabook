<div class="title">The Circle of Anguish 2.0</div>

Anguish 2.0 is the second iteration of Orna's Anguish Guild.
It spent multiple months in development and testing, with continuous feedback from the community, and started rolling out on May 27th 2025.
It addresses balance conerns and pain points of the original Anguish system.

The Circle of Anguish is a Passive Guild that aims at giving players an end-game challenge and combatting the power creep the Ascension mechanism introduced.
Pledging allegiance to the Circle of Anguish allows players to buff enemies and inflict themselves with negative battle modifiers in order to diversify and increase their battle rewards.

The Circle of Anguish has 4 different "Paths", each with its dedicated Proof currency and affecting a distinct subset of activities in Orna:

  - The Path of Despair: Affects encounters found in the overworld (both Monsters and Bosses) and Fishing encounters.
  - The Path of Melancholy: Affects encounters found in both [Battle](../Dungeons.md#battle-dungeons) and [Exploration](../Dungeons.md#exploration-dungeons) Dungeons.
  - The Path of Agony: Affects [World Raids](../Raids.md#world-raids) and [Super Raids](../Raids.md#super-raids).
  - The Path of Torment: Affects Towers of Olympia (including their Titan Boss at the top) and Monuments.

Each of these Paths is controlled independently: their levels, rewards, modifiers, ... can be adjusted for one content without affecting the other content.
This also means that increasing a Path's maximum level has no influence on other Path's levels

# Anguish Mechanism
## Path Level of Anguish
Each Path has a Level of Anguish assigned to it.
The higher the Level of Anguish, the harder the battles and the better the rewards.

Levels, aside from the first, have to be unlocked one by one (see [Requirements](#requirements) below).
Once unlocked, players may freely and at no cost adjust the Level of Anguish from 0 to the max Level they have unlocked.

### Requirements
Levelling a Path up requires the player to sacrifice [Proofs](#proofs) of the current Path, as well as having obtained [Anguished Gear](#anguished-gear) of at least the current Anguish Path Level.
The Anguished Gear is not sacrificed upon levelling a Path up.
Also, it is not required that it comes from the Path to be levelled up: an Anguished 2 Ljosalfar Sword obtained in a Dungeon (Path of Melancholy) can be used to level the Path of Despair up from level 2 to 3 (provided the player has sufficient :proof_despair:Proofs of Despair).
Note: The Anguished Gear has to be in your inventory (not your Keep).

The following table summarizes the amount of Proofs that have to be sacrified to level a Path up:

| Path Level | Proofs | Path Level | Proofs |
|:----------:|:------:|:----------:|:------:|
|     1      |     0  |            |        |
|     2      |   110  |            |        |
|     3      |   223  |            |        |
|     4      |   337  |            |        |


### Choices
Each Level of Anguish lets the player select between 4 Choices to alter their Paths.
These Choices are per-Path and only influence the content to which the selected Path applies.
Below are the Choices for the Path of Melancholy (Dungeons) at Level 1 (Choice 4 is selected):

<center>
    <div>
        <img alt="Screenshot of all 4 Choices" src="/img/melancholy_level_1_choices.png" height="300px"/>
    </div>
</center>

The [Modifiers](#modifiers) associated with selected Choices along a Path add up (they do not stack multiplicatively).
For instance, when the Anguish Level is set to 3, the Choices selected at Levels 1, 2 and 3 all add up to battles affected by the Path.

While Choices can be modified, they require the use of an [Anguished Pathspur](https://playorna.com/codex/items/anguished-pathspur/).
These can be bought from the [Guild Shop](#guild-shop) and are consumed upon use.
They reset all Choices made in the selected Path and let players make new Choices again.

While the selection amongst the 4 Choices for each Path Level is free for all players, the 4 Choices for a given Path and Level are the same across all players.
Should they want to, a player can copy another player's Choices and walk the exact same Path.

## Modifiers
Each [Choice](#choice) at each [Path Level](#path-level-of-anguish) comes with multiple modifiers:

  - Enemy stats: Each Level adds to the raw stats of enemy encountered. This modifier applies to the enemy's HP, Attack, Magic, Defence and Resistance. Contrary to Anguish 1.0, it does not apply to their Dexterity.
  - [Reward](#rewards) Modifiers: Each Level further increases the Rewards players get at the end of combats. They can be [Proofs](#proofs), [Anguished Gear](#anguished-gear) or boni to the XP, Gold, Orn, Luck or even Quality of items.
  - [Battle Modifiers](#list-of-battle-modifiers): Diverse negative effects applied to the player for each battle.

The Path of Agony (Raids) also has an extra modifier: [Raid Enraging](../Raids.md#fights).
This increses the maximum Enrage % of the Raid and rescales it according to the HP% dealt.
Furthermore, the Luck bonus for this Path also applies to Raid Rewards, unlike other Luck boni we know (Dowsing Rods, +Luck Gear, ...).

### List of Battle Modifiers
This list is currently incomplete.
Both the list and Range will be adjusted as data is gathered.

| Modifier                            | Range  | Notes                                                      |
|:------------------------------------|:------:|:-----------------------------------------------------------|
| Crit Chance                         |  -7-9% |                                                            |
| Crit Damage                         |  -7-9% |                                                            |
| Follower Act                        |    -2% |                                                            |
| Accuracy                            |    -1% |                                                            |
| Ward Absorption                     |    -5% |                                                            |
| Defend Power                        |   -15% |                                                            |
| Healing                             | -9-10% |                                                            |
| Status Effect Damage                |  +105% |                                                            |
| Status Protection                   |    -1% |                                                            |
| Damage from \[Family\]              |  +2-4% |                                                            |
| Permanent Status Effect Fade Chance |    +1% | Gives a 1% chance that a non-temporary status effect fades |
| \[Element\] Damage                  |  -2-5% |                                                            |
| 1HP Instead of Kill Chance          |    +1% | Gives a "Second Chance" with low chance on enemies         |

```admonish todo "TODO(ethiraric, 2025/05/31)"
Does Permanent Status Effect Fade Chance apply independently?
```

# Rewards
## Proofs
The Circle of Anguish has 4 different Proofs, one for each Path:

  - :proof_despair:[Proof of Despair](https://playorna.com/codex/items/proof-of-despair/) (World)
  - :proof_melancholy:[Proof of Melancholy](https://playorna.com/codex/items/proof-of-melancholy/) (Dungeons)
  - :proof_agony:[Proof of Agony](https://playorna.com/codex/items/proof-of-agony/) (Raids)
  - :proof_torment:[Proof of Torment](https://playorna.com/codex/items/proof-of-torment/) (Towers and Monuments)

Proofs are awarded when completing content for their given Path.
The odds of receiving a Proof is displayed at the Circle of Anguish Guild.

  - :proof_despair:Path of Despair: A Proof may drop upon killing a Monster, Boss or Fishing Encounter of the player's Tier on the overworld.
  - :proof_melancholy:Path of Melancholy: A Proof may drop upon killing a Monster or Boss of the player's Tier in a Battle Dungeon or an Exploration Dungeon.
  - :proof_agony:Path of Agony: A Proof is **guaranteed** to drop upon winning a battle against a Raid. More Proofs may drop in the Raid Rewards screen (1 to at least 4). These Proofs are not locked behind any Tier restriction (e.g.: a T10 player can get Proofs from a T6 Raid).
  - :proof_torment:Path of Torment: A Proof may drop upon killing a Monster or Boss of the player's Tier in a Tower or a Monument.

In Horde content, the Proof drop rolls for each eligible enemy.
In the last floor of a Horde Battle Dungeon, it is possible to drop up to 5 :proof_melancholy:Proofs of Melancholy.

```admonish todo "TODO(ethiraric, 2025/05/31)"
What dictates whether and how many Proofs of Agony drop?
```

## Anguished Gear
Anguished Gear are upgraded versions of items intended to be used in Anguish content (Path Level >= 1).
Any Ornate Gear (aside from Accessories and Adornments) drop has a chance to be granted an "Anguished Level".
That Level depends on the Anguish Level of the battle that dropped the item.
The item Anguished Level is at worst 2 Levels lower than the current Path Anguish Level (e.g.: at Path Anguish Level 8, Anguished 6, 7 or 8 Gear can be found).

Anguished pieces of Gear has slightly higher stats than a non-Anguished pieces of the same quality.
They also have an added modifier that further enhances it.

The increase to stats and modifiers applied to Anguished Gear scales with the Anguished Level of the item.
However, if an Anguished item is equipped in a lower Anguished Level, the extra stats and modifiers are scaled down.

```admonish todo "TODO(ethiraric, 2025/05/31)"
Ask for the list from Sirith

Formula for Anguished scaling down
```

# Guild Shop
```admonish todo "TODO(ethiraric, 2025/05/31)"
Pathspur, Crucible, Demonworking Tools

Scaling cost of items wrt path level.

Do the currencies rotate? They appear to
```

# Party play
Most of the Anguish content works as one would expect.
Here is a breakdown of known interactions for each Path:

  - :proof_despair:Path of Despair (World)
    - ?
  - :proof_melancholy:Path of Melancholy (Dungeons)
    - The Anguish Level is set to that of the player with the *lowest* active Anguish Level (not max unlocked Level).
    - The Choices and Battle Modifiers that apply are those of the leader.
    - :proof_melancholy:Proofs of Melancholy are rolled individually.
  - :proof_agony:Path of Agony (Raids)
    - The Anguish Level, Choices and Battle Modifiers that apply are those of the Raid summoner.
    - Every member in the battle in which the Raid was killed gets a guaranteed :proof_agony:Proof of Agony.
    - Every player who damaged the Raid is eligible for :proof_agony:Proofs of Agony on the Rewards screen.
  - :proof_torment: Path of Torment (Towers, Monuments)
    - No party play.

```admonish todo "TODO(ethiraric, 2025/05/31)"
How does it work for the Path of Despair?

What dictates whether and how many Proofs of Agony drop? wrt damage dealt

Can someone get Anguished Gear further than their max unlocked Anguish?
```

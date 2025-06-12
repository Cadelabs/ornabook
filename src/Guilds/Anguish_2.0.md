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
|     1      |     0  |    26      |        |
|     2      |   110  |    27      |        |
|     3      |   223  |    28      |        |
|     4      |   337  |    29      |        |
|     5      |   451  |    30      |        |


### Choices
#### Mechanics
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

#### List of choices
The following tables are large and collapsed by default.

##### :proof_despair: Despair
<details>

<table id="ang-despair-choices"></table>

</details>

##### :proof_melancholy: Melancholy
<details>

<table id="ang-melancholy-choices"></table>

</details>

##### :proof_agony: Agony
<details>

<table id="ang-agony-choices"></table>

</details>

##### :proof_torment: Torment
<details>

<table id="ang-torment-choices"></table>

</details>

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
See [above](#list-of-choices) the Choices that were recorded for each path.

| Modifier                            | Range  | Notes                                                       |
|:------------------------------------|:------:|:------------------------------------------------------------|
| Crit Chance                         |  -7-9% |                                                             |
| Crit Damage                         |  -7-9% |                                                             |
| Follower Act                        |    -2% |                                                             |
| Accuracy                            |    -1% |                                                             |
| Ward Absorption                     |    -5% |                                                             |
| Defend Power                        |   -15% |                                                             |
| Healing                             | -9-10% |                                                             |
| Status Effect Damage                |  +105% |                                                             |
| Status Protection                   |    -1% |                                                             |
| Damage from \[Family\]              |  +2-4% |                                                             |
| Permanent Status Effect Fade Chance |    +1% | Gives a 1% chance that a non-temporary status effect fades* |
| \[Element\] Damage                  |  -2-5% |                                                             |
| 1HP Instead of Kill Chance          |    +1% | Gives a "Second Chance" with low chance on enemies          |

```admonish note "On Permanent Status Effect Fade Chance"
Each turn and for each status effect (independently), the game rolls whether the status effect fades.
This implies that it is possible for multiple status effects to fade on a single turn.

This chance also rolls on negative status effects such as :statuses/cursed:Cursed.
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
  - :proof_agony:Path of Agony: A Proof is **guaranteed** to drop upon winning a battle against a Raid.
    More Proofs may drop in the Raid Rewards screen.
    These Proofs are not locked behind any Tier restriction (e.g.: a T10 player can get Proofs from a T6 Raid).
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
They also have an added affix or extra stat that further enhances it.
The stats are increased by 3% per Anguished Level on the item.
This increase applies to base stats and to Follower / Summon stats.

\\[ \text{anguished-stat} = \text{base-stat} * (1 + 0.03 * \text{gear-anguished-level}) \\]

However, if an Anguished item is equipped in a lower Anguished Level, the extra stats are scaled down of the current Anguish Level.
For instance, an Anguished Level 5 item equipped in an Anguish Level 2 content will only have 6% extra stats, as opposed to the 15% it would have at Anguish Level 5.
The extra stats can be expressed as

\\[ \text{anguished-stat} = \text{base-stat} * (1 + 0.03 * \text{min}(\text{gear-anguished-level}, \text{anguish-level})) \\]

### List of Anguished Gear Affixes & Extra stats

| Bonus                | Min  | Max  | :bw_weapon: | :bw_shield: | :bw_helmet: | :bw_armor: | :bw_legs: |
|:---------------------|:----:|:----:|:-----------:|:-----------:|:-----------:|:----------:|:---------:|
| HP (head)            | 200  | 300  |             |             | x           |            |           |
| HP (torso)           | 500  | 1000 |             |             |             | x          |           |
| Mana                 | 200  | 300  |             |             | x           |            |           |
| Dexterity            | 100  | 150  | x           | x           | x           | x          | x         |
| Foresight            | 50   | 100  |             |             | x           |            |           |
| Crit Chance          | 5%   | 10%  | x           |             |             |            |           |
| Crit Damage          | 4%   | 10%  | x           |             |             |            |           |
| Accuracy (head)      | 4%   | 7%   |             |             | x           |            |           |
| Accuracy (torso)     | 1%   | 2%   |             |             |             | x          |           |
| Accuracy (legs)      | 2%   | 4%   |             |             |             |            | x         |
| Pet Act              | 3%   | 7%   |             |             | x           |            |           |
| Faction Damage       | 5%   | 10%  | x           |             |             |            |           |
| Damage Limit Break   | 50%  | 100% | x           |             |             |            |           |
| Spell Casting Damage | -12% | -19% |             | x           |             |            |           |
| Def/Res Penetration  | 5%   | 10%  | x           |             |             |            |           |
| Two-Handed Power     | 8%   | 15%  |             |             |             |            | x         |
| Mana Reduction       | 5%   | 9%   |             |             |             | x          |           |
| Mana Recovery        | -96% | -99% |             |             |             |            | x         |
| Life Siphon          | 1%   | 2%   | x           |             |             |            |           |
| Ward Recovery        | 1%   | 3%   |             | x           |             | x          | x         |
| Ward Start Turns     | 1    | 3    |             |             |             |            | x         |
| Defend Power         | 10%  | 20%  |             | x           |             |            |           |
| Ult Defense          | 13%  | 30%  |             | x           |             |            |           |
| Status Protection    | 3%   | 7%   |             |             |             | x          |           |
| Buff Duration        | 2%   | 5%   |             | x           |             |            |           |
| Godforge Chance      | 1%   | 1%   |             |             |             | x          |           |

<center>Data compiled by Sirith.</center>

# Guild Shop
## Items
Outside of cosmetics, consumables and materials, the Circle of Anguish Guild Shop sells items affecting Anguish play: the :anguished_pathspur:[Anguished Pathspur](https://playorna.com/codex/items/anguished-pathspur/), :anguished_crucible:[Anguished Crucible](https://playorna.com/codex/items/anguished-crucible/) and :demonworking_tools:[Demonworking Tools](https://playorna.com/codex/items/demonworking-tools/).

The :anguished_pathspur:Anguished Pathspur allows one to reset the [Choices](#choices) that were made for a Path of Anguish.
They reset all levels of a single Path.
They can be used from the "View Path" menu of the Circle of Anguish.

The :anguished_crucible:Anguished Crucible allows you to (re)roll the bonus given to an Anguished Gear.
If the piece of Gear did not have a bonus, then the Crucible will grant it one.
The new bonus cannot be the same as the current one.
However, using another Crucible may give a previous bonus, even with a different value.

The :demonworking_tools:Demonworking Tools allows one to increase the Anguished Level of a piece of equipment.
The piece on which to apply the Demonworking Tools must at least be Masterforged.
Upon using, the Tools will increase the piece's Anguished Level by 1 and reset its level to 10.
In order to use another Tools on it, the item has to be at least Masterforged.
If used on a non-Anguished piece, the Tools will set it to Anguished Level 1 without a bonus.
An :anguished_crucible:Anguished Crucible will have to be used to assign one.

## Cost of Anguish items
The currency for each item (:anguished_pathspur::anguished_crucible::demonworking_tools:) is set daily, being Proofs of any of the 4 Paths.
Every player will see items priced with the same currencies on a given (localized) day.
The cost of the items depends on the Path Level of the currency used: the higher the Path Level, the more Proofs the item costs.

\\[ \text{item-cost} = \lceil \text{base-price} * (1 + 0.012 * \text{proof-drop-%}) \rceil \\]

where `proof-drop-%` is inserted as the percentage value rather than a probability (20 rather than 0.2 for 20% drop chance).

The base cost for each item is:
  - :anguished_pathspur:Anguished Pathspur: 20
  - :anguished_crucible:Anguished Crucible: 40
  - :demonworking_tools:Demonworking Tools: 50

## Materials
The Circle of Anguish sells 8 materials each day, 2 for each Proof.
Every player will the same materials for the same cost (amount and currency) on a given (localized) day.
Their pricing is the same as Anguish 1.0, but using the new proofs rather than :proof_anguish:Proofs of Anguish.

The cost of a pile of materials is not dependent on the Path Level, but on the Tier and Rarity of the material:

\\[ \text{materials-cost} = \lfloor \text{quantity} * (\text{material-tier} * 0.1 + \text{rarity-cost}) \rfloor \\]

where

\\[ \text{rarity} = \begin{cases} \text{Common} \Rightarrow 0 \\\\
                          \text{Superior} \Rightarrow 0.05 \\\\
                          \text{Famed} \Rightarrow 0.1 \\\\
                          \text{Legendary} \Rightarrow 0.15
                    \end{cases} \\]

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
    - The Anguish Level that applies is that of the Raid summoner at the time of summon.
    - The Choices and Battle Modifiers that apply are those of the battle Leader.
    - A player cannot start a raid above its max Agony Level.
    - A player can *join* a raid up to its max Path Level (the max of all 4 paths).
    - Every member in the battle in which the Raid was killed gets a guaranteed :proof_agony:Proof of Agony.
    - Every player who damaged the Raid is eligible for :proof_agony:Proofs of Agony on the Rewards screen.
  - :proof_torment: Path of Torment (Towers, Monuments)
    - No party play.

```admonish todo "TODO(ethiraric, 2025/05/31)"
How does it work for the Path of Despair?

What dictates whether and how many Proofs of Agony drop? wrt damage dealt

Can someone get Anguished Gear further than their max unlocked Anguish?
```
